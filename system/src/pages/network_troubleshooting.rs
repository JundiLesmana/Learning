// use axum::response::Html;

pub fn troubleshooting_html() -> String {
    r##"<div class="module-panel" id="mod3" role="tabpanel">
      <div class="sim-card glass">
        <div class="sim-title">Modul 3 &mdash; Troubleshooting Jaringan Lambat</div>
        <div class="mon-question">
          <b>Soal:</b> Jika sebuah perusahaan mengalami penurunan kecepatan jaringan secara tiba-tiba, langkah-langkah apa yang harus dilakukan administrator jaringan untuk mengidentifikasi penyebabnya?
        </div>

        <div class="error-box" id="tsErr" role="alert"></div>

        <!-- A. Gejala & Ruang Lingkup -->
        <div class="sim-title" style="margin-top:16px;">A. Gejala & Ruang Lingkup</div>
        <div class="sim-controls" style="margin-bottom:16px;">
          <div class="input-group small">
            <label for="tsScope">Terdampak</label>
            <select class="input-field" id="tsScope">
              <option value="all">Semua pengguna</option>
              <option value="segment">Satu lantai/segmen</option>
              <option value="device">Satu perangkat</option>
            </select>
          </div>
          <div class="input-group small">
            <label for="tsScopeDev">Perangkat (jika satu)</label>
            <select class="input-field" id="tsScopeDev" disabled></select>
          </div>
          <div class="input-group small">
            <label for="tsOnset">Waktu mulai (menit lalu)</label>
            <input class="input-field" id="tsOnset" type="number" min="0" value="30" />
          </div>
          <div class="input-group small">
            <label for="tsPattern">Pola</label>
            <select class="input-field" id="tsPattern">
              <option value="sudden">Tiba-tiba</option>
              <option value="gradual">Bertahap</option>
            </select>
          </div>
        </div>
        <div class="sim-controls" style="margin-bottom:16px;">
          <div class="input-group small">
            <label for="tsCoverage">Cakupan</label>
            <select class="input-field" id="tsCoverage">
              <option value="internet">Hanya internet</option>
              <option value="both">Internet + internal</option>
              <option value="internal">Hanya internal</option>
            </select>
          </div>
          <div class="input-group" style="flex:1;">
            <label>Perubahan terbaru</label>
            <div class="seg-ctl" style="flex:1;">
              <button type="button" class="ts-change-btn" data-change="config">Konfigurasi</button>
              <button type="button" class="ts-change-btn" data-change="firmware">Firmware</button>
              <button type="button" class="ts-change-btn" data-change="newdevice">Perangkat baru</button>
              <button type="button" class="ts-change-btn" data-change="cable">Kabel dipindah</button>
              <button type="button" class="ts-change-btn" data-change="backup">Backup/update massal</button>
            </div>
          </div>
        </div>
        <div class="sim-controls" style="margin-bottom:16px;">
          <div class="input-group" style="flex:1;">
            <label for="tsChangeNotes">Catatan perubahan</label>
            <textarea class="input-field" id="tsChangeNotes" rows="2" placeholder="Detail perubahan, jam, perangkat terkait..."></textarea>
          </div>
        </div>

        <!-- B. Data Ukur -->
        <div class="sim-title" style="margin-top:16px;">B. Data Ukur (kosong = belum diukur)</div>
        
        <details style="margin-bottom:12px;">
          <summary style="cursor:pointer;font-size:12px;font-weight:700;color:var(--text-secondary);text-transform:uppercase;letter-spacing:0.05em;">Ping & Traceroute</summary>
          <div class="sim-controls" style="margin-top:10px;">
            <div class="input-group small"><label for="tsPingGw">Ping Gateway (ms)</label><input class="input-field" id="tsPingGw" type="number" step="0.1" min="0" placeholder="—" /></div>
            <div class="input-group small"><label for="tsPingGwLoss">Loss GW (%)</label><input class="input-field" id="tsPingGwLoss" type="number" step="0.1" min="0" max="100" placeholder="—" /></div>
            <div class="input-group small"><label for="tsPingDns">Ping DNS (ms)</label><input class="input-field" id="tsPingDns" type="number" step="0.1" min="0" placeholder="—" /></div>
            <div class="input-group small"><label for="tsPingDnsLoss">Loss DNS (%)</label><input class="input-field" id="tsPingDnsLoss" type="number" step="0.1" min="0" max="100" placeholder="—" /></div>
            <div class="input-group small"><label for="tsPingNet">Ping Internet (ms)</label><input class="input-field" id="tsPingNet" type="number" step="0.1" min="0" placeholder="—" /></div>
            <div class="input-group small"><label for="tsPingNetLoss">Loss Internet (%)</label><input class="input-field" id="tsPingNetLoss" type="number" step="0.1" min="0" max="100" placeholder="—" /></div>
          </div>
          <div class="sim-title" style="margin-top:10px;">Traceroute (satu per baris: nama/IP, latency ms, kosong=timeout)</div>
          <div class="sim-controls">
            <div class="input-group" style="flex:1;"><textarea class="input-field" id="tsTrace" rows="6" placeholder="192.168.1.1, 2&#10;10.0.0.1, 15&#10;203.0.113.5, 45&#10;8.8.8.8, " spellcheck="false"></textarea></div>
          </div>
          <div class="input-group small" style="margin-top:10px;"><label for="tsDnsTime">DNS resolve (ms)</label><input class="input-field" id="tsDnsTime" type="number" min="0" placeholder="—" /></div>
        </details>

        <details style="margin-bottom:12px;">
          <summary style="cursor:pointer;font-size:12px;font-weight:700;color:var(--text-secondary);text-transform:uppercase;letter-spacing:0.05em;">Bandwidth & Top Talkers</summary>
          <div class="sim-controls" style="margin-top:10px;">
            <div class="input-group small"><label for="tsUplinkCap">Kapasitas uplink (Mbps)</label><input class="input-field" id="tsUplinkCap" type="number" min="0" placeholder="—" /></div>
            <div class="input-group small"><label for="tsUplinkUsage">Trafik saat ini (Mbps)</label><input class="input-field" id="tsUplinkUsage" type="number" min="0" placeholder="—" /></div>
          </div>
          <div class="sim-title" style="margin-top:10px;">Top Talkers</div>
          <div style="overflow-x:auto;">
            <table class="data-table" id="tsTopTalkersTable">
              <thead><tr><th>Perangkat</th><th>Mbps</th><th>Aplikasi/Protokol</th><th>Aksi</th></tr></thead>
              <tbody></tbody>
            </table>
          </div>
          <div class="btn-row" style="margin-top:6px;"><button class="btn-ghost" id="tsAddTalker">Tambah top talker</button></div>
        </details>

        <details style="margin-bottom:12px;">
          <summary style="cursor:pointer;font-size:12px;font-weight:700;color:var(--text-secondary);text-transform:uppercase;letter-spacing:0.05em;">Port/Link & Error Counter</summary>
          <div style="overflow-x:auto;">
            <table class="data-table" id="tsPortTable">
              <thead><tr><th>Port/Link</th><th>Status</th><th>Duplex</th><th>CRC Error</th><th>Input Error</th><th>Collision</th><th>Drop</th><th>Flapping</th></tr></thead>
              <tbody></tbody>
            </table>
          </div>
          <div class="btn-row" style="margin-top:6px;"><button class="btn-ghost" id="tsAddPort">Tambah port</button></div>
        </details>

        <details style="margin-bottom:12px;">
          <summary style="cursor:pointer;font-size:12px;font-weight:700;color:var(--text-secondary);text-transform:uppercase;letter-spacing:0.05em;">Perangkat (CPU/Memori)</summary>
          <div style="overflow-x:auto;">
            <table class="data-table" id="tsDeviceHealthTable">
              <thead><tr><th>Perangkat</th><th>CPU %</th><th>Memori %</th></tr></thead>
              <tbody></tbody>
            </table>
          </div>
          <div class="btn-row" style="margin-top:6px;"><button class="btn-ghost" id="tsAddDeviceHealth">Tambah perangkat</button></div>
        </details>

        <details style="margin-bottom:12px;">
          <summary style="cursor:pointer;font-size:12px;font-weight:700;color:var(--text-secondary);text-transform:uppercase;letter-spacing:0.05em;">Indikator Keamanan</summary>
          <div class="sim-controls" style="margin-top:10px;">
            <div class="input-group small"><label for="tsOutboundSpike">Lonjakan trafik keluar</label><select class="input-field" id="tsOutboundSpike"><option value="">—</option><option value="yes">Ya</option><option value="no">Tidak</option></select></div>
            <div class="input-group small"><label for="tsPortScan">Port scan terdeteksi</label><select class="input-field" id="tsPortScan"><option value="">—</option><option value="yes">Ya</option><option value="no">Tidak</option></select></div>
            <div class="input-group small"><label for="tsSynHalf">SYN setengah terbuka banyak</label><select class="input-field" id="tsSynHalf"><option value="">—</option><option value="yes">Ya</option><option value="no">Tidak</option></select></div>
            <div class="input-group small"><label for="tsForeignIP">Trafik ke IP asing</label><select class="input-field" id="tsForeignIP"><option value="">—</option><option value="yes">Ya</option><option value="no">Tidak</option></select></div>
          </div>
        </details>

        <details style="margin-bottom:12px;">
          <summary style="cursor:pointer;font-size:12px;font-weight:700;color:var(--text-secondary);text-transform:uppercase;letter-spacing:0.05em;">Broadcast/Multicast & STP</summary>
          <div class="sim-controls" style="margin-top:10px;">
            <div class="input-group small"><label for="tsBcastPps">Broadcast pps</label><input class="input-field" id="tsBcastPps" type="number" min="0" placeholder="—" /></div>
            <div class="input-group small"><label for="tsMacFlap">MAC flapping</label><select class="input-field" id="tsMacFlap"><option value="">—</option><option value="yes">Ya</option><option value="no">Tidak</option></select></div>
            <div class="input-group small"><label for="tsStpChange">STP topology change</label><select class="input-field" id="tsStpChange"><option value="">—</option><option value="yes">Ya</option><option value="no">Tidak</option></select></div>
          </div>
        </details>

        <!-- Presets & Actions -->
        <div class="sim-controls" style="margin-top:16px;">
          <div class="input-group small">
            <label for="tsPreset">Preset Skenario</label>
            <select class="input-field" id="tsPreset">
              <option value="">— Pilih preset —</option>
              <option value="uplink_full">Uplink jenuh karena unduhan besar</option>
              <option value="cable_broken">Kabel/port rusak (error CRC)</option>
              <option value="dns_slow">DNS lambat</option>
              <option value="ddos_malware">Serangan DDoS/malware</option>
              <option value="loop_storm">Loop jaringan (broadcast storm)</option>
              <option value="router_cpu">Router CPU tinggi setelah perubahan</option>
              <option value="isp_issue">Masalah ISP</option>
            </select>
          </div>
        </div>
        <div class="btn-row" style="margin-top:8px;">
          <button class="btn-run" id="tsRun">Diagnosa</button>
          <button class="btn-ghost" id="tsExample">Contoh</button>
          <button class="btn-reset" id="tsReset">Reset</button>
          <button class="btn-ghost" id="tsExportJson">Ekspor JSON</button>
          <button class="btn-ghost" id="tsCopyEssay">Salin Jawaban (Esai)</button>
        </div>

        <!-- Mode Latihan -->
        <div class="seg-ctl" style="margin-top:16px;margin-bottom:16px;">
          <button id="tsPracticeMode" class="">Latihan (Step-by-step)</button>
        </div>
        <div id="tsPracticeFeedback" style="padding:16px;background:var(--accent-soft);border:1px solid var(--accent-border);border-radius:var(--radius-sm);margin-bottom:16px;"></div>

        <!-- Outputs -->
        <div class="sim-title" style="margin-top:16px;">Flowchart Troubleshooting Interaktif</div>
        <div class="scene-wrap" style="min-height:420px;">
          <div id="tsFlowchart"></div>
        </div>
        <div class="detail-panel" id="tsStepDetail">
          <h4 id="tsStepTitle">Detail Langkah</h4>
          <div class="detail-grid" id="tsStepGrid"></div>
        </div>

        <div class="sim-title" style="margin-top:20px;">C. Grafik Latency per Hop (Traceroute)</div>
        <div class="chart-wrap"><svg id="tsTraceChart" viewBox="0 0 100 42" preserveAspectRatio="none" role="img" aria-label="Latency per hop"></svg></div>

        <div class="sim-title" style="margin-top:20px;">D. Peringkat Penyebab (Hipotesis)</div>
        <div style="overflow-x:auto;">
          <table class="data-table" id="tsHypothesisTable">
            <thead><tr><th>#</th><th>Hipotesis</th><th>Keyakinan</th><th>Bukti Pendukung</th><th>Bukti Penyangkal</th><th>Belum Dinilai</th></tr></thead>
            <tbody></tbody>
          </table>
        </div>

        <div class="sim-title" style="margin-top:20px;">E. Rekomendasi Tindakan</div>
        <div class="analysis-box" id="tsRecommendations"></div>

        <div class="sim-title" style="margin-top:20px;">F. Jawaban Soal (Esai Terstruktur)</div>
        <div class="analysis-box" id="tsEssay"></div>
        <div class="btn-row" style="margin-top:12px;"><button class="btn-run" id="tsCopyEssay">Salin Jawaban</button></div>

        <!-- 3D/2D Scene -->
        <div class="sim-title" style="margin-top:20px;">B. Adegan 3D — Visualisasi Penyebab</div>
        <div class="scene-toolbar">
          <button class="btn-ghost active" id="tsView3d">3D</button>
          <button class="btn-ghost" id="tsView2d">Mode 2D (SVG)</button>
          <button class="btn-ghost" id="tsViewTable">Tabel Aksesibel</button>
          <span class="spacer"></span>
          <button class="btn-ghost" id="tsAnimPlay" style="display:none;">Play</button>
          <button class="btn-ghost" id="tsAnimPause" style="display:none;">Pause</button>
          <button class="btn-ghost" id="tsAnimRestart" style="display:none;">Ulangi</button>
          <div class="input-group small" style="min-width:140px;display:none;" id="tsSpeedWrap"><label for="tsAnimSpeed">Kecepatan</label><select class="input-field" id="tsAnimSpeed"><option value="0.5">0.5x</option><option value="1" selected>1x</option><option value="2">2x</option><option value="4">4x</option></select></div>
        </div>
        <div class="scene-wrap">
          <div id="tsScene3d"></div>
          <div id="tsScene2d" class="scene-hidden"></div>
          <div class="label-layer" id="tsLabelLayer"></div>
        </div>
        <div class="legend" aria-hidden="true" id="tsLegend"></div>
        <div class="detail-panel" id="tsDetailPanel">
          <h4 id="tsDetailTitle">Detail</h4>
          <div class="detail-grid" id="tsDetailGrid"></div>
        </div>
        <div id="tsTableWrap" style="display:none; margin-top:16px; overflow-x:auto;">
          <table class="data-table" id="tsAccessTable">
            <thead><tr><th>Perangkat</th><th>Status</th><th>Hipotesis Teratas</th><th>Keyakinan</th></tr></thead>
            <tbody></tbody>
          </table>
        </div>
      </div>
    </div>

<script>
/* =====================================================================
   Modul 3 — Troubleshooting: Pure Diagnostic Engine
   Rule-based diagnosis with traceroute analysis, flowchart, and essay generation.
   ===================================================================== */
(function () {
  'use strict';

  // ---- Hypotheses with rule configurations ----
  var HYPOTHESES = [
    {
      id: 'bandwidth',
      name: 'Kapasitas/Bandwidth Jenuh',
      rules: [
        { key: 'uplinkUtil', op: 'gte', threshold: 90, weight: 35, desc: 'Utilisasi uplink ≥ 90%' },
        { key: 'topTalkerDominant', op: 'gte', threshold: 50, weight: 25, desc: 'Satu top talker > 50% trafik' },
        { key: 'latencyAllHigh', op: 'eq', threshold: true, weight: 15, desc: 'Latency naik merata ke semua tujuan' },
        { key: 'portErrorsHigh', op: 'eq', threshold: true, weight: -20, desc: 'Error port tinggi (penyangkal)' },
        { key: 'uplinkUtilLow', op: 'lte', threshold: 60, weight: -40, desc: 'Utilisasi < 60% (penyangkal)' }
      ]
    },
    {
      id: 'cable',
      name: 'Kabel/Port/Perangkat Keras',
      rules: [
        { key: 'crcHigh', op: 'eq', threshold: true, weight: 40, desc: 'CRC/error tinggi atau flapping' },
        { key: 'duplexMismatch', op: 'eq', threshold: true, weight: 35, desc: 'Duplex mismatch' },
        { key: 'limitedScope', op: 'eq', threshold: true, weight: 15, desc: 'Dampak terbatas satu perangkat/segmen' },
        { key: 'utilHigh', op: 'eq', threshold: true, weight: -10, desc: 'Utilisasi tinggi (penyangkal)' }
      ]
    },
    {
      id: 'dns',
      name: 'DNS',
      rules: [
        { key: 'dnsSlow', op: 'gte', threshold: 500, weight: 45, desc: 'Waktu resolusi DNS > 500 ms' },
        { key: 'pingIpOkNameSlow', op: 'eq', threshold: true, weight: 30, desc: 'Ping IP normal, akses nama lambat' },
        { key: 'pingNetBad', op: 'eq', threshold: true, weight: -30, desc: 'Ping internet buruk (penyangkal)' }
      ]
    },
    {
      id: 'overload',
      name: 'Perangkat Overload (CPU/Memori)',
      rules: [
        { key: 'cpuHigh', op: 'gte', threshold: 85, weight: 40, desc: 'CPU/memori ≥ 85%' },
        { key: 'latencyAtHop', op: 'eq', threshold: true, weight: 25, desc: 'Latency naik pada hop perangkat itu' },
        { key: 'latencyOtherHigh', op: 'eq', threshold: true, weight: -10, desc: 'Pada hop lain tinggi (penyangkal)' }
      ]
    },
    {
      id: 'attack',
      name: 'Serangan/Malware',
      rules: [
        { key: 'connSpike', op: 'eq', threshold: true, weight: 30, desc: 'Lonjakan koneksi/trafik abnormal' },
        { key: 'synHalfOpen', op: 'eq', threshold: true, weight: 30, desc: 'Banyak SYN setengah terbuka/port scan' },
        { key: 'foreignIP', op: 'eq', threshold: true, weight: 25, desc: 'Trafik ke IP asing' },
        { key: 'fwCpuHigh', op: 'eq', threshold: true, weight: 10, desc: 'CPU firewall tinggi' }
      ]
    },
    {
      id: 'loop',
      name: 'Loop/Broadcast Storm',
      rules: [
        { key: 'bcastHigh', op: 'gte', threshold: 1000, weight: 40, desc: 'Broadcast pps tinggi' },
        { key: 'macFlapStp', op: 'eq', threshold: true, weight: 35, desc: 'MAC flapping / STP topology change' },
        { key: 'swCpuHigh', op: 'eq', threshold: true, weight: 15, desc: 'CPU switch tinggi' },
        { key: 'segmentWide', op: 'eq', threshold: true, weight: 10, desc: 'Dampak seluruh segmen' }
      ]
    },
    {
      id: 'isp',
      name: 'Masalah ISP',
      rules: [
        { key: 'latLossExternal', op: 'eq', threshold: true, weight: 45, desc: 'Latency/loss mulai hop di luar jaringan internal' },
        { key: 'internalNormal', op: 'eq', threshold: true, weight: 25, desc: 'Internal normal' },
        { key: 'internetOnly', op: 'eq', threshold: true, weight: 15, desc: 'Masalah hanya ke internet, bukan internal' }
      ]
    },
    {
      id: 'config',
      name: 'Salah Konfigurasi/Perubahan Terbaru',
      rules: [
        { key: 'recentChange', op: 'eq', threshold: true, weight: 40, desc: 'Perubahan terbaru dicentang & waktu dekat' },
        { key: 'cpuErrorAfterChange', op: 'eq', threshold: true, weight: 20, desc: 'CPU/kesalahan setelah perubahan' },
        { key: 'noChange', op: 'eq', threshold: true, weight: -30, desc: 'Tidak ada perubahan (penyangkal)' }
      ]
    }
  ];

  var FLOWCHART_STEPS = [
    { num: 1, key: 'scope', title: 'Definisikan masalah & ruang lingkup', checks: ['scope'] },
    { num: 2, key: 'physical', title: 'Cek lapisan fisik (link, kabel, port, error counter)', checks: ['crcHigh', 'duplexMismatch', 'flapping'] },
    { num: 3, key: 'bandwidth', title: 'Cek utilisasi bandwidth & top talkers', checks: ['uplinkUtil', 'topTalkerDominant'] },
    { num: 4, key: 'latency', title: 'Ukur latency & packet loss (ping, traceroute) untuk isolasi segmen', checks: ['pingGw', 'pingDns', 'pingNet', 'trace'] },
    { num: 5, key: 'dns', title: 'Cek DNS', checks: ['dnsTime'] },
    { num: 6, key: 'device', title: 'Cek kesehatan perangkat (CPU, memori, log)', checks: ['cpuHigh', 'memHigh'] },
    { num: 7, key: 'security', title: 'Cek indikasi keamanan', checks: ['connSpike', 'synHalfOpen', 'foreignIP', 'portScan'] },
    { num: 8, key: 'change', title: 'Tinjau perubahan terbaru & rollback bila perlu', checks: ['recentChange'] },
    { num: 9, key: 'fix', title: 'Terapkan perbaikan, verifikasi dengan metrik, dokumentasikan', checks: [] }
  ];

  // ---- Helpers ----
  function hasValue(v) { return v !== undefined && v !== null && v !== ''; }
  function num(v) { return hasValue(v) ? Number(v) : null; }
  function bool(v) { return v === true || v === 'yes' || v === 'true'; }
  function clamp(v, min, max) { return Math.max(min, Math.min(max, v)); }

  // ---- Extract evaluation context from input ----
  function buildContext(input) {
    var ctx = {};
    ctx.pingGw = num(input.pingGw); ctx.pingGwLoss = num(input.pingGwLoss);
    ctx.pingDns = num(input.pingDns); ctx.pingDnsLoss = num(input.pingDnsLoss);
    ctx.pingNet = num(input.pingNet); ctx.pingNetLoss = num(input.pingNetLoss);
    ctx.dnsTime = num(input.dnsTime);
    ctx.uplinkCap = num(input.uplinkCap); ctx.uplinkUsage = num(input.uplinkUsage);
    ctx.uplinkUtil = (ctx.uplinkCap && ctx.uplinkUsage) ? (ctx.uplinkUsage / ctx.uplinkCap * 100) : null;
    var maxTalker = 0, totalTalker = 0;
    (input.topTalkers || []).forEach(function (t) { var v = num(t.mbps); if (v) { totalTalker += v; if (v > maxTalker) maxTalker = v; } });
    ctx.topTalkerDominant = totalTalker > 0 ? (maxTalker / totalTalker * 100) : null;
    var hasCrc = false, hasDuplex = false, hasFlap = false;
    (input.ports || []).forEach(function (p) {
      if (num(p.crc) > 100 || num(p.inputErr) > 100 || num(p.collision) > 50 || num(p.drop) > 50) hasCrc = true;
      if (p.duplex === 'mismatch' || p.duplex === 'half') hasDuplex = true;
      if (bool(p.flapping)) hasFlap = true;
    });
    ctx.crcHigh = hasCrc; ctx.duplexMismatch = hasDuplex; ctx.flapping = hasFlap;
    var maxCpu = 0, maxMem = 0;
    (input.deviceHealth || []).forEach(function (d) { var c = num(d.cpu); var m = num(d.mem); if (c > maxCpu) maxCpu = c; if (m > maxMem) maxMem = m; });
    ctx.cpuHigh = maxCpu >= 85; ctx.memHigh = maxMem >= 85; ctx.maxCpu = maxCpu || null; ctx.maxMem = maxMem || null;
    ctx.connSpike = bool(input.outboundSpike) || bool(input.portScan);
    ctx.synHalfOpen = bool(input.synHalfOpen); ctx.foreignIP = bool(input.foreignIP); ctx.portScan = bool(input.portScan); ctx.fwCpuHigh = false;
    ctx.bcastHigh = num(input.bcastPps) > 1000; ctx.macFlapStp = bool(input.macFlap) || bool(input.stpChange); ctx.swCpuHigh = false;
    var changes = input.changes || {};
    ctx.recentChange = (changes.config || changes.firmware || changes.newdevice || changes.cable || changes.backup) && input.onset <= 1440;
    ctx.cpuErrorAfterChange = ctx.recentChange && (ctx.cpuHigh || hasCrc); ctx.noChange = !(changes.config || changes.firmware || changes.newdevice || changes.cable || changes.backup);
    ctx.internetOnly = input.coverage === 'internet'; ctx.bothCoverage = input.coverage === 'both'; ctx.internalOnly = input.coverage === 'internal';
    ctx.limitedScope = input.scope === 'device' || input.scope === 'segment'; ctx.segmentWide = input.scope === 'all' || input.scope === 'segment';
    ctx.trace = parseTraceroute(input.traceroute);
    ctx.latencyAllHigh = ctx.trace && ctx.trace.hops.length >= 2 && ctx.trace.hops.every(function (h) { return h.latency && h.latency > 50; });
    ctx.latencyAtHop = ctx.trace && ctx.trace.problemHop !== null; ctx.latencyOtherHigh = ctx.trace && ctx.trace.problemHop !== null && ctx.trace.problemHop > 1;
    ctx.latLossExternal = ctx.trace && ctx.trace.problemHop !== null && ctx.trace.problemHop > (ctx.trace.internalHopCount || 99);
    ctx.internalNormal = ctx.trace && ctx.trace.problemHop !== null && ctx.trace.problemHop > (ctx.trace.internalHopCount || 99) && ctx.trace.hops.slice(0, ctx.trace.internalHopCount).every(function (h) { return h.latency && h.latency < 50; });
    ctx.pingIpOkNameSlow = hasValue(ctx.pingNet) && ctx.pingNet < 100 && hasValue(ctx.dnsTime) && ctx.dnsTime > 500;
    ctx.pingNetBad = hasValue(ctx.pingNet) && ctx.pingNet > 200; ctx.uplinkUtilLow = ctx.uplinkUtil !== null && ctx.uplinkUtil < 60; ctx.utilHigh = ctx.uplinkUtil !== null && ctx.uplinkUtil > 80;
    return ctx;
  }

  function parseTraceroute(text) {
    if (!text || !text.trim()) return null;
    var lines = text.trim().split(/\n/); var hops = []; var internalCount = 0;
    for (var i = 0; i < lines.length; i++) {
      var line = lines[i].trim(); if (!line) continue;
      var parts = line.split(',').map(function (s) { return s.trim(); });
      var name = parts[0] || 'hop' + (i+1); var lat = parts[1] ? Number(parts[1]) : null;
      var isPrivate = isPrivateIP(name); if (isPrivate) internalCount++;
      hops.push({ index: i+1, name: name, latency: lat, isPrivate: isPrivate, timeout: lat === null });
    }
    var problemHop = null;
    for (var j = 1; j < hops.length; j++) {
      var prev = hops[j-1], cur = hops[j];
      if (cur.timeout && prev.timeout) { problemHop = cur.index; break; }
      if (cur.latency !== null && prev.latency !== null) { if (cur.latency >= 50 && cur.latency >= 3 * prev.latency) { problemHop = cur.index; break; } }
    }
    return { hops: hops, problemHop: problemHop, internalHopCount: internalCount };
  }
  function isPrivateIP(str) { var ip = str.match(/(\d+\.\d+\.\d+\.\d+)/); if (!ip) return false; var parts = ip[1].split('.').map(Number); return parts[0] === 10 || (parts[0] === 172 && parts[1] >= 16 && parts[1] <= 31) || (parts[0] === 192 && parts[1] === 168); }

  function evaluateHypotheses(ctx) {
    var results = HYPOTHESES.map(function (h) {
      var score = 0; var supporting = [], refuting = [], pending = [];
      h.rules.forEach(function (rule) {
        var val = ctx[rule.key]; var evaluated = false;
        if (rule.op === 'gte' && val !== null) { evaluated = true; if (val >= rule.threshold) { score += rule.weight; supporting.push({ rule: rule.desc, value: val }); } else { refuting.push({ rule: rule.desc, value: val }); } }
        else if (rule.op === 'lte' && val !== null) { evaluated = true; if (val <= rule.threshold) { score += rule.weight; supporting.push({ rule: rule.desc, value: val }); } else { refuting.push({ rule: rule.desc, value: val }); } }
        else if (rule.op === 'eq' && val !== null) { evaluated = true; if (val === rule.threshold) { score += rule.weight; supporting.push({ rule: rule.desc, value: val }); } else { refuting.push({ rule: rule.desc, value: val }); } }
        if (!evaluated) pending.push({ rule: rule.desc, weight: rule.weight });
      });
      return { id: h.id, name: h.name, score: clamp(score, 0, 100), supporting: supporting, refuting: refuting, pending: pending };
    });
    results.sort(function (a, b) { return b.score - a.score; }); return results;
  }

  function assessSufficiency(ctx, hypotheses) {
    var filledCols = 0; var colsToCheck = ['pingGw', 'pingDns', 'pingNet', 'dnsTime', 'uplinkUtil', 'topTalkerDominant', 'crcHigh', 'duplexMismatch', 'cpuHigh', 'bcastHigh', 'macFlapStp', 'trace'];
    colsToCheck.forEach(function (k) { if (ctx[k] !== null && ctx[k] !== undefined) filledCols++; });
    var top1 = hypotheses[0] || { score: 0 }; var top2 = hypotheses[1] || { score: 0 }; var gap = top1.score - top2.score;
    if (filledCols < 3 || top1.score < 40 || gap < 10) {
      return { sufficient: false, reason: filledCols < 3 ? 'Data terlalu sedikit (minimal 3 kolom)' : (top1.score < 40 ? 'Keyakinan terlalu rendah (< 40)' : 'Selisih skor terlalu kecil (< 10)'), filledCols: filledCols };
    }
    return { sufficient: true, filledCols: filledCols };
  }

  function suggestNextMeasurements(hypotheses, ctx) {
    var top = hypotheses.slice(0, 2); var suggestions = [];
    top.forEach(function (h) { h.pending.sort(function (a, b) { return Math.abs(b.weight) - Math.abs(a.weight); }); h.pending.slice(0, 2).forEach(function (p) { suggestions.push({ hypothesis: h.name, measurement: p.rule, weight: Math.abs(p.weight) }); }); });
    suggestions.sort(function (a, b) { return b.weight - a.weight; }); return suggestions.slice(0, 3).map(function (s) { return s.hypothesis + ': ' + s.measurement; });
  }

  function getStepStatus(step, ctx) {
    var checked = 0, abnormal = 0;
    step.checks.forEach(function (ck) {
      var val = ctx[ck];
      if (val !== null && val !== undefined) { checked++;
        if (ck === 'uplinkUtil' && val > 80) abnormal++;
        else if (ck === 'topTalkerDominant' && val > 50) abnormal++;
        else if (['crcHigh', 'duplexMismatch', 'flapping', 'cpuHigh', 'memHigh', 'connSpike', 'synHalfOpen', 'foreignIP', 'portScan', 'bcastHigh', 'macFlapStp', 'recentChange'].indexOf(ck) >= 0 && val === true) abnormal++;
        else if (ck === 'dnsTime' && val > 500) abnormal++;
        else if (['pingGw', 'pingDns', 'pingNet'].indexOf(ck) >= 0 && val > 100) abnormal++;
      }
    });
    if (checked === 0) return 'pending'; return abnormal > 0 ? 'fail' : 'pass';
  }

  var RECOMMENDATIONS = {
    bandwidth: { quick: ['Batasi bandwidth top talker via QoS/policing', 'Blokir/limit aplikasi non-bisnis (streaming, download besar)', 'Aktifkan compression dan caching di proxy/WAN optimizer'], long: ['Upgrade kapasitas uplink ISP', 'Implementasi monitoring bandwidth per aplikasi/user', 'Rancang QoS hierarkis untuk prioritas bisnis kritis'] },
    cable: { quick: ['Ganti kabel patch yang bermasalah', 'Perbaiki duplex mismatch (set full-duplex both sides)', 'Pindahkan ke port cadangan di switch'], long: ['Audit fisik seluruh kabel struktur', 'Implementasi monitoring error counter otomatis (SNMP)', 'Standarisasi tipe kabel (Cat6/6a minimum)'] },
    dns: { quick: ['Ganti ke DNS publik cepat (1.1.1.1, 8.8.8.8) sementara', 'Flush cache DNS client & server', 'Periksa forwarder DNS internal'], long: ['Deploy DNS resolver lokal dengan caching (Unbound/PowerDNS)', 'Monitoring latency DNS berkelanjutan', 'Anycast DNS untuk redudansi'] },
    overload: { quick: ['Restart layanan/proses yang menghabiskan CPU', 'Pindahkan beban ke perangkat lain (HA/failover)', 'Nonaktifkan fitur non-esensial (deep packet inspection, logging verbose)'], long: ['Upgrade hardware (CPU/RAM) atau scale-out', 'Optimasi konfigurasi (ACL, routing table, NAT pool)', 'Kapasi planning & alerting CPU/memori > 70%'] },
    attack: { quick: ['Blokir IP sumber anomali di firewall/edge', 'Rate-limit koneksi baru (SYN flood protection)', 'Isolasi perangkat terinfeksi (quarantine VLAN)'], long: ['Deploy IDS/IPS dengan signature & behavioral', 'Network segmentation & zero trust', 'Threat intelligence feed & automated response'] },
    loop: { quick: ['Shutdown port yang MAC flapping', 'Aktifkan BPDU Guard / Root Guard / Loop Guard', 'Periksa kabel ganda / hub tidak terkelola'], long: ['Desain topologi loop-free (STP/RSTP/MSTP tuning)', 'Implementasi storm control (broadcast/multicast/unknown-unicast)', 'Monitoring STP topology change notification'] },
    isp: { quick: ['Hubungi ISP dengan bukti traceroute & ping', 'Aktifkan link backup (failover) jika ada', 'Ganti DNS ke resolver publik bypass ISP'], long: ['Negosiasi SLA ketat dengan penalty', 'Dual-homed ke ISP berbeda (BGP multihoming)', 'Monitoring eksternal kontinu (RIPE Atlas, ThousandEyes)'] },
    config: { quick: ['Rollback perubahan terakhir (config versioning)', 'Verifikasi sintaks & logika konfigurasi baru', 'Test di lab/staging sebelum production'], long: ['Implementasi CI/CD untuk konfigurasi jaringan (GitOps/NetDevOps)', 'Change management board & rollback plan wajib', 'Automated config validation (Batfish, pyATS)'] }
  };

  function generateEssay(input, ctx, hypotheses, stepStatuses, sufficiency, suggestions, trace) {
    var parts = []; var top = hypotheses[0] || { name: '—', score: 0, supporting: [], refuting: [], pending: [] }; var top2 = hypotheses[1] || { name: '—', score: 0 };
    parts.push({ title: 'Pendahuluan', text: 'Administrator jaringan harus mengikuti pendekatan sistematis (layered troubleshooting) untuk mengidentifikasi penyebab penurunan kecepatan jaringan. Metodologi ini memulai dari definisi ruang lingkup, kemudian memeriksa lapisan fisik, bandwidth, latency/traceroute, DNS, kesehatan perangkat, keamanan, dan perubahan terbaru — mengikuti model OSI dari bawah ke atas.' });
    parts.push({ title: 'Langkah-langkah Diagnosa & Temuan', text: '' });
    FLOWCHART_STEPS.forEach(function (step, idx) {
      var status = stepStatuses[step.key]; var label = status === 'fail' ? '⚠ DITEMUKAN MASALAH' : (status === 'pass' ? '✓ Lolos' : '○ Belum diukur'); var details = [];
      step.checks.forEach(function (ck) { var val = ctx[ck]; if (val !== null && val !== undefined) { var desc = ''; if (ck === 'uplinkUtil') desc = 'Utilisasi uplink = ' + val.toFixed(1) + '%'; else if (ck === 'topTalkerDominant') desc = 'Top talker dominan = ' + val.toFixed(1) + '%'; else if (ck === 'crcHigh' && val) desc = 'CRC/error tinggi terdeteksi'; else if (ck === 'duplexMismatch' && val) desc = 'Duplex mismatch'; else if (ck === 'flapping' && val) desc = 'Port flapping'; else if (ck === 'cpuHigh' && val) desc = 'CPU ≥ 85% (puncak ' + (ctx.maxCpu || '?') + '%)'; else if (ck === 'memHigh' && val) desc = 'Memori ≥ 85% (puncak ' + (ctx.maxMem || '?') + '%)'; else if (ck === 'dnsTime' && val) desc = 'DNS resolve = ' + val + ' ms'; else if (['pingGw', 'pingDns', 'pingNet'].indexOf(ck) >= 0 && val) desc = ck.replace('ping','Ping ') + ' = ' + val + ' ms'; else if (ck === 'bcastHigh' && val) desc = 'Broadcast pps tinggi'; else if (ck === 'macFlapStp' && val) desc = 'MAC flapping/STP change'; else if (ck === 'connSpike' && val) desc = 'Lonjakan trafik keluar/port scan'; else if (ck === 'synHalfOpen' && val) desc = 'SYN half-open banyak'; else if (ck === 'foreignIP' && val) desc = 'Trafik ke IP asing'; else if (ck === 'portScan' && val) desc = 'Port scan terdeteksi'; else if (ck === 'recentChange' && val) desc = 'Perubahan terbaru ada (dekat waktu mulai gejala)'; if (desc) details.push(desc); } });
      var stepText = '<b>Langkah ' + step.num + ': ' + step.title + '</b> — ' + label; if (details.length) stepText += '<br><small>' + details.join('; ') + '</small>'; parts.push({ title: '', text: stepText });
    });
    if (sufficiency.sufficient) {
      parts.push({ title: 'Dugaan Penyebab Teratas', text: 'Berdasarkan analisis bukti, hipotesis terkuat adalah <b>' + top.name + '</b> dengan tingkat keyakinan <b>' + top.score + '%</b>.' + (top2.score > 0 ? ' Hipotesis kedua: ' + top2.name + ' (' + top2.score + '%).' : '') + (top.supporting.length ? ' Bukti pendukung: ' + top.supporting.map(function (s) { return s.rule + ' (' + (typeof s.value === 'number' ? s.value.toFixed(1) : s.value) + ')'; }).join('; ') : '') + (top.refuting.length ? ' Bukti penyangkal: ' + top.refuting.map(function (r) { return r.rule; }).join('; ') : '') });
    } else {
      parts.push({ title: 'Data Belum Cukup / Ambigu', text: 'Analisis tidak dapat menentukan satu penyebab pasti karena ' + sufficiency.reason + '. Hipotesis teratas saat ini: ' + top.name + ' (' + top.score + '%)' + (top2.score > 0 ? ', diikuti ' + top2.name + ' (' + top2.score + '%)' : '') + '. Pengukuran prioritas berikutnya: ' + suggestions.join('; ') + '.' });
    }
    if (trace && trace.hops.length) {
      var loc = trace.problemHop !== null ? (trace.problemHop <= (trace.internalHopCount || 99) ? 'internal (jaringan sendiri)' : 'eksternal (ISP/upstream)') : 'tidak teridentifikasi';
      parts.push({ title: 'Isolasi Segmen via Traceroute', text: 'Traceroute menunjukkan ' + trace.hops.length + ' hop. Titik awal masalah (latency melonjak ≥3× atau timeout beruntun) berada pada hop ke-' + (trace.problemHop || 'tidak ada') + ' (' + loc + ').' + (trace.problemHop && trace.problemHop <= (trace.internalHopCount || 99) ? ' Ini mengarah ke perangkat internal — periksa switch/router pada hop tersebut.' : '') + (trace.problemHop && trace.problemHop > (trace.internalHopCount || 99) ? ' Ini mengarah ke ISP/upstream — hubungi penyedia layanan dengan bukti traceroute.' : '') });
    }
    if (sufficiency.sufficient && RECOMMENDATIONS[top.id]) { var rec = RECOMMENDATIONS[top.id]; var recText = 'Untuk <b>' + top.name + '</b>:<br><b>Tindakan cepat:</b> ' + rec.quick.join('; ') + '.<br><b>Jangka panjang:</b> ' + rec.long.join('; ') + '.'; parts.push({ title: 'Rekomendasi Tindakan', text: recText }); }
    else if (!sufficiency.sufficient) { parts.push({ title: 'Rekomendasi Tindakan', text: 'Kumpulkan data yang disarankan di atas, lalu jalankan diagnosa ulang. Prioritaskan pengukuran yang membedakan hipotesis teratas.' }); }
    var concl = 'Kesimpulannya, troubleshooting jaringan lambat harus sistematis: mulai dari ruang lingkup (siapa, kapan, cakupan), cek fisik (kabel, port, error counter), bandwidth & top talkers, latency/traceroute untuk isolasi segmen, DNS, kesehatan perangkat (CPU/memori), indikasi keamanan, dan review perubahan terbaru. ' + (sufficiency.sufficient ? 'Data mendukung <b>' + top.name + '</b> (' + top.score + '%) sebagai penyebab utama. ' : 'Data saat ini belum cukup untuk kesimpulan pasti; ' + suggestions.length + ' pengukuran prioritas diidentifikasi. ') + 'Dokumentasikan temuan, terapkan perbaikan, verifikasi dengan metrik sebelum/sesudah, dan update knowledge base.'; parts.push({ title: 'Kesimpulan', text: concl, isConclusion: true });
    return parts;
  }

  window.TroubleEngine = { HYPOTHESES: HYPOTHESES, FLOWCHART_STEPS: FLOWCHART_STEPS, RECOMMENDATIONS: RECOMMENDATIONS, buildContext: buildContext, parseTraceroute: parseTraceroute, evaluateHypotheses: evaluateHypotheses, assessSufficiency: assessSufficiency, suggestNextMeasurements: suggestNextMeasurements, getStepStatus: getStepStatus, generateEssay: generateEssay };
})();
</script>

<script>
/* =====================================================================
   Modul 3 — Troubleshooting: UI Logic & 3D/2D Visualization
   ===================================================================== */
(function () {
  'use strict';
  var $ = function (id) { return document.getElementById(id); }; var TE = window.TroubleEngine; var NetApp = window.NetApp;
  var state = { scope: 'all', scopeDevice: null, onset: 30, pattern: 'sudden', coverage: 'internet', changes: { config: false, firmware: false, newdevice: false, cable: false, backup: false }, changeNotes: '', pingGw: null, pingGwLoss: null, pingDns: null, pingDnsLoss: null, pingNet: null, pingNetLoss: null, traceroute: '', dnsTime: null, uplinkCap: null, uplinkUsage: null, topTalkers: [], ports: [], deviceHealth: [], outboundSpike: null, portScan: null, synHalfOpen: null, foreignIP: null, bcastPps: null, macFlap: null, stpChange: null, mode: 'normal', practiceStep: 0, practiceRevealed: {}, lastResults: null };
  var PRESETS = { uplink_full: { scope: 'all', onset: 10, pattern: 'sudden', coverage: 'internet', pingGw: 2, pingGwLoss: 0, pingDns: 5, pingDnsLoss: 0, pingNet: 120, pingNetLoss: 1, traceroute: '192.168.1.1, 2\n10.0.0.1, 15\n203.0.113.5, 80\n8.8.8.8, 120', dnsTime: 50, uplinkCap: 1000, uplinkUsage: 960, topTalkers: [{device: 'PC-5', mbps: 650, app: 'Backup/Download'}, {device: 'Server-1', mbps: 200, app: 'DB Sync'}], ports: [{status: 'up', duplex: 'full', crc: 0, inputErr: 0, collision: 0, drop: 0, flapping: false}], deviceHealth: [{cpu: 30, mem: 40}], outboundSpike: 'yes', portScan: 'no', synHalfOpen: 'no', foreignIP: 'no', bcastPps: 100, macFlap: 'no', stpChange: 'no' }, cable_broken: { scope: 'segment', onset: 5, pattern: 'sudden', coverage: 'both', pingGw: 5, pingGwLoss: 15, pingDns: 50, pingDnsLoss: 20, pingNet: 200, pingNetLoss: 30, traceroute: '192.168.1.1, 2\n192.168.10.1, 150\n10.0.0.1, ', dnsTime: 60, uplinkCap: 1000, uplinkUsage: 200, topTalkers: [], ports: [{status: 'up', duplex: 'full', crc: 5000, inputErr: 1200, collision: 0, drop: 0, flapping: true}], deviceHealth: [{cpu: 20, mem: 30}], outboundSpike: 'no', portScan: 'no', synHalfOpen: 'no', foreignIP: 'no', bcastPps: 50, macFlap: 'no', stpChange: 'no' }, dns_slow: { scope: 'all', onset: 60, pattern: 'gradual', coverage: 'internet', pingGw: 2, pingGwLoss: 0, pingDns: 10, pingDnsLoss: 0, pingNet: 25, pingNetLoss: 0, traceroute: '192.168.1.1, 2\n10.0.0.1, 10\n8.8.8.8, 25', dnsTime: 2500, uplinkCap: 1000, uplinkUsage: 150, topTalkers: [], ports: [{status: 'up', duplex: 'full', crc: 0, inputErr: 0, collision: 0, drop: 0, flapping: false}], deviceHealth: [{cpu: 15, mem: 25}], outboundSpike: 'no', portScan: 'no', synHalfOpen: 'no', foreignIP: 'no', bcastPps: 20, macFlap: 'no', stpChange: 'no' }, ddos_malware: { scope: 'all', onset: 15, pattern: 'sudden', coverage: 'internet', pingGw: 5, pingGwLoss: 5, pingDns: 50, pingDnsLoss: 10, pingNet: 300, pingNetLoss: 20, traceroute: '192.168.1.1, 5\n10.0.0.1, 50\n203.0.113.5, 300', dnsTime: 50, uplinkCap: 1000, uplinkUsage: 850, topTalkers: [{device: 'PC-3', mbps: 800, app: 'Unknown/High'}], ports: [{status: 'up', duplex: 'full', crc: 0, inputErr: 0, collision: 0, drop: 0, flapping: false}], deviceHealth: [{cpu: 90, mem: 85}], outboundSpike: 'yes', portScan: 'yes', synHalfOpen: 'yes', foreignIP: 'yes', bcastPps: 500, macFlap: 'no', stpChange: 'no' }, loop_storm: { scope: 'all', onset: 2, pattern: 'sudden', coverage: 'both', pingGw: 200, pingGwLoss: 30, pingDns: 300, pingDnsLoss: 40, pingNet: 500, pingNetLoss: 50, traceroute: '192.168.1.1, 200\n192.168.2.1, 300\n10.0.0.1, 500', dnsTime: 50, uplinkCap: 1000, uplinkUsage: 990, topTalkers: [], ports: [{status: 'flapping', duplex: 'full', crc: 0, inputErr: 0, collision: 0, drop: 0, flapping: true}, {status: 'flapping', duplex: 'full', crc: 0, inputErr: 0, collision: 0, drop: 0, flapping: true}], deviceHealth: [{cpu: 95, mem: 90}], outboundSpike: 'no', portScan: 'no', synHalfOpen: 'no', foreignIP: 'no', bcastPps: 50000, macFlap: 'yes', stpChange: 'yes' }, config_change: { scope: 'all', onset: 5, pattern: 'sudden', coverage: 'both', pingGw: 100, pingGwLoss: 10, pingDns: 150, pingDnsLoss: 15, pingNet: 300, pingNetLoss: 25, traceroute: '192.168.1.1, 100\n10.0.0.1, 150\n203.0.113.5, 300', dnsTime: 50, uplinkCap: 1000, uplinkUsage: 300, topTalkers: [], ports: [{status: 'up', duplex: 'full', crc: 200, inputErr: 50, collision: 10, drop: 5, flapping: false}], deviceHealth: [{cpu: 85, mem: 60}], outboundSpike: 'no', portScan: 'no', synHalfOpen: 'no', foreignIP: 'no', bcastPps: 100, macFlap: 'no', stpChange: 'no', changes: { config: true } } };
  function showError(msgs) { var box = $('tsErr'); if (!msgs || !msgs.length) { box.classList.remove('show'); box.innerHTML = ''; return; } var html = '<strong>Input tidak valid:</strong><ul>'; msgs.forEach(function (m) { html += '<li>' + m + '</li>'; }); box.innerHTML = html + '</ul>'; box.classList.add('show'); }
  function validate() { var errs = []; [state.pingGwLoss, state.pingDnsLoss, state.pingNetLoss].forEach(function (v) { if (v !== null && (v < 0 || v > 100)) errs.push('Packet loss harus 0-100%'); }); if (state.uplinkCap && state.topTalkers.length) { var sum = state.topTalkers.reduce(function (a, t) { return a + (num(t.mbps) || 0); }, 0); if (sum > state.uplinkCap * 1.1) errs.push('Total top talker (' + sum + ' Mbps) melebihi kapasitas uplink (' + state.uplinkCap + ' Mbps) — peringatan'); } if (state.traceroute) { var names = []; state.traceroute.trim().split(/\n/).forEach(function (line, i) { var p = line.split(',')[0]?.trim(); if (p && names.indexOf(p) >= 0) errs.push('Nama hop duplikat di baris ' + (i+1) + ': ' + p); if (p) names.push(p); }); } var topoDevices = NetApp.get().devices.map(function (d) { return d.name; }); state.deviceHealth.forEach(function (d) { if (d.device && topoDevices.indexOf(d.device) < 0) errs.push('Perangkat health "' + d.device + '" tidak ada di topologi'); }); state.topTalkers.forEach(function (t) { if (t.device && topoDevices.indexOf(t.device) < 0) errs.push('Perangkat top talker "' + t.device + '" tidak ada di topologi'); }); return errs; }
  function num(v) { return v !== null && v !== undefined && v !== '' ? Number(v) : null; } function bool(v) { return v === true || v === 'yes' || v === 'true'; }
  function populateDeviceDropdowns() { var devices = NetApp.get().devices; var scopeSel = $('tsScopeDev'); scopeSel.innerHTML = '<option value="">— Pilih perangkat —</option>'; devices.forEach(function (d) { var o = document.createElement('option'); o.value = d.name; o.textContent = d.name + ' (' + d.kind + ')'; scopeSel.appendChild(o); }); if (state.scopeDevice) scopeSel.value = state.scopeDevice; }
  function renderTopTalkers() { var tb = $('tsTopTalkersTable').querySelector('tbody'); tb.innerHTML = ''; var devices = NetApp.get().devices.map(function (d) { return d.name; }); state.topTalkers.forEach(function (t, i) { var tr = document.createElement('tr'); var devOpt = devices.map(function (d) { return '<option value="' + d + '"' + (t.device === d ? ' selected' : '') + '>' + d + '</option>'; }).join(''); tr.innerHTML = '<td><select class="mini-input" data-idx="' + i + '" data-field="device">' + devOpt + '<option value="">(manual)</option></select></td>' + '<td><input class="mini-input" type="number" step="0.1" min="0" data-idx="' + i + '" data-field="mbps" value="' + (t.mbps || '') + '" placeholder="Mbps" style="width:80px;"></td>' + '<td><input class="mini-input" type="text" data-idx="' + i + '" data-field="app" value="' + (t.app || '') + '" placeholder="Aplikasi/Protokol" style="width:180px;"></td>' + '<td><button class="btn-reset" style="padding:4px 10px;font-size:11px;" data-del="' + i + '">Hapus</button></td>'; tb.appendChild(tr); }); tb.querySelectorAll('select[data-field="device"], input[data-field="mbps"], input[data-field="app"]').forEach(function (el) { el.addEventListener('change', function () { var idx = +el.getAttribute('data-idx'); var field = el.getAttribute('data-field'); state.topTalkers[idx][field] = field === 'mbps' ? num(el.value) : el.value; renderAllOutputs(); }); }); tb.querySelectorAll('button[data-del]').forEach(function (btn) { btn.addEventListener('click', function () { var idx = +btn.getAttribute('data-del'); state.topTalkers.splice(idx, 1); renderTopTalkers(); renderAllOutputs(); }); }); }
  function renderPorts() { var tb = $('tsPortTable').querySelector('tbody'); tb.innerHTML = ''; state.ports.forEach(function (p, i) { var tr = document.createElement('tr'); tr.innerHTML = '<td><input class="mini-input" type="text" data-idx="' + i + '" data-field="name" value="' + (p.name || '') + '" placeholder="Port/Link" style="width:140px;"></td>' + '<td><select class="mini-input" data-idx="' + i + '" data-field="status"><option value="up"' + (p.status === 'up' ? ' selected' : '') + '>Up</option><option value="down"' + (p.status === 'down' ? ' selected' : '') + '>Down</option><option value="flapping"' + (p.status === 'flapping' ? ' selected' : '') + '>Flapping</option></select></td>' + '<td><select class="mini-input" data-idx="' + i + '" data-field="duplex"><option value="full"' + (p.duplex === 'full' ? ' selected' : '') + '>Full</option><option value="half"' + (p.duplex === 'half' ? ' selected' : '') + '>Half</option><option value="mismatch"' + (p.duplex === 'mismatch' ? ' selected' : '') + '>Mismatch</option></select></td>' + '<td><input class="mini-input" type="number" min="0" data-idx="' + i + '" data-field="crc" value="' + (p.crc || '') + '" placeholder="CRC" style="width:70px;"></td>' + '<td><input class="mini-input" type="number" min="0" data-idx="' + i + '" data-field="inputErr" value="' + (p.inputErr || '') + '" placeholder="Input Err" style="width:70px;"></td>' + '<td><input class="mini-input" type="number" min="0" data-idx="' + i + '" data-field="collision" value="' + (p.collision || '') + '" placeholder="Collision" style="width:70px;"></td>' + '<td><input class="mini-input" type="number" min="0" data-idx="' + i + '" data-field="drop" value="' + (p.drop || '') + '" placeholder="Drop" style="width:70px;"></td>' + '<td><select class="mini-input" data-idx="' + i + '" data-field="flapping"><option value="false"' + (p.flapping === false ? ' selected' : '') + '>Tidak</option><option value="true"' + (p.flapping === true ? ' selected' : '') + '>Ya</option></select></td>' + '<td><button class="btn-reset" style="padding:4px 10px;font-size:11px;" data-del="' + i + '">Hapus</button></td>'; tb.appendChild(tr); }); tb.querySelectorAll('select[data-field], input[data-field]').forEach(function (el) { el.addEventListener('change', function () { var idx = +el.getAttribute('data-idx'); var field = el.getAttribute('data-field'); state.ports[idx][field] = (field === 'crc' || field === 'inputErr' || field === 'collision' || field === 'drop') ? num(el.value) : (field === 'flapping' ? bool(el.value) : el.value); renderAllOutputs(); }); }); tb.querySelectorAll('button[data-del]').forEach(function (btn) { btn.addEventListener('click', function () { var idx = +btn.getAttribute('data-del'); state.ports.splice(idx, 1); renderPorts(); renderAllOutputs(); }); }); }
  function renderDeviceHealth() { var tb = $('tsDeviceHealthTable').querySelector('tbody'); tb.innerHTML = ''; var devices = NetApp.get().devices.map(function (d) { return d.name; }); state.deviceHealth.forEach(function (d, i) { var tr = document.createElement('tr'); var devOpt = devices.map(function (dn) { return '<option value="' + dn + '"' + (d.device === dn ? ' selected' : '') + '>' + dn + '</option>'; }).join(''); tr.innerHTML = '<td><select class="mini-input" data-idx="' + i + '" data-field="device">' + devOpt + '<option value="">(manual)</option></select></td>' + '<td><input class="mini-input" type="number" min="0" max="100" step="0.1" data-idx="' + i + '" data-field="cpu" value="' + (d.cpu || '') + '" placeholder="CPU %" style="width:80px;"></td>' + '<td><input class="mini-input" type="number" min="0" max="100" step="0.1" data-idx="' + i + '" data-field="mem" value="' + (d.mem || '') + '" placeholder="Mem %" style="width:80px;"></td>' + '<td><button class="btn-reset" style="padding:4px 10px;font-size:11px;" data-del="' + i + '">Hapus</button></td>'; tb.appendChild(tr); }); tb.querySelectorAll('select[data-field="device"], input[data-field="cpu"], input[data-field="mem"]').forEach(function (el) { el.addEventListener('change', function () { var idx = +el.getAttribute('data-idx'); var field = el.getAttribute('data-field'); state.deviceHealth[idx][field] = num(el.value); renderAllOutputs(); }); }); tb.querySelectorAll('button[data-del]').forEach(function (btn) { btn.addEventListener('click', function () { var idx = +btn.getAttribute('data-del'); state.deviceHealth.splice(idx, 1); renderDeviceHealth(); renderAllOutputs(); }); }); }
  function applyPreset(key) { var p = PRESETS[key]; if (!p) return; Object.keys(p).forEach(function (k) { if (k === 'changes') { state.changes = Object.assign({}, state.changes, p.changes); } else if (k === 'topTalkers') { state.topTalkers = p.topTalkers.map(function (t) { return Object.assign({}, t); }); } else if (k === 'ports') { state.ports = p.ports.map(function (pt) { return Object.assign({}, pt); }); } else if (k === 'deviceHealth') { state.deviceHealth = p.deviceHealth.map(function (d) { return Object.assign({}, d); }); } else { state[k] = p[k]; } }); $('tsScope').value = state.scope; $('tsOnset').value = state.onset; $('tsPattern').value = state.pattern; $('tsCoverage').value = state.coverage; $('tsPingGw').value = state.pingGw !== null ? state.pingGw : ''; $('tsPingGwLoss').value = state.pingGwLoss !== null ? state.pingGwLoss : ''; $('tsPingDns').value = state.pingDns !== null ? state.pingDns : ''; $('tsPingDnsLoss').value = state.pingDnsLoss !== null ? state.pingDnsLoss : ''; $('tsPingNet').value = state.pingNet !== null ? state.pingNet : ''; $('tsPingNetLoss').value = state.pingNetLoss !== null ? state.pingNetLoss : ''; $('tsTrace').value = state.traceroute || ''; $('tsDnsTime').value = state.dnsTime !== null ? state.dnsTime : ''; $('tsUplinkCap').value = state.uplinkCap !== null ? state.uplinkCap : ''; $('tsUplinkUsage').value = state.uplinkUsage !== null ? state.uplinkUsage : ''; $('tsOutboundSpike').value = state.outboundSpike || ''; $('tsPortScan').value = state.portScan || ''; $('tsSynHalf').value = state.synHalfOpen || ''; $('tsForeignIP').value = state.foreignIP || ''; $('tsBcastPps').value = state.bcastPps !== null ? state.bcastPps : ''; $('tsMacFlap').value = state.macFlap || ''; $('tsStpChange').value = state.stpChange || ''; Object.keys(state.changes).forEach(function (k) { var btn = document.querySelector('.ts-change-btn[data-change="' + k + '"]'); if (btn) btn.classList.toggle('active', state.changes[k]); }); renderTopTalkers(); renderPorts(); renderDeviceHealth(); renderAllOutputs(); }
  function runDiagnosis() { var errs = validate(); if (errs.length) { showError(errs); return null; } showError(null); var ctx = TE.buildContext(state); var hypotheses = TE.evaluateHypotheses(ctx); var sufficiency = TE.assessSufficiency(ctx, hypotheses); var suggestions = TE.suggestNextMeasurements(hypotheses, ctx); var stepStatuses = {}; TE.FLOWCHART_STEPS.forEach(function (step) { stepStatuses[step.key] = TE.getStepStatus(step, ctx); }); var trace = TE.parseTraceroute(state.traceroute); var essay = TE.generateEssay(state, ctx, hypotheses, stepStatuses, sufficiency, suggestions, trace); var recommendations = (sufficiency.sufficient && hypotheses[0] && TE.RECOMMENDATIONS[hypotheses[0].id]) ? TE.RECOMMENDATIONS[hypotheses[0].id] : null; var results = { ctx: ctx, hypotheses: hypotheses, sufficiency: sufficiency, suggestions: suggestions, stepStatuses: stepStatuses, trace: trace, essay: essay, recommendations: recommendations, topHypothesis: hypotheses[0] || null }; state.lastResults = results; return results; }
  function renderAllOutputs() { var results = runDiagnosis(); if (!results) return; renderFlowchart(results); renderTraceChart(results); renderHypothesisTable(results); renderRecommendations(results); renderEssay(results); render3DScene(results); render2DScene(results); renderAccessTable(results); updateLegend(results); prepareAnimation(results); }
  function renderFlowchart(results) { var host = $('tsFlowchart'); var html = '<div class="ts-flowchart">'; var firstFail = null; TE.FLOWCHART_STEPS.forEach(function (step, idx) { var status = results.stepStatuses[step.key]; var isFirstFail = status === 'fail' && firstFail === null; if (isFirstFail) firstFail = step.key; var cls = 'ts-step-box ' + status + (isFirstFail ? ' start-here' : ''); html += '<div class="ts-step" data-step="' + step.key + '">'; html += '<div class="' + cls + '" data-num="' + step.num + '">'; html += '<span class="ts-step-text">' + step.title + '</span>'; html += '<span class="ts-step-status ' + status + '">' + (status === 'fail' ? 'Ditemukan masalah' : (status === 'pass' ? 'Lolos' : 'Belum diukur')) + '</span>'; html += '</div>'; if (idx < TE.FLOWCHART_STEPS.length - 1) { html += '<div class="ts-step-connector ' + status + '"></div>'; } html += '</div>'; }); html += '</div>'; host.innerHTML = html; host.querySelectorAll('.ts-step').forEach(function (el) { el.addEventListener('click', function () { var stepKey = el.getAttribute('data-step'); showStepDetail(stepKey, results); if (state.mode === 'practice') { state.practiceRevealed[stepKey] = true; renderPracticeFeedback(); } }); }); var showKey = firstFail || TE.FLOWCHART_STEPS[0].key; showStepDetail(showKey, results); }
  function showStepDetail(stepKey, results) { var step = TE.FLOWCHART_STEPS.find(function (s) { return s.key === stepKey; }); if (!step) return; var status = results.stepStatuses[stepKey]; var ctx = results.ctx; var title = $('tsStepTitle'), grid = $('tsStepGrid'); title.textContent = 'Langkah ' + step.num + ': ' + step.title; var commands = { scope: ['Identifikasi pengguna/segmen terdampak', 'Catat waktu mulai & pola (tiba-tiba/bertahap)'], physical: ['show interface [port]', 'show interface counters errors', 'show logging | include flap'], bandwidth: ['show interface [uplink] | include utilization', 'show processes cpu | include top', 'netflow/sflow top talkers'], latency: ['ping gateway', 'ping 8.8.8.8', 'traceroute 8.8.8.8', 'tracert 8.8.8.8 (Windows)'], dns: ['nslookup example.com', 'dig example.com', 'time nslookup example.com'], device: ['show processes cpu', 'show memory', 'show logging'], security: ['show conn count', 'show threat-detection statistics', 'show access-list hits'], change: ['show archive config differences', 'show logging | include config'], fix: ['Verifikasi metrik sebelum/sesudah', 'Dokumentasikan root cause & solusi', 'Update runbook/knowledge base'] }; var detailItems = [['Status', status === 'fail' ? 'Ditemukan masalah' : (status === 'pass' ? 'Lolos (normal)' : 'Belum diukur')], ['Perintah contoh', commands[stepKey] ? commands[stepKey].join(', ') : '—']]; step.checks.forEach(function (ck) { var val = ctx[ck]; if (val !== null && val !== undefined) { var label = ck.replace(/([A-Z])/g, ' $1').replace(/^./, function (s) { return s.toUpperCase(); }); var vstr = typeof val === 'number' ? val.toFixed(1) : (val === true ? 'Ya' : (val === false ? 'Tidak' : val)); detailItems.push(['Nilai ' + label, vstr]); } }); var interp = ''; if (status === 'fail') interp = 'Langkah ini mengindikasikan anomali. Lanjutkan investigasi ke langkah berikutnya atau fokus perbaikan di sini.'; else if (status === 'pass') interp = 'Langkah ini normal. Lanjutkan ke langkah berikutnya.'; else interp = 'Data untuk langkah ini belum diisi. Isi input terkait untuk melanjutkan.'; detailItems.push(['Interpretasi', interp]); grid.innerHTML = detailItems.map(function (it) { return '<div class="def-detail-item"><div class="dl">' + it[0] + '</div><div class="dv">' + it[1] + '</div></div>'; }).join(''); $('tsStepPanel').classList.add('show'); }
  function renderTraceChart(results) { var svg = $('tsTraceChart'); var trace = results.trace; if (!trace || !trace.hops.length) { svg.innerHTML = '<text x="50" y="22" text-anchor="middle" fill="var(--text-muted)">Tidak ada data traceroute</text>'; return; } var hops = trace.hops; var width = 100, height = 42; var maxLat = Math.max.apply(null, hops.map(function (h) { return h.latency || 0; }).concat([50])); if (maxLat < 10) maxLat = 10; var parts = []; var barW = 8, gap = 3; var startX = 6; hops.forEach(function (h, i) { var x = startX + i * (barW + gap); var hLat = h.latency || 0; var bh = hLat > 0 ? (hLat / maxLat) * 32 : 0; var y = 38 - bh; var color = h.timeout ? '#ef4444' : (hLat > 50 && i > 0 && hops[i-1].latency && hLat >= 3 * hops[i-1].latency ? '#ef4444' : (hLat > 30 ? '#fbbf24' : 'var(--accent)')); parts.push('<rect x="' + x + '" y="' + y + '" width="' + barW + '" height="' + bh + '" fill="' + color + '" rx="1"></rect>'); parts.push('<text x="' + (x + barW/2) + '" y="40" text-anchor="middle" font-size="2.5" fill="var(--text-secondary)" font-family="Inter,monospace">' + (i+1) + '</text>'); if (hLat > 0) parts.push('<text x="' + (x + barW/2) + '" y="' + (y - 1) + '" text-anchor="middle" font-size="2.2" fill="var(--text-primary)" font-family="Inter,monospace" font-weight="700">' + hLat + '</text>'); else parts.push('<text x="' + (x + barW/2) + '" y="' + (y - 1) + '" text-anchor="middle" font-size="2.2" fill="#ef4444" font-family="Inter,monospace" font-weight="700">*</text>'); }); var threshY = 38 - (50 / maxLat) * 32; parts.push('<line x1="4" y1="' + threshY + '" x2="' + (width - 4) + '" y2="' + threshY + '" stroke="var(--warning)" stroke-width="0.5" stroke-dasharray="2 2"></line>'); parts.push('<text x="4" y="' + (threshY - 1) + '" font-size="2.2" fill="var(--warning)">50 ms (ambang melonjak)</text>'); svg.innerHTML = '<svg viewBox="0 0 ' + width + ' ' + height + '" preserveAspectRatio="none" width="100%" height="100%">' + parts.join('') + '</svg>'; }
  function renderHypothesisTable(results) { var tb = $('tsHypothesisTable').querySelector('tbody'); tb.innerHTML = ''; results.hypotheses.forEach(function (h, i) { var tr = document.createElement('tr'); var barFill = h.score + '%'; var supportHtml = h.supporting.map(function (s) { return '<span class="support">✓ ' + s.rule + ': ' + (typeof s.value === 'number' ? s.value.toFixed(1) : s.value) + '</span>'; }).join('<br>'); var refuteHtml = h.refuting.map(function (r) { return '<span class="refute">✗ ' + r.rule + ': ' + (typeof r.value === 'number' ? r.value.toFixed(1) : r.value) + '</span>'; }).join('<br>'); var pendingHtml = h.pending.map(function (p) { return '<span class="pending">○ ' + p.rule + ' (bobot ' + (p.weight > 0 ? '+' : '') + p.weight + ')</span>'; }).join('<br>'); tr.innerHTML = '<td style="text-align:center;font-weight:700;color:var(--accent);">' + (i+1) + '</td>' + '<td><div class="ts-hyp-name">' + h.name + '</div></td>' + '<td><div class="ts-hyp-bar"><div class="bar-bg"><div class="bar-fill" style="width:' + barFill + '"></div><span class="bar-label">' + h.score + '%</span></div></div></td>' + '<td class="ts-evidence-list">' + (supportHtml || '<span style="color:var(--text-muted);">—</span>') + '</td>' + '<td class="ts-evidence-list">' + (refuteHtml || '<span style="color:var(--text-muted);">—</span>') + '</td>' + '<td class="ts-evidence-list">' + (pendingHtml || '<span style="color:var(--text-muted);">—</span>') + '</td>'; tb.appendChild(tr); }); }
  function renderRecommendations(results) { var box = $('tsRecommendations'); if (!results.recommendations) { box.innerHTML = '<div class="empty-note">Tidak ada rekomendasi spesifik (data belum cukup atau tidak ada hipotesis dominan).</div>'; return; } var rec = results.recommendations; var html = ''; if (rec.quick && rec.quick.length) { html += '<div class="ts-rec-section"><h4>Tindakan Cepat (Quick Wins)</h4>'; rec.quick.forEach(function (r) { html += '<div class="ts-rec-item quick"><div class="ts-rec-title">⚡ ' + r + '</div></div>'; }); html += '</div>'; } if (rec.long && rec.long.length) { html += '<div class="ts-rec-section"><h4>Jangka Panjang (Strategis)</h4>'; rec.long.forEach(function (r) { html += '<div class="ts-rec-item long"><div class="ts-rec-title">📈 ' + r + '</div></div>'; }); html += '</div>'; } box.innerHTML = html; }
  function renderEssay(results) { var box = $('tsEssay'); var html = ''; results.essay.forEach(function (sec) { if (sec.isConclusion) { html += '<div class="ts-essay-section concl">' + sec.text + '</div>'; } else if (sec.title) { html += '<div class="ts-essay-section"><h4>' + sec.title + '</h4><p>' + sec.text + '</p></div>'; } else { html += '<div class="ts-essay-section"><p>' + sec.text + '</p></div>'; } }); box.innerHTML = html; box._plain = results.essay.map(function (s) { return (s.title ? s.title + ': ' : '') + s.text.replace(/<[^>]+>/g, ''); }).join('\n\n'); }

  // ---- 3D Scene (Network Path Visualization) ----
  var tsTHREE = null, tsScene = null, tsCamera = null, tsRenderer = null, tsControls = null;
  var tsNodeMeshes = {}, tsLinkMeshes = {}, tsPacketMeshes = [], tsRaf = null, tsClock = null;
  var tsAnimPlaying = false, tsAnimSpeed = 1;
  var tsReduced = window.matchMedia && window.matchMedia('(prefers-reduced-motion: reduce)').matches;

  function tsLoadThree() {
    return new Promise(function (resolve) {
      var checkCount = 0;
      function check() {
        checkCount++;
        if (window.THREE && window.OrbitControls) {
          tsTHREE = window.THREE;
          tsControls = window.OrbitControls;
          resolve();
        } else if (checkCount > 200) {
          console.error('Three.js or OrbitControls not loaded after timeout');
          resolve();
        } else {
          setTimeout(check, 100);
        }
      }
      check();
    });
  }

  function tsInit3D() {
    var host = $('tsScene3d');
    tsScene = new tsTHREE.Scene();
    tsScene.background = new tsTHREE.Color(0x0a0a0f);

    tsCamera = new tsTHREE.PerspectiveCamera(40, host.clientWidth / host.clientHeight, 0.1, 200);
    tsCamera.position.set(12, 10, 18);

    tsRenderer = new tsTHREE.WebGLRenderer({ antialias: true, alpha: false });
    tsRenderer.setPixelRatio(Math.min(window.devicePixelRatio || 1, 2));
    tsRenderer.setSize(host.clientWidth, host.clientHeight);
    host.appendChild(tsRenderer.domElement);

    tsControls = new (window.OrbitControls || tsControls)(tsCamera, tsRenderer.domElement);
    tsControls.enableDamping = true;
    tsControls.dampingFactor = 0.08;
    tsControls.minDistance = 8; tsControls.maxDistance = 40;
    tsControls.minPolarAngle = 0.25; tsControls.maxPolarAngle = 1.4;
    tsControls.enablePan = false;
    tsControls.target.set(0, 1, 0);

    tsScene.add(new tsTHREE.AmbientLight(0xffffff, 0.5));
    var key = new tsTHREE.DirectionalLight(0xffffff, 0.8); key.position.set(10, 20, 12); tsScene.add(key);
    var fill = new tsTHREE.DirectionalLight(0x88aaff, 0.3); fill.position.set(-12, 10, -10); tsScene.add(fill);

    var floor = new tsTHREE.Mesh(new tsTHREE.CylinderGeometry(18, 18, 0.6, 12), new tsTHREE.MeshStandardMaterial({ color: 0x12121a, roughness: 0.9 }));
    floor.position.y = -0.4; tsScene.add(floor);
    var grid = new tsTHREE.GridHelper(36, 36, 0x333344, 0x222233);
    grid.position.y = -0.1; tsScene.add(grid);

    tsClock = new tsTHREE.Clock();
    window.addEventListener('resize', tsOnResize);
    window.addEventListener('beforeunload', tsDispose);
  }

  function tsBuildScene(results) {
    if (!tsScene) return;
    // Clear existing
    Object.values(tsNodeMeshes).forEach(function (m) { tsScene.remove(m); });
    Object.values(tsLinkMeshes).forEach(function (m) { tsScene.remove(m); });
    tsPacketMeshes.forEach(function (p) { tsScene.remove(p.mesh); });
    tsNodeMeshes = {}; tsLinkMeshes = {}; tsPacketMeshes = [];

    var trace = results.trace;
    if (!trace || !trace.hops.length) return;

    var hops = trace.hops;
    var problemHop = trace.problemHop;

    // Create nodes in a line
    var nodePositions = [];
    var startX = -6;
    var spacing = 2.5;
    hops.forEach(function (h, i) {
      var isProblem = problemHop !== null && h.index === problemHop;
      var x = startX + i * spacing;
      var y = 0;
      var z = 0;
      nodePositions.push({ x: x, y: y, z: z, name: h.name, isProblem: isProblem, isPrivate: h.isPrivate });

      // Node mesh
      var color = isProblem ? 0xef4444 : (h.isPrivate ? 0x4ade80 : 0x6c8eff);
      var geom = new tsTHREE.SphereGeometry(isProblem ? 0.5 : 0.4, 16, 16);
      var mat = new tsTHREE.MeshStandardMaterial({ color: color, emissive: color, emissiveIntensity: isProblem ? 0.5 : 0.2, roughness: 0.5, metalness: 0.1 });
      var mesh = new tsTHREE.Mesh(geom, mat);
      mesh.position.set(x, y, z);
      tsScene.add(mesh);
      tsNodeMeshes[h.index] = mesh;

      // Label
      var label = document.createElement('div');
      label.className = 'float-label';
      label.style.fontSize = '11px';
      label.innerHTML = '<span class="fl-dot" style="background:' + (isProblem ? '#ef4444' : (h.isPrivate ? '#4ade80' : '#6c8eff')) + '"></span>' + (i+1) + '. ' + h.name;
      mesh.userData.labelEl = label;
      document.getElementById('tsLabelLayer').appendChild(label);
    });

    // Links between nodes
    for (var i = 0; i < nodePositions.length - 1; i++) {
      var a = nodePositions[i], b = nodePositions[i+1];
      var linkGeom = new tsTHREE.BufferGeometry().setFromPoints([
        new tsTHREE.Vector3(a.x, a.y, a.z),
        new tsTHREE.Vector3(b.x, b.y, b.z)
      ]);
      var isProblemLink = problemHop !== null && (b.index === problemHop || a.index === problemHop);
      var linkMat = new tsTHREE.LineBasicMaterial({ color: isProblemLink ? 0xef4444 : 0x55556a, linewidth: isProblemLink ? 3 : 1, transparent: true, opacity: isProblemLink ? 1 : 0.5 });
      var link = new tsTHREE.Line(linkGeom, linkMat);
      tsScene.add(link);
      tsLinkMeshes[i] = link;
    }

    // Target (Internet)
    var targetX = startX + nodePositions.length * spacing + 1;
    var targetGeom = new tsTHREE.SphereGeometry(0.6, 16, 16);
    var targetMat = new tsTHREE.MeshStandardMaterial({ color: 0x34d399, emissive: 0x34d399, emissiveIntensity: 0.4 });
    var targetMesh = new tsTHREE.Mesh(targetGeom, targetMat);
    targetMesh.position.set(targetX, 0, 0);
    tsScene.add(targetMesh);
    tsNodeMeshes.target = targetMesh;

    var targetLabel = document.createElement('div');
    targetLabel.className = 'float-label';
    targetLabel.innerHTML = '<span class="fl-dot" style="background:#34d399"></span>Internet (Target)';
    targetMesh.userData.labelEl = targetLabel;
    document.getElementById('tsLabelLayer').appendChild(targetLabel);

    // Source (Client)
    var sourceX = startX - 1.5;
    var sourceGeom = new tsTHREE.SphereGeometry(0.4, 16, 16);
    var sourceMat = new tsTHREE.MeshStandardMaterial({ color: 0x2b6cff, emissive: 0x2b6cff, emissiveIntensity: 0.3 });
    var sourceMesh = new tsTHREE.Mesh(sourceGeom, sourceMat);
    sourceMesh.position.set(sourceX, 0, 0);
    tsScene.add(sourceMesh);
    tsNodeMeshes.source = sourceMesh;

    var sourceLabel = document.createElement('div');
    sourceLabel.className = 'float-label';
    sourceLabel.innerHTML = '<span class="fl-dot" style="background:#2b6cff"></span>Client';
    sourceMesh.userData.labelEl = sourceLabel;
    document.getElementById('tsLabelLayer').appendChild(sourceLabel);

    // Link from source to first hop
    if (nodePositions.length > 0) {
      var linkGeom = new tsTHREE.BufferGeometry().setFromPoints([
        new tsTHREE.Vector3(sourceX, 0, 0),
        new tsTHREE.Vector3(nodePositions[0].x, nodePositions[0].y, nodePositions[0].z)
      ]);
      var linkMat = new tsTHREE.LineBasicMaterial({ color: 0x55556a, linewidth: 1, opacity: 0.5, transparent: true });
      var link = new tsTHREE.Line(linkGeom, linkMat);
      tsScene.add(link);
      tsLinkMeshes.source = link;
    }
  }

  function tsOnResize() {
    if (!tsRenderer) return;
    var host = $('tsScene3d');
    if (!host.clientWidth) return;
    tsCamera.aspect = host.clientWidth / host.clientHeight;
    tsCamera.updateProjectionMatrix();
    tsRenderer.setSize(host.clientWidth, host.clientHeight);
  }

  function tsAnimate() {
    tsRaf = requestAnimationFrame(tsAnimate);
    if (!tsAnimPlaying || tsReduced) {
      if (tsControls && typeof tsControls.update === 'function') tsControls.update();
      if (tsRenderer && tsScene && tsCamera) tsRenderer.render(tsScene, tsCamera);
      tsUpdateLabels();
      return;
    }
    var dt = tsClock.getDelta() * tsAnimSpeed;
    tsPacketMeshes.forEach(function (pk) {
      pk.t += dt * pk.speed;
      if (pk.t > 1) {
        pk.t = 0;
        // restart at source
      }
      var pt = pk.curve.getPoint(pk.t);
      pk.mesh.position.copy(pt);
    });
    if (tsControls && typeof tsControls.update === 'function') tsControls.update();
    if (tsRenderer && tsScene && tsCamera) tsRenderer.render(tsScene, tsCamera);
    tsUpdateLabels();
  }

  function tsUpdateLabels() {
    if (!tsCamera || !tsRenderer) return;
    var host = $('tsScene3d');
    var w = host.clientWidth, h = host.clientHeight;
    var tmp = new tsTHREE.Vector3();
    Object.values(tsNodeMeshes).forEach(function (mesh) {
      if (!mesh.userData.labelEl) return;
      tmp.set(mesh.position.x, mesh.position.y + 1.0, mesh.position.z);
      tmp.project(tsCamera);
      var x = (tmp.x * 0.5 + 0.5) * w;
      var y = (-tmp.y * 0.5 + 0.5) * h;
      mesh.userData.labelEl.style.left = x + 'px';
      mesh.userData.labelEl.style.top = y + 'px';
      mesh.userData.labelEl.style.display = (tmp.z > 1) ? 'none' : 'block';
    });
  }

  function tsDispose() {
    if (tsRaf) cancelAnimationFrame(tsRaf);
    if (tsRenderer) { tsRenderer.dispose(); if (tsRenderer.domElement.parentNode) tsRenderer.domElement.parentNode.removeChild(tsRenderer.domElement); }
    window.removeEventListener('resize', tsOnResize);
  }

  // ---- 2D SVG Fallback ----
  function render2DScene(results) {
    var host = $('tsScene2d');
    var trace = results.trace;
    if (!trace || !trace.hops.length) {
      host.innerHTML = '<div style="position:absolute;inset:0;display:flex;align-items:center;justify-content:center;color:var(--text-muted);font-size:13px;">Tidak ada data traceroute untuk divisualisasikan</div>';
      return;
    }
    var hops = trace.hops;
    var problemHop = trace.problemHop;
    var W = 100, H = 30, cx = 10;
    var parts = ['<svg viewBox="0 0 ' + W + ' ' + H + '" preserveAspectRatio="xMidYMid meet" role="img" aria-label="Network path 2D">'];
    var spacing = 8;
    var y = 15;
    // Nodes
    hops.forEach(function (h, i) {
      var isProblem = problemHop !== null && h.index === problemHop;
      var x = cx + i * spacing;
      var color = isProblem ? '#ef4444' : (h.isPrivate ? '#4ade80' : '#6c8eff');
      parts.push('<circle cx="' + x + '" cy="' + y + '" r="3" fill="' + color + '" stroke="#fff" stroke-width="0.5"></circle>');
      parts.push('<text x="' + x + '" y="' + (y + 6) + '" text-anchor="middle" font-size="2.5" fill="var(--text-secondary)">' + (i+1) + '</text>');
    });
    // Links
    for (var i = 0; i < hops.length - 1; i++) {
      var x1 = cx + i * spacing + 3;
      var x2 = cx + (i+1) * spacing - 3;
      var isProblemLink = problemHop !== null && (hops[i+1].index === problemHop || hops[i].index === problemHop);
      parts.push('<line x1="' + x1 + '" y1="' + y + '" x2="' + x2 + '" y2="' + y + '" stroke="' + (isProblemLink ? '#ef4444' : '#55556a') + '" stroke-width="' + (isProblemLink ? 2 : 1) + '"></line>');
    }
    // Target
    var targetX = cx + hops.length * spacing;
    parts.push('<circle cx="' + targetX + '" cy="' + y + '" r="3.5" fill="#34d399" stroke="#fff" stroke-width="0.5"></circle>');
    parts.push('<text x="' + targetX + '" y="' + (y + 6) + '" text-anchor="middle" font-size="2.5" fill="#34d399" font-weight="700">NET</text>');
    // Source
    var sourceX = cx - 5;
    parts.push('<circle cx="' + sourceX + '" cy="' + y + '" r="3" fill="#2b6cff" stroke="#fff" stroke-width="0.5"></circle>');
    parts.push('<text x="' + sourceX + '" y="' + (y + 6) + '" text-anchor="middle" font-size="2.5" fill="#2b6cff">CLIENT</text>');
    // Link source to first
    if (hops.length > 0) {
      parts.push('<line x1="' + (sourceX + 3) + '" y1="' + y + '" x2="' + (cx - 3) + '" y2="' + y + '" stroke="#55556a" stroke-width="1"></line>');
    }
    parts.push('</svg>');
    host.innerHTML = parts.join('');
  }

  // ---- Accessible Table ----
  function renderAccessTable(results) {
    var tb = $('tsAccessTable').querySelector('tbody');
    var trace = results.trace;
    tb.innerHTML = '';
    if (!trace || !trace.hops.length) return;
    trace.hops.forEach(function (h) {
      var isProblem = trace.problemHop !== null && h.index === trace.problemHop;
      var tr = document.createElement('tr');
      if (isProblem) tr.classList.add('problem-hop');
      tr.innerHTML =
        '<td>' + h.index + '</td>' +
        '<td>' + h.name + '</td>' +
        '<td>' + (h.latency !== null ? h.latency + ' ms' : 'Timeout') + '</td>' +
        '<td>' + (h.isPrivate ? 'Internal' : 'Eksternal') + '</td>' +
        '<td>' + (isProblem ? '⚠ TITIK MASALAH' : '—') + '</td>';
      tb.appendChild(tr);
    });
  }

  // ---- Legend ----
  function updateLegend(results) {
    var leg = $('tsLegend');
    leg.innerHTML =
      '<span><i class="dot" style="background:#4ade80"></i> Hop internal (normal)</span>' +
      '<span><i class="dot" style="background:#6c8eff"></i> Hop eksternal (normal)</span>' +
      '<span><i class="dot" style="background:#ef4444"></i> Hop bermasalah</span>' +
      '<span><i class="dot" style="background:#34d399"></i> Target (Internet)</span>' +
      '<span><i class="dot" style="background:#2b6cff"></i> Client (Sumber)</span>' +
      '<span><i class="dot" style="background:#fbbf24"></i> Latency tinggi (≥30 ms)</span>';
  }

  // ---- Animation Preparation ----
  function prepareAnimation(results) {
    tsPacketMeshes = [];
    var trace = results.trace;
    if (!trace || !trace.hops.length) return;

    var hops = trace.hops;
    var sourceX = -7.5;
    var startX = -6;
    var spacing = 2.5;
    var targetX = startX + hops.length * spacing + 1;

    // Build single packet path from source to target
    var curvePoints = [new tsTHREE.Vector3(sourceX, 0.5, 0)];
    for (var i = 0; i < hops.length; i++) {
      curvePoints.push(new tsTHREE.Vector3(startX + i * spacing, 0.5, 0));
    }
    curvePoints.push(new tsTHREE.Vector3(targetX, 0.5, 0));

    var curve = new tsTHREE.CatmullRomCurve3(curvePoints);
    var cube = new tsTHREE.Mesh(new tsTHREE.BoxGeometry(0.2, 0.2, 0.2), new tsTHREE.MeshStandardMaterial({ color: 0xef4444, emissive: 0xef4444, emissiveIntensity: 0.8 }));
    tsScene.add(cube);
    tsPacketMeshes.push({ mesh: cube, curve: curve, t: 0, speed: 0.4 });

    $('tsAnimPlay').style.display = 'inline-flex';
    $('tsAnimPause').style.display = 'inline-flex';
    $('tsAnimRestart').style.display = 'inline-flex';
    $('tsSpeedWrap').style.display = 'flex';
  }

  function startAnimation() { tsAnimPlaying = true; $('tsAnimPlay').textContent = 'Playing...'; }
  function pauseAnimation() { tsAnimPlaying = false; $('tsAnimPlay').textContent = 'Play'; }
  function restartAnimation() { tsPacketMeshes.forEach(function (pk) { pk.t = 0; pk.mesh.visible = true; }); tsAnimPlaying = true; $('tsAnimPlay').textContent = 'Playing...'; }
  function setAnimSpeed(v) { tsAnimSpeed = v; }

  // ---- View Switching ----
  var tsView = '3d';
  function tsSetView(v) {
    tsView = v;
    $('tsView3d').classList.toggle('active', v === '3d');
    $('tsView2d').classList.toggle('active', v === '2d');
    $('tsViewTable').classList.toggle('active', v === 'table');
    $('tsScene3d').classList.toggle('scene-hidden', v !== '3d');
    $('tsScene2d').classList.toggle('scene-hidden', v !== '2d');
    $('tsLabelLayer').classList.toggle('scene-hidden', v === 'table');
    $('tsTableWrap').style.display = v === 'table' ? 'block' : 'none';
    if (v === '2d' && state.lastResults) render2DScene(state.lastResults);
    if (v === '3d' && state.lastResults && !tsTHREE) {
      tsLoadThree().then(function () { tsInit3D(); tsBuildScene(state.lastResults); tsAnimate(); });
    }
  }

  // ---- Practice Mode ----
  function renderPracticeFeedback() {
    var box = $('tsPracticeFeedback');
    var html = '<h4>Mode Latihan: Langkah-langkah Diagnosa</h4>';
    TE.FLOWCHART_STEPS.forEach(function (step) {
      var revealed = state.practiceRevealed[step.key];
      var status = state.lastResults?.stepStatuses?.[step.key] || 'pending';
      var cls = 'ts-practice-step ' + status + (revealed ? ' revealed' : '');
      html += '<div class="' + cls + '"><span class="ts-step-num">' + step.num + '</span><span class="ts-step-title">' + step.title + '</span></div>';
    });
    box.innerHTML = html;
  }

  // ---- Export / Copy ----
  function exportJson() {
    var out = {
      input: state,
      results: state.lastResults
    };
    var blob = new Blob([JSON.stringify(out, null, 2)], { type: 'application/json' });
    var url = URL.createObjectURL(blob);
    var a = document.createElement('a'); a.href = url; a.download = 'troubleshooting.json'; document.body.appendChild(a); a.click();
    setTimeout(function () { URL.revokeObjectURL(url); a.remove(); }, 100);
  }

  function copyEssay() {
    var txt = 'Mengapa troubleshooting jaringan lambat harus sistematis (layered)? Jelaskan langkah-langkahnya.\n\n' +
      ($('tsEssay')._plain || '') +
      '\n\n---\nHipotesis teratas: ' + (state.lastResults?.topHypothesis?.name || '—') + ' (' + (state.lastResults?.topHypothesis?.score || 0) + '%)';
    if (navigator.clipboard && navigator.clipboard.writeText) {
      navigator.clipboard.writeText(txt).then(function () {
        var btn = $('tsCopyEssay'); btn.textContent = 'Tersalin!'; setTimeout(function () { btn.textContent = 'Salin Jawaban (Esai)'; }, 1500);
      });
    } else {
      var blob = new Blob([txt], { type: 'text/plain' });
      var url = URL.createObjectURL(blob);
      var a = document.createElement('a'); a.href = url; a.download = 'jawaban_troubleshooting.txt'; document.body.appendChild(a); a.click();
      setTimeout(function () { URL.revokeObjectURL(url); a.remove(); }, 100);
    }
  }

  function exampleConfig() { applyPreset('uplink_full'); }
  function resetAll() {
    state.scope = 'all'; state.scopeDevice = null; state.onset = 30; state.pattern = 'sudden'; state.coverage = 'internet';
    state.changes = { config: false, firmware: false, newdevice: false, cable: false, backup: false }; state.changeNotes = '';
    state.pingGw = null; state.pingGwLoss = null; state.pingDns = null; state.pingDnsLoss = null; state.pingNet = null; state.pingNetLoss = null;
    state.traceroute = ''; state.dnsTime = null; state.uplinkCap = null; state.uplinkUsage = null;
    state.topTalkers = []; state.ports = []; state.deviceHealth = [];
    state.outboundSpike = null; state.portScan = null; state.synHalfOpen = null; state.foreignIP = null; state.bcastPps = null; state.macFlap = null; state.stpChange = null;
    state.mode = 'normal'; state.practiceStep = 0; state.practiceRevealed = {};
    $('tsScope').value = state.scope; $('tsOnset').value = state.onset; $('tsPattern').value = state.pattern; $('tsCoverage').value = state.coverage;
    $('tsPingGw').value = ''; $('tsPingGwLoss').value = ''; $('tsPingDns').value = ''; $('tsPingDnsLoss').value = ''; $('tsPingNet').value = ''; $('tsPingNetLoss').value = '';
    $('tsTrace').value = ''; $('tsDnsTime').value = ''; $('tsUplinkCap').value = ''; $('tsUplinkUsage').value = '';
    $('tsOutboundSpike').value = ''; $('tsPortScan').value = ''; $('tsSynHalf').value = ''; $('tsForeignIP').value = ''; $('tsBcastPps').value = ''; $('tsMacFlap').value = ''; $('tsStpChange').value = '';
    document.querySelectorAll('.ts-change-btn').forEach(function (b) { b.classList.remove('active'); });
    renderTopTalkers(); renderPorts(); renderDeviceHealth(); renderAllOutputs();
  }
  function addTopTalker() { state.topTalkers.push({ device: '', mbps: 0, app: '' }); renderTopTalkers(); }
  function addPort() { state.ports.push({ name: '', status: 'up', duplex: 'full', crc: 0, inputErr: 0, collision: 0, drop: 0, flapping: false }); renderPorts(); }
  function addDeviceHealth() { state.deviceHealth.push({ device: '', cpu: 0, mem: 0 }); renderDeviceHealth(); }

  // ---- Event Wiring ----
  function wireEvents() {
    $('tsPreset').addEventListener('change', function () { applyPreset(this.value); });
    $('tsScope').addEventListener('change', function () { state.scope = this.value; renderAllOutputs(); });
    $('tsScopeDev').addEventListener('change', function () { state.scopeDevice = this.value || null; renderAllOutputs(); });
    $('tsOnset').addEventListener('change', function () { state.onset = Math.max(1, +this.value || 1); renderAllOutputs(); });
    $('tsPattern').addEventListener('change', function () { state.pattern = this.value; renderAllOutputs(); });
    $('tsCoverage').addEventListener('change', function () { state.coverage = this.value; renderAllOutputs(); });
    $('tsPingGw').addEventListener('change', function () { state.pingGw = num(this.value); renderAllOutputs(); });
    $('tsPingGwLoss').addEventListener('change', function () { state.pingGwLoss = num(this.value); renderAllOutputs(); });
    $('tsPingDns').addEventListener('change', function () { state.pingDns = num(this.value); renderAllOutputs(); });
    $('tsPingDnsLoss').addEventListener('change', function () { state.pingDnsLoss = num(this.value); renderAllOutputs(); });
    $('tsPingNet').addEventListener('change', function () { state.pingNet = num(this.value); renderAllOutputs(); });
    $('tsPingNetLoss').addEventListener('change', function () { state.pingNetLoss = num(this.value); renderAllOutputs(); });
    $('tsTrace').addEventListener('change', function () { state.traceroute = this.value; renderAllOutputs(); });
    $('tsDnsTime').addEventListener('change', function () { state.dnsTime = num(this.value); renderAllOutputs(); });
    $('tsUplinkCap').addEventListener('change', function () { state.uplinkCap = num(this.value); renderAllOutputs(); });
    $('tsUplinkUsage').addEventListener('change', function () { state.uplinkUsage = num(this.value); renderAllOutputs(); });
    $('tsAddTalker').addEventListener('click', addTopTalker);
    $('tsAddPort').addEventListener('click', addPort);
    $('tsAddHealth').addEventListener('click', addDeviceHealth);
    document.querySelectorAll('.ts-change-btn').forEach(function (btn) {
      btn.addEventListener('click', function () { var k = btn.getAttribute('data-change'); state.changes[k] = !state.changes[k]; btn.classList.toggle('active', state.changes[k]); renderAllOutputs(); });
    });
    $('tsOutboundSpike').addEventListener('change', function () { state.outboundSpike = this.value; renderAllOutputs(); });
    $('tsPortScan').addEventListener('change', function () { state.portScan = this.value; renderAllOutputs(); });
    $('tsSynHalf').addEventListener('change', function () { state.synHalfOpen = this.value; renderAllOutputs(); });
    $('tsForeignIP').addEventListener('change', function () { state.foreignIP = this.value; renderAllOutputs(); });
    $('tsBcastPps').addEventListener('change', function () { state.bcastPps = num(this.value); renderAllOutputs(); });
    $('tsMacFlap').addEventListener('change', function () { state.macFlap = this.value; renderAllOutputs(); });
    $('tsStpChange').addEventListener('change', function () { state.stpChange = this.value; renderAllOutputs(); });
    $('tsRun').addEventListener('click', renderAllOutputs);
    $('tsExample').addEventListener('click', exampleConfig);
    $('tsReset').addEventListener('click', resetAll);
    $('tsExportJson').addEventListener('click', exportJson);
    $('tsCopyEssay').addEventListener('click', copyEssay);
    $('tsView3d').addEventListener('click', function () { tsSetView('3d'); });
    $('tsView2d').addEventListener('click', function () { tsSetView('2d'); });
    $('tsViewTable').addEventListener('click', function () { tsSetView('table'); });
    $('tsAnimPlay').addEventListener('click', startAnimation);
    $('tsAnimPause').addEventListener('click', pauseAnimation);
    $('tsAnimRestart').addEventListener('click', restartAnimation);
    $('tsAnimSpeed').addEventListener('change', function () { setAnimSpeed(+this.value); });
    $('tsPracticeMode').addEventListener('click', function () { state.mode = state.mode === 'practice' ? 'normal' : 'practice'; this.classList.toggle('active', state.mode === 'practice'); if (state.mode === 'practice') state.practiceRevealed = {}; renderPracticeFeedback(); renderAllOutputs(); });
  }

  // ---- Init ----
  populateDeviceDropdowns();
  renderTopTalkers();
  renderPorts();
  renderDeviceHealth();
  wireEvents();
  renderAllOutputs();

  if (document.readyState === 'complete' || document.readyState === 'interactive') {
    setTimeout(function () { tsLoadThree().then(function () { tsInit3D(); if (state.lastResults) tsBuildScene(state.lastResults); tsAnimate(); }); }, 500);
  } else {
    window.addEventListener('DOMContentLoaded', function () {
      setTimeout(function () { tsLoadThree().then(function () { tsInit3D(); if (state.lastResults) tsBuildScene(state.lastResults); tsAnimate(); }); }, 500);
    });
  }

  NetApp.subscribe(function () { populateDeviceDropdowns(); renderTopTalkers(); renderPorts(); renderDeviceHealth(); renderAllOutputs(); });

  window.TSApp = { state: state, runDiagnosis: runDiagnosis, renderAllOutputs: renderAllOutputs };
})();
</script>"##.to_string()
}