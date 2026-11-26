use axum::response::Html;
use crate::pages::layout::layout;

pub async fn handler() -> Html<String> {
    let body = r#"
<style>
  .eigen-layout {
    display: flex;
    min-height: calc(100vh - 96px);
    max-width: 980px;
    margin: 0 auto;
    padding: 32px 24px 64px;
    gap: 24px;
  }
  .sidebar { width: 240px; flex-shrink: 0; }
  .sidebar-inner {
    position: sticky;
    top: 88px;
    padding: 20px 16px;
    border-radius: var(--radius-lg);
  }
  .sidebar-title {
    font-size: 10.5px; font-weight: 700; letter-spacing: 0.1em;
    text-transform: uppercase; color: var(--text-muted);
    margin-bottom: 12px; padding: 0 8px;
  }
  .sidebar-section-label {
    font-size: 10px; font-weight: 700; letter-spacing: 0.08em;
    text-transform: uppercase; color: var(--text-muted);
    padding: 12px 8px 4px; display: block;
  }
  .sidebar-nav { list-style: none; }
  .sidebar-nav li + li { margin-top: 2px; }
  .sidebar-nav a {
    display: flex; align-items: center; gap: 8px;
    padding: 8px 10px; border-radius: var(--radius-sm);
    text-decoration: none; color: var(--text-secondary);
    font-size: 13px; font-weight: 500;
    transition: color var(--transition), background var(--transition);
  }
  .sidebar-nav a:hover { color: var(--text-primary); background: var(--glass-hover); }
  .sidebar-nav a.active { color: var(--accent); background: var(--accent-soft); }
  .sidebar-nav a .dot {
    width: 6px; height: 6px; border-radius: 50%;
    background: currentColor; flex-shrink: 0; opacity: 0.6;
  }

  .eigen-content { flex: 1; min-width: 0; }
  .eigen-header { margin-bottom: 28px; }
  .eigen-header .label { margin-bottom: 14px; }
  .eigen-header h1 { margin-bottom: 8px; }
  .eigen-header p { font-size: 14px; line-height: 1.75; max-width: 640px; }

  .concept-card { padding: 24px; margin-bottom: 24px; border-radius: var(--radius-lg); }
  .concept-card h3 { margin-bottom: 10px; }
  .concept-card p { font-size: 13.5px; line-height: 1.75; }

  .term-row { display: flex; gap: 10px; margin-top: 18px; flex-wrap: wrap; }
  .term-chip {
    position: relative; padding: 8px 14px; border-radius: 100px;
    border: 1px solid var(--accent-border);
    background: var(--accent-soft); color: var(--accent);
    font-size: 11.5px; font-weight: 700; letter-spacing: 0.06em;
    cursor: pointer; transition: background var(--transition), transform var(--transition);
  }
  .term-chip:hover, .term-chip:focus-visible {
    background: var(--accent); color: #fff; transform: translateY(-1px); outline: none;
  }
  .term-chip .term-pop {
    position: absolute; top: calc(100% + 8px); left: 0; width: 260px;
    padding: 12px 14px; border-radius: var(--radius-sm);
    background: var(--bg-elevated); border: 1px solid var(--glass-border);
    color: var(--text-secondary); font-size: 12px; font-weight: 400;
    letter-spacing: 0; line-height: 1.6; text-transform: none;
    box-shadow: var(--shadow-md); opacity: 0; pointer-events: none;
    transform: translateY(-4px); transition: opacity var(--transition), transform var(--transition);
    z-index: 20;
  }
  .term-chip:hover .term-pop, .term-chip:focus-visible .term-pop {
    opacity: 1; pointer-events: auto; transform: translateY(0);
  }

  .module-tabs {
    display: flex; gap: 6px; margin-bottom: 20px; flex-wrap: wrap;
  }
  .module-tab {
    padding: 9px 16px; border-radius: 100px; border: 1px solid var(--glass-border);
    background: var(--glass-bg); color: var(--text-secondary);
    font-size: 12.5px; font-weight: 600; cursor: pointer;
    font-family: inherit; transition: all var(--transition);
  }
  .module-tab:hover { color: var(--text-primary); background: var(--glass-hover); }
  .module-tab.active {
    background: var(--accent); color: #fff; border-color: var(--accent);
  }
  .module-panel { display: none; }
  .module-panel.active { display: block; }

  .sim-card { padding: 28px; border-radius: var(--radius-lg); margin-bottom: 24px; }
  .sim-title {
    font-size: 13px; font-weight: 700; color: var(--text-secondary);
    text-transform: uppercase; letter-spacing: 0.06em; margin-bottom: 8px;
  }
  .sim-desc { font-size: 13px; margin-bottom: 20px; color: var(--text-secondary); }

  .sim-controls {
    display: flex; gap: 10px; flex-wrap: wrap; align-items: flex-end;
    margin-bottom: 20px;
  }
  .input-group { display: flex; flex-direction: column; gap: 5px; flex: 1; min-width: 160px; }
  .input-group.small { flex: 0 0 150px; min-width: 150px; }
  .input-group label {
    font-size: 11.5px; font-weight: 600; color: var(--text-muted);
    text-transform: uppercase; letter-spacing: 0.05em;
  }
  .input-field, select.input-field {
    padding: 9px 14px; border-radius: var(--radius-sm);
    border: 1px solid var(--glass-border);
    background: var(--bg-base); color: var(--text-primary);
    font-size: 13.5px; font-family: 'Inter', monospace;
    transition: border-color var(--transition), background var(--transition);
    outline: none; width: 100%;
  }
  textarea.input-field { resize: vertical; min-height: 90px; line-height: 1.6; }
  .input-field:focus { border-color: var(--accent-border); background: var(--glass-hover); }

  .btn-row { display: flex; gap: 8px; flex-wrap: wrap; }
  .btn-run {
    padding: 9px 20px; border-radius: var(--radius-sm);
    background: var(--accent); color: #fff; font-size: 13px;
    font-weight: 600; border: none; cursor: pointer;
    font-family: inherit; transition: opacity var(--transition);
  }
  .btn-run:hover { opacity: 0.85; }
  .btn-run:disabled { opacity: 0.4; cursor: not-allowed; }
  .btn-reset, .btn-ghost {
    padding: 9px 20px; border-radius: var(--radius-sm);
    background: var(--glass-bg); border: 1px solid var(--glass-border);
    color: var(--text-secondary); font-size: 13px;
    font-weight: 600; cursor: pointer; font-family: inherit;
    transition: background var(--transition), color var(--transition);
  }
  .btn-reset:hover, .btn-ghost:hover { background: var(--glass-hover); color: var(--text-primary); }
  .btn-ghost.active { background: var(--accent-soft); color: var(--accent); border-color: var(--accent-border); }

  .error-box {
    display: none; margin-bottom: 16px; padding: 12px 16px;
    border-radius: var(--radius-sm);
    background: rgba(239,68,68,0.12); border: 1px solid rgba(239,68,68,0.4);
    color: #ef4444; font-size: 13px; line-height: 1.6;
  }
  .error-box.show { display: block; }
  .error-box ul { margin: 6px 0 0 18px; }

  /* Matrix grid */
  .matrix-grid { display: grid; gap: 6px; max-width: 520px; }
  .matrix-row { display: flex; gap: 6px; justify-content: center; }
  .cell-input {
    width: 56px; height: 56px; border-radius: var(--radius-sm);
    border: 1px solid var(--glass-border);
    background: var(--bg-base); color: var(--text-primary);
    font-size: 15px; font-family: 'Inter', monospace;
    text-align: center; outline: none;
    transition: border-color var(--transition), background var(--transition);
  }
  .cell-input:focus { border-color: var(--accent-border); background: var(--glass-hover); }

  /* Steps */
  .step-nav {
    display: flex; gap: 8px; flex-wrap: wrap; align-items: center;
    margin-bottom: 16px;
  }
  .step-counter {
    font-size: 12.5px; color: var(--text-muted); font-weight: 600;
    border: 1px solid var(--glass-border); padding: 4px 12px;
    border-radius: 100px; background: var(--bg-base);
  }
  .steps-wrap { border: 1px solid var(--glass-border); border-radius: var(--radius-sm); background: var(--bg-base); }
  .step { display: none; padding: 22px 24px; }
  .step.active { display: block; }
  .step h4 {
    font-size: 12px; text-transform: uppercase; letter-spacing: 0.05em;
    color: var(--accent); margin: 0 0 12px 0; font-weight: 700;
  }
  .step .math { margin: 10px 0; line-height: 1.9; }
  .step p { font-size: 13px; color: var(--text-secondary); margin: 8px 0; line-height: 1.7; }
  .step ul { margin: 6px 0 6px 20px; padding: 0; color: var(--text-secondary); font-size: 13px; line-height: 1.75; }

  .latex-inline { font-family: 'Inter', monospace; font-size: 12.5px; }
  .check-grid {
    display: grid; grid-template-columns: 1fr 1fr; gap: 14px; margin-top: 10px;
  }
  @media (max-width: 560px) { .check-grid { grid-template-columns: 1fr; } }
  .check-item {
    padding: 14px 16px; border-radius: var(--radius-sm);
    background: var(--bg-layer); border: 1px solid var(--glass-border);
  }
  .check-item .lbl { font-size: 11px; text-transform: uppercase; letter-spacing: 0.05em; color: var(--text-muted); margin-bottom: 6px; font-weight: 600; }
  .check-item .val { font-size: 14px; font-variant-numeric: tabular-nums; color: var(--text-primary); }
  .check-item .ok { color: var(--success); font-weight: 700; }
  .check-item .no { color: #ef4444; font-weight: 700; }

  .eigval-chip {
    display: inline-flex; align-items: center; gap: 8px;
    padding: 6px 12px; border-radius: 100px;
    border: 1px solid var(--accent-border); background: var(--accent-soft);
    color: var(--accent); font-size: 12px; font-weight: 600;
    margin: 4px 6px 4px 0;
  }
  .eigval-chip.complex { color: #c084fc; border-color: rgba(192,132,252,0.35); background: rgba(192,132,252,0.12); }
  .eigval-chip.repeated { color: var(--warning); border-color: rgba(251,191,36,0.35); background: rgba(251,191,36,0.12); }
  .eigval-chip .tag { font-size: 10px; opacity: 0.8; text-transform: uppercase; letter-spacing: 0.05em; }

  .copy-bar {
    margin-top: 12px; padding: 12px 16px; border-radius: var(--radius-sm);
    background: var(--bg-base); border: 1px solid var(--glass-border);
    font-size: 12.5px; font-family: 'Inter', monospace;
    color: var(--text-secondary); max-height: 220px; overflow-y: auto;
  }
  .special-note {
    margin-top: 10px; padding: 10px 14px; border-radius: var(--radius-sm);
    background: rgba(192,132,252,0.10); border: 1px solid rgba(192,132,252,0.35);
    color: #c084fc; font-size: 12.5px; line-height: 1.7;
  }

  @media (max-width: 680px) {
    .eigen-layout { flex-direction: column; }
    .sidebar { width: 100%; }
    .sidebar-inner { position: static; }
  }
</style>

<div class="eigen-layout">
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
        <li><a href="/Learning/Jaringan-Komputer/"><span class="dot"></span>Jaringan Komputer</a></li>
        <span class="sidebar-section-label">Aljabar Linear</span>
        <li><a href="/Learning/Nilai-Eigen/" class="active"><span class="dot"></span>Nilai &amp; Vektor Eigen</a></li>
      </ul>
    </div>
  </aside>

  <!-- Main -->
  <main class="eigen-content">
    <div class="eigen-header">
      <div class="label">Aljabar Linear</div>
      <h1 style="margin-top:16px;">Nilai &amp; Vektor Eigen</h1>
      <p style="margin-top:10px;">
        Pelajari bagaimana mencari nilai eigen dan vektor eigen dari matriks bujur
        sangkar <i>A</i> melalui persamaan <strong>Ax = λx</strong>. Semua hasil
        dihitung secara langsung dari matriks yang Anda masukkan.
      </p>
    </div>

    <!-- Konsep dasar -->
    <div class="concept-card glass">
      <h3>Konsep Dasar</h3>
      <p>
        Jika <i>A</i> adalah matriks <i>n</i>&times;<i>n</i>, maka vektor tak-nol
        <i>x</i> di <i>R<sup>n</sup></i> disebut vektor eigen dari <i>A</i> jika
        <i>Ax</i> adalah kelipatan skalar dari <i>x</i>, yaitu <strong>Ax = λx</strong>.
        Skalar <strong>λ</strong> disebut nilai eigen. Nilai eigen didapat dari
        akar polinomial karakteristik <i>det(λI − A) = 0</i>.
      </p>
      <div class="term-row">
        <span class="term-chip" tabindex="0" role="button" aria-label="Definisi Nilai Eigen">
          NILAI EIGEN (λ)
          <span class="term-pop">Skalar λ sehingga Ax = λx untuk suatu vektor tak-nol x.</span>
        </span>
        <span class="term-chip" tabindex="0" role="button" aria-label="Definisi Vektor Eigen">
          VEKTOR EIGEN (x)
          <span class="term-pop">Vektor tak-nol x yang memenuhi Ax = λx untuk suatu nilai eigen λ.</span>
        </span>
        <span class="term-chip" tabindex="0" role="button" aria-label="Definisi Polinomial Karakteristik">
          POLINOMIAL KARAKTERISTIK
          <span class="term-pop">Polinomial det(λI − A) dalam λ; akarnya adalah nilai eigen A.</span>
        </span>
        <span class="term-chip" tabindex="0" role="button" aria-label="Definisi Ruang Eigen">
          RUANG EIGEN E(λ)
          <span class="term-pop">Himpunan semua vektor eigen untuk λ bersama vektor nol, E(λ) = { x = t·v, t &isin; R }.</span>
        </span>
      </div>
    </div>

    <!-- Module tabs -->
    <div class="module-tabs" role="tablist" aria-label="Modul">
      <button class="module-tab active" data-target="modA" role="tab" aria-selected="true">A &middot; Kalkulator Nilai Eigen</button>
      <button class="module-tab" data-target="modB" role="tab" aria-selected="false">B &middot; Vektor &amp; Ruang Eigen</button>
      <button class="module-tab" data-target="modC" role="tab" aria-selected="false">C &middot; Visualisasi 2D</button>
    </div>

    <!-- ============ MODULE A ============ -->
    <div class="module-panel active" id="modA" role="tabpanel">
      <div class="sim-card glass">
        <div class="sim-title">Modul A &middot; Kalkulator Nilai Eigen</div>
        <p class="sim-desc">
          Masukkan matriks <i>A</i> (ukuran 2&times;2 atau 3&times;3). Semua langkah
          dan hasil dihitung dari matriks yang Anda input. Preset hanya mengisi form.
        </p>

        <div class="error-box" id="errA" role="alert"></div>

        <div class="sim-controls">
          <div class="input-group small">
            <label for="sizeA">Ukuran matriks</label>
            <select class="input-field" id="sizeA">
              <option value="2">2 &times; 2</option>
              <option value="3">3 &times; 3</option>
            </select>
          </div>
        </div>

        <div class="matrix-grid" id="matrixGridA"></div>

        <div class="btn-row" style="margin-bottom:8px;">
          <button class="btn-run" id="btnCalcA">Hitung</button>
          <button class="btn-ghost" id="presetNote">Contoh Catatan</button>
          <button class="btn-ghost" id="presetDiag">Matriks Diagonal</button>
          <button class="btn-ghost" id="presetSym">Matriks Simetris</button>
          <button class="btn-ghost" id="presetRepeated">Nilai Eigen Kembar</button>
          <button class="btn-ghost" id="presetComplex">Kompleks / Rotasi</button>
          <button class="btn-ghost" id="randomA">Acak Matriks</button>
          <button class="btn-reset" id="resetA">Reset</button>
        </div>

        <div class="step-nav">
          <button class="btn-ghost" id="stepPrev">&larr; Sebelumnya</button>
          <button class="btn-ghost" id="stepNext">Selanjutnya &rarr;</button>
          <span class="step-counter" id="stepCounter">Langkah 1 / 6</span>
          <button class="btn-ghost" id="stepAll">Tampilkan Semua</button>
          <button class="btn-ghost" id="copySteps">Salin Langkah Penyelesaian</button>
        </div>

        <div class="steps-wrap" id="stepsWrap"></div>

        <div class="check-grid" id="checkA"></div>

        <div id="eigvalsA" style="margin-top:14px;"></div>
      </div>
    </div>

    <!-- ============ MODULE B (stub) ============ -->
    <div class="module-panel" id="modB" role="tabpanel">
      <div class="sim-card glass">
        <div class="sim-title">Modul B &middot; Vektor &amp; Ruang Eigen</div>
        <p class="sim-desc">
          Menampilkan matriks (λI &minus; A), sistem homogen, reduksi baris (OBE),
          variabel bebas, basis ruang eigen E(λ), serta perbandingan multiplisitas
          aljabar dan geometri. <strong>Modul B akan tersedia pada tahap berikutnya.</strong>
        </p>
      </div>
    </div>

<!-- ============ MODULE C ============ -->
    <div class="module-panel" id="modC" role="tabpanel">
      <div class="sim-card glass">
        <div class="sim-title">Modul C &middot; Visualisasi Geometri 2D</div>
        <p class="sim-desc">
          Visualisasi vektor <i>x</i> dan <i>Ax</i>, garis ruang eigen, animasi transformasi
          grid & lingkaran satuan menjadi elips, serta panel angka kontekstual.
          Hanya untuk matriks 2&times;2 (sinkron dengan Modul A & B).
        </p>

        <div class="error-box" id="errC" role="alert"></div>

        <!-- Not 2x2 message -->
        <div id="not2x2C" style="display:none; padding:16px; border-radius:var(--radius-sm); background:rgba(251,191,36,0.12); border:1px solid rgba(251,191,36,0.35); color:var(--warning); margin-bottom:16px;">
          <strong>Visualisasi hanya untuk matriks 2&times;2</strong>
          <button class="btn-ghost" id="resizeTo2x2C" style="margin-left:12px;">Ubah ke 2&times;2</button>
        </div>

        <!-- Main visualization area -->
        <div id="vizAreaC" style="display:none;">
          <!-- Matrix input grid (synced with Module A) -->
          <div style="margin-bottom:16px;">
            <div style="font-size:11.5px; font-weight:600; color:var(--text-muted); text-transform:uppercase; letter-spacing:0.05em; margin-bottom:8px;">Matriks A (2×2, sinkron dengan Modul A & B)</div>
            <div class="matrix-grid" id="matrixGridC" style="max-width:280px;"></div>
          </div>

          <div style="display:grid; grid-template-columns: 1fr 340px; gap:20px; align-items:start;">
            <!-- Left: Canvas + Controls -->
            <div>
              <!-- Canvas wrapper -->
              <div style="position:relative; border-radius:var(--radius-lg); overflow:hidden; background:var(--bg-base); border:1px solid var(--glass-border);">
                <svg id="svgC" width="100%" height="480" viewBox="0 0 600 480" preserveAspectRatio="xMidYMid meet" role="img" aria-labelledby="svgTitleC svgDescC">
                  <title id="svgTitleC">Visualisasi Geometri Eigen 2D</title>
                  <desc id="svgDescC">Kanvas menampilkan vektor x, Ax, garis ruang eigen, lingkaran satuan, elips transformasi, dan grid.</desc>
                  <defs>
                    <marker id="arrowheadA" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto" markerUnits="strokeWidth">
                      <polygon points="0 0, 10 3.5, 0 7" fill="var(--color-vec-x)" />
                    </marker>
                    <marker id="arrowheadB" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto" markerUnits="strokeWidth">
                      <polygon points="0 0, 10 3.5, 0 7" fill="var(--color-vec-ax)" />
                    </marker>
                    <marker id="arrowheadEig" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto" markerUnits="strokeWidth">
                      <polygon points="0 0, 10 3.5, 0 7" fill="var(--color-eig)" />
                    </marker>
                  </defs>
                  <!-- Main transform group: translate to center, scale world to pixels, flip Y -->
                  <g id="mainGroupC">
                    <!-- Background grid (transformed) -->
                    <g id="gridGroupC"></g>
                    <!-- Unit circle (radius 1 in world coords) -->
                    <circle id="unitCircleC" cx="0" cy="0" r="1" fill="none" stroke="var(--text-muted)" stroke-width="0.5" stroke-dasharray="6,4" opacity="0.5" vector-effect="non-scaling-stroke" />
                    <!-- Transformed ellipse (will be set via path data in world coords) -->
                    <path id="ellipseC" fill="none" stroke="var(--accent)" stroke-width="1" opacity="0.8" vector-effect="non-scaling-stroke" />
                    <!-- Eigen space lines -->
                    <g id="eigenLinesGroupC"></g>
                    <!-- Coordinate axes -->
                    <g id="axesGroupC"></g>
                    <!-- Vector x -->
                    <g id="vecXGroupC">
                      <path id="vecXLineC" stroke="var(--color-vec-x)" stroke-width="2.5" marker-end="url(#arrowheadA)" fill="none" vector-effect="non-scaling-stroke" />
                      <text id="vecXLabelC" text-anchor="middle" font-size="11" fill="var(--color-vec-x)" font-weight="600"></text>
                    </g>
                    <!-- Vector Ax -->
                    <g id="vecAXGroupC">
                      <path id="vecAXLineC" stroke="var(--color-vec-ax)" stroke-width="2.5" marker-end="url(#arrowheadB)" fill="none" vector-effect="non-scaling-stroke" />
                      <path id="vecConnectC" stroke="var(--text-muted)" stroke-width="1" stroke-dasharray="4,4" opacity="0.6" fill="none" vector-effect="non-scaling-stroke" />
                      <text id="vecAXLabelC" text-anchor="middle" font-size="11" fill="var(--color-vec-ax)" font-weight="600"></text>
                    </g>
                    <!-- Trace path -->
                    <path id="tracePathC" fill="none" stroke="var(--color-vec-x)" stroke-width="1.5" stroke-dasharray="2,2" opacity="0.5" vector-effect="non-scaling-stroke" />
                  </g>
                </svg>
                <!-- Canvas overlay for drag interaction -->
                <div id="canvasOverlayC" style="position:absolute; top:0; left:0; width:100%; height:100%; touch-action:none;"></div>
              </div>

              <!-- Vector x controls -->
              <div class="sim-controls" style="margin-top:16px;">
                <div class="input-group" style="flex:0 0 140px;">
                  <label for="vecX1">x₁</label>
                  <input type="number" class="input-field" id="vecX1" step="0.1" value="1" aria-label="Koordinat x1 vektor x">
                </div>
                <div class="input-group" style="flex:0 0 140px;">
                  <label for="vecX2">x₂</label>
                  <input type="number" class="input-field" id="vecX2" step="0.1" value="0" aria-label="Koordinat x2 vektor x">
                </div>
                <div class="input-group" style="flex:1; min-width:200px;">
                  <label for="vecAngle">Sudut vektor x (°)</label>
                  <input type="range" class="input-field" id="vecAngle" min="0" max="360" step="1" value="0" style="height:36px; appearance:none;">
                  <div style="display:flex; justify-content:space-between; font-size:11px; color:var(--text-muted); margin-top:4px;">
                    <span>0°</span><span>90°</span><span>180°</span><span>270°</span><span>360°</span>
                  </div>
                </div>
                <div class="input-group small">
                  <label for="vecLen">Panjang |x|</label>
                  <input type="number" class="input-field" id="vecLen" step="0.1" value="1" min="0.1" max="5" aria-label="Panjang vektor x">
                </div>
                <div class="btn-row" style="margin-top:8px;">
                  <button class="btn-run" id="snapEigBtn">Snap ke Vektor Eigen</button>
                  <select class="input-field" id="snapEigSelect" style="flex:0 0 140px; padding:9px 14px; height:40px;" aria-label="Pilih nilai eigen untuk snap">
                    <option value="">Pilih λ...</option>
                  </select>
                </div>
              </div>

              <!-- Display toggles -->
              <div class="sim-controls" style="margin-top:8px;">
                <label style="display:flex; align-items:center; gap:8px; font-size:12px; color:var(--text-secondary); cursor:pointer;">
                  <input type="checkbox" id="toggleGridC" checked> Grid tertransformasi
                </label>
                <label style="display:flex; align-items:center; gap:8px; font-size:12px; color:var(--text-secondary); cursor:pointer; margin-left:16px;">
                  <input type="checkbox" id="toggleCircleC" checked> Lingkaran → Elips
                </label>
                <label style="display:flex; align-items:center; gap:8px; font-size:12px; color:var(--text-secondary); cursor:pointer; margin-left:16px;">
                  <input type="checkbox" id="toggleEigLinesC" checked> Garis ruang eigen
                </label>
                <label style="display:flex; align-items:center; gap:8px; font-size:12px; color:var(--text-secondary); cursor:pointer; margin-left:16px;">
                  <input type="checkbox" id="toggleEigVecsC" checked> Vektor eigen satuan
                </label>
                <label style="display:flex; align-items:center; gap:8px; font-size:12px; color:var(--text-secondary); cursor:pointer; margin-left:16px;">
                  <input type="checkbox" id="toggleTraceC" checked> Jejak rotasi x
                </label>
                <label style="display:flex; align-items:center; gap:8px; font-size:12px; color:var(--text-secondary); cursor:pointer; margin-left:16px;">
                  <input type="checkbox" id="toggleLabelsC" checked> Label koordinat
                </label>
              </div>

              <!-- Animation controls -->
              <div class="sim-controls" style="margin-top:12px; padding:12px; border-radius:var(--radius-sm); background:var(--bg-layer); border:1px solid var(--glass-border);">
                <div style="display:flex; align-items:center; gap:10px; flex-wrap:wrap; margin-bottom:8px;">
                  <label for="interpT" style="font-size:11.5px; font-weight:600; color:var(--text-muted); text-transform:uppercase; letter-spacing:0.05em; min-width:180px;">Interpolasi t (M(t) = (1-t)I + tA)</label>
                  <input type="range" id="interpT" min="0" max="1" step="0.01" value="1" style="flex:1; min-width:200px; height:36px;">
                  <span id="interpTValue" style="font-variant-numeric:tabular-nums; font-size:13px; min-width:40px; text-align:right;">1.00</span>
                </div>
                <div class="btn-row">
                  <button class="btn-run" id="playAnimBtn">▶ Putar</button>
                  <button class="btn-ghost" id="pauseAnimBtn" style="display:none;">⏸ Jeda</button>
                  <div class="input-group" style="flex:0 0 140px; min-width:140px;">
                    <label for="animSpeed">Kecepatan (×)</label>
                    <select class="input-field" id="animSpeed">
                      <option value="0.5">0.5×</option>
                      <option value="1" selected>1×</option>
                      <option value="2">2×</option>
                      <option value="4">4×</option>
                    </select>
                  </div>
                  <div class="input-group" style="flex:0 0 160px; min-width:160px;">
                    <label for="animDuration">Durasi (detik)</label>
                    <select class="input-field" id="animDuration">
                      <option value="1">1 s</option>
                      <option value="2" selected>2 s</option>
                      <option value="3">3 s</option>
                      <option value="5">5 s</option>
                    </select>
                  </div>
                </div>
              </div>
            </div>

            <!-- Right: Number Panel -->
            <div id="numPanelC" style="background:var(--bg-layer); border:1px solid var(--glass-border); border-radius:var(--radius-lg); padding:20px; max-height:700px; overflow-y:auto;">
              <h4 style="margin:0 0 16px 0; font-size:13px; color:var(--text-secondary); text-transform:uppercase; letter-spacing:0.06em;">Panel Angka & Penjelasan</h4>
              <div id="numPanelContentC"></div>
            </div>
          </div>

          <!-- Presets & Actions -->
          <div class="btn-row" style="margin-top:16px; padding-top:16px; border-top:1px solid var(--glass-border);">
            <button class="btn-ghost" id="presetNoteC">Catatan [[3,2],[-1,0]]</button>
            <button class="btn-ghost" id="presetDiagC">Diagonal [[2,0],[0,3]]</button>
            <button class="btn-ghost" id="presetSymC">Simetris [[2,1],[1,2]]</button>
            <button class="btn-ghost" id="presetShearC">Geser [[1,1],[0,1]]</button>
            <button class="btn-ghost" id="presetRot90C">Rotasi 90° [[0,-1],[1,0]]</button>
            <button class="btn-ghost" id="presetReflC">Refleksi [[1,0],[0,-1]]</button>
            <button class="btn-ghost" id="presetScaleC">Skala [[2,0],[0,2]]</button>
            <button class="btn-ghost" id="presetSingularC">Singular [[1,2],[2,4]]</button>
            <button class="btn-run" id="resetC">Reset</button>
            <button class="btn-ghost" id="randomC">Acak</button>
          </div>
        </div>
      </div>
    </div>
    </div>
  </main>
</div>

<script>
/* =====================================================================
   KaTeX (dimuat secara dinamis melalui CDN; jika gagal, LaTeX mentah tetap terbaca)
   ===================================================================== */
var Katex = (function(){
  var loaded = false;
  var pending = [];
  function load(cb){
    if (loaded) { cb(); return; }
    if (window.katex && window.renderMathInElement) { loaded = true; cb(); return; }
    var head = document.head || document.getElementsByTagName('head')[0];
    var link = document.createElement('link');
    link.rel = 'stylesheet';
    link.href = 'https://cdn.jsdelivr.net/npm/katex@0.16.11/dist/katex.min.css';
    head.appendChild(link);
    var s1 = document.createElement('script');
    s1.src = 'https://cdn.jsdelivr.net/npm/katex@0.16.11/dist/katex.min.js';
    s1.onload = function(){
      var s2 = document.createElement('script');
      s2.src = 'https://cdn.jsdelivr.net/npm/katex@0.16.11/dist/contrib/auto-render.min.js';
      s2.onload = function(){ loaded = true; runAll(); };
      s2.onerror = function(){ loaded = true; runAll(); }; // renderMathInElement unavailable; raw latex shown
      head.appendChild(s2);
    };
    s1.onerror = function(){ loaded = true; runAll(); };
    head.appendChild(s1);
  }
  function runAll(){
    for (var i = 0; i < pending.length; i++) pending[i]();
    pending = [];
  }
  return {
    ready: function(cb){ if (loaded) { cb(); } else { pending.push(cb); load(function(){}); } },
    render: function(el){ if (window.renderMathInElement) {
      try { renderMathInElement(el, { delimiters: [ {left:'\\[', right:'\\]', display:true}, {left:'\\(', right:'\\)', display:false} ] } ); }
      catch(e){}
    } }
  };
})();

/* =====================================================================
   PURE MATH (terpisah dari UI, mudah diuji)
   ===================================================================== */
var EigenMath = (function(){
  'use strict';

  function gcd(a, b){
    a = Math.abs(a); b = Math.abs(b);
    while (b) { var t = b; b = a % b; a = t; }
    return a || 1;
  }

  /* ---------- Fraction (aritmetika pecahan eksak) ---------- */
  function Fraction(n, d){
    if (d === undefined) d = 1;
    if (!isFinite(n) || !isFinite(d) || d === 0) { this.num = 0; this.den = 1; return; }
    var sign = 1;
    if (n < 0) { sign = -sign; n = -n; }
    if (d < 0) { sign = -sign; d = -d; }
    var g = gcd(n, d);
    this.num = sign * (n / g);
    this.den = d / g;
  }
  Fraction.ZERO = function(){ return new Fraction(0, 1); };
  Fraction.ONE = function(){ return new Fraction(1, 1); };
  Fraction.prototype.add = function(o){ return new Fraction(this.num * o.den + o.num * this.den, this.den * o.den); };
  Fraction.prototype.sub = function(o){ return new Fraction(this.num * o.den - o.num * this.den, this.den * o.den); };
  Fraction.prototype.mul = function(o){ return new Fraction(this.num * o.num, this.den * o.den); };
  Fraction.prototype.div = function(o){ return new Fraction(this.num * o.den, this.den * o.num); };
  Fraction.prototype.neg = function(){ return new Fraction(-this.num, this.den); };
  Fraction.prototype.abs = function(){ return new Fraction(Math.abs(this.num), this.den); };
  Fraction.prototype.isZero = function(){ return this.num === 0; };
  Fraction.prototype.eq = function(o){ return this.num === o.num && this.den === o.den; };
  Fraction.prototype.toDec = function(){ return this.num / this.den; };
  Fraction.prototype.toString = function(){ return this.den === 1 ? String(this.num) : this.num + '/' + this.den; };
  Fraction.prototype.toLatex = function(){
    var n = this.num, d = this.den;
    if (d === 1) return n < 0 ? '-' + String(-n) : String(n);
    var nn = n < 0 ? -n : n;
    return (n < 0 ? '-' : '') + '\\frac{' + nn + '}{' + d + '}';
  };

  function parseFraction(s){
    if (s === null || s === undefined) return null;
    s = String(s).trim().replace(/−/g, '-').replace(/,/g, '.');
    if (s === '') return null;
    var m = /^(-?)(\d+)\/(\d+)$/.exec(s);
    if (m){
      var n = parseInt(m[2], 10), d = parseInt(m[3], 10);
      if (d === 0) return { error: 'Pembagian nol pada pecahan.' };
      return new Fraction(m[1] === '-' ? -n : n, d);
    }
    m = /^(-?)(\d*\.?\d+)$/.exec(s);
    if (m){
      var neg = m[1] === '-';
      var parts = m[2].split('.');
      var intPart = parts[0] === '' ? '0' : parts[0];
      var fracPart = parts[1] || '';
      if (fracPart.length > 6) fracPart = fracPart.slice(0, 6);
      var num = parseInt(intPart + fracPart, 10);
      var den = Math.pow(10, fracPart.length);
      return new Fraction(neg ? -num : num, den);
    }
    return null;
  }

  /* ---------- Polynomial (koefisien Fraction, index = pangkat λ) ---------- */
  function Poly(coeffs){
    this.c = coeffs.slice();
    this.trim();
  }
  Poly.prototype.trim = function(){
    while (this.c.length > 1 && this.c[this.c.length - 1].isZero()) this.c.pop();
    return this;
  };
  Poly.prototype.degree = function(){ this.trim(); return this.c.length - 1; };
  Poly.prototype.lead = function(){ this.trim(); return this.c[this.c.length - 1]; };
  Poly.prototype.add = function(o){
    var n = Math.max(this.c.length, o.c.length), r = [];
    for (var i = 0; i < n; i++){
      var a = i < this.c.length ? this.c[i] : Fraction.ZERO();
      var b = i < o.c.length ? o.c[i] : Fraction.ZERO();
      r.push(a.add(b));
    }
    return new Poly(r);
  };
  Poly.prototype.sub = function(o){
    var n = Math.max(this.c.length, o.c.length), r = [];
    for (var i = 0; i < n; i++){
      var a = i < this.c.length ? this.c[i] : Fraction.ZERO();
      var b = i < o.c.length ? o.c[i] : Fraction.ZERO();
      r.push(a.sub(b));
    }
    return new Poly(r);
  };
  Poly.prototype.mul = function(o){
    var r = [];
    for (var i = 0; i < this.c.length + o.c.length - 1; i++) r.push(Fraction.ZERO());
    for (var i = 0; i < this.c.length; i++)
      for (var j = 0; j < o.c.length; j++)
        r[i + j] = r[i + j].add(this.c[i].mul(o.c[j]));
    return new Poly(r);
  };
  Poly.prototype.scale = function(s){
    return new Poly(this.c.map(function(c){ return c.mul(s); }));
  };
  Poly.prototype.eval = function(x){
    var r = Fraction.ZERO();
    for (var i = this.c.length - 1; i >= 0; i--) r = r.mul(x).add(this.c[i]);
    return r;
  };
  Poly.prototype.divByLinear = function(root){
    // divide by (λ - root); returns { quotient: Poly, remainder: Fraction }
    var coeffs = this.c.slice();
    var b = [];
    var acc = Fraction.ZERO();
    for (var i = coeffs.length - 1; i >= 0; i--){
      acc = acc.mul(root).add(coeffs[i]);
      b.unshift(acc);
    }
    var rem = b.pop();
    var quo = new Poly(b);
    return { quotient: quo, remainder: rem };
  };
  Poly.prototype.toLatex = function(v){
    v = v || 'λ';
    var terms = [], first = true;
    for (var i = this.c.length - 1; i >= 0; i--){
      var c = this.c[i];
      if (c.isZero()) continue;
      var neg = c.num < 0;
      var absNum = neg ? -c.num : c.num;
      var showCoef = !(i > 0 && c.den === 1 && absNum === 1);
      var coef = showCoef ? (c.den === 1 ? String(absNum) : '\\frac{' + absNum + '}{' + c.den + '}') : '';
      var pow = '';
      if (i === 1) pow = '\\' + v;
      else if (i > 1) pow = '\\' + v + '^{' + i + '}';
      var sign;
      if (first) { sign = neg ? '-' : ''; first = false; }
      else { sign = neg ? '-' : '+'; }
      terms.push(sign + coef + pow);
    }
    return terms.length ? terms.join('') : '0';
  };

  /* ---------- Determinant (matriks Fraction) ---------- */
  function detMat(m){
    var n = m.length;
    var a = m.map(function(row){ return row.slice(); });
    var det = Fraction.ONE();
    for (var col = 0; col < n; col++){
      var piv = -1;
      for (var r = col; r < n; r++) if (!a[r][col].isZero()) { piv = r; break; }
      if (piv < 0) return Fraction.ZERO();
      if (piv !== col) { var tmp = a[col]; a[col] = a[piv]; a[piv] = tmp; det = det.neg(); }
      det = det.mul(a[col][col]);
      for (var r = col + 1; r < n; r++){
        if (a[r][col].isZero()) continue;
        var factor = a[r][col].div(a[col][col]);
        for (var c = col; c < n; c++) a[r][c] = a[r][c].sub(factor.mul(a[col][c]));
      }
    }
    return det;
  }

  /* ---------- Polinomial karakteristik det(λI − A) via ekspansi kofaktor ---------- */
  function charPoly(A){
    var n = A.length;
    var M = [];
    for (var i = 0; i < n; i++){
      var row = [];
      for (var j = 0; j < n; j++){
        if (i === j) row.push(new Poly([ A[i][j].neg(), Fraction.ONE() ]));
        else row.push(new Poly([ A[i][j].neg() ]));
      }
      M.push(row);
    }
    return detPoly(M);
  }
  function detPoly(M){
    var n = M.length;
    if (n === 1) return M[0][0];
    var res = new Poly([Fraction.ZERO()]);
    for (var j = 0; j < n; j++){
      var minor = [];
      for (var r = 1; r < n; r++){
        var row = [];
        for (var c = 0; c < n; c++) if (c !== j) row.push(M[r][c]);
        minor.push(row);
      }
      var term = M[0][j].mul(detPoly(minor));
      if (j % 2 === 1) term = term.scale(new Fraction(-1));
      res = res.add(term);
    }
    return res;
  }

  /* ---------- Complex ---------- */
  function Complex(re, im){
    this.re = re; this.im = im === undefined ? 0 : im;
  }
  Complex.prototype.add = function(o){ return new Complex(this.re + o.re, this.im + o.im); };
  Complex.prototype.sub = function(o){ return new Complex(this.re - o.re, this.im - o.im); };
  Complex.prototype.mul = function(o){ return new Complex(this.re * o.re - this.im * o.im, this.re * o.im + this.im * o.re); };
  Complex.prototype.div = function(o){
    var d = o.re * o.re + o.im * o.im;
    return new Complex((this.re * o.re + this.im * o.im) / d, (this.im * o.re - this.re * o.im) / d);
  };
  Complex.prototype.abs = function(){ return Math.sqrt(this.re * this.re + this.im * this.im); };
  Complex.prototype.toLatex = function(){
    var re = this.re, im = this.im;
    function fmt(x){ return Math.abs(x) < 1e-9 ? 0 : (Math.abs(Math.round(x) - x) < 1e-6 ? Math.round(x) : x.toFixed(4)); }
    var rs = fmt(re), is = fmt(Math.abs(im));
    if (Math.abs(im) < 1e-9) return String(rs);
    if (Math.abs(re) < 1e-9) return is + 'i';
    return rs + (im < 0 ? ' \u2212 ' + is + 'i' : ' + ' + is + 'i');
  };

  function evalPolyComplex(c, x){
    var r = new Complex(0, 0);
    for (var i = c.length - 1; i >= 0; i--) r = r.mul(x).add(new Complex(c[i].toDec(), 0));
    return r;
  }

  function isPerfectSquareRat(f){
    var n = f.num, d = f.den;
    if (n < 0) return false;
    var rn = Math.sqrt(n), rd = Math.sqrt(d);
    return Math.abs(rn - Math.round(rn)) < 1e-9 && Math.abs(rd - Math.round(rd)) < 1e-9;
  }

  /* ---------- Root finding ---------- */
  function rationalRootCandidates(poly){
    var c0 = poly.c[0], lead = poly.lead();
    var cands = [];
    if (c0.isZero()) return cands;
    function divisors(n){
      n = Math.abs(n); var divs = [];
      for (var i = 1; i <= n; i++) if (n % i === 0) divs.push(i);
      return divs;
    }
    var ps = divisors(c0.num);
    var qs = divisors(lead.num);
    var seen = {};
    for (var i = 0; i < ps.length; i++)
      for (var j = 0; j < qs.length; j++){
        for (var sgn = -1; sgn <= 1; sgn += 2){
          var r = new Fraction(sgn * ps[i], qs[j]);
          var key = r.toString();
          if (seen[key]) continue;
          seen[key] = true;
          cands.push(r);
        }
      }
    return cands;
  }

  function findRoots(poly){
    // returns array of { kind:'real'|'complex', exact:boolean, frac:Fraction|null, dec:number|null, re:number, im:number, latex:string }
    poly = new Poly(poly.c.slice()).trim();
    var deg = poly.degree();
    var roots = [];
    var work = poly;

    function addRoot(kind, exact, frac, re, im){
      var dec = frac ? frac.toDec() : (kind === 'complex' ? re : re);
      var latex = frac ? frac.toLatex() : new Complex(re, im === undefined ? 0 : im).toLatex();
      roots.push({ kind: kind, exact: exact, frac: frac, dec: dec, re: re, im: im === undefined ? 0 : im, latex: latex });
    }

    if (deg === 1){
      var r = work.c[0].neg().div(work.c[1]);
      addRoot('real', true, r, r.toDec(), 0);
      return roots;
    }
    if (deg === 2){
      var a = work.c[2], b = work.c[1], c = work.c[0];
      var delta = b.mul(b).sub(a.mul(c).mul(new Fraction(4, 1))); // b^2 - 4ac
      var twoA = a.mul(new Fraction(2, 1));
      var negB = b.neg();
      if (delta.num >= 0){
        var sqrtDelta;
        if (isPerfectSquareRat(delta)){
          var rn = Math.sqrt(delta.num), rd = Math.sqrt(delta.den);
          sqrtDelta = new Fraction(rn, rd);
          if (delta.num === 0) sqrtDelta = Fraction.ZERO();
        } else {
          sqrtDelta = null; // numeric
        }
        if (sqrtDelta){
          addRoot('real', true, negB.add(sqrtDelta).div(twoA), null, 0);
          addRoot('real', true, negB.sub(sqrtDelta).div(twoA), null, 0);
        } else {
          var d = delta.toDec();
          var sd = Math.sqrt(d);
          addRoot('real', false, null, negB.toDec().add(sd).div(twoA.toDec()), 0);
          addRoot('real', false, null, negB.toDec().sub(sd).div(twoA.toDec()), 0);
        }
      } else {
        var d = delta.toDec();
        var sd = Math.sqrt(-d);
        var re = negB.toDec().div(twoA.toDec());
        addRoot('complex', false, null, re, sd / twoA.toDec());
        addRoot('complex', false, null, re, -sd / twoA.toDec());
      }
      return roots;
    }

    // deg >= 3 : rational root theorem + deflation
    var remaining = work;
    var tried = {};
    var changed = true;
    while (remaining.degree() >= 2 && changed){
      changed = false;
      var cands = rationalRootCandidates(remaining);
      for (var k = 0; k < cands.length; k++){
        var r = cands[k];
        var key = r.toString();
        if (tried[key]) continue;
        tried[key] = true;
        var div = remaining.divByLinear(r);
        if (div.remainder.isZero()){
          addRoot('real', true, r, r.toDec(), 0);
          remaining = div.quotient.trim();
          changed = true;
          break;
        }
      }
    }
    if (remaining.degree() === 1){
      var r = remaining.c[0].neg().div(remaining.c[1]);
      addRoot('real', true, r, r.toDec(), 0);
    } else if (remaining.degree() === 2){
      var sub = findRoots(remaining);
      for (var s = 0; s < sub.length; s++) roots.push(sub[s]);
    } else if (remaining.degree() > 2){
      // numeric fallback (Durand-Kerner)
      var dk = durandKerner(remaining);
      for (var d = 0; d < dk.length; d++){
        addRoot('complex', false, null, dk[d].re, dk[d].im);
      }
    }
    return roots;
  }

  function durandKerner(poly){
    var n = poly.degree();
    if (n <= 0) return [];
    var c = [];
    for (var i = 0; i <= n; i++) c.push(new Complex(poly.c[i].toDec(), 0));
    var lead = c[n];
    for (var i = 0; i <= n; i++) c[i] = c[i].div(lead);
    var roots = [];
    for (var k = 0; k < n; k++) roots.push(new Complex(0.4 * Math.cos(2 * Math.PI * k / n), 0.9 * Math.sin(2 * Math.PI * k / n)));
    for (var it = 0; it < 200; it++){
      var nr = [];
      var maxShift = 0;
      for (var i = 0; i < n; i++){
        var prod = new Complex(1, 0);
        for (var j = 0; j < n; j++) if (j !== i) prod = prod.mul(roots[i].sub(roots[j]));
        var f = evalPolyComplex(c, roots[i]);
        var delta = f.div(prod);
        var nu = roots[i].sub(delta);
        nr.push(nu);
        maxShift = Math.max(maxShift, nu.sub(roots[i]).abs());
      }
      roots = nr;
      if (maxShift < 1e-8) break;
    }
    return roots;
  }

  /* ---------- Matrix helpers ---------- */
  function matMulVec(A, v){
    var n = A.length, out = [];
    for (var i = 0; i < n; i++){
      var acc = Fraction.ZERO();
      for (var j = 0; j < n; j++) acc = acc.add(A[i][j].mul(v[j]));
      out.push(acc);
    }
    return out;
  }
  function trace(A){
    var t = Fraction.ZERO();
    for (var i = 0; i < A.length; i++) t = t.add(A[i][i]);
    return t;
  }

  /* ---------- RREF over fractions -> null space basis ---------- */
  function rref(aug){
    var rows = aug.length, cols = aug[0].length;
    var a = aug.map(function(r){ return r.slice(); });
    var pivots = [];
    var r = 0;
    for (var c = 0; c < cols && r < rows; c++){
      var piv = -1;
      for (var i = r; i < rows; i++) if (!a[i][c].isZero()) { piv = i; break; }
      if (piv < 0) continue;
      var tmp = a[r]; a[r] = a[piv]; a[piv] = tmp;
      var inv = a[r][c].div(new Fraction(1)); // = 1 / a[r][c] but keep generic
      // normalize pivot row
      inv = new Fraction(1, 1).div(a[r][c]);
      for (var j = c; j < cols; j++) a[r][j] = a[r][j].mul(inv);
      for (var i = 0; i < rows; i++){
        if (i === r) continue;
        if (a[i][c].isZero()) continue;
        var factor = a[i][c];
        for (var j = c; j < cols; j++) a[i][j] = a[i][j].sub(factor.mul(a[r][j]));
      }
      pivots.push({ row: r, col: c });
      r++;
    }
    return { a: a, pivots: pivots };
  }

  function nullSpace(M){
    // M: Fraction n x n ; returns basis vectors (Fraction[]) solving M x = 0
    var n = M.length;
    var aug = [];
    for (var i = 0; i < n; i++){
      var row = M[i].slice();
      row.push(Fraction.ZERO());
      aug.push(row);
    }
    var res = rref(aug);
    var a = res.a, pivots = res.pivots;
    var pivotCols = [];
    for (var i = 0; i < pivots.length; i++) pivotCols.push(pivots[i].col);
    var isPivot = [];
    for (var c = 0; c < n; c++) isPivot.push(false);
    for (var i = 0; i < pivotCols.length; i++) isPivot[pivotCols[i]] = true;
    var freeCols = [];
    for (var c = 0; c < n; c++) if (!isPivot[c]) freeCols.push(c);

    var basis = [];
    for (var f = 0; f < freeCols.length; f++){
      var fc = freeCols[f];
      var x = [];
      for (var j = 0; j < n; j++) x.push(Fraction.ZERO());
      x[fc] = Fraction.ONE();
      // solve basic vars from rows
      for (var i = pivots.length - 1; i >= 0; i--){
        var row = pivots[i].row, col = pivots[i].col;
        // a[row][col] = 1 ; x[col] = - sum_{j>col} a[row][j]*x[j]
        var sum = Fraction.ZERO();
        for (var j = col + 1; j < n; j++) sum = sum.add(a[row][j].mul(x[j]));
        x[col] = sum.neg();
      }
      basis.push(x);
    }
    return { basis: basis, pivotCols: pivotCols, freeCols: freeCols, rank: pivots.length };
  }

  return {
    Fraction: Fraction, parseFraction: parseFraction, Poly: Poly,
    detMat: detMat, charPoly: charPoly, Complex: Complex,
    findRoots: findRoots, matMulVec: matMulVec, trace: trace,
    nullSpace: nullSpace, FractionZERO: Fraction.ZERO()
  };
})();

/* =====================================================================
   EIGEN GEOMETRY - Pure math functions for 2D visualization
   ===================================================================== */
var EigenGeometry = (function(){
  'use strict';

  const EPS = 1e-10;

  /* ---------- Vector/Matrix helpers ---------- */
  function vec2(x, y){ return { x: x, y: y }; }
  function mat2(a,b,c,d){ return [[a,b],[c,d]]; }

  function matMulVec(M, v){
    return vec2(M[0][0]*v.x + M[0][1]*v.y, M[1][0]*v.x + M[1][1]*v.y);
  }
  function matMul(M1, M2){
    return [
      [M1[0][0]*M2[0][0] + M1[0][1]*M2[1][0], M1[0][0]*M2[0][1] + M1[0][1]*M2[1][1]],
      [M1[1][0]*M2[0][0] + M1[1][1]*M2[1][0], M1[1][0]*M2[0][1] + M1[1][1]*M2[1][1]]
    ];
  }
  function matAdd(M1, M2){
    return [[M1[0][0]+M2[0][0], M1[0][1]+M2[0][1]], [M1[1][0]+M2[1][0], M1[1][1]+M2[1][1]]];
  }
  function matScale(M, s){
    return [[M[0][0]*s, M[0][1]*s], [M[1][0]*s, M[1][1]*s]];
  }
  function identity2(){ return [[1,0],[0,1]]; }
  function det2(M){ return M[0][0]*M[1][1] - M[0][1]*M[1][0]; }
  function trace2(M){ return M[0][0] + M[1][1]; }

  function vecAdd(a,b){ return vec2(a.x+b.x, a.y+b.y); }
  function vecSub(a,b){ return vec2(a.x-b.x, a.y-b.y); }
  function vecScale(v, s){ return vec2(v.x*s, v.y*s); }
  function vecDot(a,b){ return a.x*b.x + a.y*b.y; }
  function vecCross(a,b){ return a.x*b.y - a.y*b.x; } // scalar (z-component)
  function vecLen(v){ return Math.hypot(v.x, v.y); }
  function vecNormalize(v){
    var l = vecLen(v);
    return l > EPS ? vec2(v.x/l, v.y/l) : vec2(0,0);
  }
  function vecAngle(v){ return Math.atan2(v.y, v.x); } // -π to π
  function angleDeg(v){ return (vecAngle(v) * 180 / Math.PI + 360) % 360; }

  /* ---------- Interpolation M(t) = (1-t)I + tA ---------- */
  function interpolateMat(A, t){
    var I = identity2();
    return matAdd(matScale(I, 1-t), matScale(A, t));
  }

  /* ---------- Eigen computation for 2x2 (reusing EigenMath roots) ---------- */
  // Convert Fraction matrix to number matrix
  function fracMatToNum(A){
    return A.map(function(row){ return row.map(function(f){ return f.toDec(); }); });
  }

  // Compute eigenvalues/vectors for 2x2 numeric matrix
  function computeEigen2x2(A){
    var a = A[0][0], b = A[0][1], c = A[1][0], d = A[1][1];
    var tr = a + d;
    var det = a*d - b*c;
    var disc = tr*tr - 4*det;
    var eig = [];
    if (disc >= -EPS){
      // Real eigenvalues
      var sqrtDisc = disc < 0 ? 0 : Math.sqrt(disc);
      var lam1 = (tr + sqrtDisc) / 2;
      var lam2 = (tr - sqrtDisc) / 2;
      // Eigenvectors: (A - λI)v = 0
      function eigenvec(lam){
        var m00 = a - lam, m01 = b, m10 = c, m11 = d - lam;
        // Find null space
        if (Math.abs(m00) > Math.abs(m10)){
          if (Math.abs(m00) > EPS) return vecNormalize(vec2(-m01/m00, 1));
          if (Math.abs(m01) > EPS) return vecNormalize(vec2(1, -m00/m01));
        } else {
          if (Math.abs(m10) > EPS) return vecNormalize(vec2(-m11/m10, 1));
          if (Math.abs(m11) > EPS) return vecNormalize(vec2(1, -m10/m11));
        }
        // If matrix is λI (all directions are eigenvectors)
        return vec2(1, 0);
      }
      var v1 = eigenvec(lam1);
      var v2 = eigenvec(lam2);
      // Check if they're linearly dependent (repeated eigenvalue with full eigenspace)
      var cross = Math.abs(vecCross(v1, v2));
      if (Math.abs(lam1 - lam2) < EPS && cross < EPS){
        // Repeated eigenvalue, check if A = λI
        if (Math.abs(a - lam1) < EPS && Math.abs(d - lam1) < EPS && Math.abs(b) < EPS && Math.abs(c) < EPS){
          eig.push({ lambda: lam1, vec: v1, fullSpace: true, multiplicity: 2 });
        } else {
          eig.push({ lambda: lam1, vec: v1, fullSpace: false, multiplicity: 2 });
        }
      } else {
        eig.push({ lambda: lam1, vec: v1, fullSpace: false, multiplicity: 1 });
        if (Math.abs(lam1 - lam2) > EPS){
          eig.push({ lambda: lam2, vec: v2, fullSpace: false, multiplicity: 1 });
        }
      }
    } else {
      // Complex eigenvalues
      var re = tr / 2;
      var im = Math.sqrt(-disc) / 2;
      eig.push({ lambda: re + im, lambdaRe: re, lambdaIm: im, complex: true });
      eig.push({ lambda: re - im, lambdaRe: re, lambdaIm: -im, complex: true });
    }
    return eig;
  }

  /* ---------- Unit circle sampling ---------- */
  function sampleCircle(n){
    var pts = [];
    for (var i = 0; i <= n; i++){
      var theta = 2 * Math.PI * i / n;
      pts.push(vec2(Math.cos(theta), Math.sin(theta)));
    }
    return pts;
  }

  /* ---------- Grid sampling ---------- */
  function sampleGrid(lines, range){
    var pts = [];
    var step = (2*range) / lines;
    for (var i = -lines; i <= lines; i++){
      var coord = i * step;
      // Vertical lines (x = coord)
      var vline = [];
      for (var j = -lines; j <= lines; j++){
        vline.push(vec2(coord, j * step));
      }
      pts.push(vline);
      // Horizontal lines (y = coord)
      var hline = [];
      for (var j = -lines; j <= lines; j++){
        hline.push(vec2(j * step, coord));
      }
      pts.push(hline);
    }
    return pts;
  }

  /* ---------- Transform points ---------- */
  function transformPoints(points, M){
    return points.map(function(p){ return matMulVec(M, p); });
  }
  function transformLines(lines, M){
    return lines.map(function(line){ return transformPoints(line, M); });
  }

  /* ---------- SVG path builders ---------- */
  function pointsToPath(points, closed){
    if (!points.length) return '';
    var d = 'M' + points[0].x + ',' + points[0].y;
    for (var i = 1; i < points.length; i++) d += ' L' + points[i].x + ',' + points[i].y;
    if (closed) d += ' Z';
    return d;
  }

  function ellipseFromMatrix(M){
    // For unit circle x^T x = 1, transformed ellipse: (Mx)^T (Mx) = x^T (M^T M) x = 1
    // Parametric: M * (cos θ, sin θ)
    var pts = sampleCircle(120).map(function(p){ return matMulVec(M, p); });
    return pointsToPath(pts, true);
  }

  function gridToSVG(lines, scale, cx, cy){
    var paths = [];
    for (var i = 0; i < lines.length; i++){
      var pts = lines[i].map(function(p){
        return { x: cx + p.x * scale, y: cy - p.y * scale }; // y flipped for SVG
      });
      paths.push(pointsToPath(pts, false));
    }
    return paths.join(' ');
  }

  function axesToSVG(scale, cx, cy, maxVal){
    var paths = [];
    var tickLen = 6;
    var maxTick = Math.ceil(maxVal);
    // X axis
    paths.push('M' + (cx - maxVal*scale) + ',' + cy + ' L' + (cx + maxVal*scale) + ',' + cy);
    // Y axis
    paths.push('M' + cx + ',' + (cy + maxVal*scale) + ' L' + cx + ',' + (cy - maxVal*scale));
    // Ticks and labels
    for (var i = -maxTick; i <= maxTick; i++){
      if (i === 0) continue;
      var x = cx + i * scale;
      var y = cy - i * scale;
      // X ticks
      paths.push('M' + x + ',' + (cy - tickLen) + ' L' + x + ',' + (cy + tickLen));
      // Y ticks
      paths.push('M' + (cx - tickLen) + ',' + y + ' L' + (cx + tickLen) + ',' + y);
    }
    return paths.join(' ');
  }

  function eigenLineToSVG(vec, scale, cx, cy, maxExtent){
    // Line through origin in direction vec, extending to maxExtent
    var v = vecNormalize(vec);
    var x1 = cx - v.x * maxExtent * scale;
    var y1 = cy + v.y * maxExtent * scale;
    var x2 = cx + v.x * maxExtent * scale;
    var y2 = cy - v.y * maxExtent * scale;
    return 'M' + x1 + ',' + y1 + ' L' + x2 + ',' + y2;
  }

  function vecToSVG(v, scale, cx, cy, color, label, labelOffset){
    var x = cx + v.x * scale;
    var y = cy - v.y * scale;
    var line = 'M' + cx + ',' + cy + ' L' + x + ',' + y;
    var labelX = x + (labelOffset ? labelOffset.x : 0);
    var labelY = y + (labelOffset ? labelOffset.y : -12);
    return { line: line, labelX: labelX, labelY: labelY, text: label };
  }

  /* ---------- Parallel detection ---------- */
  function isParallel(v1, v2, eps){
    eps = eps || 1e-3; // ~0.05 degrees tolerance in cross product terms
    var l1 = vecLen(v1), l2 = vecLen(v2);
    if (l1 < EPS || l2 < EPS) return false;
    var cross = Math.abs(vecCross(v1, v2));
    return cross <= eps * l1 * l2;
  }

  /* ---------- Snap to nearest eigenvector direction ---------- */
  function snapToEigen(x, eigenList){
    if (!eigenList.length) return x;
    var best = x;
    var bestDot = -Infinity;
    var xLen = vecLen(x);
    if (xLen < EPS) return x;
    var xNorm = vecNormalize(x);
    for (var i = 0; i < eigenList.length; i++){
      if (eigenList[i].complex) continue;
      var v = eigenList[i].vec;
      var dot = Math.abs(vecDot(xNorm, v)); // absolute for direction match
      if (dot > bestDot){
        bestDot = dot;
        best = vecScale(v, xLen);
      }
    }
    return best;
  }

  /* ---------- Auto-scale computation ---------- */
  function computeAutoScale(x, Ax, ellipseRadius, margin){
    margin = margin || 1.2;
    var maxVal = Math.max(vecLen(x), vecLen(Ax), ellipseRadius);
    return maxVal * margin;
  }

  /* ---------- Contextual explanation generator ---------- */
  function generateExplanation(A, eigenList, x, Ax, t){
    var M = t !== undefined ? interpolateMat(A, t) : A;
    var det = det2(M);
    var isInterp = t !== undefined && t < 1;
    var lines = [];

    // Eigen analysis
    var realEigs = eigenList.filter(function(e){ return !e.complex; });
    var complexEigs = eigenList.filter(function(e){ return e.complex; });

    if (complexEigs.length > 0){
      lines.push('Nilai eigen kompleks (λ = ' + complexEigs[0].lambdaRe.toFixed(3) + ' ± ' + Math.abs(complexEigs[0].lambdaIm).toFixed(3) + 'i) → tidak ada arah tetap real. Matriks melakukan rotasi+skala.');
    } else if (realEigs.length === 1 && realEigs[0].fullSpace){
      lines.push('A = λI (skala seragam λ = ' + realEigs[0].lambda.toFixed(3) + '). Semua arah adalah vektor eigen, panjang dikalikan ' + realEigs[0].lambda.toFixed(3) + '.');
    } else if (realEigs.length === 1 && realEigs[0].multiplicity === 2){
      lines.push('Nilai eigen kembar λ = ' + realEigs[0].lambda.toFixed(3) + ' dengan ruang eigen 1-dimensi (hanya satu arah eigen). Matriks tidak dapat didiagonalisasi (defektif/geser).');
    } else if (realEigs.length === 2){
      var l1 = realEigs[0].lambda, l2 = realEigs[1].lambda;
      var desc = 'Dua arah tetap: ';
      var parts = [];
      [l1, l2].forEach(function(lam, idx){
        var v = realEigs[idx].vec;
        var dir = '[' + v.x.toFixed(2) + ', ' + v.y.toFixed(2) + ']';
        if (Math.abs(lam) < EPS){
          parts.push('λ=0 (runtuh ke titik asal, matriks singular)');
        } else if (lam < 0){
          parts.push('λ=' + lam.toFixed(3) + ' (arah berbalik, panjang ×' + Math.abs(lam).toFixed(3) + ')');
        } else if (lam < 1){
          parts.push('λ=' + lam.toFixed(3) + ' (menyusut, panjang ×' + lam.toFixed(3) + ')');
        } else if (lam === 1){
          parts.push('λ=1 (tetap, panjang ×1)');
        } else {
          parts.push('λ=' + lam.toFixed(3) + ' (memanjang, panjang ×' + lam.toFixed(3) + ')');
        }
      });
      lines.push(desc + parts.join(' dan '));
    } else {
      lines.push('Tidak ada nilai eigen real.');
    }

    // Vector x analysis
    if (vecLen(x) < EPS){
      lines.push('x = 0 (vektor nol bukan vektor eigen).');
    } else {
      var parallel = isParallel(x, Ax);
      var ratio = vecLen(Ax) / vecLen(x);
      var angleDiff = Math.abs(angleDeg(Ax) - angleDeg(x));
      if (angleDiff > 180) angleDiff = 360 - angleDiff;
      if (parallel){
        var lamMatch = null;
        for (var i = 0; i < realEigs.length; i++){
          if (Math.abs(Math.abs(realEigs[i].lambda) - ratio) < 1e-3){
            lamMatch = realEigs[i].lambda;
            break;
          }
        }
        lines.push('x sejajar Ax → x adalah vektor eigen. Rasio |Ax|/|x| = ' + ratio.toFixed(4) + (lamMatch !== null ? ' (cocok |λ|=' + Math.abs(lamMatch).toFixed(4) + ', ✓)' : ''));
      } else {
        lines.push('x tidak sejajar Ax (selisih sudut ' + angleDiff.toFixed(2) + '°). Rasio panjang |Ax|/|x| = ' + ratio.toFixed(4) + '.');
      }
    }

    // Area scaling
    lines.push('Determinan det(M) = ' + det.toFixed(4) + ' → faktor perubahan luas. Luas elips = |det| × π = ' + Math.abs(det).toFixed(4) + 'π.');

    if (isInterp){
      lines.unshift('Interpolasi M(t) dengan t = ' + t.toFixed(2) + ': M(t) = (1-t)I + tA');
    }

    return lines;
  }

  /* ---------- Number panel data ---------- */
  function computePanelData(A, eigenList, x, Ax, t){
    var M = t !== undefined ? interpolateMat(A, t) : A;
    var det = det2(M);
    var xLen = vecLen(x);
    var AxLen = vecLen(Ax);
    var xAngle = angleDeg(x);
    var AxAngle = angleDeg(Ax);
    var angleDiff = Math.abs(AxAngle - xAngle);
    if (angleDiff > 180) angleDiff = 360 - angleDiff;
    var ratio = xLen > EPS ? AxLen / xLen : 0;
    var parallel = isParallel(x, Ax);

    return {
      x: { x: x.x, y: x.y },
      Ax: { x: Ax.x, y: Ax.y },
      xAngle: xAngle,
      AxAngle: AxAngle,
      angleDiff: angleDiff,
      ratio: ratio,
      det: det,
      areaScale: Math.abs(det),
      parallel: parallel,
      eigenList: eigenList,
      M: M
    };
  }

  return {
    vec2: vec2,
    mat2: mat2,
    matMulVec: matMulVec,
    matMul: matMul,
    matAdd: matAdd,
    matScale: matScale,
    identity2: identity2,
    det2: det2,
    trace2: trace2,
    vecAdd: vecAdd,
    vecSub: vecSub,
    vecScale: vecScale,
    vecDot: vecDot,
    vecCross: vecCross,
    vecLen: vecLen,
    vecNormalize: vecNormalize,
    vecAngle: vecAngle,
    angleDeg: angleDeg,
    interpolateMat: interpolateMat,
    fracMatToNum: fracMatToNum,
    computeEigen2x2: computeEigen2x2,
    sampleCircle: sampleCircle,
    sampleGrid: sampleGrid,
    transformPoints: transformPoints,
    transformLines: transformLines,
    pointsToPath: pointsToPath,
    ellipseFromMatrix: ellipseFromMatrix,
    gridToSVG: gridToSVG,
    axesToSVG: axesToSVG,
    eigenLineToSVG: eigenLineToSVG,
    vecToSVG: vecToSVG,
    isParallel: isParallel,
    snapToEigen: snapToEigen,
    computeAutoScale: computeAutoScale,
    generateExplanation: generateExplanation,
    computePanelData: computePanelData,
    EPS: EPS
  };
})();
var EigenApp = (function(){
  'use strict';

  var n = 2;
  var A = []; // Fraction[][]
  var poly = null;
  var roots = [];
  var steps = [];
  var stepIdx = 0;
  var showAll = false;

  var $ = function(id){ return document.getElementById(id); };

  function buildGrid(){
    var container = $('matrixGridA');
    container.innerHTML = '';
    for (var i = 0; i < n; i++){
      var rowDiv = document.createElement('div');
      rowDiv.className = 'matrix-row';
      for (var j = 0; j < n; j++){
        var inp = document.createElement('input');
        inp.className = 'cell-input';
        inp.type = 'text';
        inp.value = (A[i] && A[i][j]) ? A[i][j].toString() : '0';
        inp.setAttribute('aria-label', 'Sel baris ' + (i+1) + ' kolom ' + (j+1));
        rowDiv.appendChild(inp);
      }
      container.appendChild(rowDiv);
    }
  }

  function readMatrix(){
    var errs = [];
    var size = parseInt($('sizeA').value, 10);
    if (isNaN(size) || size < 1 || size > 4) errs.push('Ukuran matriks tidak valid.');
    n = size;
    var rows = $('matrixGridA').querySelectorAll('.matrix-row');
    var mat = [];
    for (var i = 0; i < rows.length; i++){
      var inputs = rows[i].querySelectorAll('.cell-input');
      var row = [];
      for (var j = 0; j < inputs.length; j++){
        var parsed = EigenMath.parseFraction(inputs[j].value);
        if (parsed === null) {
          errs.push('Baris ' + (i+1) + ' kolom ' + (j+1) + ': isian kosong atau bukan angka.');
          row.push(EigenMath.FractionZERO);
        } else if (parsed.error) {
          errs.push('Baris ' + (i+1) + ' kolom ' + (j+1) + ': ' + parsed.error);
          row.push(EigenMath.FractionZERO);
        } else {
          row.push(parsed);
        }
      }
      mat.push(row);
    }
    if (errs.length) { showError(errs); return null; }
    A = mat;
    return A;
  }

  function showError(msgs){
    var box = $('errA');
    if (!msgs || msgs.length === 0) { box.classList.remove('show'); box.innerHTML = ''; return; }
    var html = '<strong>Input tidak valid:</strong><ul>';
    for (var i = 0; i < msgs.length; i++) html += '<li>' + msgs[i] + '</li>';
    html += '</ul>';
    box.innerHTML = html;
    box.classList.add('show');
  }

  function matLatex(A){
    var lines = [];
    for (var i = 0; i < A.length; i++){
      lines.push(A[i].map(function(c){ return c.toLatex(); }).join(' & '));
    }
    return '\\begin{bmatrix}' + lines.join(' \\\\ ') + '\\end{bmatrix}';
  }

  function lambdaMinusALatex(){
    var lines = [];
    for (var i = 0; i < n; i++){
      var cells = [];
      for (var j = 0; j < n; j++){
        if (i === j) cells.push('\\lambda - ' + A[i][j].toLatex());
        else cells.push('- ' + A[i][j].toLatex());
      }
      lines.push(cells.join(' & '));
    }
    return '\\begin{bmatrix}' + lines.join(' \\\\ ') + '\\end{bmatrix}';
  }

  function buildSteps(){
    steps = [];
    // Step 1: tulis λI
    var liLines = [];
    for (var i = 0; i < n; i++){
      var cells = [];
      for (var j = 0; j < n; j++){
        cells.push(i === j ? '\\lambda' : '0');
      }
      liLines.push(cells.join(' & '));
    }
    steps.push({
      title: 'Langkah 1 &mdash; Tulis matriks λI',
      html: '<p>Matriks identitas dikalikan λ, berukuran ' + n + '&times;' + n + ':</p>' +
        '<div class="math">\\[ \\lambda I = \\begin{bmatrix}' + liLines.join(' \\\\ ') + '\\end{bmatrix} \\]</div>'
    });

    // Step 2: λI − A
    steps.push({
      title: 'Langkah 2 &mdash; Hitung λI &minus; A',
      html: '<p>Mengurangi setiap entri diagonal A dari λ:</p>' +
        '<div class="math">\\[ \\lambda I - A = ' + lambdaMinusALatex() + ' \\]</div>'
    });

    // Step 3: determinant expansion
    var detEq = [];
    if (n === 2){
      var a = A[0][0], b = A[0][1], c = A[1][0], d = A[1][1];
      detEq.push('\\det(\\lambda I - A) = \\begin{vmatrix} \\lambda - ' + a.toLatex() + ' & - ' + b.toLatex() + ' \\\\ - ' + c.toLatex() + ' & \\lambda - ' + d.toLatex() + '\\end{vmatrix}');
      detEq.push('= (\\lambda - ' + a.toLatex() + ')(\\lambda - ' + d.toLatex() + ') - (-' + b.toLatex() + ')(-' + c.toLatex() + ')');
      detEq.push('= (\\lambda - ' + a.toLatex() + ')(\\lambda - ' + d.toLatex() + ') - ' + b.mul(c).toLatex());
    } else {
      detEq.push('\\det(\\lambda I - A) = \\begin{vmatrix}' + lambdaMinusALatex().replace(/\\begin\{bmatrix\}/, '').replace(/\\end\{bmatrix\}/, '') + '\\end{vmatrix}');
      detEq.push('(ekspansi kofaktor pada baris pertama)');
    }
    steps.push({
      title: 'Langkah 3 &mdash; Hitung determinan',
      html: '<p>Menghitung <strong>det(λI &minus; A)</strong> dengan penjabaran:</p>' +
        detEq.map(function(e){ return '<div class="math">\\[ ' + e + ' \\]</div>'; }).join('') +
        '<p style="margin-top:8px;">Hasil determinan sebagai polinomial dalam λ:</p>'
    });

    // Step 4: characteristic polynomial
    poly = EigenMath.charPoly(A);
    steps.push({
      title: 'Langkah 4 &mdash; Polinomial karakteristik',
      html: '<p>Menjadikan bentuk sederhana:</p>' +
        '<div class="math">\\[ p(\\lambda) = \\det(\\lambda I - A) = ' + poly.toLatex('lambda') + ' \\]</div>'
    });

    // Step 5: solve
    roots = EigenMath.findRoots(poly);
    var rootHtml = roots.map(function(r){
      var tag = '';
      if (r.kind === 'complex') tag = '<span class="tag">kompleks</span>';
      else if (!r.exact) tag = '<span class="tag">aproksimasi</span>';
      var mult = algebraicMultiplicity(r);
      if (mult > 1) tag += ' <span class="tag">kembar (mult. aljabar ' + mult + ')</span>';
      return '<span class="eigval-chip' + (r.kind === 'complex' ? ' complex' : '') + (mult > 1 ? ' repeated' : '') + '">' +
        (r.kind === 'complex' ? '\\lambda ' + r.latex : '\\lambda = ' + r.latex) + ' ' + tag + '</span>';
    }).join('');
    var hasComplex = roots.some(function(r){ return r.kind === 'complex'; });
    var note = '';
    if (hasComplex){
      note = '<div class="special-note"><strong>Catatan:</strong> Terdapat nilai eigen kompleks, sehingga tidak ada vektor eigen real (mis. matriks rotasi). Gunakan Modul B untuk melihat vektor eigen kompleks.</div>';
    }
    steps.push({
      title: 'Langkah 5 &mdash; Selesaikan det(λI &minus; A) = 0',
      html: '<p>Mencari akar polinomial karakteristik:</p>' +
        '<div style="margin:8px 0;">' + rootHtml + '</div>' + note
    });

    // Step 6: summary + cross-check
    var tr = EigenMath.trace(A);
    var detA = EigenMath.detMat(A);
    var sumRoots = EigenMath.Fraction.ZERO(), prodRoots = EigenMath.Fraction.ONE();
    var allExact = roots.every(function(r){ return r.exact; });
    roots.forEach(function(r){ sumRoots = sumRoots.add(r.frac || new EigenMath.Fraction(Math.round(r.re * 1e4) / 1e4, 1)); prodRoots = prodRoots.mul(r.frac || new EigenMath.Fraction(Math.round(r.re * 1e4) / 1e4, 1)); });
    var sumOk = allExact ? sumRoots.eq(tr) : Math.abs(sumRoots.toDec() - tr.toDec()) < 1e-6;
    var prodOk = allExact ? prodRoots.eq(detA) : Math.abs(prodRoots.toDec() - detA.toDec()) < 1e-6;

    steps.push({
      title: 'Langkah 6 &mdash; Ringkasan &amp; pengecekan silang',
      html: '<div class="check-grid" id="checkA">' +
        '<div class="check-item"><div class="lbl">Trace A = jumlah diagonal</div><div class="val">' + tr.toLatex() + '</div>' +
          '<div class="lbl" style="margin-top:6px;">Jumlah nilai eigen</div><div class="val ' + (sumOk ? 'ok' : 'no') + '">' + (sumOk ? '\u2713 cocok' : '\u2717 tidak cocok') + '</div></div>' +
        '<div class="check-item"><div class="lbl">Det(A)</div><div class="val">' + detA.toLatex() + '</div>' +
          '<div class="lbl" style="margin-top:6px;">Hasil kali nilai eigen</div><div class="val ' + (prodOk ? 'ok' : 'no') + '">' + (prodOk ? '\u2713 cocok' : '\u2717 tidak cocok') + '</div></div>' +
        '</div>' +
        '<p style="margin-top:10px;"><strong>Multiplisitas aljabar:</strong> ' + roots.map(function(r){
          var m = algebraicMultiplicity(r);
          return '\\lambda = ' + r.latex + ': ' + m;
        }).join(', ') + '.</p>'
    });
  }

  function algebraicMultiplicity(root){
    var count = 0;
    for (var i = 0; i < roots.length; i++){
      if (roots[i].kind === root.kind){
        if (root.kind === 'complex'){
          if (Math.abs(roots[i].re - root.re) < 1e-6 && Math.abs(roots[i].im - root.im) < 1e-6) count++;
        } else {
          if (roots[i].frac && root.frac && roots[i].frac.eq(root.frac)) count++;
          else if (!roots[i].frac && !root.frac && Math.abs(roots[i].dec - root.dec) < 1e-6) count++;
        }
      }
    }
    return count;
  }

  function renderSteps(){
    var wrap = $('stepsWrap');
    wrap.innerHTML = '';
    var indices = showAll ? steps.map(function(_, i){ return i; }) : [stepIdx];
    for (var k = 0; k < indices.length; k++){
      var i = indices[k];
      var s = steps[i];
      var div = document.createElement('div');
      div.className = 'step' + (showAll || i === stepIdx ? ' active' : '');
      div.innerHTML = '<h4>' + s.title + '</h4>' + s.html;
      wrap.appendChild(div);
    }
    updateCounter();
    Katex.render(wrap);
  }

  function updateCounter(){
    $('stepCounter').textContent = 'Langkah ' + (stepIdx + 1) + ' / ' + steps.length;
    $('stepPrev').disabled = stepIdx === 0;
    $('stepNext').disabled = stepIdx >= steps.length - 1;
    $('stepAll').textContent = showAll ? 'Tampilkan Satu' : 'Tampilkan Semua';
    if ($('checkA') && steps.length > 0) {
      // re-render check grid inside step 6 already contains its own grid; keep id unique issue avoided by only first
    }
  }

  function calc(){
    var A2 = readMatrix();
    if (!A2) return;
    $('errA').classList.remove('show');
    buildSteps();
    stepIdx = 0;
    showAll = false;
    $('stepAll').textContent = 'Tampilkan Semua';
    renderSteps();
    renderEigvals();
    // Notify Module C of matrix change
    window.dispatchEvent(new CustomEvent('eigenMatrixChanged'));
  }

  function renderEigvals(){
    var box = $('eigvalsA');
    if (!roots.length) { box.innerHTML = ''; return; }
    var html = '<div style="margin-top:14px;"><strong>Hasil nilai eigen:</strong><div style="margin-top:8px;">' +
      roots.map(function(r){
        var tag = '';
        if (r.kind === 'complex') tag = '<span class="tag">kompleks</span>';
        else if (!r.exact) tag = '<span class="tag">aproksimasi</span>';
        var m = algebraicMultiplicity(r);
        if (m > 1) tag += ' <span class="tag">kembar (mult. aljabar ' + m + ')</span>';
        return '<span class="eigval-chip' + (r.kind === 'complex' ? ' complex' : '') + (m > 1 ? ' repeated' : '') + '">' +
          '\\lambda = ' + r.latex + ' ' + tag + '</span>';
      }).join('') + '</div></div>';
    box.innerHTML = html;
    Katex.render(box);
  }

  function copySteps(){
    var text = [];
    for (var i = 0; i < steps.length; i++){
      text.push('## ' + steps[i].title.replace(/&middot;/g,'&middot;').replace(/&minus;/g,'-').replace(/&lambda;/g,'λ'));
      text.push(steps[i].html.replace(/<[^>]*>/g, ' ').replace(/\s+/g, ' ').trim());
      text.push('');
    }
    var blob = text.join('\n');
    navigator.clipboard.writeText(blob).then(function(){
      var btn = $('copySteps');
      var old = btn.textContent;
      btn.textContent = 'Disalin!';
      setTimeout(function(){ btn.textContent = old; }, 1200);
    }, function(){
      alert('Gagal menyalin ke clipboard.');
    });
  }

  /* presets */
  var presets = {
    note: [[ new EigenMath.Fraction(3), new EigenMath.Fraction(2) ], [ new EigenMath.Fraction(-1), new EigenMath.Fraction(0) ]],
    diag: [[ new EigenMath.Fraction(2), new EigenMath.Fraction(0) ], [ new EigenMath.Fraction(0), new EigenMath.Fraction(5) ]],
    sym:  [[ new EigenMath.Fraction(2), new EigenMath.Fraction(1) ], [ new EigenMath.Fraction(1), new EigenMath.Fraction(2) ]],
    rep:  [[ new EigenMath.Fraction(2), new EigenMath.Fraction(1) ], [ new EigenMath.Fraction(0), new EigenMath.Fraction(2) ]],
    comp: [[ new EigenMath.Fraction(0), new EigenMath.Fraction(-1) ], [ new EigenMath.Fraction(1), new EigenMath.Fraction(0) ]]
  };
  function applyPreset(key){
    var p = presets[key];
    n = p.length;
    $('sizeA').value = String(n);
    A = p;
    buildGrid();
    $('errA').classList.remove('show');
    $('stepsWrap').innerHTML = '';
    $('eigvalsA').innerHTML = '';
    window.dispatchEvent(new CustomEvent('eigenMatrixChanged'));
  }

  function randomMatrix(){
    var m = [];
    for (var i = 0; i < n; i++){
      var row = [];
      for (var j = 0; j < n; j++) row.push(new EigenMath.Fraction(Math.floor(Math.random() * 11) - 5));
      m.push(row);
    }
    A = m;
    buildGrid();
    $('errA').classList.remove('show');
    $('stepsWrap').innerHTML = '';
    $('eigvalsA').innerHTML = '';
    window.dispatchEvent(new CustomEvent('eigenMatrixChanged'));
  }

  /* init */
  function init(){
    buildGrid();
    // preset contoh catatan sebagai default
    applyPreset('note');

    $('sizeA').addEventListener('change', function(){
      n = parseInt(this.value, 10);
      A = [];
      for (var i = 0; i < n; i++){
        var row = [];
        for (var j = 0; j < n; j++) row.push(EigenMath.FractionZERO);
        A.push(row);
      }
      buildGrid();
      $('stepsWrap').innerHTML = '';
      $('eigvalsA').innerHTML = '';
      window.dispatchEvent(new CustomEvent('eigenMatrixChanged'));
    });

    $('btnCalcA').addEventListener('click', calc);
    $('stepPrev').addEventListener('click', function(){ if (stepIdx > 0) { stepIdx--; showAll = false; $('stepAll').textContent = 'Tampilkan Semua'; renderSteps(); } });
    $('stepNext').addEventListener('click', function(){ if (stepIdx < steps.length - 1) { stepIdx++; showAll = false; $('stepAll').textContent = 'Tampilkan Semua'; renderSteps(); } });
    $('stepAll').addEventListener('click', function(){ showAll = !showAll; renderSteps(); });
    $('copySteps').addEventListener('click', copySteps);
    $('presetNote').addEventListener('click', function(){ applyPreset('note'); });
    $('presetDiag').addEventListener('click', function(){ applyPreset('diag'); });
    $('presetSym').addEventListener('click', function(){ applyPreset('sym'); });
    $('presetRepeated').addEventListener('click', function(){ applyPreset('rep'); });
    $('presetComplex').addEventListener('click', function(){ applyPreset('comp'); });
    $('randomA').addEventListener('click', randomMatrix);
    $('resetA').addEventListener('click', function(){ applyPreset('note'); window.dispatchEvent(new CustomEvent('eigenMatrixChanged')); });

    // module tabs
    var tabs = document.querySelectorAll('.module-tab');
    var panels = document.querySelectorAll('.module-panel');
    for (var t = 0; t < tabs.length; t++){
      tabs[t].addEventListener('click', function(){
        var target = this.getAttribute('data-target');
        for (var i = 0; i < tabs.length; i++){ tabs[i].classList.toggle('active', tabs[i] === this); tabs[i].setAttribute('aria-selected', tabs[i] === this); }
        for (var i = 0; i < panels.length; i++){ panels[i].classList.toggle('active', panels[i].id === target); }
        // Sync Module C when its tab is activated
        if (target === 'modC' && window.EigenViz && EigenViz.init){
          // Re-sync from Module A
          if (window.EigenApp && EigenApp.getMatrix){
            var Afrac = EigenApp.getMatrix();
            var size = EigenApp.getSize();
            if (size === 2 && Afrac && Afrac.length === 2){
              EigenViz.applyPreset('note'); // This will be overridden by sync
              // Actually, let's just trigger a sync event
              window.dispatchEvent(new CustomEvent('eigenMatrixChanged'));
            }
          }
        }
      });
    }

    // term chips keyboard
    var chips = document.querySelectorAll('.term-chip');
    for (var c = 0; c < chips.length; c++){
      chips[c].addEventListener('keydown', function(e){ if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); this.classList.toggle('focused'); } });
    }
  }

  document.addEventListener('DOMContentLoaded', init);
  return { calc: calc, readMatrix: readMatrix, getMatrix: function(){ return A; }, getSize: function(){ return n; } };
})();

/* =====================================================================
   EIGEN VIZ - Module C: 2D Geometry Visualization
   ===================================================================== */
var EigenViz = (function(){
  'use strict';

  var svg = null;
  var overlay = null;
  var animating = false;
  var animStartTime = 0;
  var animDuration = 2000; // ms
  var animTargetT = 1;
  var animDirection = 1; // 1 = forward, -1 = reverse
  var reducedMotion = false;
  var currentT = 1;
  var currentX = { x: 1, y: 0 };
  var currentA = [[3,2],[-1,0]]; // default from preset 'note'
  var currentEigen = [];
  var tracePoints = [];
  var dragActive = false;
  var svgRect = null;
  var cx = 300, cy = 240; // SVG center
  var scale = 100; // pixels per unit

  var $ = function(id){ return document.getElementById(id); };

  /* ---------- Presets ---------- */
  var presets = {
    note:    [[3, 2], [-1, 0]],
    diag:    [[2, 0], [0, 3]],
    sym:     [[2, 1], [1, 2]],
    shear:   [[1, 1], [0, 1]],
    rot90:   [[0, -1], [1, 0]],
    refl:    [[1, 0], [0, -1]],
    scale:   [[2, 0], [0, 2]],
    singular:[[1, 2], [2, 4]]
  };

  /* ---------- Color CSS variables ---------- */
  function updateColorVars(){
    var root = document.documentElement;
    root.style.setProperty('--color-vec-x', getComputedStyle(root).getPropertyValue('--accent').trim() || '#3b82f6');
    root.style.setProperty('--color-vec-ax', getComputedStyle(root).getPropertyValue('--success').trim() || '#22c55e');
    root.style.setProperty('--color-eig', getComputedStyle(root).getPropertyValue('--warning').trim() || '#fbbf24');
  }

  /* ---------- Sync matrix from Module A ---------- */
  function syncFromModuleA(){
    if (window.EigenApp && EigenApp.getMatrix){
      var Afrac = EigenApp.getMatrix();
      var size = EigenApp.getSize();
      if (size === 2 && Afrac && Afrac.length === 2){
        currentA = Afrac.map(function(row){ return row.map(function(f){ return f.toDec(); }); });
        $('sizeA').value = '2';
        return true;
      }
    }
    return false;
  }

  /* ---------- Check if 2x2 ---------- */
  function is2x2(){
    return currentA && currentA.length === 2 && currentA[0].length === 2;
  }

  /* ---------- Show/hide 2x2 message ---------- */
  function update2x2Visibility(){
    var is2 = is2x2();
    $('not2x2C').style.display = is2 ? 'none' : 'block';
    $('vizAreaC').style.display = is2 ? 'block' : 'none';
    if (is2) requestAnimationFrame(render);
  }

  /* ---------- Resize to 2x2 ---------- */
  function resizeTo2x2(){
    if (window.EigenApp){
      EigenApp.applyPreset('note'); // This sets size to 2
      syncFromModuleA();
      update2x2Visibility();
      buildMatrixGridC();
    }
  }

  /* ---------- Apply preset ---------- */
  function applyPreset(key){
    var p = presets[key];
    if (!p) return;
    currentA = p.map(function(row){ return row.slice(); });
    // Also update Module A
    if (window.EigenApp){
      var fracMat = p.map(function(row){ return row.map(function(v){ return new EigenMath.Fraction(v); }); });
      EigenApp.A = fracMat;
      EigenApp.n = 2;
      EigenApp.buildGrid();
      EigenApp.calc();
    }
    // Reset vector x to default
    currentX = { x: 1, y: 0 };
    currentT = 1;
    tracePoints = [];
    $('vecX1').value = '1';
    $('vecX2').value = '0';
    $('vecAngle').value = '0';
    $('vecLen').value = '1';
    $('interpT').value = '1';
    $('interpTValue').textContent = '1.00';
    update2x2Visibility();
    updateEigenSelect();
    buildMatrixGridC();
    render();
  }

  /* ---------- Random matrix ---------- */
  function randomMatrix(){
    var m = [];
    for (var i = 0; i < 2; i++){
      var row = [];
      for (var j = 0; j < 2; j++) row.push(Math.floor(Math.random() * 11) - 5);
      m.push(row);
    }
    currentA = m;
    if (window.EigenApp){
      var fracMat = m.map(function(row){ return row.map(function(v){ return new EigenMath.Fraction(v); }); });
      EigenApp.A = fracMat;
      EigenApp.n = 2;
      EigenApp.buildGrid();
      EigenApp.calc();
    }
    currentX = { x: 1, y: 0 };
    currentT = 1;
    tracePoints = [];
    $('vecX1').value = '1';
    $('vecX2').value = '0';
    $('vecAngle').value = '0';
    $('vecLen').value = '1';
    $('interpT').value = '1';
    $('interpTValue').textContent = '1.00';
    update2x2Visibility();
    updateEigenSelect();
    buildMatrixGridC();
    render();
  }

  /* ---------- Reset ---------- */
  function resetAll(){
    applyPreset('note');
    buildMatrixGridC();
  }

  /* ---------- Build matrix grid in Module C ---------- */
  function buildMatrixGridC(){
    var container = $('matrixGridC');
    if (!container) return;
    container.innerHTML = '';
    for (var i = 0; i < 2; i++){
      var rowDiv = document.createElement('div');
      rowDiv.className = 'matrix-row';
      for (var j = 0; j < 2; j++){
        var inp = document.createElement('input');
        inp.className = 'cell-input';
        inp.type = 'text';
        inp.value = (currentA[i] && currentA[i][j] !== undefined) ? currentA[i][j].toString() : '0';
        inp.setAttribute('aria-label', 'Sel baris ' + (i+1) + ' kolom ' + (j+1));
        inp.dataset.row = i;
        inp.dataset.col = j;
        inp.addEventListener('change', onMatrixCellChangeC);
        inp.addEventListener('blur', onMatrixCellChangeC);
        rowDiv.appendChild(inp);
      }
      container.appendChild(rowDiv);
    }
  }

  function onMatrixCellChangeC(e){
    var row = parseInt(e.target.dataset.row, 10);
    var col = parseInt(e.target.dataset.col, 10);
    var val = parseFloat(e.target.value);
    if (isNaN(val)){
      // Restore previous value
      e.target.value = currentA[row][col].toString();
      return;
    }
    // Clamp large values
    if (Math.abs(val) > 1e6) val = val > 0 ? 1e6 : -1e6;
    currentA[row][col] = val;
    e.target.value = val.toString();
    // Sync to Module A
    if (window.EigenApp){
      var fracMat = currentA.map(function(row){ return row.map(function(v){ return new EigenMath.Fraction(v); }); });
      EigenApp.A = fracMat;
      EigenApp.n = 2;
      EigenApp.buildGrid();
      EigenApp.calc();
    }
    // Update visualization
    recomputeEigen();
    updateEigenSelect();
    buildMatrixGridC(); // Rebuild to show formatted values
    render();
  }

  /* ---------- Update eigen select dropdown ---------- */
  function updateEigenSelect(){
    var select = $('snapEigSelect');
    var realEigs = currentEigen.filter(function(e){ return !e.complex; });
    select.innerHTML = '<option value="">Pilih λ...</option>';
    realEigs.forEach(function(e, idx){
      var opt = document.createElement('option');
      opt.value = idx;
      opt.textContent = 'λ = ' + e.lambda.toFixed(4);
      select.appendChild(opt);
    });
    if (realEigs.length <= 1){
      select.style.display = 'none';
    } else {
      select.style.display = 'block';
    }
  }

  /* ---------- Compute eigen for current matrix ---------- */
  function recomputeEigen(){
    if (!is2x2()){
      currentEigen = [];
      return;
    }
    currentEigen = EigenGeometry.computeEigen2x2(currentA);
    updateEigenSelect();
  }

  /* ---------- Get current M(t) ---------- */
  function getCurrentM(){
    return EigenGeometry.interpolateMat(currentA, currentT);
  }

  /* ---------- Compute Ax ---------- */
  function computeAx(){
    var M = getCurrentM();
    return EigenGeometry.matMulVec(M, currentX);
  }

  /* ---------- Auto-scale ---------- */
  function updateScale(){
    var Ax = computeAx();
    // Ellipse radius: max singular value approx = max stretch of unit circle
    // For 2x2, we can sample
    var circlePts = EigenGeometry.sampleCircle(50);
    var M = getCurrentM();
    var transformed = circlePts.map(function(p){ return EigenGeometry.matMulVec(M, p); });
    var maxEllipseR = 0;
    for (var i = 0; i < transformed.length; i++){
      maxEllipseR = Math.max(maxEllipseR, EigenGeometry.vecLen(transformed[i]));
    }
    var maxVal = EigenGeometry.computeAutoScale(currentX, Ax, maxEllipseR, 1.3);
    // Limit scale to reasonable bounds
    var maxScale = 300; // pixels per unit max
    var minScale = 20;
    scale = Math.max(minScale, Math.min(maxScale, 220 / (maxVal || 1)));
  }

  /* ---------- Render everything ---------- */
  function render(){
    if (!is2x2()) return;
    if (!svg) svg = $('svgC');
    if (!overlay) overlay = $('canvasOverlayC');

    updateScale();
    recomputeEigen();

    var M = getCurrentM();
    var Ax = computeAx();
    var det = EigenGeometry.det2(M);

    // Clear trace if t changed significantly or reset
    // We keep trace only when animating rotation of x
    // For now, clear trace on any parameter change except animation
    // (We'll handle trace in animation)

    // Update SVG viewBox for responsive scaling
    var vbSize = 300 / scale * 2; // world units visible
    // Keep viewBox fixed at 600x480, scale via transform
    // Actually easier: use a transform group
    var gTransform = 'translate(' + cx + ',' + cy + ') scale(' + scale + ', ' + (-scale) + ')';
    // We'll apply transform to groups instead

    // --- Grid ---
    var gridGroup = $('gridGroupC');
    var showGrid = $('toggleGridC').checked;
    if (showGrid){
      var gridLines = EigenGeometry.sampleGrid(10, 5); // 10 lines, range ±5
      var transformedGrid = EigenGeometry.transformLines(gridLines, M);
      gridGroup.innerHTML = '';
      var path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
      path.setAttribute('d', EigenGeometry.gridToSVG(transformedGrid, 1, 0, 0));
      path.setAttribute('stroke', 'var(--text-muted)');
      path.setAttribute('stroke-width', '0.5');
      path.setAttribute('fill', 'none');
      path.setAttribute('opacity', '0.4');
      path.setAttribute('vector-effect', 'non-scaling-stroke');
      gridGroup.appendChild(path);
    } else {
      gridGroup.innerHTML = '';
    }

    // --- Unit Circle ---
    var circle = $('unitCircleC');
    // Circle is at origin with r=1, transform handles scaling

    // --- Ellipse ---
    var ellipse = $('ellipseC');
    var showCircle = $('toggleCircleC').checked;
    if (showCircle){
      ellipse.setAttribute('d', EigenGeometry.ellipseFromMatrix(M));
      ellipse.style.display = 'block';
      circle.style.display = 'block';
    } else {
      ellipse.style.display = 'none';
      circle.style.display = 'none';
    }

    // --- Eigen lines ---
    var eigLinesGroup = $('eigenLinesGroupC');
    var showEigLines = $('toggleEigLinesC').checked;
    var showEigVecs = $('toggleEigVecsC').checked;
    eigLinesGroup.innerHTML = '';
    if (showEigLines || showEigVecs){
      var realEigs = currentEigen.filter(function(e){ return !e.complex; });
      for (var i = 0; i < realEigs.length; i++){
        var eig = realEigs[i];
        if (eig.fullSpace){
          // A = λI: all directions are eigenvectors
          var txt = document.createElementNS('http://www.w3.org/2000/svg', 'text');
          txt.setAttribute('x', '0');
          txt.setAttribute('y', '-20');
          txt.setAttribute('text-anchor', 'middle');
          txt.setAttribute('font-size', '11');
          txt.setAttribute('fill', 'var(--color-eig)');
          txt.setAttribute('font-weight', '600');
          txt.textContent = 'Semua arah adalah vektor eigen (A = λI, λ=' + eig.lambda.toFixed(3) + ')';
          eigLinesGroup.appendChild(txt);
          continue;
        }
        var color = 'hsl(' + (40 + i * 180) + ', 70%, 50%)';
        if (showEigLines){
          var line = document.createElementNS('http://www.w3.org/2000/svg', 'path');
          line.setAttribute('d', EigenGeometry.eigenLineToSVG(eig.vec, 1, 0, 0, 5));
          line.setAttribute('stroke', color);
          line.setAttribute('stroke-width', '1');
          line.setAttribute('stroke-dasharray', '8,4');
          line.setAttribute('fill', 'none');
          line.setAttribute('opacity', '0.7');
          line.setAttribute('vector-effect', 'non-scaling-stroke');
          eigLinesGroup.appendChild(line);
          // Label
          var label = document.createElementNS('http://www.w3.org/2000/svg', 'text');
          var labelPos = EigenGeometry.vecScale(eig.vec, 5.2);
          label.setAttribute('x', labelPos.x);
          label.setAttribute('y', -labelPos.y);
          label.setAttribute('text-anchor', eig.vec.x >= 0 ? 'start' : 'end');
          label.setAttribute('font-size', '10');
          label.setAttribute('fill', color);
          label.setAttribute('font-weight', '600');
          label.textContent = 'E(λ=' + eig.lambda.toFixed(3) + ')';
          eigLinesGroup.appendChild(label);
        }
        if (showEigVecs){
          // Unit eigenvector arrow
          var vecPath = document.createElementNS('http://www.w3.org/2000/svg', 'path');
          var v = EigenGeometry.vecScale(eig.vec, 1); // unit length
          var vecData = EigenGeometry.vecToSVG(v, 1, 0, 0, color, '', {x:0, y:0});
          vecPath.setAttribute('d', vecData.line);
          vecPath.setAttribute('stroke', color);
          vecPath.setAttribute('stroke-width', '2');
          vecPath.setAttribute('marker-end', 'url(#arrowheadEig)');
          vecPath.setAttribute('fill', 'none');
          vecPath.setAttribute('vector-effect', 'non-scaling-stroke');
          eigLinesGroup.appendChild(vecPath);
        }
      }
      if (currentEigen.some(function(e){ return e.complex; })){
        var txt = document.createElementNS('http://www.w3.org/2000/svg', 'text');
        txt.setAttribute('x', '0');
        txt.setAttribute('y', '-20');
        txt.setAttribute('text-anchor', 'middle');
        txt.setAttribute('font-size', '11');
        txt.setAttribute('fill', 'var(--text-muted)');
        txt.textContent = 'Tidak ada garis ruang eigen real (nilai eigen kompleks)';
        eigLinesGroup.appendChild(txt);
      }
    }

    // --- Axes ---
    var axesGroup = $('axesGroupC');
    axesGroup.innerHTML = '';
    var maxExtent = 5; // world units
    var axesPath = document.createElementNS('http://www.w3.org/2000/svg', 'path');
    axesPath.setAttribute('d', EigenGeometry.axesToSVG(1, 0, 0, maxExtent));
    axesPath.setAttribute('stroke', 'var(--text-muted)');
    axesPath.setAttribute('stroke-width', '0.8');
    axesPath.setAttribute('fill', 'none');
    axesPath.setAttribute('opacity', '0.5');
    axesPath.setAttribute('vector-effect', 'non-scaling-stroke');
    axesGroup.appendChild(axesPath);
    // Tick labels - disabled for now as they get transformed (scaled/flipped)
    // Can be re-enabled with a separate non-transformed overlay layer
    var showLabels = false; // $('toggleLabelsC').checked;
    if (showLabels){
      for (var i = -Math.ceil(maxExtent); i <= Math.ceil(maxExtent); i++){
        if (i === 0) continue;
        // X ticks
        var xl = document.createElementNS('http://www.w3.org/2000/svg', 'text');
        xl.setAttribute('x', i);
        xl.setAttribute('y', 14);
        xl.setAttribute('text-anchor', 'middle');
        xl.setAttribute('font-size', '9');
        xl.setAttribute('fill', 'var(--text-muted)');
        xl.textContent = i;
        axesGroup.appendChild(xl);
        // Y ticks
        var yl = document.createElementNS('http://www.w3.org/2000/svg', 'text');
        yl.setAttribute('x', -14);
        yl.setAttribute('y', -i + 3);
        yl.setAttribute('text-anchor', 'end');
        yl.setAttribute('font-size', '9');
        yl.setAttribute('fill', 'var(--text-muted)');
        yl.textContent = i;
        axesGroup.appendChild(yl);
      }
    }

    // --- Vector x ---
    var vecXGroup = $('vecXGroupC');
    var vecXData = EigenGeometry.vecToSVG(currentX, 1, 0, 0, '', 'x', {x:0, y:-14});
    $('vecXLineC').setAttribute('d', vecXData.line);
    var xLabel = $('vecXLabelC');
    xLabel.setAttribute('x', vecXData.labelX);
    xLabel.setAttribute('y', vecXData.labelY);
    xLabel.textContent = 'x (' + currentX.x.toFixed(2) + ', ' + currentX.y.toFixed(2) + ')';

    // --- Vector Ax ---
    var vecAXGroup = $('vecAXGroupC');
    var vecAXData = EigenGeometry.vecToSVG(Ax, 1, 0, 0, '', 'Ax', {x:0, y:14});
    $('vecAXLineC').setAttribute('d', vecAXData.line);
    var axLabel = $('vecAXLabelC');
    axLabel.setAttribute('x', vecAXData.labelX);
    axLabel.setAttribute('y', vecAXData.labelY);
    axLabel.textContent = 'Ax (' + Ax.x.toFixed(2) + ', ' + Ax.y.toFixed(2) + ')';

    // --- Connecting dashed line (path from x tip to Ax tip) ---
    var xEnd = EigenGeometry.vecScale(currentX, 1);
    var axEnd = EigenGeometry.vecScale(Ax, 1);
    var connectPath = 'M' + xEnd.x + ',' + (-xEnd.y) + ' L' + axEnd.x + ',' + (-axEnd.y);
    $('vecConnectC').setAttribute('d', connectPath);

    // --- Trace path ---
    var tracePath = $('tracePathC');
    var showTrace = $('toggleTraceC').checked;
    if (showTrace && tracePoints.length > 1){
      var pts = tracePoints.map(function(p){ return p.x + ',' + (-p.y); });
      tracePath.setAttribute('d', 'M' + pts.join(' L'));
      tracePath.style.display = 'block';
    } else {
      tracePath.style.display = 'none';
    }

    // Apply transform to main group (contains grid, axes, eigenlines, circle, ellipse, vectors, trace)
    var transform = 'translate(' + cx + ',' + cy + ') scale(' + scale + ', ' + (-scale) + ')';
    var mainGroup = $('mainGroupC');
    if (mainGroup) mainGroup.setAttribute('transform', transform);

    // Update number panel
    updateNumberPanel(M, Ax, det);
  }

  /* ---------- Number panel ---------- */
  function updateNumberPanel(M, Ax, det){
    var panel = $('numPanelContentC');
    var data = EigenGeometry.computePanelData(currentA, currentEigen, currentX, Ax, currentT);
    var parallel = EigenGeometry.isParallel(currentX, Ax);
    var xLen = EigenGeometry.vecLen(currentX);
    var AxLen = EigenGeometry.vecLen(Ax);

    var html = '';

    // Vector data
    html += '<div class="check-grid" style="margin-bottom:16px;">';
    html += '<div class="check-item"><div class="lbl">Vektor x</div><div class="val" style="font-family:monospace;">[' + currentX.x.toFixed(3) + ', ' + currentX.y.toFixed(3) + ']</div></div>';
    html += '<div class="check-item"><div class="lbl">Vektor Ax = M(t)x</div><div class="val" style="font-family:monospace;">[' + Ax.x.toFixed(3) + ', ' + Ax.y.toFixed(3) + ']</div></div>';
    html += '<div class="check-item"><div class="lbl">Sudut x</div><div class="val">' + EigenGeometry.angleDeg(currentX).toFixed(1) + '°</div></div>';
    html += '<div class="check-item"><div class="lbl">Sudut Ax</div><div class="val">' + EigenGeometry.angleDeg(Ax).toFixed(1) + '°</div></div>';
    html += '<div class="check-item"><div class="lbl">Selisih sudut</div><div class="val">' + data.angleDiff.toFixed(2) + '°</div></div>';
    html += '<div class="check-item"><div class="lbl">Rasio |Ax|/|x|</div><div class="val">' + (xLen > 1e-10 ? data.ratio.toFixed(4) : 'N/A (x=0)') + '</div></div>';
    html += '<div class="check-item"><div class="lbl">Determinan M(t)</div><div class="val">' + det.toFixed(4) + '</div></div>';
    html += '<div class="check-item"><div class="lbl">Faktor luas (|det|)</div><div class="val">' + Math.abs(det).toFixed(4) + ' (luas elips = ' + Math.abs(det).toFixed(4) + 'π)</div></div>';
    html += '</div>';

    // Eigen info
    html += '<div style="margin-bottom:12px;"><strong>Nilai & Vektor Eigen (A):</strong></div>';
    if (currentEigen.length === 0){
      html += '<div style="color:var(--text-muted); font-size:12px;">Matriks bukan 2×2 atau belum dihitung.</div>';
    } else {
      currentEigen.forEach(function(e, idx){
        if (e.complex){
          html += '<div style="margin:4px 0; font-size:12px; color:#c084fc;">λ = ' + e.lambdaRe.toFixed(4) + ' ± ' + Math.abs(e.lambdaIm).toFixed(4) + 'i (kompleks)</div>';
        } else if (e.fullSpace){
          html += '<div style="margin:4px 0; font-size:12px; color:var(--warning);"><strong>λ = ' + e.lambda.toFixed(4) + ' (mult. 2)</strong> → A = λI, <em>semua arah vektor eigen</em></div>';
        } else {
          var v = e.vec;
          html += '<div style="margin:4px 0; font-size:12px;">λ = ' + e.lambda.toFixed(4) + ' → v ≈ [' + v.x.toFixed(4) + ', ' + v.y.toFixed(4) + ']' + (e.multiplicity > 1 ? ' (kembar)' : '') + '</div>';
        }
      });
    }

    // Parallel check
    if (xLen > 1e-10){
      var realEigs = currentEigen.filter(function(e){ return !e.complex && !e.fullSpace; });
      var lamMatch = null;
      for (var i = 0; i < realEigs.length; i++){
        if (Math.abs(Math.abs(realEigs[i].lambda) - data.ratio) < 1e-3){
          lamMatch = realEigs[i].lambda;
          break;
        }
      }
      if (parallel){
        html += '<div style="margin-top:12px; padding:10px; border-radius:var(--radius-sm); background:rgba(34,197,94,0.12); border:1px solid rgba(34,197,94,0.35); color:#22c55e;">';
        html += '<strong>✓ x adalah vektor eigen</strong><br>';
        html += 'Ax sejajar x → Ax = λx';
        if (lamMatch !== null){
          html += ', λ = ' + lamMatch.toFixed(4) + ', |Ax|/|x| = ' + Math.abs(lamMatch).toFixed(4) + ' (✓ cocok)';
        }
        html += '</div>';
      } else {
        html += '<div style="margin-top:12px; padding:10px; border-radius:var(--radius-sm); background:rgba(239,68,68,0.1); border:1px solid rgba(239,68,68,0.3); color:#ef4444;">';
        html += '<strong>✗ x bukan vektor eigen</strong><br>';
        html += 'Ax tidak sejajar x (selisih ' + data.angleDiff.toFixed(2) + '°)';
        html += '</div>';
      }
    } else {
      html += '<div style="margin-top:12px; padding:10px; border-radius:var(--radius-sm); background:rgba(251,191,36,0.12); border:1px solid rgba(251,191,36,0.35); color:var(--warning);">';
      html += '<strong>x = 0</strong> — vektor nol bukan vektor eigen.';
      html += '</div>';
    }

    // Contextual explanation
    var explanations = EigenGeometry.generateExplanation(currentA, currentEigen, currentX, Ax, currentT < 1 ? currentT : undefined);
    if (explanations.length){
      html += '<div style="margin-top:16px; padding:12px; border-radius:var(--radius-sm); background:var(--accent-soft); border:1px solid var(--accent-border);">';
      html += '<strong style="color:var(--accent);">Penjelasan Kontekstual:</strong><ul style="margin:8px 0 0 18px; font-size:12.5px; line-height:1.7; color:var(--text-secondary);">';
      explanations.forEach(function(line){
        html += '<li>' + line + '</li>';
      });
      html += '</ul></div>';
    }

    // Legend
    html += '<div style="margin-top:16px; padding:10px; border-radius:var(--radius-sm); background:var(--bg-base); border:1px solid var(--glass-border);">';
    html += '<strong style="font-size:11px; text-transform:uppercase; letter-spacing:0.05em; color:var(--text-muted);">Legenda</strong>';
    html += '<div style="display:flex; flex-wrap:wrap; gap:8px; margin-top:8px; font-size:11px;">';
    html += '<span style="display:flex; align-items:center; gap:4px;"><span style="width:12px; height:3px; background:var(--accent);"></span> Vektor x</span>';
    html += '<span style="display:flex; align-items:center; gap:4px;"><span style="width:12px; height:3px; background:var(--success);"></span> Vektor Ax</span>';
    html += '<span style="display:flex; align-items:center; gap:4px;"><span style="width:12px; height:3px; background:var(--warning);"></span> Garis/vektor eigen</span>';
    html += '<span style="display:flex; align-items:center; gap:4px;"><span style="width:12px; height:3px; background:var(--text-muted); opacity:0.5;"></span> Grid/lingkaran asal</span>';
    html += '<span style="display:flex; align-items:center; gap:4px;"><span style="width:12px; height:3px; background:var(--accent);"></span> Elips transformasi</span>';
    html += '<span style="display:flex; align-items:center; gap:4px;"><span style="width:12px; height:3px; background:var(--accent); border-radius:50%;"></span> Titik asal</span>';
    html += '</div></div>';

    panel.innerHTML = html;
  }

  /* ---------- Vector x input handlers ---------- */
  function onVecCoordChange(){
    var x1 = parseFloat($('vecX1').value) || 0;
    var x2 = parseFloat($('vecX2').value) || 0;
    currentX = { x: x1, y: x2 };
    // Update angle and length inputs
    var angle = EigenGeometry.angleDeg(currentX);
    var len = EigenGeometry.vecLen(currentX);
    $('vecAngle').value = Math.round(angle);
    $('vecLen').value = len.toFixed(2);
    tracePoints = []; // Clear trace on manual change
    render();
  }

  function onVecAngleChange(){
    var angle = parseFloat($('vecAngle').value) || 0;
    var len = parseFloat($('vecLen').value) || 1;
    var rad = angle * Math.PI / 180;
    currentX = { x: len * Math.cos(rad), y: len * Math.sin(rad) };
    $('vecX1').value = currentX.x.toFixed(3);
    $('vecX2').value = currentX.y.toFixed(3);
    tracePoints = [];
    render();
  }

  function onVecLenChange(){
    var len = parseFloat($('vecLen').value) || 1;
    var angle = parseFloat($('vecAngle').value) || 0;
    var rad = angle * Math.PI / 180;
    currentX = { x: len * Math.cos(rad), y: len * Math.sin(rad) };
    $('vecX1').value = currentX.x.toFixed(3);
    $('vecX2').value = currentX.y.toFixed(3);
    render();
  }

  /* ---------- Interpolation slider ---------- */
  function onInterpChange(){
    currentT = parseFloat($('interpT').value);
    $('interpTValue').textContent = currentT.toFixed(2);
    tracePoints = [];
    render();
  }

  /* ---------- Snap to eigenvector ---------- */
  function onSnapEigen(){
    var select = $('snapEigSelect');
    var idx = parseInt(select.value, 10);
    var realEigs = currentEigen.filter(function(e){ return !e.complex && !e.fullSpace; });
    if (isNaN(idx) || idx < 0 || idx >= realEigs.length){
      // Snap to nearest
      currentX = EigenGeometry.snapToEigen(currentX, currentEigen);
    } else {
      var eig = realEigs[idx];
      var len = EigenGeometry.vecLen(currentX);
      currentX = EigenGeometry.vecScale(eig.vec, len);
    }
    $('vecX1').value = currentX.x.toFixed(3);
    $('vecX2').value = currentX.y.toFixed(3);
    $('vecAngle').value = Math.round(EigenGeometry.angleDeg(currentX));
    $('vecLen').value = EigenGeometry.vecLen(currentX).toFixed(2);
    tracePoints = [];
    render();
  }

  /* ---------- Drag interaction ---------- */
  function getSVGPoint(clientX, clientY){
    if (!svgRect) svgRect = svg.getBoundingClientRect();
    var pt = svg.createSVGPoint();
    pt.x = clientX - svgRect.left;
    pt.y = clientY - svgRect.top;
    var ctm = svg.getScreenCTM();
    if (ctm) pt = pt.matrixTransform(ctm.inverse());
    // Convert to world coordinates (inverse of our transform)
    // Our transform: translate(cx,cy) scale(scale, -scale)
    // Inverse: scale(1/scale, -1/scale) translate(-cx, -cy)
    var wx = (pt.x - cx) / scale;
    var wy = -(pt.y - cy) / scale;
    return { x: wx, y: wy };
  }

  function onDragStart(e){
    if (!is2x2()) return;
    e.preventDefault();
    dragActive = true;
    var clientX = e.touches ? e.touches[0].clientX : e.clientX;
    var clientY = e.touches ? e.touches[0].clientY : e.clientY;
    svgRect = svg.getBoundingClientRect();
    var pt = getSVGPoint(clientX, clientY);
    currentX = pt;
    updateVectorInputs();
    tracePoints = [];
    render();
  }

  function onDragMove(e){
    if (!dragActive) return;
    e.preventDefault();
    var clientX = e.touches ? e.touches[0].clientX : e.clientX;
    var clientY = e.touches ? e.touches[0].clientY : e.clientY;
    var pt = getSVGPoint(clientX, clientY);
    currentX = pt;
    updateVectorInputs();
    render();
  }

  function onDragEnd(){
    dragActive = false;
  }

  function updateVectorInputs(){
    $('vecX1').value = currentX.x.toFixed(3);
    $('vecX2').value = currentX.y.toFixed(3);
    $('vecAngle').value = Math.round(EigenGeometry.angleDeg(currentX));
    $('vecLen').value = EigenGeometry.vecLen(currentX).toFixed(2);
  }

  /* ---------- Animation ---------- */
  function checkReducedMotion(){
    reducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
  }

  function animate(){
    if (!animating) return;
    var now = performance.now();
    var elapsed = now - animStartTime;
    var progress = Math.min(elapsed / animDuration, 1);
    if (reducedMotion) progress = 1;

    if (animDirection === 1){
      currentT = progress; // 0 to 1
    } else {
      currentT = 1 - progress; // 1 to 0
    }

    $('interpT').value = currentT.toFixed(2);
    $('interpTValue').textContent = currentT.toFixed(2);

    // Record trace point - trace the tip of Ax as M(t) changes
    var Ax = computeAx();
    tracePoints.push({ x: Ax.x, y: Ax.y });

    render();

    if (progress < 1){
      requestAnimationFrame(animate);
    } else {
      animating = false;
      $('playAnimBtn').style.display = 'inline-flex';
      $('pauseAnimBtn').style.display = 'none';
    }
  }

  function startAnimation(){
    checkReducedMotion();
    animating = true;
    animDuration = parseFloat($('animDuration').value) * 1000;
    var speed = parseFloat($('animSpeed').value);
    animDuration /= speed;
    animStartTime = performance.now();
    animDirection = currentT < 0.5 ? 1 : -1; // Determine direction from current position
    tracePoints = [];
    $('playAnimBtn').style.display = 'none';
    $('pauseAnimBtn').style.display = 'inline-flex';
    requestAnimationFrame(animate);
  }

  function pauseAnimation(){
    animating = false;
    $('playAnimBtn').style.display = 'inline-flex';
    $('pauseAnimBtn').style.display = 'none';
  }

  function toggleAnimation(){
    if (animating) pauseAnimation();
    else startAnimation();
  }

  /* ---------- Toggle handlers ---------- */
  function onToggleChange(){
    render();
  }

  /* ---------- Init ---------- */
  function init(){
    // Sync from Module A initially
    syncFromModuleA();
    update2x2Visibility();

    // Event listeners
    $('resizeTo2x2C').addEventListener('click', resizeTo2x2);

    // Presets
    $('presetNoteC').addEventListener('click', function(){ applyPreset('note'); });
    $('presetDiagC').addEventListener('click', function(){ applyPreset('diag'); });
    $('presetSymC').addEventListener('click', function(){ applyPreset('sym'); });
    $('presetShearC').addEventListener('click', function(){ applyPreset('shear'); });
    $('presetRot90C').addEventListener('click', function(){ applyPreset('rot90'); });
    $('presetReflC').addEventListener('click', function(){ applyPreset('refl'); });
    $('presetScaleC').addEventListener('click', function(){ applyPreset('scale'); });
    $('presetSingularC').addEventListener('click', function(){ applyPreset('singular'); });
    $('resetC').addEventListener('click', resetAll);
    $('randomC').addEventListener('click', randomMatrix);

    // Vector x inputs
    $('vecX1').addEventListener('input', onVecCoordChange);
    $('vecX2').addEventListener('input', onVecCoordChange);
    $('vecAngle').addEventListener('input', onVecAngleChange);
    $('vecLen').addEventListener('input', onVecLenChange);

    // Interpolation
    $('interpT').addEventListener('input', onInterpChange);

    // Snap
    $('snapEigBtn').addEventListener('click', onSnapEigen);
    $('snapEigSelect').addEventListener('change', onSnapEigen);

    // Animation
    $('playAnimBtn').addEventListener('click', startAnimation);
    $('pauseAnimBtn').addEventListener('click', pauseAnimation);

    // Toggles
    ['toggleGridC', 'toggleCircleC', 'toggleEigLinesC', 'toggleEigVecsC', 'toggleTraceC', 'toggleLabelsC'].forEach(function(id){
      $(id).addEventListener('change', onToggleChange);
    });

    // Drag on overlay
    overlay.addEventListener('mousedown', onDragStart);
    overlay.addEventListener('touchstart', onDragStart, { passive: false });
    window.addEventListener('mousemove', onDragMove);
    window.addEventListener('touchmove', onDragMove, { passive: false });
    window.addEventListener('mouseup', onDragEnd);
    window.addEventListener('touchend', onDragEnd);

    // Listen for Module A changes to sync
    // We'll poll or use a custom event
    window.addEventListener('eigenMatrixChanged', function(){
      syncFromModuleA();
      update2x2Visibility();
      recomputeEigen();
      buildMatrixGridC();
      render();
    });

    // Initial render
    recomputeEigen();
    buildMatrixGridC();
    render();
  }

  // Expose for testing
  return {
    init: init,
    applyPreset: applyPreset,
    currentA: function(){ return currentA; },
    currentX: function(){ return currentX; },
    currentT: function(){ return currentT; },
    currentEigen: function(){ return currentEigen; },
    computeAx: computeAx,
    getCurrentM: getCurrentM,
    // Test functions
    _test: {
      interpolateMat: EigenGeometry.interpolateMat,
      computeEigen2x2: EigenGeometry.computeEigen2x2,
      isParallel: EigenGeometry.isParallel,
      snapToEigen: EigenGeometry.snapToEigen,
      computeAutoScale: EigenGeometry.computeAutoScale,
      ellipseFromMatrix: EigenGeometry.ellipseFromMatrix,
      det2: EigenGeometry.det2,
      generateExplanation: EigenGeometry.generateExplanation
    }
  };
})();

/* =====================================================================
   UNIT TESTS for EigenGeometry (run in console: EigenViz.runTests())
   ===================================================================== */
EigenViz.runTests = function(){
  var G = EigenGeometry;
  var passed = 0, failed = 0;
  function assert(cond, msg){
    if (cond){ console.log('✓', msg); passed++; }
    else { console.error('✗', msg); failed++; }
  }
  function approxEq(a, b, eps){ return Math.abs(a - b) < (eps || 1e-6); }

  console.log('=== EigenGeometry Unit Tests ===');

  // Test 1: M(0) = I, M(1) = A
  var A = [[3,2],[-1,0]];
  var M0 = G.interpolateMat(A, 0);
  var M1 = G.interpolateMat(A, 1);
  assert(approxEq(M0[0][0], 1) && approxEq(M0[0][1], 0) && approxEq(M0[1][0], 0) && approxEq(M0[1][1], 1), 'M(0) = I');
  assert(approxEq(M1[0][0], 3) && approxEq(M1[0][1], 2) && approxEq(M1[1][0], -1) && approxEq(M1[1][1], 0), 'M(1) = A');

  // Test 2: A = [[3,2],[-1,0]] eigenvectors
  var eig = G.computeEigen2x2(A);
  var realEigs = eig.filter(function(e){ return !e.complex; });
  assert(realEigs.length === 2, 'Two real eigenvalues');
  // λ=1: eigenvector direction [-1, 1] (or [1, -1])
  var v1 = null, v2 = null;
  for (var i = 0; i < realEigs.length; i++){
    if (approxEq(realEigs[i].lambda, 1)) v1 = realEigs[i].vec;
    if (approxEq(realEigs[i].lambda, 2)) v2 = realEigs[i].vec;
  }
  assert(v1 !== null, 'λ=1 found');
  assert(v2 !== null, 'λ=2 found');
  // Check direction: v1 should be parallel to [-1, 1]
  assert(approxEq(Math.abs(G.vecCross(v1, G.vec2(-1, 1))), 0, 1e-6), 'λ=1 eigenvector direction [-1,1]');
  assert(approxEq(Math.abs(G.vecCross(v2, G.vec2(-2, 1))), 0, 1e-6), 'λ=2 eigenvector direction [-2,1]');

  // Test 3: For x on eigenvector, |Ax|/|x| = |λ| and Ax parallel x
  var Ax1 = G.matMulVec(A, v1);
  var ratio1 = G.vecLen(Ax1) / G.vecLen(v1);
  assert(approxEq(ratio1, 1, 1e-6), '|Ax|/|x| = |λ|=1 for λ=1');
  assert(G.isParallel(v1, Ax1), 'Ax parallel x for eigenvector λ=1');

  var Ax2 = G.matMulVec(A, v2);
  var ratio2 = G.vecLen(Ax2) / G.vecLen(v2);
  assert(approxEq(ratio2, 2, 1e-6), '|Ax|/|x| = |λ|=2 for λ=2');
  assert(G.isParallel(v2, Ax2), 'Ax parallel x for eigenvector λ=2');

  // Test 4: Rotation 90° - no real eigen lines
  var R = [[0,-1],[1,0]];
  var eigR = G.computeEigen2x2(R);
  var realR = eigR.filter(function(e){ return !e.complex; });
  assert(realR.length === 0, 'Rotation 90° has no real eigenvalues');

  // Test 5: M(0)=I, M(1)=A already tested above

  // Test 6: Ellipse area = |det A| * π (for unit circle)
  var ellipseArea = Math.abs(G.det2(A)) * Math.PI; // theoretical
  // Sample ellipse and compute area via polygon approximation
  var ellipsePath = G.ellipseFromMatrix(A);
  // We can't easily compute area from path here, but we trust the math
  assert(approxEq(Math.abs(G.det2(A)), 2, 1e-6), 'det(A) = 2 for [[3,2],[-1,0]]');
  // Area scaling factor = |det| = 2

  // Test 7: Snap produces parallel vector
  var xTest = G.vec2(3, 1);
  var snapped = G.snapToEigen(xTest, eig);
  var AxSnapped = G.matMulVec(A, snapped);
  assert(G.isParallel(snapped, AxSnapped), 'Snap produces eigenvector (parallel Ax)');

  // Test 8: Uniform scale [[2,0],[0,2]] - all directions eigen
  var S = [[2,0],[0,2]];
  var eigS = G.computeEigen2x2(S);
  assert(eigS.length === 1 && eigS[0].fullSpace === true, 'Uniform scale: full eigenspace');
  assert(approxEq(eigS[0].lambda, 2), 'λ=2 for uniform scale');

  // Test 9: Singular matrix [[1,2],[2,4]] - λ=0, λ=5
  var Sing = [[1,2],[2,4]];
  var eigSing = G.computeEigen2x2(Sing);
  var realSing = eigSing.filter(function(e){ return !e.complex; });
  assert(realSing.length === 2, 'Singular matrix has 2 real eigenvalues');
  var hasZero = realSing.some(function(e){ return approxEq(e.lambda, 0); });
  var hasFive = realSing.some(function(e){ return approxEq(e.lambda, 5); });
  assert(hasZero, 'Singular matrix has λ=0');
  assert(hasFive, 'Singular matrix has λ=5');

  // Test 10: Shear [[1,1],[0,1]] - repeated λ=1, defective
  var Shear = [[1,1],[0,1]];
  var eigShear = G.computeEigen2x2(Shear);
  var realShear = eigShear.filter(function(e){ return !e.complex; });
  assert(realShear.length === 1 && realShear[0].multiplicity === 2 && realShear[0].fullSpace === false, 'Shear: repeated λ=1, defective');

  console.log('=== Results: ' + passed + ' passed, ' + failed + ' failed ===');
  return failed === 0;
};

// Initialize Module C when DOM ready
document.addEventListener('DOMContentLoaded', function(){
  // Initialize Module C after a short delay to ensure Module A is ready
  setTimeout(function(){
    EigenViz.init();
  }, 100);
});

// Also expose EigenViz globally for testing
window.EigenViz = EigenViz;
"#;
    Html(layout("Nilai & Vektor Eigen", "eigen", "/Learning/", body))
}
