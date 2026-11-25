// use axum::response::Html;

pub fn monitoring_html() -> String {
    r##"<div class="module-panel active" id="mod1" role="tabpanel">
      <div class="sim-card glass">
        <div class="sim-title">Modul 1 &mdash; Monitoring Jaringan</div>
        <div class="mon-question">
          <b>Soal:</b> Mengapa monitoring jaringan menjadi elemen penting dalam menjaga kinerja infrastruktur TI sebuah organisasi?
        </div>

        <div class="error-box" id="monErr" role="alert"></div>

        <div class="sim-controls">
          <div class="input-group small"><label for="monSeed">Seed</label><input class="input-field" id="monSeed" type="number" value="42" /></div>
          <div class="input-group small"><label for="monPoints">Titik data</label><input class="input-field" id="monPoints" type="number" min="10" max="30" value="20" /></div>
          <div class="input-group small"><label for="monInterval">Interval (menit/titik)</label><input class="input-field" id="monInterval" type="number" min="1" value="1" /></div>
          <div class="input-group small"><label for="monBaseWin">Jendela baseline</label><input class="input-field" id="monBaseWin" type="number" min="1" value="5" /></div>
          <div class="input-group small"><label for="monReportDelay">Keterlambatan laporan (menit)</label><input class="input-field" id="monReportDelay" type="number" min="0" value="10" /></div>
          <div class="input-group small"><label for="monAdminResp">Respon admin (menit)</label><input class="input-field" id="monAdminResp" type="number" min="0" value="5" /></div>
        </div>
        <div class="btn-row" style="margin-bottom:8px;">
          <button class="btn-run" id="monSample">Hasilkan data contoh</button>
          <button class="btn-run" id="monAnalyze">Analisis</button>
          <button class="btn-ghost" id="monTemplate">Unduh template CSV</button>
          <button class="btn-ghost" id="monExportJson">Ekspor JSON</button>
          <button class="btn-ghost" id="monExportCsv">Ekspor CSV</button>
          <button class="btn-reset" id="monReset">Reset</button>
        </div>

        <details style="margin-top:8px;">
          <summary style="cursor:pointer;font-size:12px;font-weight:700;color:var(--text-secondary);text-transform:uppercase;letter-spacing:0.05em;">Panel Threshold</summary>
          <div class="threshold-grid" id="monThresholdGrid"></div>
          <div class="btn-row" style="margin-top:10px;"><button class="btn-ghost" id="monThrDefault">Kembalikan default</button></div>
        </details>

        <div class="sim-title" style="margin-top:22px;">Data Metrik (titik waktu terpilih)</div>
        <div class="sim-controls" style="margin-bottom:6px;">
          <div class="input-group small"><label for="monTimeSel">Titik waktu</label><select class="input-field" id="monTimeSel"></select></div>
        </div>
        <div style="overflow-x:auto;">
          <table class="data-table" id="monDataTable"><thead><tr id="monDataHead"></tr></thead><tbody></tbody></table>
        </div>

        <div class="series-editor">
          <div class="se-row">
            <div class="input-group small"><label for="monSeriesDev">Perangkat</label><select class="input-field" id="monSeriesDev"></select></div>
            <div class="input-group small"><label for="monSeriesMetric">Metrik</label><select class="input-field" id="monSeriesMetric"></select></div>
          </div>
          <div class="input-group"><label for="monSeriesText">Deret waktu (pisahkan dengan koma)</label><textarea class="input-field" id="monSeriesText" spellcheck="false" placeholder="10, 12, 11, 13, ..."></textarea></div>
          <div class="btn-row"><button class="btn-run" id="monSeriesApply">Terapkan deret</button></div>
        </div>

        <div class="series-editor">
          <div class="input-group"><label for="monCsvText">Tempel CSV (waktu,perangkat,metrik,nilai)</label><textarea class="input-field" id="monCsvText" spellcheck="false" placeholder="0,Internet,latency,20&#10;1,Internet,latency,22"></textarea></div>
          <div class="btn-row"><button class="btn-run" id="monCsvApply">Impor CSV</button></div>
        </div>

        <div class="sim-title" style="margin-top:22px;">Timeline</div>
        <div class="btn-row">
          <button class="btn-run" id="monPlay">Play</button>
          <button class="btn-ghost" id="monStep">&larr; Step</button>
          <input type="range" id="monTimeline" min="0" max="0" value="0" style="flex:1;accent-color:var(--accent);" aria-label="Timeline" />
          <span id="monTimeLabel" style="font-size:12px;color:var(--text-muted);font-variant-numeric:tabular-nums;">t=0</span>
          <select class="input-field" id="monSpeed" style="flex:0 0 90px;"><option value="0.5">0.5x</option><option value="1" selected>1x</option><option value="2">2x</option></select>
        </div>

        <div class="sim-title" style="margin-top:22px;">Papan Skor</div>
        <div class="gauge-wrap">
          <svg id="monGauge" viewBox="0 0 120 70" width="180" height="105" role="img" aria-label="Gauge skor kesehatan jaringan"></svg>
          <div class="gauge-legend">
            <div>Skor kesehatan: <b id="monHealthVal">&ndash;</b> <span id="monHealthLabel"></span></div>
            <div>Normal: <b id="monCountNormal">0</b> &middot; Waspada: <b id="monCountWarn">0</b> &middot; Kritis: <b id="monCountCrit">0</b></div>
            <div>Rata-rata uptime: <b id="monAvgUptime">&ndash;</b></div>
            <div>Alert aktif: <b id="monAlertCount">0</b></div>
          </div>
        </div>
        <p class="hint" id="monFormula">Rumus skor: 0.7 &times; rata-rata berbobot skor perangkat + 0.3 &times; skor perangkat terburuk. Sub-skor: 100 di bawah warning, 50 di critical, 0 di batas ekstrem.</p>

        <div class="sim-title" style="margin-top:22px;">Grafik Metrik</div>
        <div class="sim-controls" style="margin-bottom:6px;">
          <div class="input-group small"><label for="monChartDev">Perangkat</label><select class="input-field" id="monChartDev"></select></div>
          <div class="input-group small"><label for="monChartMetric">Metrik</label><select class="input-field" id="monChartMetric"></select></div>
          <button class="btn-ghost" id="monChartTable">Tabel data</button>
        </div>
        <div class="chart-wrap"><svg class="mon-chart" id="monChart" viewBox="0 0 100 42" preserveAspectRatio="none" role="img" aria-label="Grafik metrik"></svg></div>
        <div id="monChartTableWrap" style="display:none;overflow-x:auto;margin-top:10px;"></div>
        <div class="legend" aria-hidden="true">
          <span><i style="border-top-color:var(--accent)"></i> Nilai metrik</span>
          <span><i style="border-top-color:var(--warning)"></i> Threshold warning</span>
          <span><i style="border-top-color:#ef4444"></i> Threshold critical</span>
          <span><i class="dot" style="background:var(--warning)"></i> Titik anomali</span>
          <span><i class="dot" style="background:#6c8eff"></i> Waktu saat ini</span>
        </div>

        <div class="sim-title" style="margin-top:22px;">Daftar Alert</div>
        <div class="alert-list" id="monAlertList"></div>

        <div class="sim-title" style="margin-top:22px;">Baseline vs Aktual</div>
        <div style="overflow-x:auto;">
          <table class="data-table" id="monBaselineTable">
            <thead><tr><th>Perangkat</th><th>Metrik</th><th>Baseline</th><th>Aktual</th><th>Deviasi</th><th>Status</th></tr></thead>
            <tbody></tbody>
          </table>
        </div>

        <div class="sim-title" style="margin-top:22px;">Dengan Monitoring vs Tanpa Monitoring</div>
        <div class="seg-ctl" style="margin-bottom:8px;"><button id="monModeWith" class="active">Dengan monitoring</button><button id="monModeWithout">Tanpa monitoring</button></div>
        <div class="compare-wrap" id="monCompare"></div>

        <div class="sim-title" style="margin-top:22px;">Mengapa monitoring penting?</div>
        <div class="analysis-box" id="monAnalysis"></div>
        <div class="btn-row" style="margin-top:12px;"><button class="btn-run" id="monCopy">Salin jawaban</button></div>
      </div>
    </div>

<script>
/* =====================================================================
   Modul 1 — Monitoring Jaringan
   Pure functions mirror src/monitoring_algo.rs (tested reference).
   All outputs are computed from the input series only.
   ===================================================================== */
(function () {
  'use strict';
  var $ = function (id) { return document.getElementById(id); };

  // ---- metric configuration (defaults mirror monitoring_algo.rs) ----
  var METRICS = [
    { key: 'latency', label: 'Latency', unit: 'ms', dirUp: true, warn: 100, crit: 200, weight: 20 },
    { key: 'packetLoss', label: 'Packet Loss', unit: '%', dirUp: true, warn: 1, crit: 5, weight: 20 },
    { key: 'cpu', label: 'CPU', unit: '%', dirUp: true, warn: 70, crit: 90, weight: 15 },
    { key: 'bandwidth', label: 'Bandwidth', unit: '%', dirUp: true, warn: 70, crit: 90, weight: 15 },
    { key: 'memori', label: 'Memori', unit: '%', dirUp: true, warn: 80, crit: 95, weight: 10 },
    { key: 'uptime', label: 'Uptime', unit: '%', dirUp: false, warn: 99.9, crit: 99, weight: 10 },
    { key: 'errorInterface', label: 'Error Interface', unit: '', dirUp: true, warn: 10, crit: 50, weight: 5 },
    { key: 'koneksiAktif', label: 'Koneksi Aktif', unit: '', dirUp: true, warn: 800, crit: 1200, weight: 5 }
  ];
  var MMAP = {}; METRICS.forEach(function (m) { MMAP[m.key] = m; });

  function cfgOf(key) {
    var m = MMAP[key];
    var t = S.thr[key] || {};
    return { key: m.key, label: m.label, unit: m.unit, dirUp: m.dirUp, weight: m.weight,
      warn: (t.warn != null ? t.warn : m.warn), crit: (t.crit != null ? t.crit : m.crit) };
  }

  var S = {
    series: {},          // "deviceId|metric" -> [numbers]
    thr: {},             // metric -> {warn, crit}
    interval: 1, baseWin: 5, reportDelay: 10, adminResp: 5,
    time: 0, playing: false, speed: 1, mode: 'with',
    chartDev: null, chartMetric: 'latency', tableMode: false
  };
  METRICS.forEach(function (m) { S.thr[m.key] = { warn: m.warn, crit: m.crit }; });

  function devices() { return NetApp.get().devices; }
  function points() {
    var max = 0;
    Object.keys(S.series).forEach(function (k) { max = Math.max(max, S.series[k].length); });
    return max;
  }
  function val(dev, key, t) {
    var a = S.series[dev + '|' + key];
    if (!a) return null;
    var v = a[t];
    return (v === undefined || v === null) ? null : v;
  }

  // ---- pure analysis (mirror of Rust) ----
  function statusOf(key, v) {
    if (v == null) return 'none';
    var c = cfgOf(key);
    if (c.dirUp) { if (v >= c.crit) return 'kritis'; if (v >= c.warn) return 'waspada'; return 'normal'; }
    if (v <= c.crit) return 'kritis'; if (v <= c.warn) return 'waspada'; return 'normal';
  }
  function severity(s) { return s === 'kritis' ? 2 : (s === 'waspada' ? 1 : 0); }

  function subScore(key, v) {
    var c = cfgOf(key);
    if (c.dirUp) {
      if (v <= c.warn) return 100;
      if (v <= c.crit) return 100 - 50 * (v - c.warn) / (c.crit - c.warn);
      var ext = c.crit * 1.5;
      if (v >= ext) return 0;
      return 50 - 50 * (v - c.crit) / (ext - c.crit);
    }
    if (v >= c.warn) return 100;
    if (v >= c.crit) return 100 - 50 * (c.warn - v) / (c.warn - c.crit);
    return Math.max(0, 50 - 50 * (c.crit - v) / c.crit);
  }

  function deviceScore(dev, t) {
    var sum = 0, wsum = 0;
    METRICS.forEach(function (m) {
      var v = val(dev, m.key, t);
      if (v != null) { sum += m.weight * subScore(m.key, v); wsum += m.weight; }
    });
    return wsum > 0 ? sum / wsum : 100;
  }
  function deviceWeight(kind) { return (kind === 'router' || kind === 'firewall' || kind === 'server') ? 2 : 1; }

  function health(t) {
    var ds = devices();
    if (!ds.length) return { score: 0, label: 'Kritis', avg: 0, worst: 100 };
    var sum = 0, wsum = 0, worst = 999;
    ds.forEach(function (d) {
      var s = deviceScore(d.id, t), w = deviceWeight(d.kind);
      sum += w * s; wsum += w; if (s < worst) worst = s;
    });
    var avg = sum / wsum;
    var score = 0.7 * avg + 0.3 * worst;
    var label = score >= 85 ? 'Sehat' : (score >= 60 ? 'Waspada' : 'Kritis');
    return { score: score, label: label, avg: avg, worst: worst };
  }

  function baseline(arr, win) {
    if (!arr || !arr.length) return { mean: 0, std: 0 };
    var n = Math.min(win, arr.length), s = 0, i;
    for (i = 0; i < n; i++) s += arr[i];
    var mean = s / n, v = 0;
    for (i = 0; i < n; i++) v += (arr[i] - mean) * (arr[i] - mean);
    return { mean: mean, std: Math.sqrt(v / n) };
  }

  function anomalies(dev, key) {
    var arr = S.series[dev + '|' + key]; if (!arr) return [];
    var b = baseline(arr, S.baseWin), c = cfgOf(key), out = [];
    for (var i = 0; i < arr.length; i++) {
      var v = arr[i];
      var stat = b.std > 0 && (v - b.mean) / b.std > 2;
      var thr = severity(statusOf(key, v)) > 0;
      if (stat || thr) out.push({ index: i, kind: stat && thr ? 'keduanya' : (stat ? 'statistik' : 'threshold') });
    }
    return out;
  }

  function buildAlerts() {
    var alerts = [];
    devices().forEach(function (d) {
      METRICS.forEach(function (m) {
        var arr = S.series[d.id + '|' + m.key]; if (!arr) return;
        var prev = 'normal';
        for (var i = 0; i < arr.length; i++) {
          var cur = statusOf(m.key, arr[i]);
          if (severity(cur) > severity(prev)) {
            var c = cfgOf(m.key);
            alerts.push({ time: i, device: d.name, deviceId: d.id, metric: m.label, key: m.key,
              value: arr[i], threshold: cur === 'kritis' ? c.crit : c.warn, level: cur });
          }
          prev = cur;
        }
      });
    });
    alerts.sort(function (a, b) { return severity(b.level) - severity(a.level) || b.time - a.time; });
    return alerts;
  }

  function firstCritical() {
    var first = null;
    devices().forEach(function (d) {
      METRICS.forEach(function (m) {
        var arr = S.series[d.id + '|' + m.key]; if (!arr) return;
        for (var i = 0; i < arr.length; i++) {
          if (statusOf(m.key, arr[i]) === 'kritis') { if (first === null || i < first) first = i; break; }
        }
      });
    });
    return first;
  }

  function mttd(alerts) {
    if (!alerts.length) return null;
    var firstAlert = Math.min.apply(null, alerts.map(function (a) { return a.time; }));
    var fc = firstCritical();
    if (fc === null) return null;
    var userReport = fc + S.reportDelay;
    return { firstAlert: firstAlert, firstCritical: fc, userReport: userReport, faster: userReport - firstAlert };
  }

  function linreg(arr) {
    var n = arr.length; if (n < 2) return { slope: 0, intercept: arr[0] || 0 };
    var mx = (n - 1) / 2, my = 0, i;
    for (i = 0; i < n; i++) my += arr[i]; my /= n;
    var num = 0, den = 0;
    for (i = 0; i < n; i++) { num += (i - mx) * (arr[i] - my); den += (i - mx) * (i - mx); }
    var slope = den > 0 ? num / den : 0;
    return { slope: slope, intercept: my - slope * mx };
  }
  function capacity(dev) {
    var arr = S.series[dev + '|bandwidth']; if (!arr || arr.length < 8) return null;
    var r = linreg(arr); if (r.slope <= 0) return null;
    var x90 = (90 - r.intercept) / r.slope, last = arr.length - 1;
    if (x90 > last) return (x90 - last) * S.interval;
    return 0;
  }
  function availability() {
    var all = [];
    devices().forEach(function (d) { var a = S.series[d.id + '|uptime']; if (a) all = all.concat(a); });
    if (!all.length) return { avg: 0, downtime: 0 };
    var avg = all.reduce(function (x, y) { return x + y; }, 0) / all.length;
    return { avg: avg, downtime: (100 - avg) / 100 * all.length * S.interval };
  }

  // ---- seeded RNG + sample generator (mirror) ----
  function mulberry32(seed) {
    var a = seed >>> 0;
    return function () {
      a |= 0; a = (a + 0x6D2B79F5) | 0;
      var t = Math.imul(a ^ (a >>> 15), 1 | a);
      t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
      return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
    };
  }
  function hashStr(s) { var h = 2166136261; for (var i = 0; i < s.length; i++) { h ^= s.charCodeAt(i); h = Math.imul(h, 16777619); } return h >>> 0; }
  function metricsForType(kind) {
    if (kind === 'pc') return ['cpu', 'memori', 'bandwidth', 'latency', 'packetLoss', 'uptime', 'koneksiAktif'];
    if (kind === 'router' || kind === 'switch' || kind === 'firewall') return ['cpu', 'memori', 'bandwidth', 'latency', 'packetLoss', 'uptime', 'errorInterface', 'koneksiAktif'];
    if (kind === 'server' || kind === 'database') return ['cpu', 'memori', 'bandwidth', 'latency', 'packetLoss', 'uptime', 'errorInterface', 'koneksiAktif'];
    if (kind === 'ap') return ['cpu', 'memori', 'bandwidth', 'latency', 'packetLoss', 'uptime', 'koneksiAktif'];
    if (kind === 'internet') return ['bandwidth', 'latency', 'packetLoss', 'uptime'];
    if (kind === 'ids' || kind === 'cctv') return ['cpu', 'memori', 'uptime', 'errorInterface'];
    return ['cpu', 'memori', 'bandwidth', 'latency', 'packetLoss', 'uptime', 'koneksiAktif'];
  }
  function r2(x) { return Math.round(x * 100) / 100; }

  function generateSample(seed, points) {
    var out = {}; var ds = devices();
    var routerId = (ds.filter(function (d) { return d.kind === 'router'; })[0] || {}).id;
    var incidentAt = Math.floor(points / 2);
    ds.forEach(function (d, di) {
      metricsForType(d.kind).forEach(function (key) {
        var m = MMAP[key];
        var rng = mulberry32((seed + di * 131 + hashStr(key)) >>> 0);
        var base = m.dirUp ? m.warn * (0.35 + 0.2 * rng()) : m.warn - (100 - m.warn) * 0.1 * rng();
        var arr = [];
        for (var t = 0; t < points; t++) {
          var span = m.dirUp ? m.warn * 0.12 : 0.05;
          var v = base + (rng() - 0.5) * span;
          if (d.id === routerId && (key === 'latency' || key === 'bandwidth') && t >= incidentAt && t < incidentAt + 3) {
            v = key === 'latency' ? m.crit * 1.15 : m.crit * 1.05;
          }
          v = m.dirUp ? Math.max(0, v) : Math.min(100, Math.max(0, v));
          arr.push(r2(v));
        }
        out[d.id + '|' + key] = arr;
      });
    });
    return out;
  }

  // ---- CSV ----
  function parseCsv(text) {
    var pts = [], errs = [];
    var lines = text.split(/\n/);
    for (var li = 0; li < lines.length; li++) {
      var line = lines[li].trim(); if (!line) continue;
      var cols = line.split(',').map(function (c) { return c.trim(); });
      if (cols.length !== 4) { errs.push('Baris ' + (li + 1) + ': harus 4 kolom (waktu,perangkat,metrik,nilai), ditemukan ' + cols.length + '.'); continue; }
      var time = parseInt(cols[0], 10);
      if (isNaN(time)) { errs.push("Baris " + (li + 1) + ": waktu '" + cols[0] + "' bukan bilangan bulat."); continue; }
      if (!MMAP[cols[2]]) { errs.push("Baris " + (li + 1) + ": metrik '" + cols[2] + "' tidak dikenal."); continue; }
      var v = Number(cols[3]);
      if (isNaN(v)) { errs.push("Baris " + (li + 1) + ": nilai '" + cols[3] + "' bukan angka."); continue; }
      if (v < 0) { errs.push('Baris ' + (li + 1) + ': nilai negatif tidak diizinkan.'); continue; }
      if (['cpu', 'memori', 'bandwidth', 'packetLoss', 'uptime'].indexOf(cols[2]) >= 0 && v > 100) { errs.push('Baris ' + (li + 1) + ': nilai persen harus 0-100.'); continue; }
      pts.push({ time: time, device: cols[1], metric: cols[2], value: v });
    }
    return { points: pts, errors: errs };
  }

  // ---- error/ok boxes ----
  function showErr(msgs) {
    var box = $('monErr');
    if (!msgs || !msgs.length) { box.classList.remove('show'); box.innerHTML = ''; return; }
    var html = '<strong>Input tidak valid:</strong><ul>';
    msgs.forEach(function (m) { html += '<li>' + m + '</li>'; });
    box.innerHTML = html + '</ul>'; box.classList.add('show');
  }

  // ---- renderers ----
  function renderThresholds() {
    var g = $('monThresholdGrid'); g.innerHTML = '';
    METRICS.forEach(function (m) {
      var t = S.thr[m.key];
      var item = document.createElement('div'); item.className = 'thr-item';
      item.innerHTML =
        '<div class="tl">' + m.label + ' <span class="dir">(' + (m.dirUp ? 'naik = buruk' : 'turun = buruk') + ')</span></div>' +
        '<div class="thr-row">' +
        '<div class="input-group"><label>Warning</label><input class="mini-input" type="number" data-k="' + m.key + '" data-f="warn" value="' + t.warn + '" /></div>' +
        '<div class="input-group"><label>Critical</label><input class="mini-input" type="number" data-k="' + m.key + '" data-f="crit" value="' + t.crit + '" /></div>' +
        '</div>';
      g.appendChild(item);
    });
    g.querySelectorAll('input.mini-input').forEach(function (inp) {
      inp.addEventListener('change', function () {
        var k = inp.getAttribute('data-k'), f = inp.getAttribute('data-f');
        var v = Number(inp.value);
        if (isNaN(v) || v < 0) { showErr(['Threshold harus angka >= 0.']); return; }
        S.thr[k][f] = v;
        var c = cfgOf(k);
        if (!c.dirUp && c.warn <= c.crit) { showErr(['Untuk uptime (turun = buruk), warning harus lebih besar dari critical.']); }
        if (c.dirUp && c.warn >= c.crit) { showErr(['Untuk metrik naik, warning harus lebih kecil dari critical.']); }
        showErr(null); renderAll();
      });
    });
  }

  function renderTimeSelect() {
    var sel = $('monTimeSel'); var n = points();
    sel.innerHTML = '';
    for (var i = 0; i < n; i++) { var o = document.createElement('option'); o.value = i; o.textContent = 'Menit ke-' + (i * S.interval); sel.appendChild(o); }
    sel.value = Math.max(0, Math.min(S.time, n - 1));
  }

  function renderDataTable() {
    var ds = devices();
    var head = $('monDataHead'); head.innerHTML = '<th>Perangkat</th>';
    METRICS.forEach(function (m) { head.innerHTML += '<th>' + m.label + (m.unit ? ' (' + m.unit + ')' : '') + '</th>'; });
    var tb = $('monDataTable').querySelector('tbody'); tb.innerHTML = '';
    if (!ds.length) { tb.innerHTML = '<tr><td colspan="9" style="color:var(--text-muted);">Tidak ada perangkat. Tambahkan di Editor Topologi.</td></tr>'; return; }
    ds.forEach(function (d) {
      var tr = document.createElement('tr');
      var html = '<td style="font-weight:600;">' + d.name + '</td>';
      METRICS.forEach(function (m) {
        var v = val(d.id, m.key, S.time);
        var stt = statusOf(m.key, v);
        var cell = v == null ? '<span style="color:var(--text-muted);">—</span>' :
          '<span class="status-pill ' + stt + '" title="' + stt + '"><span class="sdot"></span>' + v + '</span>';
        html += '<td>' + cell + '</td>';
      });
      tr.innerHTML = html; tb.appendChild(tr);
    });
  }

  function fillSelect(sel, items, cur) {
    sel.innerHTML = '';
    items.forEach(function (it) { var o = document.createElement('option'); o.value = it.value; o.textContent = it.label; if (it.value === cur) o.selected = true; sel.appendChild(o); });
  }

  function renderSeriesControls() {
    var ds = devices();
    if (!ds.length) return;
    if (!S.chartDev || !ds.some(function (d) { return d.id === S.chartDev; })) S.chartDev = ds[0].id;
    fillSelect($('monSeriesDev'), ds.map(function (d) { return { value: d.id, label: d.name }; }), $('monSeriesDev').value || S.chartDev);
    fillSelect($('monChartDev'), ds.map(function (d) { return { value: d.id, label: d.name }; }), S.chartDev);
    fillSelect($('monSeriesMetric'), METRICS.map(function (m) { return { value: m.key, label: m.label }; }), $('monSeriesMetric').value || 'latency');
    fillSelect($('monChartMetric'), METRICS.map(function (m) { return { value: m.key, label: m.label }; }), S.chartMetric);
    loadSeriesIntoEditor();
  }

  function loadSeriesIntoEditor() {
    var dev = $('monSeriesDev').value, key = $('monSeriesMetric').value;
    var arr = S.series[dev + '|' + key];
    $('monSeriesText').value = arr ? arr.join(', ') : '';
  }

  function renderTimeline() {
    var n = points();
    var tl = $('monTimeline'); tl.max = Math.max(0, n - 1); tl.value = Math.min(S.time, n - 1);
    $('monTimeLabel').textContent = 't=' + S.time + ' (' + (S.time * S.interval) + ' mnt)';
  }

  function renderScoreboard() {
    var h = health(S.time);
    var counts = { normal: 0, waspada: 0, kritis: 0, mati: 0 };
    devices().forEach(function (d) {
      var worst = 'normal';
      METRICS.forEach(function (m) { var s = statusOf(m.key, val(d.id, m.key, S.time)); if (severity(s) > severity(worst)) worst = s; });
      if (worst === 'none') worst = 'mati';
      counts[worst] = (counts[worst] || 0) + 1;
    });
    $('monHealthVal').textContent = Math.round(h.score);
    $('monHealthLabel').textContent = '(' + h.label + ')';
    $('monCountNormal').textContent = counts.normal;
    $('monCountWarn').textContent = counts.waspada;
    $('monCountCrit').textContent = counts.kritis;
    var av = availability();
    $('monAvgUptime').textContent = av.avg ? av.avg.toFixed(2) + '%' : '–';
    var alerts = buildAlerts();
    $('monAlertCount').textContent = alerts.length;
    drawGauge(h.score);
  }

  function drawGauge(score) {
    var svg = $('monGauge');
    var cx = 60, cy = 60, r = 48;
    var a0 = Math.PI, a1 = 2 * Math.PI;
    function pt(a) { return [cx + r * Math.cos(a), cy + r * Math.sin(a)]; }
    function arc(a0, a1, color, w) {
      var p0 = pt(a0), p1 = pt(a1);
      var large = (a1 - a0) > Math.PI ? 1 : 0;
      return '<path d="M ' + p0[0] + ' ' + p0[1] + ' A ' + r + ' ' + r + ' 0 ' + large + ' 1 ' + p1[0] + ' ' + p1[1] + '" fill="none" stroke="' + color + '" stroke-width="' + w + '" stroke-linecap="round"/>';
    }
    var frac = Math.max(0, Math.min(100, score)) / 100;
    var col = score >= 85 ? '#4ade80' : (score >= 60 ? '#fbbf24' : '#ef4444');
    var html = arc(a0, a1, 'var(--glass-border)', 10);
    if (frac > 0) html += arc(a0, a0 + (a1 - a0) * frac, col, 10);
    html += '<text x="60" y="52" text-anchor="middle" style="font-size:16px;font-weight:700;fill:var(--text-primary);">' + Math.round(score) + '</text>';
    html += '<text x="60" y="64" text-anchor="middle" style="font-size:6px;fill:var(--text-muted);">SKOR KESEHATAN</text>';
    svg.innerHTML = html;
  }

  function renderChart() {
    var svg = $('monChart');
    var dev = S.chartDev, key = S.chartMetric;
    var arr = S.series[dev + '|' + key];
    var c = cfgOf(key);
    if (!arr || !arr.length) { svg.innerHTML = '<text x="50" y="22" text-anchor="middle">Tidak ada data untuk metrik ini</text>'; renderChartTable(); return; }
    var n = arr.length;
    var vals = arr.filter(function (v) { return v != null; });
    var maxV = Math.max.apply(null, vals.concat([c.crit, c.warn]));
    var minV = Math.min.apply(null, vals.concat([0]));
    if (maxV <= minV) maxV = minV + 1;
    var X = function (i) { return n <= 1 ? 50 : 4 + (i / (n - 1)) * 92; };
    var Y = function (v) { return 38 - ((v - minV) / (maxV - minV)) * 32; };
    var parts = [];
    // grid
    for (var g = 0; g <= 4; g++) { var yy = 6 + g * 8; parts.push('<line class="grid-line" x1="4" y1="' + yy + '" x2="96" y2="' + yy + '"></line>'); }
    // threshold lines
    parts.push('<line class="thr-line" x1="4" y1="' + Y(c.warn) + '" x2="96" y2="' + Y(c.warn) + '" stroke="var(--warning)"></line>');
    parts.push('<line class="thr-line" x1="4" y1="' + Y(c.crit) + '" x2="96" y2="' + Y(c.crit) + '" stroke="#ef4444"></line>');
    // baseline
    var b = baseline(arr, S.baseWin);
    if (b.mean >= minV && b.mean <= maxV) parts.push('<line class="base-line" x1="4" y1="' + Y(b.mean) + '" x2="96" y2="' + Y(b.mean) + '" stroke="var(--text-muted)"></line>');
    // series
    var dpath = '';
    for (var i = 0; i < n; i++) { if (arr[i] == null) continue; dpath += (dpath ? ' L ' : 'M ') + X(i) + ' ' + Y(arr[i]); }
    if (dpath) parts.push('<path class="series-line" d="' + dpath + '" stroke="var(--accent)"></path>');
    // anomalies
    anomalies(dev, key).forEach(function (an) { parts.push('<circle class="anom-dot" cx="' + X(an.index) + '" cy="' + Y(arr[an.index]) + '" r="1.2" fill="var(--warning)"></circle>'); });
    // now marker
    if (S.time < n) parts.push('<line class="now-line" x1="' + X(S.time) + '" y1="4" x2="' + X(S.time) + '" y2="40" stroke="#6c8eff"></line>');
    svg.innerHTML = parts.join('');
    renderChartTable();
  }

  function renderChartTable() {
    var wrap = $('monChartTableWrap');
    if (!S.tableMode) { wrap.style.display = 'none'; return; }
    wrap.style.display = 'block';
    var dev = S.chartDev, key = S.chartMetric, arr = S.series[dev + '|' + key] || [];
    var dn = (devices().filter(function (d) { return d.id === dev; })[0] || {}).name || dev;
    var html = '<table class="data-table"><thead><tr><th>Titik</th><th>Waktu</th><th>' + MMAP[key].label + '</th><th>Status</th></tr></thead><tbody>';
    for (var i = 0; i < arr.length; i++) html += '<tr><td>' + i + '</td><td>' + (i * S.interval) + ' mnt</td><td>' + arr[i] + '</td><td>' + statusOf(key, arr[i]) + '</td></tr>';
    html += '</tbody></table><p class="hint">Perangkat: ' + dn + '</p>';
    wrap.innerHTML = html;
  }

  function renderAlerts() {
    var list = $('monAlertList'); var alerts = buildAlerts();
    if (!alerts.length) { list.innerHTML = '<div class="empty-note">Tidak ada alert pada data ini.</div>'; return; }
    list.innerHTML = '';
    alerts.forEach(function (a) {
      var el = document.createElement('div');
      el.className = 'alert-item ' + a.level;
      el.innerHTML = '<span class="at">Menit ke-' + a.time + '</span>' +
        '<span class="an">' + a.device + ' · ' + a.metric + '</span>' +
        '<span class="av">' + a.value + ' (ambang ' + a.threshold + ') · ' + a.level + '</span>';
      el.addEventListener('click', function () { S.time = a.time; S.playing = false; renderAll(); if (window.NetShowDetail) { var d = devices().filter(function (x) { return x.id === a.deviceId; })[0]; if (d) window.NetShowDetail(d); } });
      list.appendChild(el);
    });
  }

  function renderBaseline() {
    var tb = $('monBaselineTable').querySelector('tbody'); tb.innerHTML = '';
    var rows = [];
    devices().forEach(function (d) {
      METRICS.forEach(function (m) {
        var arr = S.series[d.id + '|' + m.key]; if (!arr) return;
        var b = baseline(arr, S.baseWin);
        var act = val(d.id, m.key, S.time); if (act == null) return;
        var dev = b.mean !== 0 ? ((act - b.mean) / b.mean) * 100 : 0;
        rows.push({ dev: d.name, metric: m.label, base: b.mean, act: act, devp: dev, st: statusOf(m.key, act) });
      });
    });
    if (!rows.length) { tb.innerHTML = '<tr><td colspan="6" style="color:var(--text-muted);">Belum ada data.</td></tr>'; return; }
    rows.forEach(function (r) {
      var tr = document.createElement('tr');
      var sign = r.devp >= 0 ? '+' : '';
      tr.innerHTML = '<td>' + r.dev + '</td><td>' + r.metric + '</td><td>' + r.base.toFixed(2) + '</td><td>' + r.act + '</td><td>' + sign + r.devp.toFixed(1) + '%</td>' +
        '<td><span class="status-pill ' + r.st + '"><span class="sdot"></span>' + r.st + '</span></td>';
      tb.appendChild(tr);
    });
  }

  function renderCompare() {
    var wrap = $('monCompare');
    var alerts = buildAlerts();
    var m = mttd(alerts);
    var n = Math.max(points(), 1);
    var scale = function (t) { return Math.max(0, Math.min(100, (t / n) * 100)); };
    var html = '';
    if (!m) {
      html += '<div class="empty-note">Tidak ada insiden (titik kritis) pada data ini — perbandingan MTTD tidak dapat dibuat.</div>';
      wrap.innerHTML = html; return;
    }
    var fixTime = Math.min(n, m.firstCritical + S.reportDelay + S.adminResp);
    // With monitoring
    html += '<div class="cmp-row"><span class="cmp-label">Dengan monitoring</span><div class="cmp-track">';
    html += '<div class="cmp-seg ok" style="left:0;width:' + scale(m.firstAlert) + '%;">normal</div>';
    html += '<div class="cmp-seg alert" style="left:' + scale(m.firstAlert) + '%;width:' + Math.max(2, scale(fixTime) - scale(m.firstAlert)) + '%;">alert t=' + m.firstAlert + '</div>';
    html += '<div class="cmp-tick" style="left:' + scale(fixTime) + '%"></div></div></div>';
    // Without monitoring
    var repTime = m.userReport, fix2 = Math.min(n, repTime + S.adminResp);
    html += '<div class="cmp-row"><span class="cmp-label">Tanpa monitoring</span><div class="cmp-track">';
    html += '<div class="cmp-seg ok" style="left:0;width:' + scale(repTime) + '%;">normal</div>';
    html += '<div class="cmp-seg report" style="left:' + scale(repTime) + '%;width:' + Math.max(2, scale(fix2) - scale(repTime)) + '%;">laporan t=' + repTime + '</div>';
    html += '<div class="cmp-tick" style="left:' + scale(fix2) + '%"></div></div></div>';
    html += '<p class="hint">MTTD: monitoring mendeteksi pada menit ke-' + m.firstAlert + ', sedangkan tanpa monitoring baru diketahui pada menit ke-' + m.userReport +
      ' (insiden kritis di menit ke-' + m.firstCritical + ' + keterlambatan laporan ' + S.reportDelay + ' menit). <b style="color:var(--success)">Deteksi ' + m.faster + ' menit lebih cepat.</b></p>';
    wrap.innerHTML = html;
  }

  function renderAnalysis() {
    var box = $('monAnalysis');
    var h = health(S.time);
    var alerts = buildAlerts();
    var m = mttd(alerts);
    var av = availability();
    var ds = devices();
    var parts = [];

    // 1) deteksi dini
    if (m) {
      parts.push(['Deteksi dini', 'Monitoring mendeteksi lonjakan metrik pada menit ke-' + m.firstAlert +
        ', sedangkan tanpa monitoring pengguna baru melapor pada menit ke-' + m.userReport +
        ' (insiden terasa di menit ke-' + m.firstCritical + ' + keterlambatan ' + S.reportDelay +
        ' menit). Selisihnya <b>' + m.faster + ' menit lebih cepat</b> terdeteksi.']);
    }

    // 2) pencegahan downtime
    var nearCrit = [];
    ds.forEach(function (d) {
      var worst = 'normal', wkey = '';
      METRICS.forEach(function (mm) { var s = statusOf(mm.key, val(d.id, mm.key, S.time)); if (severity(s) > severity(worst)) { worst = s; wkey = mm.label; } });
      if (worst === 'waspada' || worst === 'kritis') nearCrit.push(d.name + ' (' + wkey + ' ' + worst + ')');
    });
    if (av.avg || nearCrit.length) {
      var t1 = 'Rata-rata uptime ' + av.avg.toFixed(2) + '% dengan perkiraan downtime ' + av.downtime.toFixed(1) + ' menit';
      if (nearCrit.length) t1 += '; perangkat yang mendekati kritis: ' + nearCrit.join(', ');
      parts.push(['Pencegahan downtime', t1 + '. Memantau lebih dini mencegah gangguan berkepanjangan.']);
    }

    // 3) perencanaan kapasitas
    var cap = [];
    ds.forEach(function (d) { var c = capacity(d.id); if (c != null && c > 0) cap.push(d.name + ' ~' + c.toFixed(0) + ' menit lagi'); });
    if (cap.length) parts.push(['Perencanaan kapasitas', 'Dengan tren bandwidth naik, perkiraan kasar mencapai 90%: ' + cap.join(', ') + '.']);

    // 4) bukti kinerja/SLA
    if (av.avg) {
      var sla = av.avg >= 99.9 ? 'memenuhi' : 'di bawah';
      parts.push(['Bukti kinerja / SLA', 'Uptime rata-rata ' + av.avg.toFixed(2) + '% ' + sla + ' target 99.9%. Data monitoring menjadi bukti kinerja yang terukur.']);
    }

    // 5) dasar troubleshooting
    var firstDev = null;
    if (alerts.length) {
      var a0 = alerts[alerts.length - 1];
      firstDev = a0.device + ' pada metrik ' + a0.metric + ' (menit ke-' + a0.time + ')';
      parts.push(['Dasar troubleshooting', 'Titik pertama yang menyimpang adalah ' + firstDev + ', sehingga penelusuran dapat langsung diarahkan ke sana.']);
    }

    // 6) keamanan
    var sec = [];
    ds.forEach(function (d) {
      var arr = S.series[d.id + '|packetLoss'];
      var conns = S.series[d.id + '|koneksiAktif'];
      if (arr) { var mx = Math.max.apply(null, arr); if (mx >= cfgOf('packetLoss').crit) sec.push(d.name + ' packet loss puncak ' + mx + '%'); }
      if (conns) { var mc = Math.max.apply(null, conns); if (mc >= cfgOf('koneksiAktif').crit) sec.push(d.name + ' koneksi aktif puncak ' + mc); }
    });
    if (sec.length) parts.push(['Keamanan', 'Terdeteksi lonjakan abnormal: ' + sec.join('; ') + '. Perlu ditinjau untuk indikasi serangan/malware.']);

    var html = '';
    if (!parts.length) {
      html = '<div class="empty-note">Belum ada data yang cukup untuk analisis. Klik "Hasilkan data contoh" atau masukkan data lalu "Analisis".</div>';
    } else {
      parts.forEach(function (p) { html += '<h4>' + p[0] + '</h4><p>' + p[1] + '</p>'; });
      var concl = 'Kesimpulannya, monitoring jaringan penting karena mengubah data menjadi keputusan: skor kesehatan saat ini ' + Math.round(h.score) +
        '/100 (' + h.label + '), ' + alerts.length + ' alert tercatat' + (m ? ', dengan deteksi ' + m.faster + ' menit lebih cepat' : '') +
        '. Tanpa monitoring, ' + (m ? 'gangguan baru diketahui setelah pengguna mengeluh' : 'gangguan berisiko tidak terdeteksi') +
        ', sehingga downtime, biaya, dan risiko keamanan meningkat.';
      html += '<div class="concl">' + concl + '</div>';
    }
    box.innerHTML = html;
    box._plain = parts.map(function (p) { return p[0] + ': ' + p[1].replace(/<[^>]+>/g, ''); }).join('\n');
  }

  function buildMonState() {
    var status = {}, bandwidth = {}, packetLoss = {}, latency = {}, worst = {};
    devices().forEach(function (d) {
      var wSev = -1, wLabel = '', wVal = null, set = false;
      METRICS.forEach(function (m) {
        var v = val(d.id, m.key, S.time);
        var s = statusOf(m.key, v);
        if (s === 'kritis') { status[d.id] = 'kritis'; set = true; }
        else if (s === 'waspada') { if (status[d.id] !== 'kritis') status[d.id] = 'waspada'; set = true; }
        else if (s === 'normal') { if (!status[d.id]) status[d.id] = 'normal'; set = true; }
        var sev = severity(s);
        if (sev > wSev && v != null) { wSev = sev; wLabel = m.label; wVal = v; }
        if (m.key === 'bandwidth' && v != null) bandwidth[d.id] = v;
        if (m.key === 'packetLoss' && v != null) packetLoss[d.id] = v;
        if (m.key === 'latency' && v != null) latency[d.id] = v;
      });
      if (!set) status[d.id] = 'mati';
      worst[d.id] = { label: wLabel, value: wVal };
    });
    return { active: true, time: S.time, interval: S.interval, status: status, bandwidth: bandwidth,
      packetLoss: packetLoss, latency: latency, worst: worst, mode: S.mode };
  }

  function applyMon() {
    window.NetMon = buildMonState();
    if (window.NetApplyMon) window.NetApplyMon(window.NetMon);
  }

  function renderAll() {
    renderTimeSelect();
    renderThresholds();
    renderDataTable();
    renderSeriesControls();
    renderTimeline();
    renderScoreboard();
    renderChart();
    renderAlerts();
    renderBaseline();
    renderCompare();
    renderAnalysis();
    applyMon();
  }

  // ---- actions ----
  function doSample() {
    var seed = parseInt($('monSeed').value, 10) || 0;
    var p = Math.max(10, Math.min(30, parseInt($('monPoints').value, 10) || 20));
    if (!devices().length) { showErr(['Tidak ada perangkat. Tambahkan di Editor Topologi.']); return; }
    S.series = generateSample(seed, p);
    S.time = 0; showErr(null); renderAll();
  }
  function doAnalyze() { showErr(null); renderAll(); }
  function doReset() {
    S.series = {}; S.time = 0; S.playing = false;
    METRICS.forEach(function (m) { S.thr[m.key] = { warn: m.warn, crit: m.crit }; });
    $('monCsvText').value = ''; $('monSeriesText').value = '';
    showErr(null); renderAll();
  }

  function download(name, text, type) {
    var blob = new Blob([text], { type: type || 'text/plain' });
    var url = URL.createObjectURL(blob);
    var a = document.createElement('a'); a.href = url; a.download = name; document.body.appendChild(a); a.click();
    setTimeout(function () { URL.revokeObjectURL(url); a.remove(); }, 100);
  }
  function exportJson() {
    var out = { interval: S.interval, thresholds: S.thr, series: S.series };
    download('monitoring.json', JSON.stringify(out, null, 2), 'application/json');
  }
  function exportCsv() {
    var lines = ['waktu,perangkat,metrik,nilai'];
    var names = {}; devices().forEach(function (d) { names[d.id] = d.name; });
    Object.keys(S.series).forEach(function (k) {
      var bar = k.indexOf('|'); var id = k.slice(0, bar), key = k.slice(bar + 1);
      S.series[k].forEach(function (v, t) { lines.push(t + ',' + (names[id] || id) + ',' + key + ',' + v); });
    });
    download('monitoring.csv', lines.join('\n'), 'text/csv');
  }
  function templateCsv() {
    var ds = devices();
    var lines = ['waktu,perangkat,metrik,nilai'];
    var d = ds[0] ? ds[0].name : 'Internet';
    for (var t = 0; t < 5; t++) lines.push(t + ',' + d + ',latency,' + (20 + t));
    download('template_monitoring.csv', lines.join('\n'), 'text/csv');
  }

  // ---- wiring ----
  $('monSample').addEventListener('click', doSample);
  $('monAnalyze').addEventListener('click', doAnalyze);
  $('monReset').addEventListener('click', doReset);
  $('monTemplate').addEventListener('click', templateCsv);
  $('monExportJson').addEventListener('click', exportJson);
  $('monExportCsv').addEventListener('click', exportCsv);
  $('monThrDefault').addEventListener('click', function () { METRICS.forEach(function (m) { S.thr[m.key] = { warn: m.warn, crit: m.crit }; }); showErr(null); renderAll(); });

  $('monTimeSel').addEventListener('change', function () { S.time = +$('monTimeSel').value || 0; S.playing = false; renderAll(); });
  $('monTimeline').addEventListener('input', function () { S.time = +$('monTimeline').value || 0; S.playing = false; renderAll(); });
  $('monStep').addEventListener('click', function () { S.time = Math.max(0, S.time - 1); S.playing = false; renderAll(); });
  $('monSpeed').addEventListener('change', function () { S.speed = +$('monSpeed').value || 1; });
  $('monPlay').addEventListener('click', function () {
    S.playing = !S.playing; $('monPlay').textContent = S.playing ? 'Pause' : 'Play';
    if (S.playing) tick();
  });
  function tick() {
    if (!S.playing) return;
    var n = points();
    S.time = (S.time + 1) % Math.max(1, n);
    renderAll();
    setTimeout(tick, 700 / S.speed);
  }

  $('monSeriesDev').addEventListener('change', loadSeriesIntoEditor);
  $('monSeriesMetric').addEventListener('change', loadSeriesIntoEditor);
  $('monSeriesApply').addEventListener('click', function () {
    var dev = $('monSeriesDev').value, key = $('monSeriesMetric').value;
    var raw = $('monSeriesText').value.split(',').map(function (s) { return s.trim(); }).filter(function (s) { return s !== ''; });
    var arr = [], errs = [];
    raw.forEach(function (s, i) {
      var v = Number(s);
      if (isNaN(v) || v < 0) { errs.push('Nilai ke-' + (i + 1) + " '" + s + "' bukan angka >= 0."); return; }
      if (['cpu', 'memori', 'bandwidth', 'packetLoss', 'uptime'].indexOf(key) >= 0 && v > 100) { errs.push('Nilai ke-' + (i + 1) + ' persen harus 0-100.'); return; }
      arr.push(v);
    });
    if (errs.length) { showErr(errs); return; }
    if (!arr.length) { showErr(['Deret kosong.']); return; }
    S.series[dev + '|' + key] = arr; showErr(null); renderAll();
  });

  $('monCsvApply').addEventListener('click', function () {
    var res = parseCsv($('monCsvText').value);
    var errs = res.errors.slice();
    var names = {}; devices().forEach(function (d) { names[d.name] = d.id; });
    var grouped = {};
    res.points.forEach(function (p) {
      var id = names[p.device];
      if (!id) { errs.push("Perangkat '" + p.device + "' tidak dikenal di topologi."); return; }
      var k = id + '|' + p.metric;
      (grouped[k] = grouped[k] || {})[p.time] = p.value;
    });
    if (errs.length) { showErr(errs); return; }
    Object.keys(grouped).forEach(function (k) {
      var maxT = Math.max.apply(null, Object.keys(grouped[k]).map(Number));
      var arr = [];
      for (var t = 0; t <= maxT; t++) arr.push(grouped[k][t] != null ? grouped[k][t] : null);
      S.series[k] = arr;
    });
    showErr(null); renderAll();
  });

  $('monChartDev').addEventListener('change', function () { S.chartDev = $('monChartDev').value; renderChart(); });
  $('monChartMetric').addEventListener('change', function () { S.chartMetric = $('monChartMetric').value; renderChart(); });
  $('monChartTable').addEventListener('click', function () { S.tableMode = !S.tableMode; $('monChartTable').classList.toggle('active', S.tableMode); renderChart(); });

  $('monModeWith').addEventListener('click', function () { S.mode = 'with'; $('monModeWith').classList.add('active'); $('monModeWithout').classList.remove('active'); renderCompare(); applyMon(); });
  $('monModeWithout').addEventListener('click', function () { S.mode = 'without'; $('monModeWithout').classList.add('active'); $('monModeWith').classList.remove('active'); renderCompare(); applyMon(); });

  $('monCopy').addEventListener('click', function () {
    var txt = 'Mengapa monitoring jaringan penting?\n\n' + ($('monAnalysis')._plain || '') +
      '\n\nSkor kesehatan: ' + Math.round(health(S.time).score) + '/100 (' + health(S.time).label + ').';
    if (navigator.clipboard && navigator.clipboard.writeText) navigator.clipboard.writeText(txt).then(function () { $('monCopy').textContent = 'Tersalin!'; setTimeout(function () { $('monCopy').textContent = 'Salin jawaban'; }, 1500); });
    else download('jawaban_monitoring.txt', txt);
  });

  // re-render when topology changes (device list)
  NetApp.subscribe(function () { renderSeriesControls(); renderDataTable(); renderScoreboard(); renderAlerts(); renderBaseline(); renderAnalysis(); applyMon(); });

  // init: start empty; user can generate sample or import.
  renderAll();
  window.MonApp = { state: S, generateSample: generateSample, parseCsv: parseCsv, statusOf: statusOf, subScore: subScore, health: health, renderAll: renderAll };
})();
</script>"##.to_string()
}