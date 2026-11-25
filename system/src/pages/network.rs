use axum::response::Html;
use crate::pages::layout::layout;

pub async fn handler() -> Html<String> {
    let body = r##"<style>
  .net-layout {
    display: flex;
    min-height: calc(100vh - 96px);
    max-width: 980px;
    margin: 0 auto;
    padding: 32px 24px 64px;
    gap: 24px;
  }

  /* Sidebar */
  .sidebar { width: 240px; flex-shrink: 0; }
  .sidebar-inner {
    position: sticky;
    top: 88px;
    padding: 20px 16px;
    border-radius: var(--radius-lg);
  }
  .sidebar-title {
    font-size: 10.5px;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--text-muted);
    margin-bottom: 12px;
    padding: 0 8px;
  }
  .sidebar-nav { list-style: none; }
  .sidebar-nav li + li { margin-top: 2px; }
  .sidebar-section-label {
    font-size: 10px;
    font-weight: 700;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--text-muted);
    padding: 12px 8px 4px;
    display: block;
  }
  .sidebar-nav a {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 10px;
    border-radius: var(--radius-sm);
    text-decoration: none;
    color: var(--text-secondary);
    font-size: 13px;
    font-weight: 500;
    transition: color var(--transition), background var(--transition);
  }
  .sidebar-nav a:hover { color: var(--text-primary); background: var(--glass-hover); }
  .sidebar-nav a.active { color: var(--accent); background: var(--accent-soft); }
  .sidebar-nav a .dot {
    width: 6px; height: 6px; border-radius: 50%;
    background: currentColor; flex-shrink: 0; opacity: 0.6;
  }

  /* Main */
  .net-content { flex: 1; min-width: 0; }
  .net-header { margin-bottom: 28px; }
  .net-header .label { margin-bottom: 14px; }
  .net-header h1 { margin-bottom: 8px; }
  .net-header p { font-size: 14px; line-height: 1.75; max-width: 640px; }

  /* Concept card */
  .concept-card {
    padding: 24px;
    margin-bottom: 24px;
    border-radius: var(--radius-lg);
  }
  .concept-card h3 { margin-bottom: 10px; }
  .concept-card p { font-size: 13.5px; line-height: 1.75; }

  .term-row {
    display: flex;
    gap: 10px;
    margin-top: 18px;
    flex-wrap: wrap;
  }
  .term-chip {
    position: relative;
    padding: 8px 14px;
    border-radius: 100px;
    border: 1px solid var(--accent-border);
    background: var(--accent-soft);
    color: var(--accent);
    font-size: 11.5px;
    font-weight: 700;
    letter-spacing: 0.06em;
    cursor: pointer;
    transition: background var(--transition), transform var(--transition);
  }
  .term-chip:hover, .term-chip:focus-visible {
    background: var(--accent);
    color: #fff;
    transform: translateY(-1px);
    outline: none;
  }
  .term-chip .term-pop {
    position: absolute;
    top: calc(100% + 8px);
    left: 0;
    width: 250px;
    padding: 12px 14px;
    border-radius: var(--radius-sm);
    background: var(--bg-elevated);
    border: 1px solid var(--glass-border);
    color: var(--text-secondary);
    font-size: 12px;
    font-weight: 400;
    letter-spacing: 0;
    line-height: 1.6;
    text-transform: none;
    box-shadow: var(--shadow-md);
    opacity: 0;
    pointer-events: none;
    transform: translateY(-4px);
    transition: opacity var(--transition), transform var(--transition);
    z-index: 30;
  }
  .term-chip:hover .term-pop,
  .term-chip:focus-visible .term-pop { opacity: 1; pointer-events: auto; transform: translateY(0); }

  /* Cards */
  .sim-card {
    padding: 28px;
    border-radius: var(--radius-lg);
    margin-bottom: 24px;
  }
  .sim-title {
    font-size: 13px;
    font-weight: 700;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.06em;
    margin-bottom: 8px;
  }
  .sim-desc { font-size: 13px; margin-bottom: 20px; }

  .sim-controls {
    display: flex;
    gap: 10px;
    flex-wrap: wrap;
    align-items: flex-end;
    margin-bottom: 20px;
  }
  .input-group { display: flex; flex-direction: column; gap: 5px; flex: 1; min-width: 150px; }
  .input-group.small { flex: 0 0 130px; min-width: 130px; }
  .input-group label {
    font-size: 11.5px;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }
  .input-field, select.input-field {
    padding: 9px 14px;
    border-radius: var(--radius-sm);
    border: 1px solid var(--glass-border);
    background: var(--bg-base);
    color: var(--text-primary);
    font-size: 13.5px;
    font-family: 'Inter', monospace;
    transition: border-color var(--transition), background var(--transition);
    outline: none;
    width: 100%;
  }
  textarea.input-field { resize: vertical; min-height: 80px; line-height: 1.6; }
  .input-field:focus { border-color: var(--accent-border); background: var(--glass-hover); }

  .btn-row { display: flex; gap: 8px; flex-wrap: wrap; align-items: center; }
  .btn-run {
    padding: 9px 20px;
    border-radius: var(--radius-sm);
    background: var(--accent);
    color: #fff;
    font-size: 13px;
    font-weight: 600;
    border: none;
    cursor: pointer;
    font-family: inherit;
    transition: opacity var(--transition);
  }
  .btn-run:hover { opacity: 0.85; }
  .btn-run:disabled { opacity: 0.4; cursor: not-allowed; }
  .btn-reset, .btn-ghost {
    padding: 9px 20px;
    border-radius: var(--radius-sm);
    background: var(--glass-bg);
    border: 1px solid var(--glass-border);
    color: var(--text-secondary);
    font-size: 13px;
    font-weight: 600;
    cursor: pointer;
    font-family: inherit;
    transition: background var(--transition), color var(--transition);
  }
  .btn-reset:hover, .btn-ghost:hover { background: var(--glass-hover); color: var(--text-primary); }
  .btn-ghost.active { background: var(--accent-soft); color: var(--accent); border-color: var(--accent-border); }

  .error-box {
    display: none;
    margin-bottom: 16px;
    padding: 12px 16px;
    border-radius: var(--radius-sm);
    background: rgba(239,68,68,0.12);
    border: 1px solid rgba(239,68,68,0.4);
    color: #ef4444;
    font-size: 13px;
    line-height: 1.6;
  }
  .error-box.show { display: block; }
  .error-box ul { margin: 6px 0 0 18px; }
  .ok-box {
    display: none;
    margin-bottom: 16px;
    padding: 10px 16px;
    border-radius: var(--radius-sm);
    background: rgba(74,222,128,0.12);
    border: 1px solid rgba(74,222,128,0.4);
    color: var(--success);
    font-size: 13px;
  }
  .ok-box.show { display: block; }

  /* Device table (editable) */
  .data-table {
    width: 100%;
    border-collapse: collapse;
    margin-top: 12px;
    font-size: 12.5px;
  }
  .data-table th, .data-table td {
    text-align: left;
    padding: 7px 8px;
    border-bottom: 1px solid var(--glass-border);
    vertical-align: middle;
  }
  .data-table th {
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    color: var(--text-muted);
  }
  .data-table .mini-input {
    padding: 5px 8px;
    border-radius: 6px;
    border: 1px solid var(--glass-border);
    background: var(--bg-base);
    color: var(--text-primary);
    font-size: 12px;
    font-family: inherit;
    width: 100%;
    min-width: 70px;
    outline: none;
  }
  .data-table .mini-input:focus { border-color: var(--accent-border); }
  .data-table select.mini-input { cursor: pointer; }
  .status-pill {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 3px 10px;
    border-radius: 100px;
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.03em;
  }
  .status-pill .sdot { width: 8px; height: 8px; border-radius: 50%; display: inline-block; }
  .status-pill.normal { background: rgba(74,222,128,0.14); color: var(--success); }
  .status-pill.normal .sdot { background: var(--success); }
  .status-pill.mati { background: rgba(120,120,140,0.18); color: var(--text-muted); }
  .status-pill.mati .sdot { background: var(--text-muted); }
  .status-pill.waspada { background: rgba(251,191,36,0.14); color: var(--warning); }
  .status-pill.waspada .sdot { background: var(--warning); }
  .status-pill.kritis { background: rgba(239,68,68,0.14); color: #ef4444; }
  .status-pill.kritis .sdot { background: #ef4444; }

  .link-list { display: flex; flex-wrap: wrap; gap: 8px; margin-top: 12px; }
  .link-chip {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 5px 8px 5px 12px;
    border-radius: 100px;
    background: var(--bg-base);
    border: 1px solid var(--glass-border);
    font-size: 11.5px;
    font-family: 'Inter', monospace;
    color: var(--text-secondary);
  }
  .link-chip button {
    border: none;
    background: var(--glass-hover);
    color: var(--text-secondary);
    border-radius: 50%;
    width: 18px; height: 18px;
    cursor: pointer;
    font-size: 12px;
    line-height: 1;
    font-family: inherit;
  }
  .link-chip button:hover { background: rgba(239,68,68,0.2); color: #ef4444; }

  /* Scene */
  .scene-wrap {
    position: relative;
    border: 1px solid var(--glass-border);
    border-radius: var(--radius-sm);
    background: var(--bg-base);
    margin-top: 8px;
    overflow: hidden;
    aspect-ratio: 16 / 10;
    min-height: 320px;
  }
  #scene3d, #scene2d {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
  }
  #scene3d canvas { display: block; width: 100%; height: 100%; }
  #scene2d svg { width: 100%; height: 100%; display: block; }
  .scene-hidden { display: none !important; }

  .label-layer {
    position: absolute;
    inset: 0;
    pointer-events: none;
    overflow: hidden;
  }
  .float-label {
    position: absolute;
    transform: translate(-50%, -120%);
    padding: 2px 8px;
    border-radius: 6px;
    background: var(--glass-bg);
    border: 1px solid var(--glass-border);
    backdrop-filter: var(--glass-blur);
    -webkit-backdrop-filter: var(--glass-blur);
    color: var(--text-primary);
    font-size: 10px;
    font-weight: 600;
    white-space: nowrap;
    font-family: 'Inter', sans-serif;
    letter-spacing: 0.01em;
  }
  .float-label .fl-dot {
    display: inline-block;
    width: 7px; height: 7px;
    border-radius: 50%;
    margin-right: 5px;
    vertical-align: middle;
  }

  .scene-toolbar {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
    align-items: center;
    margin-bottom: 12px;
  }
  .scene-toolbar .spacer { flex: 1; }

  .legend {
    display: flex;
    gap: 16px;
    flex-wrap: wrap;
    margin-top: 12px;
    font-size: 11.5px;
    color: var(--text-secondary);
  }
  .legend span { display: inline-flex; align-items: center; gap: 6px; }
  .legend i.dot { width: 10px; height: 10px; border-radius: 50%; display: inline-block; }

  /* Detail panel */
  .detail-panel {
    margin-top: 16px;
    padding: 16px 18px;
    border-radius: var(--radius-sm);
    background: var(--bg-layer);
    border: 1px solid var(--glass-border);
    display: none;
  }
  .detail-panel.show { display: block; }
  .detail-panel h4 { margin-bottom: 10px; }
  .detail-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(130px, 1fr));
    gap: 12px;
  }
  .detail-item .dl {
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-muted);
  }
  .detail-item .dv {
    font-size: 14px;
    font-weight: 600;
    font-variant-numeric: tabular-nums;
  }

  /* Module tabs */
  .module-tabs {
    display: flex;
    gap: 6px;
    margin-bottom: 20px;
    flex-wrap: wrap;
  }
  .module-tab {
    padding: 9px 16px;
    border-radius: 100px;
    border: 1px solid var(--glass-border);
    background: var(--glass-bg);
    color: var(--text-secondary);
    font-size: 12.5px;
    font-weight: 600;
    cursor: pointer;
    font-family: inherit;
    transition: all var(--transition);
  }
  .module-tab:hover { color: var(--text-primary); background: var(--glass-hover); }
  .module-tab.active { background: var(--accent); color: #fff; border-color: var(--accent); }
  .module-panel { display: none; }
  .module-panel.active { display: block; }
  .soon {
    padding: 28px;
    border-radius: var(--radius-lg);
    text-align: center;
    color: var(--text-secondary);
    font-size: 13.5px;
    border: 1px dashed var(--glass-border);
  }
  .soon .big { font-size: 15px; font-weight: 700; color: var(--text-primary); margin-bottom: 6px; }

  .hint { font-size: 11.5px; color: var(--text-muted); margin-top: 10px; line-height: 1.6; }

  @media (max-width: 760px) {
    .detail-grid { grid-template-columns: 1fr 1fr; }
  }
  @media (max-width: 680px) {
    .net-layout { flex-direction: column; }
    .sidebar { width: 100%; }
    .sidebar-inner { position: static; }
    .term-chip .term-pop { width: 200px; }
    .sim-card { padding: 20px; }
  }
</style>

<div class="net-layout">
  <!-- Sidebar -->
  <aside class="sidebar">
    <div class="sidebar-inner glass">
      <div class="sidebar-title">Pembelajaran</div>
      <ul class="sidebar-nav">
        <span class="sidebar-section-label">Algoritma Sorting</span>
        <li><a href="/Learning/Insert-Sorting/"><span class="dot"></span>Insertion Sort</a></li>
        <li><a href="/Learning/Bubble-Sorting/"><span class="dot"></span>Bubble Sort</a></li>
        <span class="sidebar-section-label">Pointer &amp; Memori</span>
        <li><a href="/Learning/Pointer/"><span class="dot"></span>Pointer Simulation</a></li>
        <span class="sidebar-section-label">Statistika &amp; Probabilitas</span>
        <li><a href="/Learning/Distribusi-Normal/"><span class="dot"></span>Distribusi Normal</a></li>
        <span class="sidebar-section-label">Graf Terapan</span>
        <li><a href="/Learning/Pohon/"><span class="dot"></span>Pohon (Tree)</a></li>
        <span class="sidebar-section-label">Jaringan Komputer</span>
        <li><a href="/Learning/Jaringan-Komputer/" class="active"><span class="dot"></span>Jaringan Komputer</a></li>
      </ul>
    </div>
  </aside>

  <!-- Main -->
  <main class="net-content">
    <div class="net-header">
      <div class="label">Jaringan Komputer</div>
      <h1 style="margin-top:16px;">Jaringan Komputer</h1>
      <p style="margin-top:10px;">
        Pelajari cara membangun, memantau, dan mengamankan jaringan komputer melalui
        simulasi diorama 3D interaktif. Susun topologi, lalu jalankan modul Monitoring,
        Defense in Depth, dan Troubleshooting. Seluruh status, skor, dan grafik dihitung
        langsung dari data yang Anda masukkan.
      </p>
    </div>

    <!-- Concept -->
    <div class="concept-card glass">
      <h3>Konsep Dasar Jaringan Komputer</h3>
      <p>
        Jaringan komputer adalah kumpulan perangkat yang saling terhubung untuk berbagi
        sumber daya dan informasi. Kinerjanya harus dijaga melalui pemantauan, pertahanan
        berlapis, dan penanganan masalah yang sistematis.
      </p>
      <div class="term-row">
        <span class="term-chip" tabindex="0" role="button" aria-label="Definisi Monitoring">
          MONITORING
          <span class="term-pop">Proses berkelanjutan mengumpulkan dan menganalisis metrik (CPU, bandwidth, latency, packet loss) untuk mendeteksi anomali lebih dini.</span>
        </span>
        <span class="term-chip" tabindex="0" role="button" aria-label="Definisi Defense in Depth">
          DEFENSE IN DEPTH
          <span class="term-pop">Strategi keamanan berlapis: edukasi, fisik, perimeter, jaringan, host, aplikasi, dan data — sehingga tidak ada satu titik kegagalan tunggal.</span>
        </span>
        <span class="term-chip" tabindex="0" role="button" aria-label="Definisi Troubleshooting">
          TROUBLESHOOTING
          <span class="term-pop">Metode sistematis mengisolasi penyebab masalah jaringan lewat pengukuran bertahap dari lapisan fisik hingga aplikasi.</span>
        </span>
        <span class="term-chip" tabindex="0" role="button" aria-label="Definisi Baseline">
          BASELINE
          <span class="term-pop">Nilai acuan kinerja normal jaringan yang dipakai untuk mendeteksi penyimpangan (deviasi) dari kondisi seharusnya.</span>
        </span>
      </div>
    </div>

    <!-- Topology editor -->
    <div class="sim-card glass">
      <div class="sim-title">Editor Topologi</div>
      <p class="sim-desc">
        Topologi ini adalah state bersama yang dibaca oleh Modul 1, 2, dan 3.
        Pilih preset, lalu tambah/hapus perangkat, ubah tipe atau nama, dan hubungkan
        perangkat lewat daftar link. Nama harus unik, tidak boleh ada self-link, dan
        setiap perangkat wajib memiliki jalur ke Internet (maksimum 20 perangkat).
      </p>

      <div class="error-box" id="netErr" role="alert"></div>
      <div class="ok-box" id="netOk"></div>

      <div class="sim-controls">
        <div class="input-group small">
          <label for="presetSel">Preset Topologi</label>
          <select class="input-field" id="presetSel">
            <option value="kantor">Kantor Kecil</option>
            <option value="kampus">Kampus</option>
            <option value="datacenter">Data Center Mini</option>
          </select>
        </div>
        <div class="input-group small">
          <label for="newName">Nama Perangkat Baru</label>
          <input class="input-field" id="newName" type="text" placeholder="mis. PC-9" />
        </div>
        <div class="input-group small">
          <label for="newType">Tipe</label>
          <select class="input-field" id="newType"></select>
        </div>
      </div>

      <div class="btn-row" style="margin-bottom:8px;">
        <button class="btn-run" id="btnAddDevice">Tambah Perangkat</button>
        <button class="btn-ghost" id="btnLoadPreset">Muat Preset</button>
        <button class="btn-reset" id="btnResetTopo">Reset</button>
      </div>

      <table class="data-table" id="deviceTable">
        <thead>
          <tr><th>Nama</th><th>Tipe</th><th>Status</th><th>Aksi</th></tr>
        </thead>
        <tbody></tbody>
      </table>

      <div class="sim-title" style="margin-top:24px;">Daftar Link</div>
      <div class="sim-controls" style="margin-bottom:8px;">
        <div class="input-group small">
          <label for="linkA">Dari</label>
          <select class="input-field" id="linkA"></select>
        </div>
        <div class="input-group small">
          <label for="linkB">Ke</label>
          <select class="input-field" id="linkB"></select>
        </div>
        <div style="flex:0 0 auto;">
          <button class="btn-run" id="btnAddLink" style="height:38px;">Hubungkan</button>
        </div>
      </div>
      <div class="link-list" id="linkList"></div>
      <p class="hint">Anda juga dapat menghubungkan perangkat langsung di adegan 3D: klik satu perangkat lalu klik perangkat lain.</p>
    </div>

    <!-- Scene -->
    <div class="sim-card glass">
      <div class="sim-title">Visualisasi Topologi</div>
      <div class="scene-toolbar">
        <button class="btn-ghost active" id="view3d">3D</button>
        <button class="btn-ghost" id="view2d">Mode 2D</button>
        <button class="btn-ghost" id="viewTable">Tabel</button>
        <span class="spacer"></span>
        <button class="btn-ghost" id="btnResetView">Reset Kamera</button>
      </div>

      <div class="scene-wrap">
        <div id="scene3d"></div>
        <div id="scene2d" class="scene-hidden"></div>
        <div class="label-layer" id="labelLayer"></div>
      </div>

      <div class="legend" aria-hidden="true">
        <span><i class="dot" style="background:#4ade80"></i> Normal (terjangkau)</span>
        <span><i class="dot" style="background:#fbbf24"></i> Waspada</span>
        <span><i class="dot" style="background:#ef4444"></i> Kritis</span>
        <span><i class="dot" style="background:#55556a"></i> Mati / tidak terjangkau</span>
        <span><i class="dot" style="background:#6c8eff"></i> Paket data</span>
      </div>

      <div class="detail-panel" id="detailPanel">
        <h4 id="detailTitle">Detail Perangkat</h4>
        <div class="detail-grid" id="detailGrid"></div>
      </div>

      <div id="tableWrap" style="display:none; margin-top:16px; overflow-x:auto;">
        <table class="data-table" id="accessTable">
          <thead>
            <tr><th>Perangkat</th><th>Tipe</th><th>Status</th><th>Jalur ke Internet</th><th>Layer</th></tr>
          </thead>
          <tbody></tbody>
        </table>
      </div>
      <p class="hint" id="sceneHint">Klik perangkat pada kanvas untuk melihat detail. Seret untuk memutar kamera, gulir untuk zoom.</p>
    </div>

    <!-- Module tabs -->
    <div class="module-tabs" role="tablist" aria-label="Modul pembelajaran">
      <button class="module-tab active" data-target="mod1" role="tab" aria-selected="true">Modul 1 &middot; Monitoring</button>
      <button class="module-tab" data-target="mod2" role="tab" aria-selected="false">Modul 2 &middot; Defense in Depth</button>
      <button class="module-tab" data-target="mod3" role="tab" aria-selected="false">Modul 3 &middot; Troubleshooting</button>
    </div>
    <div class="module-panel active" id="mod1" role="tabpanel">
      <div class="soon"><div class="big">Modul 1 — Monitoring Jaringan</div>Segera hadir pada tahap berikutnya.</div>
    </div>
    <div class="module-panel" id="mod2" role="tabpanel">
      <div class="soon"><div class="big">Modul 2 — Defense in Depth</div>Segera hadir pada tahap berikutnya.</div>
    </div>
    <div class="module-panel" id="mod3" role="tabpanel">
      <div class="soon"><div class="big">Modul 3 — Troubleshooting Jaringan Lambat</div>Segera hadir pada tahap berikutnya.</div>
    </div>
  </main>
</div>

<script type="importmap">
{
  "imports": {
    "three": "https://unpkg.com/three@0.160.0/build/three.module.js",
    "three/addons/": "https://unpkg.com/three@0.160.0/examples/jsm/"
  }
}
</script>

<script>
/* =====================================================================
   Jaringan Komputer — shared topology store + pure logic (stage a)
   Pure functions mirror src/network_algo.rs (tested reference).
   ===================================================================== */
var NetApp = (function () {
  'use strict';

  var MAX_DEVICES = 20;

  var TYPES = {
    internet:  { label: 'Internet',        color: '#6c8eff' },
    firewall:  { label: 'Firewall',         color: '#ef4444' },
    router:    { label: 'Router',           color: '#3a6bdf' },
    switch:    { label: 'Switch',           color: '#22d3ee' },
    pc:        { label: 'PC / Klien',       color: '#4ade80' },
    server:    { label: 'Server',           color: '#a78bfa' },
    ap:        { label: 'Access Point WiFi',color: '#fbbf24' },
    database:  { label: 'Database',         color: '#34d399' },
    cctv:      { label: 'CCTV / Sensor',    color: '#f472b6' },
    ids:       { label: 'IDS / IPS',        color: '#fb7185' }
  };

  // ---- seeded RNG (mulberry32) for deterministic animation ----
  function mulberry32(seed) {
    var a = seed >>> 0;
    return function () {
      a |= 0; a = (a + 0x6D2B79F5) | 0;
      var t = Math.imul(a ^ (a >>> 15), 1 | a);
      t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
      return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
    };
  }

  var uidCounter = 0;
  function uid(prefix) { uidCounter++; return (prefix || 'd') + '_' + uidCounter; }

  // ---- presets (only fill the form; editable afterwards) ----
  function preset(kind) {
    if (kind === 'kampus') {
      var d = [
        { id: uid('i'), name: 'Internet', kind: 'internet' },
        { id: uid('f'), name: 'Firewall', kind: 'firewall' },
        { id: uid('r'), name: 'Router', kind: 'router' },
        { id: uid('s1'), name: 'Switch-GedungA', kind: 'switch' },
        { id: uid('s2'), name: 'Switch-GedungB', kind: 'switch' },
        { id: uid('sv'), name: 'Server', kind: 'server' },
        { id: uid('p1'), name: 'PC-A1', kind: 'pc' },
        { id: uid('p2'), name: 'PC-A2', kind: 'pc' },
        { id: uid('p3'), name: 'PC-B1', kind: 'pc' }
      ];
      return { devices: d, links: [
        { a: d[0].id, b: d[1].id }, { a: d[1].id, b: d[2].id },
        { a: d[2].id, b: d[3].id }, { a: d[2].id, b: d[4].id },
        { a: d[3].id, b: d[5].id }, { a: d[3].id, b: d[6].id },
        { a: d[3].id, b: d[7].id }, { a: d[4].id, b: d[8].id }
      ] };
    }
    if (kind === 'datacenter') {
      var e = [
        { id: uid('i'), name: 'Internet', kind: 'internet' },
        { id: uid('f'), name: 'Firewall', kind: 'firewall' },
        { id: uid('r'), name: 'Router-Core', kind: 'router' },
        { id: uid('s1'), name: 'Switch-Top1', kind: 'switch' },
        { id: uid('s2'), name: 'Switch-Top2', kind: 'switch' },
        { id: uid('db'), name: 'DB-Primary', kind: 'database' },
        { id: uid('sv1'), name: 'App-Server-1', kind: 'server' },
        { id: uid('sv2'), name: 'App-Server-2', kind: 'server' },
        { id: uid('ids'), name: 'IDS', kind: 'ids' }
      ];
      return { devices: e, links: [
        { a: e[0].id, b: e[1].id }, { a: e[1].id, b: e[2].id },
        { a: e[2].id, b: e[3].id }, { a: e[2].id, b: e[4].id },
        { a: e[3].id, b: e[5].id }, { a: e[3].id, b: e[6].id },
        { a: e[4].id, b: e[7].id }, { a: e[4].id, b: e[8].id }
      ] };
    }
    // default: kantor kecil
    var f = [
      { id: uid('i'), name: 'Internet', kind: 'internet' },
      { id: uid('f'), name: 'Firewall', kind: 'firewall' },
      { id: uid('r'), name: 'Router-1', kind: 'router' },
      { id: uid('s'), name: 'Switch-1', kind: 'switch' },
      { id: uid('p1'), name: 'PC-1', kind: 'pc' },
      { id: uid('p2'), name: 'PC-2', kind: 'pc' },
      { id: uid('sv'), name: 'Server', kind: 'server' },
      { id: uid('ap'), name: 'AP-WiFi', kind: 'ap' }
    ];
    return { devices: f, links: [
      { a: f[0].id, b: f[1].id }, { a: f[1].id, b: f[2].id },
      { a: f[2].id, b: f[3].id }, { a: f[3].id, b: f[4].id },
      { a: f[3].id, b: f[5].id }, { a: f[3].id, b: f[6].id },
      { a: f[3].id, b: f[7].id }
    ] };
  }

  // ---- pure logic ----
  function validate(devices, links) {
    var errors = [];
    if (devices.length > MAX_DEVICES) {
      errors.push('Jumlah perangkat ' + devices.length + ' melebihi batas maksimum ' + MAX_DEVICES + '.');
    }
    var seenId = {}, seenName = {};
    for (var i = 0; i < devices.length; i++) {
      if (seenId[devices[i].id]) errors.push('ID perangkat ganda: ' + devices[i].id + '.');
      seenId[devices[i].id] = true;
      var key = devices[i].name.trim().toLowerCase();
      if (seenName[key]) errors.push('Nama perangkat ganda: ' + devices[i].name + '.');
      seenName[key] = true;
    }
    var known = {};
    devices.forEach(function (d) { known[d.id] = true; });
    var seenLink = {};
    for (var j = 0; j < links.length; j++) {
      var l = links[j];
      if (l.a === l.b) { errors.push('Link ke diri sendiri tidak diizinkan: ' + l.a + '.'); continue; }
      if (!known[l.a] || !known[l.b]) { errors.push('Link memakai perangkat yang tidak dikenal.'); continue; }
      var k = l.a < l.b ? l.a + '|' + l.b : l.b + '|' + l.a;
      if (seenLink[k]) errors.push('Link ganda: ' + l.a + '-' + l.b + '.');
      seenLink[k] = true;
    }
    if (!devices.some(function (d) { return d.kind === 'internet'; })) {
      errors.push("Topologi harus memiliki perangkat 'Internet'.");
    }
    var reach = reachableFromInternet(devices, links);
    devices.forEach(function (d) {
      if (!reach[d.id]) errors.push("Perangkat '" + d.name + "' tidak memiliki jalur ke Internet.");
    });
    return errors;
  }

  function reachableFromInternet(devices, links) {
    var adj = {}, hasInternet = false;
    devices.forEach(function (d) { adj[d.id] = []; if (d.kind === 'internet') hasInternet = true; });
    if (!hasInternet) return {};
    links.forEach(function (l) {
      if (adj[l.a] && adj[l.b]) { adj[l.a].push(l.b); adj[l.b].push(l.a); }
    });
    var reach = {}, stack = [];
    devices.forEach(function (d) { if (d.kind === 'internet') { reach[d.id] = true; stack.push(d.id); } });
    while (stack.length) {
      var cur = stack.pop();
      adj[cur].forEach(function (n) { if (!reach[n]) { reach[n] = true; stack.push(n); } });
    }
    return reach;
  }

  function healthScore(devices, links) {
    if (!devices.length) return 0;
    var reach = reachableFromInternet(devices, links);
    return (Object.keys(reach).length / devices.length) * 100;
  }

  function statusOf(id, devices, links) {
    return reachableFromInternet(devices, links)[id] ? 'normal' : 'mati';
  }

  // Layered layout by BFS depth from Internet (deterministic).
  function computeDepths(devices, links) {
    var adj = {};
    devices.forEach(function (d) { adj[d.id] = []; });
    links.forEach(function (l) { if (adj[l.a] && adj[l.b]) { adj[l.a].push(l.b); adj[l.b].push(l.a); } });
    var depth = {}, q = [];
    devices.forEach(function (d) { if (d.kind === 'internet') { depth[d.id] = 0; q.push(d.id); } });
    while (q.length) {
      var cur = q.shift();
      adj[cur].forEach(function (n) { if (depth[n] === undefined) { depth[n] = depth[cur] + 1; q.push(n); } });
    }
    devices.forEach(function (d) { if (depth[d.id] === undefined) depth[d.id] = 99; });
    return depth;
  }

  function layout(devices, links) {
    var depth = computeDepths(devices, links);
    var layers = {};
    devices.forEach(function (d) { var k = depth[d.id]; (layers[k] = layers[k] || []).push(d); });
    var maxLayer = 0;
    Object.keys(layers).forEach(function (k) { maxLayer = Math.max(maxLayer, parseInt(k, 10)); });
    var pos = {}, spacingX = 3.4;
    Object.keys(layers).forEach(function (k) {
      var arr = layers[k], d = parseInt(k, 10);
      arr.forEach(function (dev, i) {
        pos[dev.id] = { x: d * spacingX - (maxLayer * spacingX) / 2, z: (i - (arr.length - 1) / 2) * 2.6, layer: d };
      });
    });
    return pos;
  }

  // ---- shared store ----
  var store = { devices: [], links: [] };
  var listeners = [];

  function get() { return store; }
  function set(next) { store = next; emit(); }
  function subscribe(fn) { listeners.push(fn); }
  function emit() { listeners.forEach(function (fn) { fn(store); }); }

  function loadPreset(kind) { var p = preset(kind); set(p); }

  return {
    MAX_DEVICES: MAX_DEVICES, TYPES: TYPES, mulberry32: mulberry32,
    preset: preset, validate: validate, reachableFromInternet: reachableFromInternet,
    healthScore: healthScore, statusOf: statusOf, computeDepths: computeDepths, layout: layout,
    get: get, set: set, subscribe: subscribe, loadPreset: loadPreset,
    typeLabel: function (k) { return (TYPES[k] || { label: k }).label; },
    typeColor: function (k) { return (TYPES[k] || { color: '#9898b0' }).color; }
  };
})();

/* =====================================================================
   UI wiring: editor, tables, view switching
   ===================================================================== */
(function () {
  'use strict';
  var $ = function (id) { return document.getElementById(id); };

  function fillTypeSelect(sel, includeBlank) {
    sel.innerHTML = '';
    if (includeBlank) {
      var o0 = document.createElement('option'); o0.value = ''; o0.textContent = '— pilih —'; sel.appendChild(o0);
    }
    Object.keys(NetApp.TYPES).forEach(function (k) {
      var o = document.createElement('option'); o.value = k; o.textContent = NetApp.TYPES[k].label; sel.appendChild(o);
    });
  }

  function showError(msgs) {
    var box = $('netErr');
    if (!msgs || !msgs.length) { box.classList.remove('show'); box.innerHTML = ''; return; }
    var html = '<strong>Topologi tidak valid:</strong><ul>';
    msgs.forEach(function (m) { html += '<li>' + m + '</li>'; });
    html += '</ul>';
    box.innerHTML = html; box.classList.add('show');
  }
  function showOk(msg) {
    var box = $('netOk');
    if (!msg) { box.classList.remove('show'); return; }
    box.textContent = msg; box.classList.add('show');
  }

  function nameExists(name, exceptId) {
    var k = name.trim().toLowerCase();
    return NetApp.get().devices.some(function (d) { return d.id !== exceptId && d.name.trim().toLowerCase() === k; });
  }

  function renderDevices() {
    var st = NetApp.get();
    var reach = NetApp.reachableFromInternet(st.devices, st.links);
    var tb = $('deviceTable').querySelector('tbody');
    tb.innerHTML = '';
    st.devices.forEach(function (d) {
      var status = reach[d.id] ? 'normal' : 'mati';
      var tr = document.createElement('tr');

      var tdName = document.createElement('td');
      var inpName = document.createElement('input');
      inpName.className = 'mini-input'; inpName.value = d.name;
      inpName.addEventListener('change', function () {
        var v = inpName.value.trim();
        if (!v) { showError(['Nama perangkat tidak boleh kosong.']); inpName.value = d.name; return; }
        if (nameExists(v, d.id)) { showError(['Nama perangkat ganda: ' + v + '.']); inpName.value = d.name; return; }
        d.name = v; showError(null); showOk(null); NetApp.set(st); renderAll();
      });
      tdName.appendChild(inpName);

      var tdType = document.createElement('td');
      var selType = document.createElement('select');
      selType.className = 'mini-input';
      Object.keys(NetApp.TYPES).forEach(function (k) {
        var o = document.createElement('option'); o.value = k; o.textContent = NetApp.TYPES[k].label;
        if (k === d.kind) o.selected = true; selType.appendChild(o);
      });
      selType.addEventListener('change', function () { d.kind = selType.value; NetApp.set(st); renderAll(); });
      tdType.appendChild(selType);

      var tdStatus = document.createElement('td');
      var pill = document.createElement('span');
      pill.className = 'status-pill ' + status;
      pill.innerHTML = '<span class="sdot"></span>' + (status === 'normal' ? 'Normal' : 'Mati');
      tdStatus.appendChild(pill);

      var tdAct = document.createElement('td');
      var del = document.createElement('button');
      del.className = 'btn-reset'; del.style.padding = '4px 12px'; del.style.fontSize = '12px';
      del.textContent = 'Hapus';
      del.addEventListener('click', function () {
        var next = {
          devices: st.devices.filter(function (x) { return x.id !== d.id; }),
          links: st.links.filter(function (l) { return l.a !== d.id && l.b !== d.id; })
        };
        NetApp.set(next); renderAll();
      });
      tdAct.appendChild(del);

      tr.appendChild(tdName); tr.appendChild(tdType); tr.appendChild(tdStatus); tr.appendChild(tdAct);
      tb.appendChild(tr);
    });
  }

  function renderLinks() {
    var st = NetApp.get();
    var list = $('linkList'); list.innerHTML = '';
    if (!st.links.length) { list.innerHTML = '<span style="font-size:12px;color:var(--text-muted);">Belum ada link.</span>'; }
    st.links.forEach(function (l, idx) {
      var na = (st.devices.find(function (d) { return d.id === l.a; }) || {}).name || l.a;
      var nb = (st.devices.find(function (d) { return d.id === l.b; }) || {}).name || l.b;
      var chip = document.createElement('span');
      chip.className = 'link-chip';
      chip.innerHTML = '<span>' + na + ' — ' + nb + '</span>';
      var btn = document.createElement('button'); btn.textContent = '×';
      btn.setAttribute('aria-label', 'Hapus link ' + na + ' ke ' + nb);
      btn.addEventListener('click', function () {
        var next = { devices: st.devices.slice(), links: st.links.filter(function (_, i) { return i !== idx; }) };
        NetApp.set(next); renderAll();
      });
      chip.appendChild(btn); list.appendChild(chip);
    });

    // link selects
    var selA = $('linkA'), selB = $('linkB');
    var prevA = selA.value, prevB = selB.value;
    selA.innerHTML = ''; selB.innerHTML = '';
    st.devices.forEach(function (d) {
      var o1 = document.createElement('option'); o1.value = d.id; o1.textContent = d.name; selA.appendChild(o1);
      var o2 = document.createElement('option'); o2.value = d.id; o2.textContent = d.name; selB.appendChild(o2);
    });
    if (prevA) selA.value = prevA; if (prevB) selB.value = prevB;
  }

  function renderAccessTable() {
    var st = NetApp.get();
    var reach = NetApp.reachableFromInternet(st.devices, st.links);
    var depth = NetApp.computeDepths(st.devices, st.links);
    var tb = $('accessTable').querySelector('tbody'); tb.innerHTML = '';
    st.devices.forEach(function (d) {
      var status = reach[d.id] ? 'normal' : 'mati';
      var tr = document.createElement('tr');
      tr.innerHTML =
        '<td>' + d.name + '</td>' +
        '<td>' + NetApp.typeLabel(d.kind) + '</td>' +
        '<td><span class="status-pill ' + status + '"><span class="sdot"></span>' + (status === 'normal' ? 'Normal' : 'Mati') + '</span></td>' +
        '<td>' + (reach[d.id] ? 'Ya' : 'Tidak') + '</td>' +
        '<td>' + (depth[d.id] === 99 ? '—' : depth[d.id]) + '</td>';
      tb.appendChild(tr);
    });
  }

  function renderAll() {
    renderDevices(); renderLinks(); renderAccessTable();
    var errs = NetApp.validate(NetApp.get().devices, NetApp.get().links);
    if (errs.length) { showError(errs); showOk(null); }
    else { showError(null); showOk('Topologi valid — ' + NetApp.get().devices.length + ' perangkat, skor kesehatan ' + Math.round(NetApp.healthScore(NetApp.get().devices, NetApp.get().links)) + '/100.'); }
    window.dispatchEvent(new CustomEvent('netstorechange'));
  }

  // ---- editor actions ----
  $('newType').innerHTML = '';
  fillTypeSelect($('newType'), false);

  $('btnAddDevice').addEventListener('click', function () {
    var st = NetApp.get();
    if (st.devices.length >= NetApp.MAX_DEVICES) { showError(['Batas maksimum ' + NetApp.MAX_DEVICES + ' perangkat tercapai.']); return; }
    var name = $('newName').value.trim();
    var kind = $('newType').value;
    if (!name) { showError(['Masukkan nama perangkat.']); return; }
    if (nameExists(name, null)) { showError(['Nama perangkat ganda: ' + name + '.']); return; }
    var id = 'd_' + Date.now() + '_' + Math.floor(Math.random() * 1000);
    var next = { devices: st.devices.concat([{ id: id, name: name, kind: kind }]), links: st.links.slice() };
    NetApp.set(next); $('newName').value = ''; renderAll();
  });

  $('btnLoadPreset').addEventListener('click', function () { NetApp.loadPreset($('presetSel').value); renderAll(); });
  $('btnResetTopo').addEventListener('click', function () { NetApp.loadPreset($('presetSel').value); renderAll(); });

  $('btnAddLink').addEventListener('click', function () {
    var st = NetApp.get();
    var a = $('linkA').value, b = $('linkB').value;
    if (!a || !b) { showError(['Pilih kedua perangkat untuk dihubungkan.']); return; }
    if (a === b) { showError(['Link ke diri sendiri tidak diizinkan.']); return; }
    var dup = st.links.some(function (l) { return (l.a === a && l.b === b) || (l.a === b && l.b === a); });
    if (dup) { showError(['Link sudah ada.']); return; }
    var next = { devices: st.devices.slice(), links: st.links.concat([{ a: a, b: b }]) };
    NetApp.set(next); renderAll();
  });

  // ---- view switching ----
  var view = '3d';
  function setView(v) {
    view = v;
    $('view3d').classList.toggle('active', v === '3d');
    $('view2d').classList.toggle('active', v === '2d');
    $('viewTable').classList.toggle('active', v === 'table');
    $('scene3d').classList.toggle('scene-hidden', v !== '3d');
    $('scene2d').classList.toggle('scene-hidden', v !== '2d');
    $('labelLayer').classList.toggle('scene-hidden', v === 'table');
    $('tableWrap').style.display = v === 'table' ? 'block' : 'none';
    $('sceneHint').textContent = v === '3d'
      ? 'Klik perangkat pada kanvas untuk melihat detail. Seret untuk memutar kamera, gulir untuk zoom.'
      : (v === '2d' ? 'Tampilan 2D (SVG) — klik perangkat untuk melihat detail.' : 'Tabel aksesibel berisi data topologi yang sama.');
    window.dispatchEvent(new CustomEvent('netviewchange', { detail: v }));
    if (v === '2d') render2D();
  }
  $('view3d').addEventListener('click', function () { setView('3d'); });
  $('view2d').addEventListener('click', function () { setView('2d'); });
  $('viewTable').addEventListener('click', function () { setView('table'); });
  $('btnResetView').addEventListener('click', function () { window.dispatchEvent(new CustomEvent('netresetview')); });

  // ---- detail panel ----
  function showDetail(dev) {
    var st = NetApp.get();
    var reach = NetApp.reachableFromInternet(st.devices, st.links);
    var depth = NetApp.computeDepths(st.devices, st.links);
    var status = reach[dev.id] ? 'normal' : 'mati';
    $('detailTitle').textContent = dev.name + ' · ' + NetApp.typeLabel(dev.kind);
    var items = [
      ['Status', status === 'normal' ? 'Normal' : 'Mati / tidak terjangkau'],
      ['Tipe', NetApp.typeLabel(dev.kind)],
      ['Jalur ke Internet', reach[dev.id] ? 'Ya' : 'Tidak'],
      ['Layer (dari Internet)', depth[dev.id] === 99 ? '—' : depth[dev.id]],
      ['Skor Kesehatan', Math.round(NetApp.healthScore(st.devices, st.links)) + '/100']
    ];
    var g = $('detailGrid'); g.innerHTML = '';
    items.forEach(function (it) {
      var d = document.createElement('div'); d.className = 'detail-item';
      d.innerHTML = '<div class="dl">' + it[0] + '</div><div class="dv">' + it[1] + '</div>';
      g.appendChild(d);
    });
    $('detailPanel').classList.add('show');
  }
  window.NetShowDetail = showDetail;

  // ---- 2D fallback renderer ----
  function render2D() {
    var st = NetApp.get();
    var pos = NetApp.layout(st.devices, st.links);
    var reach = NetApp.reachableFromInternet(st.devices, st.links);
    var xs = Object.keys(pos).map(function (k) { return pos[k].x; });
    var zs = Object.keys(pos).map(function (k) { return pos[k].z; });
    var minX = Math.min.apply(null, xs.concat([0])), maxX = Math.max.apply(null, xs.concat([0]));
    var minZ = Math.min.apply(null, zs.concat([0])), maxZ = Math.max.apply(null, zs.concat([0]));
    var W = 100, H = 62, pad = 10;
    function sx(x) { return maxX === minX ? W / 2 : pad + (x - minX) / (maxX - minX) * (W - 2 * pad); }
    function sy(z) { return maxZ === minZ ? H / 2 : pad + (z - minZ) / (maxZ - minZ) * (H - 2 * pad); }
    var parts = ['<svg viewBox="0 0 ' + W + ' ' + H + '" preserveAspectRatio="xMidYMid meet" role="img" aria-label="Topologi jaringan 2D">'];
    // links
    st.links.forEach(function (l) {
      var a = pos[l.a], b = pos[l.b]; if (!a || !b) return;
      var live = reach[l.a] && reach[l.b];
      parts.push('<line x1="' + sx(a.x) + '" y1="' + sy(a.z) + '" x2="' + sx(b.x) + '" y2="' + sy(b.z) + '" stroke="' + (live ? '#6c8eff' : '#55556a') + '" stroke-width="0.6" ' + (live ? '' : 'stroke-dasharray="1.5 1.5"') + '></line>');
    });
    // nodes
    st.devices.forEach(function (d) {
      var p = pos[d.id]; if (!p) return;
      var status = reach[d.id] ? 'normal' : 'mati';
      var color = status === 'normal' ? '#4ade80' : '#55556a';
      var cx = sx(p.x), cy = sy(p.z);
      parts.push('<g data-id="' + d.id + '" style="cursor:pointer;">');
      parts.push('<circle cx="' + cx + '" cy="' + cy + '" r="2.6" fill="' + NetApp.typeColor(d.kind) + '" stroke="' + color + '" stroke-width="0.8"></circle>');
      parts.push('<text x="' + cx + '" y="' + (cy - 4) + '" text-anchor="middle" font-size="2.6" fill="var(--text-primary)" font-family="Inter,sans-serif">' + d.name + '</text>');
      parts.push('</g>');
    });
    parts.push('</svg>');
    var host = $('scene2d'); host.innerHTML = parts.join('');
    host.querySelectorAll('g[data-id]').forEach(function (g) {
      g.addEventListener('click', function () {
        var id = g.getAttribute('data-id');
        var dev = st.devices.find(function (x) { return x.id === id; });
        if (dev) showDetail(dev);
      });
    });
  }
  window.NetRender2D = render2D;

  // ---- module tabs ----
  var tabs = document.querySelectorAll('.module-tab');
  tabs.forEach(function (t) {
    t.addEventListener('click', function () {
      tabs.forEach(function (x) { x.classList.remove('active'); x.setAttribute('aria-selected', 'false'); });
      t.classList.add('active'); t.setAttribute('aria-selected', 'true');
      document.querySelectorAll('.module-panel').forEach(function (p) { p.classList.remove('active'); });
      $(t.getAttribute('data-target')).classList.add('active');
    });
  });

  // ---- init ----
  NetApp.loadPreset('kantor');
  renderAll();
  setView('3d');
  NetApp.subscribe(function () { if (view === '2d') render2D(); });

  window.NetUI = { renderAll: renderAll, setView: setView, showDetail: showDetail };
})();

/* =====================================================================
   3D scene (Three.js, lazy-loaded) — isometric low-poly diorama
   ===================================================================== */
(function () {
  'use strict';
  var THREE = null, OrbitControls = null;
  var scene, camera, renderer, controls, raycaster;
  var deviceGroup, linkGroup, packetGroup, labelLayer;
  var meshes = {};      // deviceId -> group
  var packetMeshes = []; // {mesh, curve, t, speed, linkIndex}
  var labels = {};      // deviceId -> htmlDivElement
  var rafId = null, running = false;
  var reduced = window.matchMedia && window.matchMedia('(prefers-reduced-motion: reduce)').matches;
  var linkFrom = null;  // deviceId selected for click-to-connect
  var clock = null;

  function loadThree() {
    return Promise.all([
      import('three'),
      import('three/addons/controls/OrbitControls.js')
    ]).then(function (mods) {
      THREE = mods[0];
      OrbitControls = mods[1].OrbitControls;
    });
  }

  function colorOf(status) {
    if (status === 'normal') return 0x4ade80;
    if (status === 'waspada') return 0xfbbf24;
    if (status === 'kritis') return 0xef4444;
    return 0x55556a;
  }

  function mat(color, opts) {
    return new THREE.MeshStandardMaterial(Object.assign({ color: color, roughness: 0.75, metalness: 0.1, flatShading: true }, opts || {}));
  }

  function box(w, h, d, color) { return new THREE.Mesh(new THREE.BoxGeometry(w, h, d), mat(color)); }
  function cyl(rt, rb, h, color, seg) { return new THREE.Mesh(new THREE.CylinderGeometry(rt, rb, h, seg || 12), mat(color)); }
  function sph(r, color, seg) { return new THREE.Mesh(new THREE.SphereGeometry(r, seg || 16, seg || 16), mat(color)); }

  // Build a low-poly model for a device kind. Returns a THREE.Group centered at origin.
  function buildModel(kind, colorHex) {
    var g = new THREE.Group();
    var c = colorHex;
    if (kind === 'internet') {
      var cloud = new THREE.Group();
      var puffs = [[0, 0.5, 0, 0.9], [0.8, 0.3, 0.1, 0.7], [-0.8, 0.3, -0.1, 0.7], [0.3, 0.7, -0.2, 0.6], [-0.3, 0.65, 0.2, 0.6]];
      puffs.forEach(function (p) { var s = sph(p[3], c); s.position.set(p[0], p[1], p[2]); cloud.add(s); });
      g.add(cloud);
    } else if (kind === 'firewall') {
      for (var r = 0; r < 2; r++) for (var b = 0; b < 3; b++) {
        var brick = box(0.55, 0.28, 0.3, 0xb04a3a);
        brick.position.set((b - 1) * 0.6 + (r % 2) * 0.15, 0.15 + r * 0.3, 0);
        g.add(brick);
      }
      var flame = new THREE.Mesh(new THREE.ConeGeometry(0.22, 0.5, 8), mat(0xff7a1a, { emissive: 0xff5500, emissiveIntensity: 0.6 }));
      flame.position.set(0, 1.05, 0); g.add(flame);
    } else if (kind === 'router') {
      g.add(box(1.0, 0.4, 0.7, c));
      [-0.35, 0.35].forEach(function (x) {
        var ant = cyl(0.03, 0.03, 0.7, 0x222233, 6); ant.position.set(x, 0.55, 0); g.add(ant);
      });
    } else if (kind === 'switch') {
      g.add(box(1.1, 0.28, 0.8, c));
      for (var p = 0; p < 4; p++) { var led = sph(0.05, 0x22ff88, 8); led.position.set(-0.4 + p * 0.26, 0.18, 0.42); g.add(led); }
    } else if (kind === 'pc') {
      g.add(box(0.9, 0.6, 0.12, 0x222233)); // monitor
      var screen = box(0.78, 0.48, 0.03, 0x113355); screen.position.set(0, 0, 0.08); g.add(screen);
      var stand = box(0.12, 0.25, 0.12, 0x333344); stand.position.set(0, -0.42, 0); g.add(stand);
      var kbd = box(0.5, 0.05, 0.3, 0x444455); kbd.position.set(0, -0.55, 0.1); g.add(kbd);
    } else if (kind === 'server') {
      g.add(box(0.8, 1.6, 0.8, 0x2a2a3a));
      for (var u = 0; u < 4; u++) { var unit = box(0.7, 0.06, 0.05, 0x555566); unit.position.set(0, 0.6 - u * 0.36, 0.42); g.add(unit); var l = sph(0.04, 0x44ff99, 8); l.position.set(0.28, 0.6 - u * 0.36, 0.44); g.add(l); }
    } else if (kind === 'ap') {
      g.add(cyl(0.5, 0.5, 0.12, c, 20));
      for (var w = 1; w <= 2; w++) {
        var ring = new THREE.Mesh(new THREE.TorusGeometry(0.3 * w, 0.02, 6, 24), mat(c, { emissive: c, emissiveIntensity: 0.4 }));
        ring.rotation.x = Math.PI / 2; ring.position.y = 0.1 + w * 0.05; g.add(ring);
      }
    } else if (kind === 'database') {
      g.add(cyl(0.55, 0.55, 1.0, c, 20));
      var cap1 = cyl(0.6, 0.6, 0.08, 0xdddddd, 20); cap1.position.y = 0.5; g.add(cap1);
      var cap2 = cyl(0.6, 0.6, 0.08, 0xdddddd, 20); cap2.position.y = -0.5; g.add(cap2);
    } else if (kind === 'cctv') {
      g.add(box(0.5, 0.3, 0.3, c));
      var lens = cyl(0.1, 0.1, 0.15, 0x111111, 12); lens.rotation.z = Math.PI / 2; lens.position.set(0.3, 0, 0); g.add(lens);
    } else if (kind === 'ids') {
      g.add(box(0.6, 0.6, 0.6, c));
      var eye = sph(0.15, 0xff2222, 12); eye.position.set(0, 0.1, 0.32); g.add(eye);
    } else {
      g.add(box(0.6, 0.6, 0.6, c));
    }
    return g;
  }

  // Human character: round head + box body. role: 'user'|'admin'|'attacker'
  function buildHuman(role) {
    var g = new THREE.Group();
    var bodyColor = role === 'attacker' ? 0x222222 : (role === 'admin' ? 0x2b6cff : 0x4ade80);
    var body = box(0.5, 0.6, 0.35, bodyColor); body.position.y = 0.3; g.add(body);
    var head = sph(0.24, 0xf2c9a0, 16); head.position.y = 0.85; g.add(head);
    if (role === 'admin') {
      var glasses = box(0.34, 0.08, 0.05, 0x111111); glasses.position.set(0, 0.88, 0.22); g.add(glasses);
      var tablet = box(0.3, 0.2, 0.03, 0x88aaff); tablet.position.set(0.35, 0.35, 0.15); g.add(tablet);
    }
    if (role === 'attacker') {
      var hood = sph(0.3, 0x181818, 16); hood.position.y = 0.9; g.add(hood);
      var laptop = box(0.4, 0.3, 0.05, 0xcc2222); laptop.position.set(0, 0.35, 0.3); g.add(laptop);
    }
    return g;
  }

  function clearGroup(grp) {
    if (!grp) return;
    grp.traverse(function (o) {
      if (o.geometry) o.geometry.dispose();
      if (o.material) { if (Array.isArray(o.material)) o.material.forEach(function (m) { m.dispose(); }); else o.material.dispose(); }
    });
    while (grp.children.length) grp.remove(grp.children[0]);
  }

  function buildScene() {
    var st = NetApp.get();
    var pos = NetApp.layout(st.devices, st.links);
    var reach = NetApp.reachableFromInternet(st.devices, st.links);

    clearGroup(deviceGroup); clearGroup(linkGroup); clearGroup(packetGroup);
    meshes = {}; packetMeshes = [];
    if (labelLayer) { labelLayer.innerHTML = ''; labels = {}; }

    // floor tile
    // (persistent floor built once in init; here we only rebuild devices/links)

    // links + packets
    st.links.forEach(function (l, idx) {
      var a = pos[l.a], b = pos[l.b]; if (!a || !b) return;
      var live = reach[l.a] && reach[l.b];
      var start = new THREE.Vector3(a.x, 0.25, a.z);
      var end = new THREE.Vector3(b.x, 0.25, b.z);
      var mid = start.clone().add(end).multiplyScalar(0.5); mid.y += 0.4;
      var curve = new THREE.QuadraticBezierCurve3(start, mid, end);
      var tube = new THREE.Mesh(new THREE.TubeGeometry(curve, 20, 0.04, 6, false), mat(live ? 0x6c8eff : 0x55556a, { emissive: live ? 0x223366 : 0x000000, emissiveIntensity: 0.4 }));
      linkGroup.add(tube);
      if (live && !reduced) {
        var nPackets = 2 + (idx % 3); // deterministic count per link
        for (var p = 0; p < nPackets; p++) {
          var cube = new THREE.Mesh(new THREE.BoxGeometry(0.14, 0.14, 0.14), mat(0x6c8eff, { emissive: 0x6c8eff, emissiveIntensity: 0.8 }));
          packetGroup.add(cube);
          var rng = NetApp.mulberry32(1000 + idx * 17 + p);
          packetMeshes.push({ mesh: cube, curve: curve, t: rng(), speed: 0.15 + (idx % 5) * 0.02, linkIndex: idx });
        }
      }
    });

    // devices
    var hasUser = false, hasAdmin = false;
    st.devices.forEach(function (d) {
      var p = pos[d.id]; if (!p) return;
      var status = reach[d.id] ? 'normal' : 'mati';
      var grp = new THREE.Group();
      grp.position.set(p.x, 0, p.z);

      var model = buildModel(d.kind, new THREE.Color(NetApp.typeColor(d.kind)).getHex());
      model.position.y = 0.5;
      grp.add(model);

      // status light
      var lightMat = new THREE.MeshStandardMaterial({ color: colorOf(status), emissive: colorOf(status), emissiveIntensity: 0.9, flatShading: true });
      var bulb = new THREE.Mesh(new THREE.SphereGeometry(0.12, 12, 12), lightMat);
      bulb.position.set(0, 1.5, 0); grp.add(bulb);
      grp.userData.bulb = bulb; grp.userData.status = status;

      // humans
      if (d.kind === 'pc' && !hasUser) {
        var human = buildHuman('user'); human.position.set(0.7, 0, 0.4); grp.add(human); grp.userData.human = 'user'; hasUser = true;
      }
      if ((d.kind === 'server' || d.kind === 'database') && !hasAdmin) {
        var admin = buildHuman('admin'); admin.position.set(-0.8, 0, 0.3); grp.add(admin); grp.userData.human = 'admin'; hasAdmin = true;
      }

      grp.userData.id = d.id;
      deviceGroup.add(grp);
      meshes[d.id] = grp;

      // floating label
      var el = document.createElement('div');
      el.className = 'float-label';
      el.innerHTML = '<span class="fl-dot" style="background:' + (status === 'normal' ? '#4ade80' : '#55556a') + '"></span>' + d.name;
      labelLayer.appendChild(el);
      labels[d.id] = el;
    });
  }

  function initThree() {
    var host = document.getElementById('scene3d');
    labelLayer = document.getElementById('labelLayer');
    scene = new THREE.Scene();
    scene.background = new THREE.Color(0x0a0a0f);

    camera = new THREE.PerspectiveCamera(45, host.clientWidth / host.clientHeight, 0.1, 100);
    camera.position.set(9, 9, 12);

    renderer = new THREE.WebGLRenderer({ antialias: true, alpha: false });
    renderer.setPixelRatio(Math.min(window.devicePixelRatio || 1, 2));
    renderer.setSize(host.clientWidth, host.clientHeight);
    host.appendChild(renderer.domElement);

    controls = new OrbitControls(camera, renderer.domElement);
    controls.enableDamping = true;
    controls.dampingFactor = 0.08;
    controls.minDistance = 6; controls.maxDistance = 22;
    controls.minPolarAngle = 0.3; controls.maxPolarAngle = 1.35;
    controls.enablePan = false;
    controls.target.set(0, 0.5, 0);

    // lights
    scene.add(new THREE.AmbientLight(0xffffff, 0.55));
    var key = new THREE.DirectionalLight(0xffffff, 0.9); key.position.set(6, 12, 8); scene.add(key);
    var fill = new THREE.DirectionalLight(0x88aaff, 0.35); fill.position.set(-8, 6, -6); scene.add(fill);

    // floor tile (small island)
    var floor = new THREE.Mesh(new THREE.CylinderGeometry(11, 11, 0.5, 8), mat(0x16161f, { roughness: 0.9 }));
    floor.position.y = -0.3; scene.add(floor);
    var grid = new THREE.GridHelper(20, 20, 0x333344, 0x222233);
    grid.position.y = -0.04; scene.add(grid);

    deviceGroup = new THREE.Group(); linkGroup = new THREE.Group(); packetGroup = new THREE.Group();
    scene.add(linkGroup); scene.add(deviceGroup); scene.add(packetGroup);

    raycaster = new THREE.Raycaster();
    clock = new THREE.Clock();

    renderer.domElement.addEventListener('pointerdown', onPointerDown);
    window.addEventListener('resize', onResize);
    document.addEventListener('visibilitychange', function () { document.hidden ? stop() : start(); });
    window.addEventListener('beforeunload', dispose);
    window.addEventListener('netstorechange', buildScene);
    window.addEventListener('netresetview', function () { if (controls) { camera.position.set(9, 9, 12); controls.target.set(0, 0.5, 0); } });

    buildScene();
    start();
  }

  var downPos = null;
  function onPointerDown(evt) {
    downPos = { x: evt.clientX, y: evt.clientY };
    var rect = renderer.domElement.getBoundingClientRect();
    var mouse = new THREE.Vector2(
      ((evt.clientX - rect.left) / rect.width) * 2 - 1,
      -((evt.clientY - rect.top) / rect.height) * 2 + 1
    );
    raycaster.setFromCamera(mouse, camera);
    var hits = raycaster.intersectObjects(deviceGroup.children, true);
    if (!hits.length) return;
    var obj = hits[0].object;
    while (obj && !obj.userData.id) obj = obj.parent;
    if (!obj) return;
    var id = obj.userData.id;
    var st = NetApp.get();
    var dev = st.devices.find(function (x) { return x.id === id; });
    if (!dev) return;
    // click-to-connect flow
    if (linkFrom && linkFrom !== id) {
      var dup = st.links.some(function (l) { return (l.a === linkFrom && l.b === id) || (l.a === id && l.b === linkFrom); });
      if (!dup) {
        var next = { devices: st.devices.slice(), links: st.links.concat([{ a: linkFrom, b: id }]) };
        NetApp.set(next); window.NetUI.renderAll();
      }
      linkFrom = null;
      return;
    }
    linkFrom = id;
    window.NetShowDetail(dev);
  }

  function onResize() {
    if (!renderer) return;
    var host = document.getElementById('scene3d');
    if (!host.clientWidth) return;
    camera.aspect = host.clientWidth / host.clientHeight;
    camera.updateProjectionMatrix();
    renderer.setSize(host.clientWidth, host.clientHeight);
  }

  var tmpVec = null;
  function animate() {
    rafId = requestAnimationFrame(animate);
    var dt = clock ? clock.getDelta() : 0.016;
    if (!reduced) {
      for (var i = 0; i < packetMeshes.length; i++) {
        var pk = packetMeshes[i];
        pk.t += dt * pk.speed;
        if (pk.t > 1) pk.t -= 1;
        var pt = pk.curve.getPoint(pk.t);
        pk.mesh.position.copy(pt);
      }
    }
    if (controls) controls.update();
    if (renderer && scene && camera) {
      renderer.render(scene, camera);
      updateLabels();
    }
  }

  function updateLabels() {
    if (!camera || !renderer) return;
    var host = document.getElementById('scene3d');
    var w = host.clientWidth, h = host.clientHeight;
    var st = NetApp.get();
    st.devices.forEach(function (d) {
      var el = labels[d.id], grp = meshes[d.id];
      if (!el || !grp) return;
      tmpVec = tmpVec || new THREE.Vector3();
      tmpVec.set(grp.position.x, 1.9, grp.position.z);
      tmpVec.project(camera);
      var x = (tmpVec.x * 0.5 + 0.5) * w;
      var y = (-tmpVec.y * 0.5 + 0.5) * h;
      el.style.left = x + 'px';
      el.style.top = y + 'px';
      el.style.display = (tmpVec.z > 1) ? 'none' : 'block';
    });
  }

  function start() { if (!running && renderer) { running = true; clock = clock || new THREE.Clock(); animate(); } }
  function stop() { running = false; if (rafId) { cancelAnimationFrame(rafId); rafId = null; } }

  var disposed = false;
  function dispose() {
    if (disposed) return; disposed = true;
    stop();
    window.removeEventListener('resize', onResize);
    if (scene) scene.traverse(function (o) {
      if (o.geometry) o.geometry.dispose();
      if (o.material) { if (Array.isArray(o.material)) o.material.forEach(function (m) { m.dispose(); }); else o.material.dispose(); }
    });
    if (renderer) { renderer.dispose(); if (renderer.domElement && renderer.domElement.parentNode) renderer.domElement.parentNode.removeChild(renderer.domElement); }
  }

  // Lazy boot: only load three.js on this page, after first paint.
  function boot() {
    if (reduced) {
      // still load so 3D works, but no packet animation
    }
    loadThree().then(function () {
      try { initThree(); } catch (e) {
        console.error('Gagal menyiapkan adegan 3D:', e);
        if (window.NetUI && window.NetUI.setView) window.NetUI.setView('2d');
      }
    }).catch(function (e) {
      console.error('Gagal memuat Three.js:', e);
      var host = document.getElementById('scene3d');
      if (host) host.innerHTML = '<div style="position:absolute;inset:0;display:flex;align-items:center;justify-content:center;color:var(--text-muted);font-size:13px;padding:20px;text-align:center;">Modul 3D tidak dapat dimuat (butuh koneksi internet). Beralih ke Mode 2D otomatis.</div>';
      // Auto-fallback so the visualization area is never blank.
      if (window.NetUI && window.NetUI.setView) window.NetUI.setView('2d');
    });
  }

  if (document.readyState === 'complete' || document.readyState === 'interactive') setTimeout(boot, 300);
  else window.addEventListener('DOMContentLoaded', function () { setTimeout(boot, 300); });
})();
</script>
"##;
    Html(layout("Jaringan Komputer", "network", "/Learning/", body))
}