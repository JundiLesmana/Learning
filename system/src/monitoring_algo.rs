#[derive(Clone, Debug)]
pub struct MetricCfg {
    pub key: &'static str,
    pub label: &'static str,
    pub unit: &'static str,
    /// true = higher is worse (cpu, latency, ...); false = lower is worse (uptime).
    pub dir_up: bool,
    pub warn: f64,
    pub crit: f64,
    pub weight: f64,
}

/// Default metric configuration (thresholds editable in the UI).
pub fn default_metrics() -> Vec<MetricCfg> {
    vec![
        MetricCfg { key: "latency", label: "Latency", unit: "ms", dir_up: true, warn: 100.0, crit: 200.0, weight: 20.0 },
        MetricCfg { key: "packetLoss", label: "Packet Loss", unit: "%", dir_up: true, warn: 1.0, crit: 5.0, weight: 20.0 },
        MetricCfg { key: "cpu", label: "CPU", unit: "%", dir_up: true, warn: 70.0, crit: 90.0, weight: 15.0 },
        MetricCfg { key: "bandwidth", label: "Bandwidth", unit: "%", dir_up: true, warn: 70.0, crit: 90.0, weight: 15.0 },
        MetricCfg { key: "memori", label: "Memori", unit: "%", dir_up: true, warn: 80.0, crit: 95.0, weight: 10.0 },
        MetricCfg { key: "uptime", label: "Uptime", unit: "%", dir_up: false, warn: 99.9, crit: 99.0, weight: 10.0 },
        MetricCfg { key: "errorInterface", label: "Error Interface", unit: "", dir_up: true, warn: 10.0, crit: 50.0, weight: 5.0 },
        MetricCfg { key: "koneksiAktif", label: "Koneksi Aktif", unit: "", dir_up: true, warn: 800.0, crit: 1200.0, weight: 5.0 },
    ]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Normal,
    Waspada,
    Kritis,
    /// metric not provided for this device -> ignored, never lowers score.
    None,
}

impl Status {
    pub fn severity(&self) -> u8 {
        match self {
            Status::None | Status::Normal => 0,
            Status::Waspada => 1,
            Status::Kritis => 2,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::Normal => "normal",
            Status::Waspada => "waspada",
            Status::Kritis => "kritis",
            Status::None => "none",
        }
    }
}

/// Status of a single metric value given its config and direction.
pub fn metric_status(cfg: &MetricCfg, v: Option<f64>) -> Status {
    let v = match v {
        Some(x) => x,
        None => return Status::None,
    };
    if cfg.dir_up {
        if v >= cfg.crit {
            Status::Kritis
        } else if v >= cfg.warn {
            Status::Waspada
        } else {
            Status::Normal
        }
    } else if v <= cfg.crit {
        Status::Kritis
    } else if v <= cfg.warn {
        Status::Waspada
    } else {
        Status::Normal
    }
}

pub fn sub_score(cfg: &MetricCfg, v: f64) -> f64 {
    if cfg.dir_up {
        if v <= cfg.warn {
            100.0
        } else if v <= cfg.crit {
            100.0 - 50.0 * (v - cfg.warn) / (cfg.crit - cfg.warn)
        } else {
            let ext = cfg.crit * 1.5;
            if v >= ext {
                0.0
            } else {
                50.0 - 50.0 * (v - cfg.crit) / (ext - cfg.crit)
            }
        }
    } else if v >= cfg.warn {
        100.0
    } else if v >= cfg.crit {
        100.0 - 50.0 * (cfg.warn - v) / (cfg.warn - cfg.crit)
    } else {
        (50.0 - 50.0 * (cfg.crit - v) / cfg.crit).max(0.0)
    }
}

pub fn device_score(metrics: &[(MetricCfg, Option<f64>)]) -> f64 {
    let mut sum = 0.0;
    let mut wsum = 0.0;
    for (cfg, v) in metrics {
        if let Some(x) = v {
            sum += cfg.weight * sub_score(cfg, *x);
            wsum += cfg.weight;
        }
    }
    if wsum <= 0.0 {
        100.0
    } else {
        sum / wsum
    }
}

