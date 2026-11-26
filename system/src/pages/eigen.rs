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

    <!-- ============ MODULE C (stub) ============ -->
    <div class="module-panel" id="modC" role="tabpanel">
      <div class="sim-card glass">
        <div class="sim-title">Modul C &middot; Visualisasi Geometri 2D</div>
        <p class="sim-desc">
          Visualisasi vektor x dan Ax, garis ruang eigen, animasi transformasi
          grid &amp; lingkaran satuan menjadi elips, serta panel angka.
          <strong>Modul C akan tersedia pada tahap berikutnya (hanya matriks 2&times;2).</strong>
        </p>
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
   UI - Modul A
   ===================================================================== */
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
    var sumRoots = Fraction.ZERO(), prodRoots = Fraction.ONE();
    var allExact = roots.every(function(r){ return r.exact; });
    roots.forEach(function(r){ sumRoots = sumRoots.add(r.frac || new Fraction(Math.round(r.re * 1e4) / 1e4, 1)); prodRoots = prodRoots.mul(r.frac || new Fraction(Math.round(r.re * 1e4) / 1e4, 1)); });
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
    note: [[ new Fraction(3), new Fraction(2) ], [ new Fraction(-1), new Fraction(0) ]],
    diag: [[ new Fraction(2), new Fraction(0) ], [ new Fraction(0), new Fraction(5) ]],
    sym:  [[ new Fraction(2), new Fraction(1) ], [ new Fraction(1), new Fraction(2) ]],
    rep:  [[ new Fraction(2), new Fraction(1) ], [ new Fraction(0), new Fraction(2) ]],
    comp: [[ new Fraction(0), new Fraction(-1) ], [ new Fraction(1), new Fraction(0) ]]
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
  }

  function randomMatrix(){
    var m = [];
    for (var i = 0; i < n; i++){
      var row = [];
      for (var j = 0; j < n; j++) row.push(new Fraction(Math.floor(Math.random() * 11) - 5));
      m.push(row);
    }
    A = m;
    buildGrid();
    $('errA').classList.remove('show');
    $('stepsWrap').innerHTML = '';
    $('eigvalsA').innerHTML = '';
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
    $('resetA').addEventListener('click', function(){ applyPreset('note'); });

    // module tabs
    var tabs = document.querySelectorAll('.module-tab');
    var panels = document.querySelectorAll('.module-panel');
    for (var t = 0; t < tabs.length; t++){
      tabs[t].addEventListener('click', function(){
        var target = this.getAttribute('data-target');
        for (var i = 0; i < tabs.length; i++){ tabs[i].classList.toggle('active', tabs[i] === this); tabs[i].setAttribute('aria-selected', tabs[i] === this); }
        for (var i = 0; i < panels.length; i++){ panels[i].classList.toggle('active', panels[i].id === target); }
      });
    }

    // term chips keyboard
    var chips = document.querySelectorAll('.term-chip');
    for (var c = 0; c < chips.length; c++){
      chips[c].addEventListener('keydown', function(e){ if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); this.classList.toggle('focused'); } });
    }
  }

  document.addEventListener('DOMContentLoaded', init);
  return { calc: calc, readMatrix: readMatrix };
})();
</script>
"#;
    Html(layout("Nilai & Vektor Eigen", "eigen", "/Learning/", body))
}
