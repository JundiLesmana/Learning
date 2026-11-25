//! Pure topology logic for the "Jaringan Komputer" learning page.
//!
//! These functions are intentionally free of any UI or I/O so they can be
//! unit-tested in isolation. The interactive page re-implements the same logic
//! in JavaScript (the generated site is static HTML with no backend); this
//! module is the tested reference. Later stages (monitoring, defense in depth,
//! troubleshooting) will extend this file with their own pure analysis
//! functions and tests.

/// Maximum number of devices allowed in one topology.
pub const MAX_DEVICES: usize = 20;

/// A network device. `id` is a stable unique key, `name` is the display label
/// (must be unique case-insensitively), `kind` is one of the supported types.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub kind: String,
}

impl Device {
    pub fn new(id: &str, name: &str, kind: &str) -> Self {
        Device {
            id: id.to_string(),
            name: name.to_string(),
            kind: kind.to_string(),
        }
    }
}

/// An undirected link between two devices, referenced by device id.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Link {
    pub a: String,
    pub b: String,
}

impl Link {
    pub fn new(a: &str, b: &str) -> Self {
        Link {
            a: a.to_string(),
            b: b.to_string(),
        }
    }
}

/// Device status derived purely from reachability to the Internet.
/// `normal` = reachable (green), `mati` = unreachable (grey).
/// Modules 1-3 will refine this into `waspada`/`kritis` from live metrics.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Normal,
    Mati,
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::Normal => "normal",
            Status::Mati => "mati",
        }
    }
}

/// Validate a topology and return a list of human-readable error messages.
/// An empty list means the topology is valid.
pub fn validate(devices: &[Device], links: &[Link]) -> Vec<String> {
    let mut errors = Vec::new();

    if devices.len() > MAX_DEVICES {
        errors.push(format!(
            "Jumlah perangkat {} melebihi batas maksimum {}.",
            devices.len(),
            MAX_DEVICES
        ));
    }

    // Unique ids.
    for i in 0..devices.len() {
        for j in (i + 1)..devices.len() {
            if devices[i].id == devices[j].id {
                errors.push(format!("ID perangkat ganda: '{}'.", devices[i].id));
            }
        }
    }

    // Unique names (case-insensitive).
    for i in 0..devices.len() {
        for j in (i + 1)..devices.len() {
            if devices[i].name.eq_ignore_ascii_case(&devices[j].name) {
                errors.push(format!("Nama perangkat ganda: '{}'.", devices[i].name));
            }
        }
    }

    let known: std::collections::HashSet<&str> =
        devices.iter().map(|d| d.id.as_str()).collect();

    // Link checks: self-link, unknown endpoint, duplicate.
    let mut seen_links: std::collections::HashSet<(String, String)> =
        std::collections::HashSet::new();
    for l in links {
        if l.a == l.b {
            errors.push(format!("Link ke diri sendiri tidak diizinkan: '{}'.", l.a));
            continue;
        }
        if !known.contains(l.a.as_str()) || !known.contains(l.b.as_str()) {
            errors.push(format!(
                "Link '{}'-{}' memakai perangkat yang tidak dikenal.",
                l.a, l.b
            ));
            continue;
        }
        let key = if l.a <= l.b {
            (l.a.clone(), l.b.clone())
        } else {
            (l.b.clone(), l.a.clone())
        };
        if !seen_links.insert(key) {
            errors.push(format!("Link ganda: '{}-{}'.", l.a, l.b));
        }
    }

    // Must contain an Internet device.
    if !devices.iter().any(|d| d.kind == "internet") {
        errors.push("Topologi harus memiliki perangkat 'Internet'.".to_string());
    }

    // Every device must have a path to the Internet.
    let reachable = reachable_from_internet(devices, links);
    for d in devices {
        if !reachable.contains(&d.id) {
            errors.push(format!(
                "Perangkat '{}' tidak memiliki jalur ke Internet.",
                d.name
            ));
        }
    }

    errors
}

/// Return the set of device ids that have a path to the Internet device.
/// Uses BFS over the undirected link graph. Empty if there is no Internet.
pub fn reachable_from_internet(devices: &[Device], links: &[Link]) -> std::collections::HashSet<String> {
    let mut result = std::collections::HashSet::new();
    let internet: Vec<&str> = devices
        .iter()
        .filter(|d| d.kind == "internet")
        .map(|d| d.id.as_str())
        .collect();
    if internet.is_empty() {
        return result;
    }

    // Build adjacency.
    let mut adj: std::collections::HashMap<&str, Vec<&str>> = std::collections::HashMap::new();
    for l in links {
        adj.entry(l.a.as_str()).or_default().push(l.b.as_str());
        adj.entry(l.b.as_str()).or_default().push(l.a.as_str());
    }

    let mut stack: Vec<&str> = internet.clone();
    for id in &internet {
        result.insert(id.to_string());
    }
    while let Some(cur) = stack.pop() {
        if let Some(neigh) = adj.get(cur) {
            for n in neigh {
                if !result.contains(*n) {
                    result.insert(n.to_string());
                    stack.push(n);
                }
            }
        }
    }
    result
}