pub fn health_score(devices: &[(f64, f64)]) -> (f64, &'static str) {
    if devices.is_empty() {
        return (0.0, "Kritis");
    }
    let mut sum = 0.0;
    let mut wsum = 0.0;
    let mut worst = f64::MAX;
    for (w, s) in devices {
        sum += w * s;
        wsum += w;
        if *s < worst {
            worst = *s;
        }
    }
    let avg = if wsum > 0.0 { sum / wsum } else { 0.0 };
    let score = 0.7 * avg + 0.3 * worst;
    let label = if score >= 85.0 {
        "Sehat"
    } else if score >= 60.0 {
        "Waspada"
    } else {
        "Kritis"
    };
    (score, label)
}

pub fn baseline(series: &[f64], window: usize) -> (f64, f64) {
    if series.is_empty() {
        return (0.0, 0.0);
    }
    let n = window.min(series.len());
    let slice = &series[..n];
    let mean = slice.iter().sum::<f64>() / n as f64;
    let var = slice.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n as f64;
    (mean, var.sqrt())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnomalyKind {
    Statistik,
    Threshold,
    Keduanya,
}

#[derive(Clone, Debug)]
pub struct Anomaly {
    pub index: usize,
    pub kind: AnomalyKind,
}

pub fn detect_anomalies(series: &[f64], cfg: &MetricCfg, window: usize) -> Vec<Anomaly> {
    let (mean, std) = baseline(series, window);
    let mut out = Vec::new();
    for (i, v) in series.iter().enumerate() {
        let stat = std > 0.0 && (v - mean) / std > 2.0;
        let thr = metric_status(cfg, Some(*v)).severity() > 0;
        if stat || thr {
            let kind = if stat && thr {
                AnomalyKind::Keduanya
            } else if stat {
                AnomalyKind::Statistik
            } else {
                AnomalyKind::Threshold
            };
            out.push(Anomaly { index: i, kind });
        }
    }
    out
}

#[derive(Clone, Debug)]
pub struct Alert {
    pub time: usize,
    pub device: String,
    pub metric: String,
    pub value: f64,
    pub threshold: f64,
    pub level: Status,
}

pub fn build_alerts(
    devices: &[String],
    series: &std::collections::HashMap<(String, String), Vec<f64>>,
    cfgs: &[MetricCfg],
) -> Vec<Alert> {
    let mut alerts = Vec::new();
    for dev in devices {
        for cfg in cfgs {
            let key = (dev.clone(), cfg.key.to_string());
            let s = match series.get(&key) {
                Some(s) => s,
                None => continue,
            };
            let mut prev = Status::Normal;
            for (i, v) in s.iter().enumerate() {
                let cur = metric_status(cfg, Some(*v));
                if cur.severity() > prev.severity() {
                    let threshold = if cur == Status::Kritis { cfg.crit } else { cfg.warn };
                    alerts.push(Alert {
                        time: i,
                        device: dev.clone(),
                        metric: cfg.label.to_string(),
                        value: *v,
                        threshold,
                        level: cur,
                    });
                }
                prev = cur;
            }
        }
    }
    alerts.sort_by(|a, b| {
        b.level
            .severity()
            .cmp(&a.level.severity())
            .then(b.time.cmp(&a.time))
    });
    alerts
}

/// Mean-time-to-detect comparison. Returns (first_alert, user_report, faster_minutes).
/// `first_critical` is the first time any device is critical; user_report = that
/// + report_delay. If there is no critical point, returns None.
pub fn mttd(
    alerts: &[Alert],
    first_critical: Option<usize>,
    report_delay: f64,
) -> Option<(usize, f64, f64)> {
    let first_alert = alerts.iter().map(|a| a.time).min()?;
    let fc = first_critical?;
    let user_report = fc as f64 + report_delay;
    let faster = user_report - first_alert as f64;
    Some((first_alert, user_report, faster))
}

pub fn linear_regression(series: &[f64]) -> (f64, f64) {
    let n = series.len();
    if n < 2 {
        return (0.0, series.first().copied().unwrap_or(0.0));
    }
    let xs: Vec<f64> = (0..n).map(|i| i as f64).collect();
    let mx = xs.iter().sum::<f64>() / n as f64;
    let my = series.iter().sum::<f64>() / n as f64;
    let mut num = 0.0;
    let mut den = 0.0;
    for i in 0..n {
        num += (xs[i] - mx) * (series[i] - my);
        den += (xs[i] - mx).powi(2);
    }
    let slope = if den > 0.0 { num / den } else { 0.0 };
    let intercept = my - slope * mx;
    (slope, intercept)
}

pub fn capacity_estimate(series: &[f64], interval: f64) -> Option<f64> {
    if series.len() < 8 {
        return None;
    }
    let (slope, intercept) = linear_regression(series);
    if slope <= 0.0 {
        return None;
    }
    let x90 = (90.0 - intercept) / slope;
    let last = (series.len() - 1) as f64;
    if x90 > last {
        Some((x90 - last) * interval)
    } else {
        Some(0.0)
    }
}

pub fn availability(uptime_series: &[f64], interval: f64) -> (f64, f64) {
    if uptime_series.is_empty() {
        return (0.0, 0.0);
    }
    let avg = uptime_series.iter().sum::<f64>() / uptime_series.len() as f64;
    let total_minutes = (uptime_series.len() as f64) * interval;
    let downtime = (100.0 - avg) / 100.0 * total_minutes;
    (avg, downtime)
}

// Seeded RNG (mulberry32) + sample data generator
pub struct Rng(u32);
impl Rng {
    pub fn new(seed: u32) -> Self {
        Rng(seed)
    }
    pub fn next_f64(&mut self) -> f64 {
        self.0 = self.0.wrapping_add(0x6D2B79F5);
        let mut t = self.0;
        t = (t ^ (t >> 15)).wrapping_mul(t | 1);
        t ^= t.wrapping_add((t ^ (t >> 7)).wrapping_mul(t | 61));
        ((t ^ (t >> 14)) as f64) / 4294967296.0
    }
}

pub fn metrics_for_type(kind: &str) -> Vec<&'static str> {
    match kind {
        "pc" => vec!["cpu", "memori", "bandwidth", "latency", "packetLoss", "uptime", "koneksiAktif"],
        "router" | "switch" | "firewall" => vec![
            "cpu", "memori", "bandwidth", "latency", "packetLoss", "uptime", "errorInterface", "koneksiAktif",
        ],
        "server" | "database" => vec![
            "cpu", "memori", "bandwidth", "latency", "packetLoss", "uptime", "errorInterface", "koneksiAktif",
        ],
        "ap" => vec!["cpu", "memori", "bandwidth", "latency", "packetLoss", "uptime", "koneksiAktif"],
        "internet" => vec!["bandwidth", "latency", "packetLoss", "uptime"],
        "ids" | "cctv" => vec!["cpu", "memori", "uptime", "errorInterface"],
        _ => vec!["cpu", "memori", "bandwidth", "latency", "packetLoss", "uptime", "koneksiAktif"],
    }
}

