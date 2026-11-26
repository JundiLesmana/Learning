use axum::response::Html;
use crate::pages::layout::layout;

pub async fn handler() -> Html<String> {
    let body = r#"
<style>
  .tree-layout {
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
  .tree-content { flex: 1; min-width: 0; }
  .tree-header { margin-bottom: 28px; }
  .tree-header .label { margin-bottom: 14px; }
  .tree-header h1 { margin-bottom: 8px; }
  .tree-header p { font-size: 14px; line-height: 1.75; max-width: 620px; }

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
    width: 240px;
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
    z-index: 20;
  }
  .term-chip:hover .term-pop,
  .term-chip:focus-visible .term-pop { opacity: 1; pointer-events: auto; transform: translateY(0); }

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
  .module-tab.active {
    background: var(--accent);
    color: #fff;
    border-color: var(--accent);
  }

  .module-panel { display: none; }
  .module-panel.active { display: block; }

  /* Simulation card */
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
  .input-group { display: flex; flex-direction: column; gap: 5px; flex: 1; min-width: 160px; }
  .input-group.small { flex: 0 0 120px; min-width: 120px; }
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
  textarea.input-field { resize: vertical; min-height: 90px; line-height: 1.6; }
  .input-field:focus { border-color: var(--accent-border); background: var(--glass-hover); }

  .btn-row { display: flex; gap: 8px; flex-wrap: wrap; }
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

  /* Graph canvas */
  .canvas-wrap {
    position: relative;
    border: 1px solid var(--glass-border);
    border-radius: var(--radius-sm);
    background: var(--bg-base);
    margin-top: 8px;
    overflow: hidden;
  }
  svg.graph {
    width: 100%;
    height: auto;
    aspect-ratio: 100 / 70;
    display: block;
    touch-action: none;
    cursor: crosshair;
  }
  svg.graph .edge-line { stroke: var(--text-muted); stroke-width: 0.6; }
  svg.graph .edge-hit { stroke: transparent; stroke-width: 3; cursor: pointer; }
  svg.graph .edge-label {
    font-size: 3.2px;
    fill: var(--text-secondary);
    font-family: 'Inter', monospace;
    pointer-events: none;
  }
  svg.graph .node { cursor: grab; }
  svg.graph .node:active { cursor: grabbing; }
  svg.graph .node circle {
    fill: var(--bg-elevated);
    stroke: var(--accent);
    stroke-width: 0.7;
    transition: fill var(--transition);
  }
  svg.graph .node text {
    font-size: 3.4px;
    font-weight: 700;
    fill: var(--text-primary);
    text-anchor: middle;
    dominant-baseline: central;
    pointer-events: none;
    font-family: 'Inter', sans-serif;
  }
  svg.graph .node.root circle { fill: var(--accent); }
  svg.graph .node.root text { fill: #fff; }
  svg.graph .node.leaf circle { stroke: var(--warning); }

  /* edge states */
  svg.graph .edge-line.cycle { stroke: #ef4444; stroke-width: 1.1; }
  svg.graph .edge-line.bridge { stroke: var(--success); stroke-width: 1.1; }
  svg.graph .edge-line.chosen { stroke: var(--accent); stroke-width: 1.4; }
  svg.graph .edge-line.candidate { stroke: var(--warning); stroke-width: 1.4; stroke-dasharray: 1.5 1; }
  svg.graph .edge-line.rejected { stroke: #ef4444; stroke-width: 1.1; stroke-dasharray: 1.2 1.2; }
  svg.graph .edge-line.tree-dir { stroke: var(--accent); stroke-width: 1.1; marker-end: url(#arrowhead); }
  svg.graph .edge-line.dim { opacity: 0.35; }

  .legend {
    display: flex;
    gap: 16px;
    flex-wrap: wrap;
    margin-top: 12px;
    font-size: 11.5px;
    color: var(--text-secondary);
  }
  .legend span { display: inline-flex; align-items: center; gap: 6px; }
  .legend i { width: 16px; height: 0; border-top-width: 3px; border-top-style: solid; display: inline-block; }
  .legend i.node-dot { width: 10px; height: 10px; border-radius: 50%; border: none; background: var(--accent); }

  /* Verdict */
  .verdict {
    margin-top: 20px;
    padding: 20px;
    border-radius: var(--radius-lg);
    border: 1px solid var(--glass-border);
    background: var(--bg-layer);
    display: none;
  }
  .verdict.show { display: block; }
  .verdict-badge {
    display: inline-block;
    padding: 6px 16px;
    border-radius: 100px;
    font-size: 13px;
    font-weight: 700;
    letter-spacing: 0.04em;
    text-transform: uppercase;
    margin-bottom: 10px;
  }
  .verdict-badge.tree { background: rgba(74,222,128,0.15); color: var(--success); border: 1px solid rgba(74,222,128,0.4); }
  .verdict-badge.forest { background: rgba(251,191,36,0.15); color: var(--warning); border: 1px solid rgba(251,191,36,0.4); }
  .verdict-badge.no { background: rgba(239,68,68,0.15); color: #ef4444; border: 1px solid rgba(239,68,68,0.4); }
  .verdict-reason { font-size: 13.5px; line-height: 1.7; }

  /* Theorem checklist */
  .theorem-table {
    width: 100%;
    border-collapse: collapse;
    margin-top: 18px;
    font-size: 12.5px;
  }
  .theorem-table th, .theorem-table td {
    text-align: left;
    padding: 9px 10px;
    border-bottom: 1px solid var(--glass-border);
    vertical-align: top;
  }
  .theorem-table th {
    font-size: 10.5px;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-muted);
  }
  .theorem-table td.num { width: 28px; font-weight: 700; color: var(--accent); }
  .theorem-table .pass { color: var(--success); font-weight: 700; white-space: nowrap; }
  .theorem-table .fail { color: #ef4444; font-weight: 700; white-space: nowrap; }
  .theorem-table .real { color: var(--text-secondary); font-family: 'Inter', monospace; font-size: 12px; }

  .sub-list {
    margin-top: 16px;
    font-size: 12.5px;
    line-height: 1.7;
  }
  .sub-list h4 {
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-muted);
    margin-bottom: 6px;
  }
  .pill {
    display: inline-block;
    padding: 2px 10px;
    margin: 2px 4px 2px 0;
    border-radius: 100px;
    background: var(--glass-hover);
    border: 1px solid var(--glass-border);
    font-size: 11.5px;
    font-family: 'Inter', monospace;
  }

  /* Animation controls */
  .anim-controls {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
    align-items: center;
    margin-top: 16px;
  }
  .speed-slider { display: flex; align-items: center; gap: 8px; font-size: 12px; color: var(--text-muted); }
  .speed-slider input { accent-color: var(--accent); }

  .log-panel {
    margin-top: 18px;
    padding: 16px;
    border-radius: var(--radius-sm);
    background: var(--bg-base);
    border: 1px solid var(--glass-border);
    font-size: 12.5px;
    line-height: 1.75;
    max-height: 260px;
    overflow-y: auto;
  }
  .log-panel .log-line { padding: 3px 0; border-bottom: 1px dashed var(--glass-border); }
  .log-panel .log-line:last-child { border-bottom: none; }
  .log-panel .log-step { color: var(--accent); font-weight: 700; }
  .log-panel .log-ok { color: var(--success); }
  .log-panel .log-no { color: #ef4444; }

  .side-panel {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
    margin-top: 18px;
  }
  .side-box {
    padding: 16px;
    border-radius: var(--radius-sm);
    background: var(--bg-base);
    border: 1px solid var(--glass-border);
  }
  .side-box h4 {
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-muted);
    margin-bottom: 10px;
  }
  .counter-line { font-size: 13px; margin-top: 6px; font-variant-numeric: tabular-nums; }
  .counter-line b { color: var(--accent); font-size: 15px; }
  .compare-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
    margin-top: 8px;
  }

  .tree-table {
    width: 100%;
    border-collapse: collapse;
    margin-top: 16px;
    font-size: 12px;
  }
  .tree-table th, .tree-table td {
    text-align: left;
    padding: 7px 8px;
    border-bottom: 1px solid var(--glass-border);
  }
  .tree-table th {
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    color: var(--text-muted);
  }
  .tree-table .role-root { color: var(--accent); font-weight: 700; }
  .tree-table .role-leaf { color: var(--warning); font-weight: 700; }

  .spanning-list { display: flex; flex-wrap: wrap; gap: 10px; margin-top: 12px; }
  .spanning-item {
    padding: 10px 12px;
    border-radius: var(--radius-sm);
    background: var(--bg-base);
    border: 1px solid var(--glass-border);
    font-size: 11.5px;
    font-family: 'Inter', monospace;
    min-width: 120px;
  }
  .spanning-item b { color: var(--accent); }

  .hint { font-size: 11.5px; color: var(--text-muted); margin-top: 8px; line-height: 1.6; }

  @media (max-width: 760px) {
    .side-panel, .compare-grid { grid-template-columns: 1fr; }
  }
  @media (max-width: 680px) {
    .tree-layout { flex-direction: column; }
    .sidebar { width: 100%; }
    .sidebar-inner { position: static; }
    .term-chip .term-pop { width: 200px; }
  }
</style>

<div class="tree-layout">
  <!-- Sidebar -->
  <aside class="sidebar">
    <div class="sidebar-inner glass">
      <div class="sidebar-title">Pembelajaran</div>
      <ul class="sidebar-nav">
        <span class="sidebar-section-label">Algoritma Sorting</span>
        <li>
          <a href="/Learning/Insert-Sorting/">
            <span class="dot"></span>
            Insertion Sort
          </a>
        </li>
        <li>
          <a href="/Learning/Bubble-Sorting/">
            <span class="dot"></span>
            Bubble Sort
          </a>
        </li>
        <span class="sidebar-section-label">Pointer &amp; Memori</span>
        <li>
          <a href="/Learning/Pointer/">
            <span class="dot"></span>
            Pointer Simulation
          </a>
        </li>
        <span class="sidebar-section-label">Statistika &amp; Probabilitas</span>
        <li>
          <a href="/Learning/Distribusi-Normal/">
            <span class="dot"></span>
            Distribusi Normal
          </a>
        </li>
        <span class="sidebar-section-label">Graf Terapan</span>
        <li>
          <a href="/Learning/Pohon/" class="active">
            <span class="dot"></span>
            Pohon (Tree)
          </a>
        </li>
        <span class="sidebar-section-label">Jaringan Komputer</span>
        <li>
          <a href="/Learning/Jaringan-Komputer/" >
            <span class="dot"></span>
            Jaringan Komputer
          </a>
        </li>
        <span class="sidebar-section-label">Aljabar Linear</span>
        <li>
          <a href="/Learning/Nilai-Eigen/">
            <span class="dot"></span>
            Nilai &amp; Vektor Eigen
          </a>
        </li>
      </ul>
    </div>
  </aside>

  <!-- Main -->
  <main class="tree-content">
    <div class="tree-header">
      <div class="label">Graf Terapan &middot; Bab 3</div>
      <h1 style="margin-top:16px;">Pohon (Tree)</h1>
      <p style="margin-top:10px;">
        Pohon adalah graf tak-berarah yang terhubung dan tidak memiliki sirkuit.
        Pelajari konsep pohon, hutan, spanning tree, algoritma Prim &amp; Kruskal,
        serta pohon berakar melalui visualisasi interaktif di bawah.
      </p>
    </div>

    <!-- Concept -->
    <div class="concept-card glass">
      <h3>Konsep Dasar Pohon</h3>
      <p>
        Pohon adalah graf tak-berarah yang <strong style="color:var(--text-primary)">terhubung</strong> dan
        <strong style="color:var(--text-primary)">tidak memiliki sirkuit</strong>, sehingga setiap pasang
        simpul hanya dihubungkan oleh satu lintasan tunggal. Graf dengan n simpul adalah pohon jika dan
        hanya jika memiliki tepat n&minus;1 sisi dan tidak mengandung sirkuit.
      </p>
      <div class="term-row">
        <span class="term-chip" tabindex="0" role="button" aria-label="Definisi Pohon">
          POHON
          <span class="term-pop">Graf tak-berarah yang terhubung dan tidak memiliki sirkuit; setiap dua simpul dihubungkan oleh tepat satu lintasan.</span>
        </span>
        <span class="term-chip" tabindex="0" role="button" aria-label="Definisi Hutan">
          HUTAN
          <span class="term-pop">Kumpulan pohon yang saling lepas (graf tidak terhubung tanpa sirkuit). Setiap komponen terhubungnya adalah sebuah pohon.</span>
        </span>
        <span class="term-chip" tabindex="0" role="button" aria-label="Definisi Spanning Tree">
          SPANNING TREE
          <span class="term-pop">Subgraf merentang yang berupa pohon, diperoleh dengan menghilangkan sirkuit. Setiap graf terhubung punya minimal satu spanning tree.</span>
        </span>
        <span class="term-chip" tabindex="0" role="button" aria-label="Definisi Akar dan Daun">
          AKAR / DAUN
          <span class="term-pop">Pohon berakar: satu simpul dijadikan akar (derajat-masuk 0), sisi mengarah menjauhi akar. Simpul berderajat-keluar 0 disebut daun.</span>
        </span>
      </div>
    </div>

    <!-- Module tabs -->
    <div class="module-tabs" role="tablist" aria-label="Modul visualisasi">
      <button class="module-tab active" data-target="modA" role="tab" aria-selected="true">A &middot; Pengecek Pohon / Hutan</button>
      <button class="module-tab" data-target="modB" role="tab" aria-selected="false">B &middot; Spanning Tree &amp; MST</button>
      <button class="module-tab" data-target="modC" role="tab" aria-selected="false">C &middot; Pohon Berakar</button>
    </div>

    <!-- ============ MODULE A ============ -->
    <div class="module-panel active" id="modA" role="tabpanel">
      <div class="sim-card glass">
        <div class="sim-title">Modul A &mdash; Pengecek Pohon / Hutan</div>
        <p class="sim-desc">
          Masukkan jumlah simpul dan daftar sisi (format <code>a-b</code> atau <code>a-b:4</code> untuk berbobot).
          Anda juga dapat mengedit graf langsung di kanvas: klik area kosong untuk menambah simpul,
          tarik dari simpul ke simpul untuk menambah sisi, klik sisi untuk mengubah bobot / menghapus.
        </p>

        <div class="error-box" id="errA" role="alert"></div>

        <div class="sim-controls">
          <div class="input-group small">
            <label for="nA">Jumlah Simpul (n)</label>
            <input class="input-field" id="nA" type="number" min="1" max="15" value="6" />
          </div>
          <div class="input-group">
            <label for="edgesA">Daftar Sisi (satu per baris)</label>
            <textarea class="input-field" id="edgesA" spellcheck="false">a-b
b-c
c-d
d-e
e-f</textarea>
          </div>
        </div>

        <div class="btn-row" style="margin-bottom:8px;">
          <button class="btn-run" id="btnCheckA">Periksa Graf</button>
          <button class="btn-ghost" id="presetG1">Contoh G1 (Pohon)</button>
          <button class="btn-ghost" id="presetG3">Contoh G3 (Sirkuit)</button>
          <button class="btn-ghost" id="presetG4">Contoh G4 (Hutan)</button>
          <button class="btn-reset" id="resetA">Reset</button>
        </div>
        <p class="hint">Klik kanvas untuk menambah simpul &middot; tarik antar simpul untuk menambah sisi &middot; klik sisi untuk ubah bobot (kosong = hapus).</p>

        <div class="canvas-wrap">
          <svg class="graph" id="svgA" viewBox="0 0 100 70" preserveAspectRatio="xMidYMid meet" role="img" aria-label="Graf untuk pengecek pohon"></svg>
        </div>
        <div class="legend" aria-hidden="true">
          <span><i class="node-dot"></i> Simpul</span>
          <span><i style="border-top-color:var(--success)"></i> Jembatan (bridge)</span>
          <span><i style="border-top-color:#ef4444"></i> Bagian sirkuit</span>
          <span><i style="border-top-color:var(--text-muted)"></i> Sisi biasa</span>
        </div>

        <div class="verdict" id="verdictA">
          <span class="verdict-badge" id="verdictBadgeA"></span>
          <div class="verdict-reason" id="verdictReasonA"></div>
          <table class="theorem-table" id="theoremA">
            <thead>
              <tr><th>#</th><th>Sifat (Teorema 3.1)</th><th>Nilai Nyata</th><th>Status</th></tr>
            </thead>
            <tbody></tbody>
          </table>
          <div class="sub-list" id="subListA"></div>
        </div>
      </div>
    </div>

    <!-- ============ MODULE B ============ -->
    <div class="module-panel" id="modB" role="tabpanel">
      <div class="sim-card glass">
        <div class="sim-title">Modul B &mdash; Spanning Tree &amp; MST (Prim vs Kruskal)</div>
        <p class="sim-desc">
          Masukkan sisi berbobot atau acak graf. Pilih algoritma, lalu jalankan animasi langkah demi langkah.
          Gunakan mode bandingkan untuk melihat Prim dan Kruskal berdampingan pada graf yang sama.
        </p>

        <div class="error-box" id="errB" role="alert"></div>

        <div class="sim-controls">
          <div class="input-group small">
            <label for="nB">Simpul (n)</label>
            <input class="input-field" id="nB" type="number" min="2" max="15" value="8" />
          </div>
          <div class="input-group">
            <label for="edgesB">Daftar Sisi Berbobot</label>
            <textarea class="input-field" id="edgesB" spellcheck="false">a-c:4
a-b:5
c-b:3
a-d:4
a-e:4
b-e:5
d-e:5
d-h:5
e-g:4
h-g:4
e-f:3
f-g:2</textarea>
          </div>
        </div>

        <div class="sim-controls">
          <div class="input-group small">
            <label for="algoB">Algoritma</label>
            <select class="input-field" id="algoB">
              <option value="prim">Prim</option>
              <option value="kruskal">Kruskal</option>
            </select>
          </div>
          <div class="input-group small">
            <label for="startB">Simpul Awal (Prim)</label>
            <select class="input-field" id="startB"></select>
          </div>
          <div class="input-group small">
            <label for="densityB">Kepadatan Acak</label>
            <input class="input-field" id="densityB" type="range" min="20" max="90" value="50" />
          </div>
        </div>

        <div class="btn-row" style="margin-bottom:8px;">
          <button class="btn-run" id="btnBuildB">Bangun Graf</button>
          <button class="btn-ghost" id="presetB">Contoh 3.1</button>
          <button class="btn-ghost" id="randomB">Acak Graf</button>
          <button class="btn-ghost" id="compareB">Bandingkan Prim vs Kruskal</button>
          <button class="btn-ghost" id="allTreesB">Semua Spanning Tree (n&le;5)</button>
          <button class="btn-reset" id="resetB">Reset</button>
        </div>

        <div class="canvas-wrap">
          <svg class="graph" id="svgB" viewBox="0 0 100 70" preserveAspectRatio="xMidYMid meet" role="img" aria-label="Graf untuk spanning tree"></svg>
        </div>
        <div class="legend" aria-hidden="true">
          <span><i class="node-dot"></i> Simpul</span>
          <span><i style="border-top-color:var(--accent)"></i> Sisi terpilih</span>
          <span><i style="border-top-color:var(--warning)"></i> Kandidat</span>
          <span><i style="border-top-color:#ef4444"></i> Ditolak (sirkuit)</span>
          <span><i style="border-top-color:var(--text-muted)"></i> Belum diproses</span>
        </div>

        <div class="anim-controls">
          <button class="btn-run" id="playB">Play</button>
          <button class="btn-ghost" id="stepFwdB">Step Maju &rarr;</button>
          <button class="btn-ghost" id="stepBackB">&larr; Step Mundur</button>
          <button class="btn-ghost" id="animResetB">Reset Animasi</button>
          <div class="speed-slider">
            <span>Kecepatan</span>
            <input type="range" id="speedB" min="1" max="10" value="4" />
          </div>
        </div>

        <div class="side-panel">
          <div class="side-box">
            <h4 id="sideTitleB">Panel Sisi</h4>
            <div id="sideContentB"></div>
          </div>
          <div class="side-box">
            <h4>Progres</h4>
            <div class="counter-line">Sisi terpilih: <b id="chosenCountB">0</b> / <span id="needCountB">0</span></div>
            <div class="counter-line">Total bobot: <b id="totalWeightB">0</b></div>
            <div class="counter-line" id="mstNoteB" style="font-size:12px;color:var(--text-muted);"></div>
          </div>
        </div>

        <div class="log-panel" id="logB" aria-live="polite"></div>

        <div id="compareWrapB" style="display:none;">
          <div class="compare-grid">
            <div>
              <h4 style="margin:12px 0 6px;font-size:12px;color:var(--text-muted);">PRIM</h4>
              <div class="canvas-wrap"><svg class="graph" id="svgBPrim" viewBox="0 0 100 70" preserveAspectRatio="xMidYMid meet" role="img" aria-label="Graf Prim"></svg></div>
              <div class="counter-line">Total bobot Prim: <b id="primTotalB">-</b></div>
            </div>
            <div>
              <h4 style="margin:12px 0 6px;font-size:12px;color:var(--text-muted);">KRUSKAL</h4>
              <div class="canvas-wrap"><svg class="graph" id="svgBKruskal" viewBox="0 0 100 70" preserveAspectRatio="xMidYMid meet" role="img" aria-label="Graf Kruskal"></svg></div>
              <div class="counter-line">Total bobot Kruskal: <b id="kruskalTotalB">-</b></div>
            </div>
          </div>
          <div class="hint" id="compareNoteB"></div>
        </div>

        <div id="allTreesWrapB" style="display:none;">
          <div class="spanning-list" id="allTreesListB"></div>
          <div class="hint" id="allTreesNoteB"></div>
        </div>
      </div>
    </div>

    <!-- ============ MODULE C ============ -->
    <div class="module-panel" id="modC" role="tabpanel">
      <div class="sim-card glass">
        <div class="sim-title">Modul C &mdash; Pohon Berakar (Rooted Tree)</div>
        <p class="sim-desc">
          Masukkan daftar sisi pohon dan pilih simpul akar. Sisi akan diarahkan menjauhi akar dan
          ditata hierarkis. Mengganti akar pada pohon yang sama menghasilkan pohon berakar yang berbeda.
        </p>

        <div class="error-box" id="errC" role="alert"></div>

        <div class="sim-controls">
          <div class="input-group small">
            <label for="nC">Simpul (n)</label>
            <input class="input-field" id="nC" type="number" min="1" max="15" value="7" />
          </div>
          <div class="input-group">
            <label for="edgesC">Daftar Sisi Pohon</label>
            <textarea class="input-field" id="edgesC" spellcheck="false">a-b
a-c
b-d
b-e
c-f
c-g</textarea>
          </div>
          <div class="input-group small">
            <label for="rootC">Simpul Akar</label>
            <select class="input-field" id="rootC"></select>
          </div>
        </div>

        <div class="btn-row" style="margin-bottom:8px;">
          <button class="btn-run" id="btnRootC">Bangun Pohon Berakar</button>
          <button class="btn-ghost" id="useTreeAC">Pakai Graf dari Modul A</button>
          <button class="btn-ghost" id="useTreeBC">Pakai MST dari Modul B</button>
          <button class="btn-reset" id="resetC">Reset</button>
        </div>

        <div class="canvas-wrap">
          <svg class="graph" id="svgC" viewBox="0 0 100 70" preserveAspectRatio="xMidYMid meet" role="img" aria-label="Pohon berakar hierarkis"></svg>
        </div>
        <div class="legend" aria-hidden="true">
          <span><i class="node-dot"></i> Akar</span>
          <span><i class="node-dot" style="background:var(--warning)"></i> Daun</span>
          <span><i style="border-top-color:var(--accent)"></i> Sisi (menjauhi akar)</span>
        </div>

        <div class="counter-line" style="margin-top:16px;">Tinggi pohon: <b id="treeHeightC">-</b></div>

        <table class="tree-table" id="treeTableC">
          <thead>
            <tr>
              <th>Simpul</th><th>Der.Masuk</th><th>Der.Keluar</th><th>Peran</th><th>Induk</th><th>Anak</th><th>Level</th>
            </tr>
          </thead>
          <tbody></tbody>
        </table>
        <p class="hint" id="hintC">Pilih akar lewat dropdown atau klik simpul pada kanvas untuk melihat transisi re-root.</p>
      </div>
    </div>

  </main>
</div>

<script>
/* =====================================================================
   Shared helpers & pure algorithms (mirror of src/tree_algo.rs)
   ===================================================================== */
var TreeApp = (function () {
  'use strict';

  function label(i) {
    if (i < 26) return String.fromCharCode(97 + i);
    return 'v' + (i - 25);
  }

  function labelIndex(tok) {
    tok = String(tok).trim().toLowerCase();
    if (/^[a-z]$/.test(tok)) return tok.charCodeAt(0) - 97;
    var m = /^v(\d+)$/.exec(tok);
    if (m) return 25 + parseInt(m[1], 10);
    return -1;
  }

  // Parse edge list text -> { edges:[{u,v,w}], errors:[], maxUsed }
  function parseEdges(text, n) {
    var errors = [];
    var edges = [];
    var seen = {};
    var maxUsed = -1;
    var lines = String(text).split(/\n/);
    for (var li = 0; li < lines.length; li++) {
      var raw = lines[li].trim();
      if (raw === '') continue;
      var w = 1;
      var body = raw;
      var colon = raw.lastIndexOf(':');
      if (colon >= 0) {
        body = raw.slice(0, colon).trim();
        var wStr = raw.slice(colon + 1).trim();
        if (wStr === '' || isNaN(Number(wStr))) {
          errors.push('Baris ' + (li + 1) + ': bobot \"' + wStr + '\" bukan angka.');
          continue;
        }
        w = Number(wStr);
        if (!isFinite(w)) { errors.push('Baris ' + (li + 1) + ': bobot tidak valid.'); continue; }
      }
      var parts = body.split('-');
      if (parts.length !== 2) {
        errors.push('Baris ' + (li + 1) + ': format \"' + raw + '\" tidak dikenal (harus a-b atau a-b:4).');
        continue;
      }
      var u = labelIndex(parts[0]);
      var v = labelIndex(parts[1]);
      if (u < 0 || v < 0) {
        errors.push('Baris ' + (li + 1) + ': simpul \"' + parts[0] + '\" atau \"' + parts[1] + '\" tidak dikenal.');
        continue;
      }
      if (u === v) { errors.push('Baris ' + (li + 1) + ': self-loop pada simpul ' + label(u) + ' tidak diizinkan.'); continue; }
      var key = Math.min(u, v) + '-' + Math.max(u, v);
      if (seen[key]) { errors.push('Baris ' + (li + 1) + ': sisi ' + label(u) + '-' + label(v) + ' ganda.'); continue; }
      seen[key] = true;
      edges.push({ u: u, v: v, w: w });
      if (u > maxUsed) maxUsed = u;
      if (v > maxUsed) maxUsed = v;
    }
    if (n != null) {
      for (var i = 0; i < edges.length; i++) {
        if (edges[i].u >= n || edges[i].v >= n) {
          errors.push('Sisi ' + label(edges[i].u) + '-' + label(edges[i].v) + ' memakai simpul di luar n = ' + n + '.');
        }
      }
    }
    return { edges: edges, errors: errors, maxUsed: maxUsed };
  }

  function adjacency(n, edges) {
    var adj = [];
    for (var i = 0; i < n; i++) adj.push([]);
    for (var e = 0; e < edges.length; e++) {
      adj[edges[e].u].push({ to: edges[e].v, id: e });
      adj[edges[e].v].push({ to: edges[e].u, id: e });
    }
    return adj;
  }

  function components(n, edges) {
    var adj = adjacency(n, edges);
    var seen = [];
    for (var i = 0; i < n; i++) seen.push(false);
    var comps = [];
    for (var s = 0; s < n; s++) {
      if (seen[s]) continue;
      var stack = [s];
      seen[s] = true;
      var comp = [];
      while (stack.length) {
        var x = stack.pop();
        comp.push(x);
        for (var k = 0; k < adj[x].length; k++) {
          var y = adj[x][k].to;
          if (!seen[y]) { seen[y] = true; stack.push(y); }
        }
      }
      comp.sort(function (a, b) { return a - b; });
      comps.push(comp);
    }
    comps.sort(function (a, b) {
      if (a.length !== b.length) return a.length - b.length;
      return a[0] - b[0];
    });
    return comps;
  }

  function hasCycle(n, edges) {
    var parent = [];
    for (var i = 0; i < n; i++) parent.push(i);
    function find(x) { while (parent[x] !== x) { parent[x] = parent[parent[x]]; x = parent[x]; } return x; }
    for (var e = 0; e < edges.length; e++) {
      var a = find(edges[e].u), b = find(edges[e].v);
      if (a === b) return true;
      parent[a] = b;
    }
    return false;
  }

  // Tarjan low-link, iterative. Returns array of bridge edge-ids.
  function findBridges(n, edges) {
    var adj = adjacency(n, edges);
    var disc = [], low = [], visited = [];
    for (var i = 0; i < n; i++) { disc.push(0); low.push(0); visited.push(false); }
    var timer = 0;
    var isBridge = [];
    for (var e = 0; e < edges.length; e++) isBridge.push(false);
    for (var start = 0; start < n; start++) {
      if (visited[start]) continue;
      visited[start] = true;
      timer++; disc[start] = timer; low[start] = timer;
      var stack = [{ v: start, pe: -1, idx: 0 }];
      while (stack.length) {
        var top = stack[stack.length - 1];
        if (top.idx < adj[top.v].length) {
          var item = adj[top.v][top.idx];
          top.idx++;
          if (item.id === top.pe) continue;
          if (!visited[item.to]) {
            visited[item.to] = true;
            timer++; disc[item.to] = timer; low[item.to] = timer;
            stack.push({ v: item.to, pe: item.id, idx: 0 });
          } else {
            if (disc[item.to] < low[top.v]) low[top.v] = disc[item.to];
          }
        } else {
          var done = stack.pop();
          if (stack.length) {
            var parentV = stack[stack.length - 1].v;
            if (low[done.v] < low[parentV]) low[parentV] = low[done.v];
            if (low[done.v] > disc[parentV]) isBridge[done.pe] = true;
          }
        }
      }
    }
    var res = [];
    for (var b = 0; b < isBridge.length; b++) if (isBridge[b]) res.push(b);
    return res;
  }

  function isTree(n, edges) {
    if (n === 0) return false;
    return components(n, edges).length === 1 && edges.length + 1 === n && !hasCycle(n, edges);
  }

  // Prim: returns { chosen:[edgeIds], total, connected, steps:[...] }
  // start < 0 => material default: seed T with the globally minimum-weight edge.
  // start >= 0 => standard Prim growing from that vertex.
  function prim(n, edges, start) {
    var adj = adjacency(n, edges);
    var inTree = [];
    for (var i = 0; i < n; i++) inTree.push(false);
    var usedEdge = [];
    for (var e = 0; e < edges.length; e++) usedEdge.push(false);
    var chosen = [];
    var total = 0;
    var steps = [];
    if (n === 0) return { chosen: chosen, total: 0, connected: true, steps: steps };

    if (start < 0) {
      // Material step 1: pick the global minimum-weight edge into T.
      var first = -1;
      for (var e = 0; e < edges.length; e++) {
        if (first < 0 || edges[e].w < edges[first].w) first = e;
      }
      if (first >= 0) {
        var fe = edges[first];
        usedEdge[first] = true;
        inTree[fe.u] = true;
        inTree[fe.v] = true;
        chosen.push(first);
        total += fe.w;
        steps.push({
          candidate: first, accepted: true, chosen: chosen.slice(),
          total: total,
          log: 'Langkah 1: pilih sisi ' + label(fe.u) + '-' + label(fe.v) + ' (bobot ' + fe.w + '), sisi berbobot minimum pada G, masukkan ke T.'
        });
      }
    } else {
      if (start >= n) start = 0;
      inTree[start] = true;
    }

    var visitedCount = 0;
    for (var vi = 0; vi < n; vi++) if (inTree[vi]) visitedCount++;

    while (visitedCount < n) {
      var best = null;
      for (var u = 0; u < n; u++) {
        if (!inTree[u]) continue;
        for (var k = 0; k < adj[u].length; k++) {
          var it = adj[u][k];
          if (!inTree[it.to] && !usedEdge[it.id]) {
            if (!best || edges[it.id].w < best.w || (edges[it.id].w === best.w && it.id < best.id)) {
              best = { w: edges[it.id].w, id: it.id, to: it.to };
            }
          }
        }
      }
      if (!best) break;
      usedEdge[best.id] = true;
      inTree[best.to] = true;
      visitedCount++;
      chosen.push(best.id);
      total += best.w;
      var be = edges[best.id];
      steps.push({
        candidate: best.id, accepted: true, chosen: chosen.slice(),
        total: total,
        log: 'Langkah ' + (steps.length + 1) + ': pilih sisi ' + label(be.u) + '-' + label(be.v) + ' (bobot ' + be.w + '), berbobot minimum yang bersisian dengan simpul di T dan tidak membentuk sirkuit.'
      });
    }
    return { chosen: chosen, total: total, connected: visitedCount === n, steps: steps };
  }

  // Kruskal: returns { chosen, total, connected, steps }
  function kruskal(n, edges) {
    var order = [];
    for (var i = 0; i < edges.length; i++) order.push(i);
    order.sort(function (a, b) { return edges[a].w - edges[b].w || a - b; });
    var parent = [];
    for (var j = 0; j < n; j++) parent.push(j);
    function find(x) { while (parent[x] !== x) { parent[x] = parent[parent[x]]; x = parent[x]; } return x; }
    var chosen = [];
    var total = 0;
    var steps = [];
    for (var oi = 0; oi < order.length; oi++) {
      var id = order[oi];
      var e = edges[id];
      var ra = find(e.u), rb = find(e.v);
      var accept = ra !== rb;
      var log;
      if (accept) {
        parent[ra] = rb;
        chosen.push(id);
        total += e.w;
        log = 'pilih sisi ' + label(e.u) + '-' + label(e.v) + ' (bobot ' + e.w + '), tidak membentuk sirkuit.';
      } else {
        log = 'tolak sisi ' + label(e.u) + '-' + label(e.v) + ' (bobot ' + e.w + '), karena membentuk sirkuit.';
      }
      steps.push({
        candidate: id, accepted: accept, chosen: chosen.slice(),
        total: total,
        log: 'Langkah ' + (oi + 1) + ': ' + log + ' (sisi terpilih ' + chosen.length + ' dari ' + (n - 1) + ')'
      });
      if (chosen.length + 1 === n) break;
    }
    var connected = (chosen.length + 1 === n) || n === 1;
    return { chosen: chosen, total: total, connected: connected, steps: steps };
  }

  // Rooted tree from valid tree edges.
  function rooted(n, edges, root) {
    var adj = adjacency(n, edges);
    var parent = [], depth = [], children = [], order = [], seen = [];
    for (var i = 0; i < n; i++) { parent.push(null); depth.push(0); children.push([]); seen.push(false); }
    if (n > 0) {
      seen[root] = true;
      var queue = [root];
      var head = 0;
      while (head < queue.length) {
        var v = queue[head++];
        order.push(v);
        for (var k = 0; k < adj[v].length; k++) {
          var to = adj[v][k].to;
          if (!seen[to]) {
            seen[to] = true;
            parent[to] = v;
            depth[to] = depth[v] + 1;
            children[v].push(to);
            queue.push(to);
          }
        }
      }
    }
    var height = 0;
    for (var d = 0; d < n; d++) if (depth[d] > height) height = depth[d];
    var indeg = [], outdeg = [];
    for (var x = 0; x < n; x++) { indeg.push(parent[x] === null ? 0 : 1); outdeg.push(children[x].length); }
    return { root: root, parent: parent, children: children, depth: depth, indeg: indeg, outdeg: outdeg, height: height, order: order };
  }

  // Enumerate all spanning trees for small graphs (n <= 5).
  function allSpanningTrees(n, edges) {
    var results = [];
    if (n < 1) return results;
    var need = n - 1;
    if (edges.length < need) return results;
    var m = edges.length;
    var limit = Math.pow(2, m);
    for (var mask = 0; mask < limit; mask++) {
      var cnt = 0;
      for (var b = 0; b < m; b++) if (mask & (1 << b)) cnt++;
      if (cnt !== need) continue;
      var sub = [];
      for (var b2 = 0; b2 < m; b2++) if (mask & (1 << b2)) sub.push(edges[b2]);
      if (components(n, sub).length === 1 && !hasCycle(n, sub)) {
        var wsum = 0;
        for (var s = 0; s < sub.length; s++) wsum += sub[s].w;
        results.push({ edges: sub, total: wsum });
      }
    }
    results.sort(function (a, b) { return a.total - b.total; });
    return results;
  }

  return {
    label: label, parseEdges: parseEdges, components: components,
    hasCycle: hasCycle, findBridges: findBridges, isTree: isTree,
    prim: prim, kruskal: kruskal, rooted: rooted, allSpanningTrees: allSpanningTrees
  };
})();

/* =====================================================================
   Graph canvas: interactive SVG editor/renderer
   state = { n, edges:[{u,v,w}], pos:[[x,y],...] }  (viewBox 0..100 x 0..70)
   ===================================================================== */
function GraphCanvas(svg, opts) {
  opts = opts || {};
  var self = this;
  this.svg = svg;
  this.state = { n: 0, edges: [], pos: [] };
  this.onChange = opts.onChange || function () {};
  this.onEdgeClick = opts.onEdgeClick || null;
  this.onNodeClick = opts.onNodeClick || null;
  this.readOnly = !!opts.readOnly;
  this.edgeClasses = {}; // edgeId -> 'cycle'|'bridge'|'chosen'|...
  this.nodeClasses = {}; // vertex -> 'root'|'leaf'
  this.dirEdges = null; // if set, draw directed edges away from root
  this._drag = null;
  this._bind();
}

GraphCanvas.prototype._bind = function () {
  var self = this;
  var svg = this.svg;

  function toSvg(evt) {
    var r = svg.getBoundingClientRect();
    var vb = svg.viewBox.baseVal;
    var x = (evt.clientX - r.left) / r.width * vb.width + vb.x;
    var y = (evt.clientY - r.top) / r.height * vb.height + vb.y;
    return { x: x, y: y };
  }

  svg.addEventListener('pointerdown', function (evt) {
    if (self.readOnly) return;
    var target = evt.target;
    var nodeEl = target.closest ? target.closest('.node') : null;
    if (nodeEl) {
      var idx = parseInt(nodeEl.getAttribute('data-v'), 10);
      self._drag = { type: 'node', idx: idx, moved: false };
      svg.setPointerCapture(evt.pointerId);
      evt.preventDefault();
      return;
    }
    var edgeEl = target.closest ? target.closest('.edge-hit') : null;
    if (edgeEl && self.onEdgeClick) {
      var eid = parseInt(edgeEl.getAttribute('data-e'), 10);
      self.onEdgeClick(eid);
      evt.preventDefault();
      return;
    }
    // empty area -> add node
    if (target === svg || (target.tagName === 'svg')) {
      self._drag = { type: 'addnode', start: toSvg(evt), moved: false };
      svg.setPointerCapture(evt.pointerId);
      evt.preventDefault();
    }
  });

  svg.addEventListener('pointermove', function (evt) {
    if (!self._drag) return;
    if (self._drag.type === 'node') {
      var p = toSvg(evt);
      self.state.pos[self._drag.idx] = [p.x, p.y];
      self._drag.moved = true;
      self.render();
    }
  });

  svg.addEventListener('pointerup', function (evt) {
    if (!self._drag) return;
    var drag = self._drag;
    self._drag = null;
    if (drag.type === 'addnode') {
      var p = toSvg(evt);
      // ignore tiny drags that were meant as clicks on background
      var d = Math.sqrt(Math.pow(p.x - drag.start.x, 2) + Math.pow(p.y - drag.start.y, 2));
      if (d < 1.5) {
        self.addNodeAt(p.x, p.y);
      }
    }
  });

  // drag from node to node to add edge
  svg.addEventListener('pointerdown', function (evt) {
    if (self.readOnly) return;
    var nodeEl = evt.target.closest ? evt.target.closest('.node') : null;
    if (!nodeEl) return;
    var idx = parseInt(nodeEl.getAttribute('data-v'), 10);
    self._linkFrom = idx;
  });
  svg.addEventListener('pointerup', function (evt) {
    if (self.readOnly) return;
    if (self._linkFrom == null) return;
    var nodeEl = evt.target.closest ? evt.target.closest('.node') : null;
    var from = self._linkFrom;
    self._linkFrom = null;
    if (!nodeEl) return;
    var to = parseInt(nodeEl.getAttribute('data-v'), 10);
    if (from !== to) self.addEdge(from, to);
  });
};

GraphCanvas.prototype.addNodeAt = function (x, y) {
  if (this.state.n >= 15) { alert('Batas maksimum 15 simpul.'); return; }
  this.state.pos.push([x, y]);
  this.state.n++;
  this._emit();
};

GraphCanvas.prototype.addEdge = function (u, v) {
  var edges = this.state.edges;
  for (var i = 0; i < edges.length; i++) {
    if ((edges[i].u === u && edges[i].v === v) || (edges[i].u === v && edges[i].v === u)) {
      return; // already exists
    }
  }
  edges.push({ u: u, v: v, w: 1 });
  this._emit();
};

GraphCanvas.prototype.removeEdge = function (eid) {
  this.state.edges.splice(eid, 1);
  this._emit();
};

GraphCanvas.prototype.setWeight = function (eid, w) {
  if (this.state.edges[eid]) this.state.edges[eid].w = w;
  this._emit();
};

GraphCanvas.prototype._emit = function () {
  this.render();
  this.onChange(this.state);
};

GraphCanvas.prototype.setData = function (data) {
  this.state = {
    n: data.n,
    edges: data.edges.map(function (e) { return { u: e.u, v: e.v, w: e.w }; }),
    pos: data.pos ? data.pos.map(function (p) { return [p[0], p[1]]; }) : []
  };
  this.ensurePositions();
  this.render();
};

GraphCanvas.prototype.ensurePositions = function () {
  var n = this.state.n;
  while (this.state.pos.length < n) {
    // circle layout for new vertices
    var i = this.state.pos.length;
    var ang = (i / Math.max(n, 1)) * Math.PI * 2 - Math.PI / 2;
    this.state.pos.push([50 + 32 * Math.cos(ang), 35 + 24 * Math.sin(ang)]);
  }
  if (this.state.pos.length > n) this.state.pos.length = n;
};

GraphCanvas.prototype.toJSON = function () {
  return { n: this.state.n, edges: this.state.edges, pos: this.state.pos };
};

GraphCanvas.prototype.render = function () {
  var svg = this.svg;
  var st = this.state;
  this.ensurePositions();
  var parts = [];

  // arrow marker def
  parts.push('<defs><marker id="arrowhead" markerWidth="6" markerHeight="6" refX="5" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="var(--accent)"></path></marker></defs>');

  // edges
  for (var e = 0; e < st.edges.length; e++) {
    var ed = st.edges[e];
    var a = st.pos[ed.u], b = st.pos[ed.v];
    if (!a || !b) continue;
    var cls = 'edge-line';
    if (this.edgeClasses[e]) cls += ' ' + this.edgeClasses[e];
    if (this.dirEdges) cls += ' tree-dir';
    // shorten line for directed arrow
    var x1 = a[0], y1 = a[1], x2 = b[0], y2 = b[1];
    if (this.dirEdges) {
      var dx = x2 - x1, dy = y2 - y1;
      var len = Math.sqrt(dx * dx + dy * dy) || 1;
      var ux = dx / len, uy = dy / len;
      x1 += ux * 2.6; y1 += uy * 2.6;
      x2 -= ux * 3.2; y2 -= uy * 3.2;
    }
    parts.push('<line class="' + cls + '" x1="' + x1 + '" y1="' + y1 + '" x2="' + x2 + '" y2="' + y2 + '"></line>');
    if (!this.readOnly) {
      parts.push('<line class="edge-hit" data-e="' + e + '" x1="' + a[0] + '" y1="' + a[1] + '" x2="' + b[0] + '" y2="' + b[1] + '"></line>');
    }
    var mx = (a[0] + b[0]) / 2, my = (a[1] + b[1]) / 2 - 1.2;
    parts.push('<text class="edge-label" x="' + mx + '" y="' + my + '" text-anchor="middle">' + ed.w + '</text>');
  }

  // nodes
  for (var v = 0; v < st.n; v++) {
    var p = st.pos[v];
    if (!p) continue;
    var ncls = 'node';
    if (this.nodeClasses[v]) ncls += ' ' + this.nodeClasses[v];
    parts.push('<g class="' + ncls + '" data-v="' + v + '">');
    parts.push('<circle cx="' + p[0] + '" cy="' + p[1] + '" r="2.8"></circle>');
    parts.push('<text x="' + p[0] + '" y="' + p[1] + '">' + TreeApp.label(v) + '</text>');
    parts.push('</g>');
  }

  svg.innerHTML = parts.join('');
};

GraphCanvas.prototype.setEdgeClasses = function (map) { this.edgeClasses = map || {}; this.render(); };
GraphCanvas.prototype.setNodeClasses = function (map) { this.nodeClasses = map || {}; this.render(); };
GraphCanvas.prototype.setDirected = function (on) { this.dirEdges = on; this.render(); };

/* =====================================================================
   Module A: Pengecek Pohon / Hutan
   ===================================================================== */
var canvasA = new GraphCanvas(document.getElementById('svgA'), {
  onChange: function (st) {
    // sync back to textarea (positions preserved in canvas, edges -> text)
    var lines = [];
    for (var i = 0; i < st.edges.length; i++) {
      var e = st.edges[i];
      lines.push(TreeApp.label(e.u) + '-' + TreeApp.label(e.v) + (e.w !== 1 ? ':' + e.w : ''));
    }
    document.getElementById('edgesA').value = lines.join('\n');
    document.getElementById('nA').value = st.n;
  },
  onEdgeClick: function (eid) {
    var ed = canvasA.state.edges[eid];
    var cur = ed.w;
    var inp = prompt('Bobot sisi ' + TreeApp.label(ed.u) + '-' + TreeApp.label(ed.v) + ' (kosongkan untuk hapus):', cur);
    if (inp === null) return;
    if (inp.trim() === '') { canvasA.removeEdge(eid); return; }
    var w = Number(inp);
    if (isNaN(w)) { alert('Bobot harus angka.'); return; }
    canvasA.setWeight(eid, w);
  }
});

function showError(id, msgs) {
  var box = document.getElementById(id);
  if (!msgs || msgs.length === 0) { box.classList.remove('show'); box.innerHTML = ''; return; }
  var html = '<strong>Input tidak valid:</strong><ul>';
  for (var i = 0; i < msgs.length; i++) html += '<li>' + msgs[i] + '</li>';
  html += '</ul>';
  box.innerHTML = html;
  box.classList.add('show');
}

function buildFromInputs(nId, edgesId, errId, canvas) {
  var n = parseInt(document.getElementById(nId).value, 10);
  var text = document.getElementById(edgesId).value;
  var parsed = TreeApp.parseEdges(text, n);
  var errs = parsed.errors.slice();
  if (isNaN(n) || n < 1) errs.push('Jumlah simpul n harus angka >= 1.');
  if (n > 15) errs.push('Jumlah simpul terlalu besar (maksimum 15).');
  showError(errId, errs);
  if (errs.length) return null;
  var maxUsed = parsed.maxUsed;
  var effN = Math.max(n, maxUsed + 1);
  if (effN > 15) { showError(errId, ['Graf memakai lebih dari 15 simpul.']); return null; }
  // positions: keep existing if canvas already has them, else circle
  var pos = (canvas && canvas.state.n === effN) ? canvas.state.pos.slice() : null;
  var data = { n: effN, edges: parsed.edges, pos: pos };
  return data;
}

function layoutCircle(n) {
  var pos = [];
  for (var i = 0; i < n; i++) {
    var ang = (i / Math.max(n, 1)) * Math.PI * 2 - Math.PI / 2;
    pos.push([50 + 32 * Math.cos(ang), 35 + 24 * Math.sin(ang)]);
  }
  return pos;
}

function runCheckA() {
  var data = buildFromInputs('nA', 'edgesA', 'errA', canvasA);
  var verdict = document.getElementById('verdictA');
  if (!data) { verdict.classList.remove('show'); return; }
  if (canvasA.state.n !== data.n) canvasA.setData({ n: data.n, edges: data.edges, pos: layoutCircle(data.n) });
  else canvasA.setData(data);

  var n = data.n, edges = data.edges;
  var m = edges.length;
  var comps = TreeApp.components(n, edges);
  var cyc = TreeApp.hasCycle(n, edges);
  var bridges = TreeApp.findBridges(n, edges);
  var bridgeSet = {};
  for (var i = 0; i < bridges.length; i++) bridgeSet[bridges[i]] = true;
  var tree = TreeApp.isTree(n, edges);
  var connected = comps.length === 1;

  // edge classes
  var ecls = {};
  for (var e = 0; e < edges.length; e++) {
    if (bridgeSet[e]) ecls[e] = 'bridge';
    else if (cyc) ecls[e] = 'cycle'; // non-bridge edge lies on a cycle
  }
  canvasA.setEdgeClasses(ecls);
  canvasA.setNodeClasses({});

  // verdict
  var badge = document.getElementById('verdictBadgeA');
  var reason = document.getElementById('verdictReasonA');
  badge.className = 'verdict-badge';
  if (tree) {
    badge.classList.add('tree');
    badge.textContent = 'Pohon';
    reason.innerHTML = 'Graf ini terhubung, tidak memiliki sirkuit, dan memiliki tepat <strong>n&minus;1 = ' + (n - 1) + '</strong> sisi. Jadi G adalah sebuah pohon.';
  } else if (!connected && !cyc) {
    badge.classList.add('forest');
    badge.textContent = 'Hutan (' + comps.length + ' komponen)';
    reason.innerHTML = 'Graf tidak terhubung (' + comps.length + ' komponen) dan tidak memiliki sirkuit. Setiap komponennya adalah pohon, sehingga G adalah sebuah <strong>hutan</strong>.';
  } else {
    badge.classList.add('no');
    badge.textContent = 'Bukan pohon';
    var why = [];
    if (!connected) why.push('graf tidak terhubung (' + comps.length + ' komponen)');
    if (cyc) why.push('graf mengandung sirkuit');
    if (m + 1 !== n) why.push('m = ' + m + ' &ne; n&minus;1 = ' + (n - 1));
    reason.innerHTML = 'Graf ini <strong>bukan pohon</strong> karena ' + why.join(', ') + '.';
  }
  verdict.classList.add('show');

  // theorem 3.1 checklist
  var tbody = document.querySelector('#theoremA tbody');
  var rows = [];
  function row(num, text, real, pass) {
    rows.push('<tr><td class="num">' + num + '</td><td>' + text + '</td><td class="real">' + real + '</td><td><span class="' + (pass ? 'pass' : 'fail') + '">' + (pass ? 'Lulus ✓' : 'Gagal ✗') + '</span></td></tr>');
  }
  // 1 pohon
  row('1', 'G adalah pohon', 'n=' + n + ', m=' + m, tree);
  // 2 every pair single path == tree
  row('2', 'Setiap pasang simpul terhubung lintasan tunggal', 'terhubung=' + connected + ', sirkuit=' + (cyc ? 'ada' : 'tidak'), connected && !cyc);
  // 3 connected & m=n-1
  row('3', 'Terhubung dan m = n&minus;1', 'terhubung=' + (connected ? '✓' : '✗') + ', m=' + m + ', n&minus;1=' + (n - 1), connected && m === n - 1);
  // 4 acyclic & m=n-1
  row('4', 'Tidak ada sirkuit dan m = n&minus;1', 'sirkuit=' + (cyc ? 'ada' : 'tidak') + ', m=' + m + ', n&minus;1=' + (n - 1), !cyc && m === n - 1);
  // 5 acyclic & adding edge makes exactly one cycle
  row('5', 'Tidak ada sirkuit dan +1 sisi membuat tepat satu sirkuit', 'sirkuit=' + (cyc ? 'ada' : 'tidak'), !cyc);
  // 6 connected & all edges bridges
  var allBridge = bridges.length === m;
  row('6', 'Terhubung dan semua sisi adalah jembatan', 'terhubung=' + (connected ? '✓' : '✗') + ', jembatan ' + bridges.length + '/' + m, connected && allBridge);
  tbody.innerHTML = rows.join('');

  // sub lists: components & cycle edges
  var sub = document.getElementById('subListA');
  var html = '';
  html += '<h4>Komponen Terhubung (' + comps.length + ')</h4>';
  for (var c = 0; c < comps.length; c++) {
    var names = comps[c].map(function (x) { return TreeApp.label(x); }).join(', ');
    html += '<span class="pill">{' + names + '}</span>';
  }
  html += '<h4 style="margin-top:14px;">Sisi yang Membentuk Sirkuit</h4>';
  var cycEdges = [];
  for (var e2 = 0; e2 < edges.length; e2++) if (!bridgeSet[e2] && cyc) cycEdges.push(TreeApp.label(edges[e2].u) + '-' + TreeApp.label(edges[e2].v));
  if (cycEdges.length === 0) html += '<span style="color:var(--text-muted);">Tidak ada sisi dalam sirkuit.</span>';
  else html += cycEdges.map(function (s) { return '<span class="pill">' + s + '</span>'; }).join('');
  html += '<h4 style="margin-top:14px;">Jembatan (Bridge)</h4>';
  if (bridges.length === 0) html += '<span style="color:var(--text-muted);">Tidak ada jembatan.</span>';
  else html += bridges.map(function (id) { return '<span class="pill">' + TreeApp.label(edges[id].u) + '-' + TreeApp.label(edges[id].v) + '</span>'; }).join('');
  sub.innerHTML = html;

  return { n: n, edges: edges };
}

/* presets A */
function presetA(kind) {
  var presets = {
    G1: { n: 6, edges: 'a-b\nb-c\nc-d\nd-e\ne-f' }, // path -> tree
    G3: { n: 6, edges: 'a-b\nb-c\nc-a\nc-d\nd-e\ne-f' }, // cycle
    G4: { n: 7, edges: 'a-b\nb-c\nc-d\ne-f\nf-g' } // forest 2 comp
  };
  var p = presets[kind];
  document.getElementById('nA').value = p.n;
  document.getElementById('edgesA').value = p.edges;
  canvasA.setData({ n: p.n, edges: TreeApp.parseEdges(p.edges, p.n).edges, pos: layoutCircle(p.n) });
  runCheckA();
}

document.getElementById('btnCheckA').addEventListener('click', runCheckA);
document.getElementById('presetG1').addEventListener('click', function () { presetA('G1'); });
document.getElementById('presetG3').addEventListener('click', function () { presetA('G3'); });
document.getElementById('presetG4').addEventListener('click', function () { presetA('G4'); });
document.getElementById('resetA').addEventListener('click', function () {
  document.getElementById('nA').value = 6;
  document.getElementById('edgesA').value = 'a-b\nb-c\nc-d\nd-e\ne-f';
  canvasA.setData({ n: 6, edges: TreeApp.parseEdges('a-b\nb-c\nc-d\nd-e\ne-f', 6).edges, pos: layoutCircle(6) });
  document.getElementById('verdictA').classList.remove('show');
  showError('errA', []);
});

/* =====================================================================
   Module B: Spanning Tree & MST
   ===================================================================== */
var canvasB = new GraphCanvas(document.getElementById('svgB'), { readOnly: true });
var canvasBPrim = new GraphCanvas(document.getElementById('svgBPrim'), { readOnly: true });
var canvasBKruskal = new GraphCanvas(document.getElementById('svgBKruskal'), { readOnly: true });

var B = {
  n: 8, edges: [], frames: [], idx: -1, playing: false, timer: null,
  mode: 'prim', lastResult: null
};

function populateStartSelect() {
  var sel = document.getElementById('startB');
  var cur = sel.value;
  sel.innerHTML = '';
  var def = document.createElement('option');
  def.value = '-1';
  def.textContent = 'Default (sisi minimum)';
  sel.appendChild(def);
  for (var i = 0; i < B.n; i++) {
    var o = document.createElement('option');
    o.value = i;
    o.textContent = TreeApp.label(i) + ' (simpul ' + i + ')';
    sel.appendChild(o);
  }
  if (cur === '' || cur === '-1') sel.value = '-1';
  else if (parseInt(cur, 10) < B.n) sel.value = cur;
  else sel.value = '-1';
}

function buildB() {
  var data = buildFromInputs('nB', 'edgesB', 'errB', null);
  var compareWrap = document.getElementById('compareWrapB');
  var allWrap = document.getElementById('allTreesWrapB');
  compareWrap.style.display = 'none';
  allWrap.style.display = 'none';
  if (!data) return null;
  B.n = data.n;
  B.edges = data.edges;
  canvasB.setData({ n: data.n, edges: data.edges, pos: layoutCircle(data.n) });
  populateStartSelect();
  document.getElementById('needCountB').textContent = Math.max(0, data.n - 1);
  computeFrames();
  return data;
}

function computeFrames() {
  var algo = document.getElementById('algoB').value;
  B.mode = algo;
  var res;
  if (algo === 'prim') {
    var start = parseInt(document.getElementById('startB').value, 10);
    if (isNaN(start) || start >= B.n) start = 0;
    res = TreeApp.prim(B.n, B.edges, start);
  } else {
    res = TreeApp.kruskal(B.n, B.edges);
  }
  B.frames = res.steps;
  B.lastResult = res;
  B.idx = -1;
  // side panel content
  renderSidePanel(res, algo);
  document.getElementById('mstNoteB').textContent = res.connected ? '' : 'Graf tidak terhubung — MST tidak ada; hasil adalah spanning forest.';
  drawFrame(-1);
}

function renderSidePanel(res, algo) {
  var title = document.getElementById('sideTitleB');
  var content = document.getElementById('sideContentB');
  if (algo === 'kruskal') {
    title.textContent = 'Sisi Terurut (Kruskal)';
    var sorted = B.edges.map(function (e, i) { return { e: e, i: i }; });
    sorted.sort(function (a, b) { return a.e.w - b.e.w || a.i - b.i; });
    var html = '';
    for (var k = 0; k < sorted.length; k++) {
      var it = sorted[k];
      html += '<div class="counter-line"><span class="pill">' + TreeApp.label(it.e.u) + '-' + TreeApp.label(it.e.v) + '</span> bobot ' + it.e.w + '</div>';
    }
    content.innerHTML = html;
  } else {
    title.textContent = 'Himpunan Simpul di T (Prim)';
    content.innerHTML = '<div id="primSetB" style="font-family:monospace;font-size:12px;"></div>';
  }
}

function drawFrame(idx) {
  var chosen = [];
  var candidate = -1;
  var total = 0;
  var log = '';
  if (idx >= 0 && idx < B.frames.length) {
    var f = B.frames[idx];
    chosen = f.chosen;
    candidate = f.candidate;
    total = f.total;
    log = f.log;
  }
  var chosenSet = {};
  for (var i = 0; i < chosen.length; i++) chosenSet[chosen[i]] = true;
  var ecls = {};
  for (var e = 0; e < B.edges.length; e++) {
    if (chosenSet[e]) ecls[e] = 'chosen';
    else if (e === candidate) {
      var f = B.frames[idx];
      ecls[e] = f && f.accepted ? 'candidate' : 'rejected';
    }
  }
  canvasB.setEdgeClasses(ecls);
  // node classes for Prim: nodes in T
  if (B.mode === 'prim') {
    var inT = {};
    if (idx >= 0) {
      // reconstruct T nodes from chosen edges
      for (var c = 0; c < chosen.length; c++) {
        inT[B.edges[chosen[c]].u] = true;
        inT[B.edges[chosen[c]].v] = true;
      }
      if (B.frames[idx] && B.frames[idx].chosen.length === 0) {
        // first step may pick global min edge; nodes already in chosen
      }
    }
    canvasB.setNodeClasses({});
  } else {
    canvasB.setNodeClasses({});
  }

  document.getElementById('chosenCountB').textContent = chosen.length;
  document.getElementById('totalWeightB').textContent = total;

  // prim set display
  if (B.mode === 'prim') {
    var setEl = document.getElementById('primSetB');
    if (setEl) {
      var nodes = {};
      for (var cc = 0; cc < chosen.length; cc++) { nodes[B.edges[chosen[cc]].u] = true; nodes[B.edges[chosen[cc]].v] = true; }
      var arr = Object.keys(nodes).map(Number).sort(function (a, b) { return a - b; });
      setEl.textContent = arr.length ? 'T = { ' + arr.map(function (x) { return TreeApp.label(x); }).join(', ') + ' }' : 'T = { }';
    }
  }

  // log
  var logEl = document.getElementById('logB');
  if (idx < 0) {
    logEl.innerHTML = '<div class="log-line" style="color:var(--text-muted);">Tekan Play atau Step Maju untuk memulai animasi ' + (B.mode === 'prim' ? 'Prim' : 'Kruskal') + '.</div>';
  } else {
    var lines = [];
    for (var li = 0; li <= idx && li < B.frames.length; li++) {
      var fr = B.frames[li];
      lines.push('<div class="log-line"><span class="log-step">[Langkah ' + (li + 1) + ']</span> ' + fr.log + ' <span class="' + (fr.accepted ? 'log-ok' : 'log-no') + '">' + (fr.accepted ? '(diterima)' : '(ditolak)') + '</span></div>');
    }
    logEl.innerHTML = lines.join('');
    logEl.scrollTop = logEl.scrollHeight;
  }
}

function stepForward() {
  if (B.frames.length === 0) return;
  if (B.idx < B.frames.length - 1) { B.idx++; drawFrame(B.idx); }
  else stopPlay();
}
function stepBack() {
  if (B.idx > -1) { B.idx--; drawFrame(B.idx); }
}
function speedMs() {
  var v = parseInt(document.getElementById('speedB').value, 10) || 4;
  return Math.round(1000 - (v - 1) * 90); // 1000ms .. 190ms
}
function startPlay() {
  if (B.frames.length === 0) return;
  if (B.idx >= B.frames.length - 1) B.idx = -1;
  B.playing = true;
  document.getElementById('playB').textContent = 'Pause';
  tick();
}
function tick() {
  if (!B.playing) return;
  if (B.idx < B.frames.length - 1) {
    B.idx++;
    drawFrame(B.idx);
    B.timer = setTimeout(tick, speedMs());
  } else {
    stopPlay();
  }
}
function stopPlay() {
  B.playing = false;
  if (B.timer) { clearTimeout(B.timer); B.timer = null; }
  document.getElementById('playB').textContent = 'Play';
}

document.getElementById('playB').addEventListener('click', function () {
  if (B.playing) stopPlay(); else startPlay();
});
document.getElementById('stepFwdB').addEventListener('click', function () { stopPlay(); stepForward(); });
document.getElementById('stepBackB').addEventListener('click', function () { stopPlay(); stepBack(); });
document.getElementById('animResetB').addEventListener('click', function () { stopPlay(); B.idx = -1; drawFrame(-1); });
document.getElementById('btnBuildB').addEventListener('click', function () { stopPlay(); buildB(); });
document.getElementById('algoB').addEventListener('change', function () { stopPlay(); if (B.edges.length) computeFrames(); });

document.getElementById('presetB').addEventListener('click', function () {
  var txt = 'a-c:4\na-b:5\nc-b:3\na-d:4\na-e:4\nb-e:5\nd-e:5\nd-h:5\ne-g:4\nh-g:4\ne-f:3\nf-g:2';
  document.getElementById('nB').value = 8;
  document.getElementById('edgesB').value = txt;
  stopPlay();
  buildB();
});

document.getElementById('randomB').addEventListener('click', function () {
  var n = parseInt(document.getElementById('nB').value, 10) || 8;
  if (n < 2) n = 2; if (n > 15) n = 15;
  var density = parseInt(document.getElementById('densityB').value, 10) || 50;
  var edges = [];
  var seen = {};
  for (var i = 0; i < n; i++) {
    for (var j = i + 1; j < n; j++) {
      if (Math.random() * 100 < density) {
        var w = 1 + Math.floor(Math.random() * 9);
        edges.push(TreeApp.label(i) + '-' + TreeApp.label(j) + ':' + w);
      }
    }
  }
  // ensure connected: chain fallback
  var connected = false;
  if (edges.length) {
    var parsed = TreeApp.parseEdges(edges.join('\n'), n);
    connected = TreeApp.components(n, parsed.edges).length === 1;
  }
  if (!connected) {
    for (var k = 0; k < n - 1; k++) {
      edges.push(TreeApp.label(k) + '-' + TreeApp.label(k + 1) + ':' + (1 + Math.floor(Math.random() * 9)));
    }
  }
  document.getElementById('edgesB').value = edges.join('\n');
  stopPlay();
  buildB();
});

document.getElementById('compareB').addEventListener('click', function () {
  var data = buildFromInputs('nB', 'edgesB', 'errB', null);
  if (!data) return;
  B.n = data.n; B.edges = data.edges;
  var wrap = document.getElementById('compareWrapB');
  document.getElementById('allTreesWrapB').style.display = 'none';
  var start = parseInt(document.getElementById('startB').value, 10);
  if (isNaN(start) || start >= B.n) start = 0;
  var pr = TreeApp.prim(B.n, B.edges, start);
  var kr = TreeApp.kruskal(B.n, B.edges);
  var pos = layoutCircle(data.n);
  canvasBPrim.setData({ n: data.n, edges: data.edges, pos: pos });
  canvasBKruskal.setData({ n: data.n, edges: data.edges, pos: pos });
  var pset = {}; for (var i = 0; i < pr.chosen.length; i++) pset[pr.chosen[i]] = 'chosen';
  var kset = {}; for (var j = 0; j < kr.chosen.length; j++) kset[kr.chosen[j]] = 'chosen';
  canvasBPrim.setEdgeClasses(pset);
  canvasBKruskal.setEdgeClasses(kset);
  document.getElementById('primTotalB').textContent = pr.connected ? pr.total : pr.total + ' (forest)';
  document.getElementById('kruskalTotalB').textContent = kr.connected ? kr.total : kr.total + ' (forest)';
  var note = document.getElementById('compareNoteB');
  if (!pr.connected || !kr.connected) {
    note.innerHTML = 'Graf tidak terhubung: MST tidak ada. Kedua algoritma menghasilkan <strong>spanning forest</strong> dengan total bobot sama (jika bobot unik).';
  } else if (pr.total === kr.total) {
    note.innerHTML = 'Kedua algoritma menghasilkan total bobot MST yang sama: <strong>' + pr.total + '</strong>. Jumlah sisi: Prim ' + pr.chosen.length + ', Kruskal ' + kr.chosen.length + '.';
  } else {
    note.innerHTML = 'Total berbeda (Prim ' + pr.total + ', Kruskal ' + kr.total + ') — periksa kembali input.';
  }
  // check multiple MST (equal weights)
  var sortedW = B.edges.map(function (e) { return e.w; }).sort(function (a, b) { return a - b; });
  var hasDup = false;
  for (var d = 1; d < sortedW.length; d++) if (sortedW[d] === sortedW[d - 1]) hasDup = true;
  if (hasDup && pr.total === kr.total) {
    note.innerHTML += ' <strong>Catatan:</strong> terdapat bobot kembar, sehingga mungkin ada lebih dari satu MST valid dengan total sama.';
  }
  wrap.style.display = 'block';
});

document.getElementById('allTreesB').addEventListener('click', function () {
  var data = buildFromInputs('nB', 'edgesB', 'errB', null);
  if (!data) return;
  var wrap = document.getElementById('allTreesWrapB');
  document.getElementById('compareWrapB').style.display = 'none';
  if (data.n > 5) {
    wrap.style.display = 'block';
    document.getElementById('allTreesListB').innerHTML = '';
    document.getElementById('allTreesNoteB').innerHTML = 'Opsi ini hanya untuk graf kecil (n &le; 5). Graf Anda memiliki n = ' + data.n + '.';
    return;
  }
  var trees = TreeApp.allSpanningTrees(data.n, data.edges);
  var html = '';
  for (var t = 0; t < trees.length; t++) {
    var names = trees[t].edges.map(function (e) { return TreeApp.label(e.u) + '-' + TreeApp.label(e.v); }).join(', ');
    html += '<div class="spanning-item"><b>T' + (t + 1) + '</b> bobot ' + trees[t].total + '<br><span style="color:var(--text-secondary);font-size:10.5px;">' + names + '</span></div>';
  }
  document.getElementById('allTreesListB').innerHTML = html || '<span style="color:var(--text-muted);">Tidak ada spanning tree (graf tidak terhubung).</span>';
  document.getElementById('allTreesNoteB').innerHTML = 'Total ' + trees.length + ' spanning tree untuk graf dengan n = ' + data.n + ' (Gambar 3.2 style: T1, T2, ...).';
  wrap.style.display = 'block';
});

document.getElementById('resetB').addEventListener('click', function () {
  stopPlay();
  var txt = 'a-c:4\na-b:5\nc-b:3\na-d:4\na-e:4\nb-e:5\nd-e:5\nd-h:5\ne-g:4\nh-g:4\ne-f:3\nf-g:2';
  document.getElementById('nB').value = 8;
  document.getElementById('edgesB').value = txt;
  document.getElementById('compareWrapB').style.display = 'none';
  document.getElementById('allTreesWrapB').style.display = 'none';
  showError('errB', []);
  buildB();
});

/* =====================================================================
   Module C: Pohon Berakar
   ===================================================================== */
var canvasC = new GraphCanvas(document.getElementById('svgC'), { readOnly: true });

function populateRootSelect() {
  var sel = document.getElementById('rootC');
  var cur = sel.value;
  sel.innerHTML = '';
  var n = parseInt(document.getElementById('nC').value, 10) || 0;
  for (var i = 0; i < n; i++) {
    var o = document.createElement('option');
    o.value = i;
    o.textContent = TreeApp.label(i);
    sel.appendChild(o);
  }
  if (cur !== '' && parseInt(cur, 10) < n) sel.value = cur;
}

function layoutHierarchical(n, edges, root) {
  var t = TreeApp.rooted(n, edges, root);
  var pos = [];
  for (var i = 0; i < n; i++) pos.push([50, 35]);
  // group by depth
  var maxDepth = 0;
  for (var d = 0; d < n; d++) if (t.depth[d] > maxDepth) maxDepth = t.depth[d];
  var levels = [];
  for (var lv = 0; lv <= maxDepth; lv++) levels.push([]);
  for (var v = 0; v < n; v++) levels[t.depth[v]].push(v);
  var topY = 8, bottomY = 62;
  for (var lv2 = 0; lv2 < levels.length; lv2++) {
    var arr = levels[lv2];
    var y = maxDepth === 0 ? 35 : topY + (bottomY - topY) * (lv2 / maxDepth);
    for (var k = 0; k < arr.length; k++) {
      var x = arr.length === 1 ? 50 : 15 + (70 * (k / (arr.length - 1)));
      pos[arr[k]] = [x, y];
    }
  }
  return { pos: pos, tree: t };
}

function buildC() {
  var data = buildFromInputs('nC', 'edgesC', 'errC', null);
  if (!data) return null;
  if (!TreeApp.isTree(data.n, data.edges)) {
    showError('errC', ['Graf yang dimasukkan bukan pohon (ada sirkuit / tidak terhubung / jumlah sisi bukan n-1). Perbaiki di Modul A terlebih dahulu.']);
    return null;
  }
  populateRootSelect();
  var root = parseInt(document.getElementById('rootC').value, 10);
  if (isNaN(root) || root >= data.n) root = 0;
  renderRooted(data.n, data.edges, root);
  return data;
}

function renderRooted(n, edges, root) {
  var lay = layoutHierarchical(n, edges, root);
  var t = lay.tree;
  canvasC.setData({ n: n, edges: edges, pos: lay.pos });
  canvasC.setDirected(true);
  // node classes
  var ncls = {};
  ncls[root] = 'root';
  for (var v = 0; v < n; v++) {
    if (v !== root && t.outdeg[v] === 0) ncls[v] = 'leaf';
  }
  // clear then set (root takes precedence)
  var finalCls = {};
  for (var v2 = 0; v2 < n; v2++) if (ncls[v2] === 'leaf') finalCls[v2] = 'leaf';
  finalCls[root] = 'root';
  canvasC.setNodeClasses(finalCls);
  // if first frame, disable transition to avoid jump; subsequent frames animate via CSS on positions
  document.getElementById('treeHeightC').textContent = t.height;

  // table
  var tbody = document.querySelector('#treeTableC tbody');
  var rows = [];
  for (var x = 0; x < n; x++) {
    var role = (x === root) ? 'akar' : (t.outdeg[x] === 0 ? 'daun' : 'simpul dalam');
    var roleCls = (x === root) ? 'role-root' : (t.outdeg[x] === 0 ? 'role-leaf' : '');
    var parent = t.parent[x] === null ? '-' : TreeApp.label(t.parent[x]);
    var kids = t.children[x].length ? t.children[x].map(function (c) { return TreeApp.label(c); }).join(', ') : '-';
    rows.push('<tr><td><strong>' + TreeApp.label(x) + '</strong></td><td>' + t.indeg[x] + '</td><td>' + t.outdeg[x] + '</td><td class="' + roleCls + '">' + role + '</td><td>' + parent + '</td><td>' + kids + '</td><td>' + t.depth[x] + '</td></tr>');
  }
  tbody.innerHTML = rows.join('');
}

document.getElementById('btnRootC').addEventListener('click', buildC);
document.getElementById('rootC').addEventListener('change', function () {
  var n = parseInt(document.getElementById('nC').value, 10);
  var parsed = TreeApp.parseEdges(document.getElementById('edgesC').value, n);
  if (parsed.errors.length) { showError('errC', parsed.errors); return; }
  if (!TreeApp.isTree(n, parsed.edges)) { showError('errC', ['Graf bukan pohon.']); return; }
  showError('errC', []);
  var root = parseInt(document.getElementById('rootC').value, 10);
  renderRooted(n, parsed.edges, root);
});
document.getElementById('useTreeAC').addEventListener('click', function () {
  var txt = document.getElementById('edgesA').value;
  var n = parseInt(document.getElementById('nA').value, 10);
  document.getElementById('nC').value = n;
  document.getElementById('edgesC').value = txt;
  populateRootSelect();
  buildC();
});
document.getElementById('useTreeBC').addEventListener('click', function () {
  if (!B.lastResult) { alert('Bangun graf di Modul B terlebih dahulu.'); return; }
  var chosen = B.lastResult.chosen;
  var lines = [];
  for (var i = 0; i < chosen.length; i++) {
    var e = B.edges[chosen[i]];
    lines.push(TreeApp.label(e.u) + '-' + TreeApp.label(e.v));
  }
  document.getElementById('nC').value = B.n;
  document.getElementById('edgesC').value = lines.join('\n');
  populateRootSelect();
  buildC();
});
document.getElementById('resetC').addEventListener('click', function () {
  document.getElementById('nC').value = 7;
  document.getElementById('edgesC').value = 'a-b\na-c\nb-d\nb-e\nc-f\nc-g';
  showError('errC', []);
  populateRootSelect();
  buildC();
});

/* re-root on node click */
document.getElementById('svgC').addEventListener('click', function (evt) {
  var nodeEl = evt.target.closest ? evt.target.closest('.node') : null;
  if (!nodeEl) return;
  var v = parseInt(nodeEl.getAttribute('data-v'), 10);
  var sel = document.getElementById('rootC');
  sel.value = v;
  var n = parseInt(document.getElementById('nC').value, 10);
  var parsed = TreeApp.parseEdges(document.getElementById('edgesC').value, n);
  if (!parsed.errors.length && TreeApp.isTree(n, parsed.edges)) renderRooted(n, parsed.edges, v);
});

/* =====================================================================
   Tabs
   ===================================================================== */
var tabs = document.querySelectorAll('.module-tab');
for (var ti = 0; ti < tabs.length; ti++) {
  tabs[ti].addEventListener('click', function () {
    for (var j = 0; j < tabs.length; j++) {
      tabs[j].classList.remove('active');
      tabs[j].setAttribute('aria-selected', 'false');
    }
    this.classList.add('active');
    this.setAttribute('aria-selected', 'true');
    var panels = document.querySelectorAll('.module-panel');
    for (var p = 0; p < panels.length; p++) panels[p].classList.remove('active');
    document.getElementById(this.getAttribute('data-target')).classList.add('active');
  });
}

/* =====================================================================
   Init
   ===================================================================== */
window.addEventListener('DOMContentLoaded', function () {
  // Module A default
  var aEdges = TreeApp.parseEdges('a-b\nb-c\nc-d\nd-e\ne-f', 6).edges;
  canvasA.setData({ n: 6, edges: aEdges, pos: layoutCircle(6) });
  runCheckA();

  // Module B default (Contoh 3.1)
  buildB();

  // Module C default
  populateRootSelect();
  buildC();
});
</script>
"#;
    Html(layout("Pohon (Tree)", "tree", "/Learning/", body))
}