/// Health score 0-100 = percentage of devices reachable from the Internet.
/// 0 when there are no devices. Fully transparent: `reachable / total * 100`.
pub fn health_score(devices: &[Device], links: &[Link]) -> f64 {
    if devices.is_empty() {
        return 0.0;
    }
    let reachable = reachable_from_internet(devices, links);
    let n = devices.len() as f64;
    (reachable.len() as f64 / n) * 100.0
}

/// Status of a single device from reachability.
pub fn status_of(device_id: &str, devices: &[Device], links: &[Link]) -> Status {
    let reachable = reachable_from_internet(devices, links);
    if reachable.contains(device_id) {
        Status::Normal
    } else {
        Status::Mati
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kantor_kecil() -> (Vec<Device>, Vec<Link>) {
        let devices = vec![
            Device::new("inet", "Internet", "internet"),
            Device::new("fw", "Firewall", "firewall"),
            Device::new("r1", "Router-1", "router"),
            Device::new("sw1", "Switch-1", "switch"),
            Device::new("pc1", "PC-1", "pc"),
            Device::new("pc2", "PC-2", "pc"),
            Device::new("srv", "Server", "server"),
            Device::new("ap", "AP-WiFi", "ap"),
        ];
        let links = vec![
            Link::new("inet", "fw"),
            Link::new("fw", "r1"),
            Link::new("r1", "sw1"),
            Link::new("sw1", "pc1"),
            Link::new("sw1", "pc2"),
            Link::new("sw1", "srv"),
            Link::new("sw1", "ap"),
        ];
        (devices, links)
    }

    #[test]
    fn kantor_kecil_is_valid_and_fully_reachable() {
        let (d, l) = kantor_kecil();
        assert!(validate(&d, &l).is_empty());
        let reach = reachable_from_internet(&d, &l);
        assert_eq!(reach.len(), d.len());
        assert_eq!(health_score(&d, &l), 100.0);
    }

    #[test]
    fn self_link_is_rejected() {
        let (mut d, mut l) = kantor_kecil();
        l.push(Link::new("pc1", "pc1"));
        let errs = validate(&d, &l);
        assert!(errs.iter().any(|e| e.contains("diri sendiri")));
        d.clear();
        d.push(Device::new("inet", "Internet", "internet"));
    }

    #[test]
    fn duplicate_name_is_rejected() {
        let (mut d, l) = kantor_kecil();
        d.push(Device::new("pc3", "pc-1", "pc")); // same name, different case
        let errs = validate(&d, &l);
        assert!(errs.iter().any(|e| e.contains("Nama perangkat ganda")));
    }

    #[test]
    fn duplicate_link_is_rejected() {
        let (d, mut l) = kantor_kecil();
        l.push(Link::new("r1", "sw1")); // duplicate of existing
        let errs = validate(&d, &l);
        assert!(errs.iter().any(|e| e.contains("Link ganda")));
    }

    #[test]
    fn disconnected_device_is_flagged_and_lowers_score() {
        let (mut d, mut l) = kantor_kecil();
        d.push(Device::new("iso", "PC-Isolasi", "pc")); // no link
        let errs = validate(&d, &l);
        assert!(errs.iter().any(|e| e.contains("tidak memiliki jalur ke Internet")));
        // 8 of 9 reachable
        assert!((health_score(&d, &l) - (8.0 / 9.0) * 100.0).abs() < 1e-9);
        assert_eq!(status_of("iso", &d, &l), Status::Mati);
        assert_eq!(status_of("pc1", &d, &l), Status::Normal);
        l.clear();
    }

    #[test]
    fn missing_internet_is_rejected() {
        let (d, l) = kantor_kecil();
        let d2: Vec<Device> = d.into_iter().filter(|x| x.kind != "internet").collect();
        let errs = validate(&d2, &l);
        assert!(errs.iter().any(|e| e.contains("'Internet'")));
    }

    #[test]
    fn max_devices_enforced() {
        let mut d = vec![Device::new("inet", "Internet", "internet")];
        for i in 0..MAX_DEVICES {
            d.push(Device::new(&format!("d{}", i), &format!("Dev {}", i), "pc"));
        }
        let errs = validate(&d, &[]);
        assert!(errs.iter().any(|e| e.contains("melebihi batas maksimum")));
    }

    #[test]
    fn empty_topology_has_zero_score() {
        assert_eq!(health_score(&[], &[]), 0.0);
    }
}