fn hash(s: &str) -> u32 {
    let mut h: u32 = 2166136261;
    for b in s.bytes() {
        h ^= b as u32;
        h = h.wrapping_mul(16777619);
    }
    h
}

fn round2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

pub fn generate_sample(
    devices: &[(String, String)],
    seed: u32,
    points: usize,
) -> std::collections::HashMap<(String, String), Vec<f64>> {
    let cfgs = default_metrics();
    let mut out = std::collections::HashMap::new();
    let router_id = devices
        .iter()
        .find(|(_, k)| k == "router")
        .map(|(id, _)| id.clone());
    let incident_at = points / 2;
    for (d_idx, (id, kind)) in devices.iter().enumerate() {
        let keys = metrics_for_type(kind);
        for key in keys {
            let cfg = cfgs.iter().find(|c| c.key == key).unwrap();
            let mut rng = Rng::new(seed.wrapping_add((d_idx as u32) * 131).wrapping_add(hash(key)));
            let mut series = Vec::with_capacity(points);
            let base = if cfg.dir_up {
                cfg.warn * (0.35 + 0.2 * rng.next_f64())
            } else {
                cfg.warn - (100.0 - cfg.warn) * 0.1 * rng.next_f64()
            };
            for t in 0..points {
                let span = if cfg.dir_up { cfg.warn * 0.12 } else { 0.05 };
                let noise = (rng.next_f64() - 0.5) * span;
                let mut v = base + noise;
                if Some(id) == router_id.as_ref()
                    && (key == "latency" || key == "bandwidth")
                    && t >= incident_at
                    && t < incident_at + 3
                {
                    v = if key == "latency" { cfg.crit * 1.15 } else { cfg.crit * 1.05 };
                }
                if cfg.dir_up {
                    v = v.max(0.0);
                } else {
                    v = v.min(100.0).max(0.0);
                }
                series.push(round2(v));
            }
            out.insert((id.clone(), key.to_string()), series);
        }
    }
    out
}

// CSV parsing: `waktu,perangkat,metrik,nilai`
#[derive(Clone, Debug)]
pub struct CsvPoint {
    pub time: usize,
    pub device: String,
    pub metric: String,
    pub value: f64,
}

pub fn parse_csv(text: &str) -> (Vec<CsvPoint>, Vec<String>) {
    let cfgs = default_metrics();
    let mut points = Vec::new();
    let mut errors = Vec::new();
    for (li, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split(',').map(|c| c.trim()).collect();
        if cols.len() != 4 {
            errors.push(format!("Baris {}: harus 4 kolom (waktu,perangkat,metrik,nilai), ditemukan {}.", li + 1, cols.len()));
            continue;
        }
        let time = match cols[0].parse::<usize>() {
            Ok(t) => t,
            Err(_) => {
                errors.push(format!("Baris {}: waktu '{}' bukan bilangan bulat.", li + 1, cols[0]));
                continue;
            }
        };
        let metric = cols[2].to_string();
        if !cfgs.iter().any(|c| c.key == metric) {
            errors.push(format!("Baris {}: metrik '{}' tidak dikenal.", li + 1, metric));
            continue;
        }
        let value = match cols[3].parse::<f64>() {
            Ok(v) => v,
            Err(_) => {
                errors.push(format!("Baris {}: nilai '{}' bukan angka.", li + 1, cols[3]));
                continue;
            }
        };
        if value < 0.0 {
            errors.push(format!("Baris {}: nilai negatif tidak diizinkan.", li + 1));
            continue;
        }
        if (cols[2] == "cpu" || cols[2] == "memori" || cols[2] == "bandwidth" || cols[2] == "packetLoss" || cols[2] == "uptime") && value > 100.0 {
            errors.push(format!("Baris {}: nilai persen harus 0-100.", li + 1));
            continue;
        }
        points.push(CsvPoint { time, device: cols[1].to_string(), metric, value });
    }
    (points, errors)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn cfg(key: &str) -> MetricCfg {
        default_metrics().into_iter().find(|c| c.key == key).unwrap()
    }

    #[test]
    fn latency_above_critical_is_kritis() {
        let c = cfg("latency");
        assert_eq!(metric_status(&c, Some(250.0)), Status::Kritis);
        assert_eq!(metric_status(&c, Some(150.0)), Status::Waspada);
        assert_eq!(metric_status(&c, Some(50.0)), Status::Normal);
    }

    #[test]
    fn uptime_below_99_is_kritis_reverse_direction() {
        let c = cfg("uptime");
        assert_eq!(metric_status(&c, Some(98.0)), Status::Kritis);
        assert_eq!(metric_status(&c, Some(99.5)), Status::Waspada);
        assert_eq!(metric_status(&c, Some(100.0)), Status::Normal);
    }

    #[test]
    fn sub_score_boundaries() {
        let c = cfg("latency"); // warn 100, crit 200, extreme 300
        assert_eq!(sub_score(&c, 50.0), 100.0);
        assert_eq!(sub_score(&c, 100.0), 100.0);
        assert_eq!(sub_score(&c, 200.0), 50.0);
        assert_eq!(sub_score(&c, 300.0), 0.0);
        assert_eq!(sub_score(&c, 400.0), 0.0);
        // uptime: warn 99.9, crit 99, extreme 0
        let u = cfg("uptime");
        assert_eq!(sub_score(&u, 100.0), 100.0);
        assert_eq!(sub_score(&u, 99.0), 50.0);
        assert_eq!(sub_score(&u, 0.0), 0.0);
    }

    #[test]
    fn health_score_drops_when_worsened_and_recovers() {
        let good = vec![(1.0, 100.0), (1.0, 100.0)];
        let (sg, _) = health_score(&good);
        let bad = vec![(1.0, 100.0), (1.0, 10.0)];
        let (sb, _) = health_score(&bad);
        assert!(sb < sg, "health should drop when a device worsens");
        let fixed = vec![(1.0, 100.0), (1.0, 100.0)];
        let (sf, _) = health_score(&fixed);
        assert!(sf > sb, "health should recover when fixed");
    }

    #[test]
    fn anomaly_detected_on_spike_not_on_flat() {
        let c = cfg("latency");
        let flat = vec![10.0; 10];
        assert!(detect_anomalies(&flat, &c, 5).is_empty());
        let mut spike = vec![10.0; 10];
        spike[7] = 500.0;
        let a = detect_anomalies(&spike, &c, 5);
        assert!(a.iter().any(|x| x.index == 7));
    }

    #[test]
    fn alerts_no_consecutive_dup_and_sorted_by_severity() {
        let cfgs = default_metrics();
        let mut series: HashMap<(String, String), Vec<f64>> = HashMap::new();
        // latency: normal, waspada, waspada, kritis, kritis -> alerts at 1 (waspada) and 3 (kritis)
        series.insert(("r".into(), "latency".into()), vec![50.0, 150.0, 150.0, 250.0, 250.0]);
        let alerts = build_alerts(&["r".to_string()], &series, &cfgs);
        assert_eq!(alerts.len(), 2);
        assert_eq!(alerts[0].level, Status::Kritis);
        assert_eq!(alerts[0].time, 3);
        assert_eq!(alerts[1].level, Status::Waspada);
        assert_eq!(alerts[1].time, 1);
    }

    #[test]
    fn mttd_alert_earlier_by_report_delay_and_none_without_incident() {
        let cfgs = default_metrics();
        let mut series: HashMap<(String, String), Vec<f64>> = HashMap::new();
        series.insert(("r".into(), "latency".into()), vec![50.0, 150.0, 250.0]);
        let alerts = build_alerts(&["r".to_string()], &series, &cfgs);
        let r = mttd(&alerts, Some(2), 10.0).unwrap();
        assert_eq!(r.0, 1); // first alert
        assert_eq!(r.1, 12.0); // user report = 2 + 10
        assert_eq!(r.2, 11.0); // faster by 11 (12 - 1)
        let empty: Vec<Alert> = vec![];
        assert!(mttd(&empty, None, 10.0).is_none());
    }

    #[test]
    fn same_seed_same_sample() {
        let devices = vec![("r".to_string(), "router".to_string()), ("p".to_string(), "pc".to_string())];
        let a = generate_sample(&devices, 42, 20);
        let b = generate_sample(&devices, 42, 20);
        assert_eq!(a, b);
        let c = generate_sample(&devices, 43, 20);
        assert_ne!(a, c);
    }

    #[test]
    fn csv_rejects_bad_format() {
        let (pts, errs) = parse_csv("0,r,latency,10\n1,r,latency,abc\n2,r,unknown,5\n3,r,latency");
        assert_eq!(pts.len(), 1);
        assert!(!errs.is_empty());
        assert!(errs.iter().any(|e| e.contains("bukan angka")));
        assert!(errs.iter().any(|e| e.contains("tidak dikenal")));
        assert!(errs.iter().any(|e| e.contains("4 kolom")));
    }

    #[test]
    fn capacity_and_availability() {
        let up = vec![50.0, 55.0, 60.0, 65.0, 70.0, 75.0, 80.0, 85.0];
        assert!(capacity_estimate(&up, 1.0).is_some());
        let flat = vec![10.0; 10];
        assert!(capacity_estimate(&flat, 1.0).is_none());
        let (avg, down) = availability(&[99.0, 99.0], 1.0);
        assert!((avg - 99.0).abs() < 1e-9);
        assert!((down - 0.02).abs() < 1e-9);
    }
}