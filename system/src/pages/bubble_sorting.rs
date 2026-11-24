use axum::response::Html;
use crate::pages::layout::layout;

pub async fn handler() -> Html<String> {
    let body = r#"
<style>
  .sort-layout {
    display: flex;
    min-height: calc(100vh - 96px);
    max-width: 980px;
    margin: 0 auto;
    padding: 32px 24px 64px;
    gap: 24px;
  }

  .sidebar { width: 240px; flex-shrink: 0; }
  .sidebar-inner { position: sticky; top: 88px; padding: 20px 16px; border-radius: var(--radius-lg); }
  .sidebar-title { font-size: 10.5px; font-weight: 700; letter-spacing: 0.1em; text-transform: uppercase; color: var(--text-muted); margin-bottom: 12px; padding: 0 8px; }
  .sidebar-nav { list-style: none; }
  .sidebar-nav li + li { margin-top: 2px; }
  .sidebar-section-label { font-size: 10px; font-weight: 700; letter-spacing: 0.08em; text-transform: uppercase; color: var(--text-muted); padding: 12px 8px 4px; display: block; }
  .sidebar-nav a { display: flex; align-items: center; gap: 8px; padding: 8px 10px; border-radius: var(--radius-sm); text-decoration: none; color: var(--text-secondary); font-size: 13px; font-weight: 500; transition: color var(--transition), background var(--transition); }
  .sidebar-nav a:hover { color: var(--text-primary); background: var(--glass-hover); }
  .sidebar-nav a.active { color: var(--accent); background: var(--accent-soft); }
  .sidebar-nav a .dot { width: 6px; height: 6px; border-radius: 50%; background: currentColor; flex-shrink: 0; opacity: 0.6; }

  .sort-content { flex: 1; min-width: 0; }
  .sort-header { margin-bottom: 28px; }
  .sort-header .label { margin-bottom: 14px; }
  .sort-header h1 { margin-bottom: 8px; }
  .sort-header p { font-size: 14px; line-height: 1.75; max-width: 560px; }

  .concept-card { padding: 24px; margin-bottom: 24px; border-radius: var(--radius-lg); }
  .concept-card h3 { margin-bottom: 10px; }
  .concept-card p { font-size: 13.5px; line-height: 1.75; }
  .complexity-row { display: flex; gap: 20px; margin-top: 16px; flex-wrap: wrap; }
  .complexity-item { display: flex; flex-direction: column; gap: 3px; }
  .complexity-label { font-size: 10.5px; font-weight: 700; color: var(--text-muted); text-transform: uppercase; letter-spacing: 0.06em; }
  .complexity-value { font-size: 14px; font-weight: 600; color: var(--text-primary); font-variant-numeric: tabular-nums; }

  .sim-card { padding: 28px; border-radius: var(--radius-lg); margin-bottom: 24px; }
  .sim-title { font-size: 13px; font-weight: 700; color: var(--text-secondary); text-transform: uppercase; letter-spacing: 0.06em; margin-bottom: 20px; }
  .sim-controls { display: flex; gap: 10px; flex-wrap: wrap; align-items: flex-end; margin-bottom: 24px; }
  .input-group { display: flex; flex-direction: column; gap: 5px; flex: 1; min-width: 200px; }
  .input-group label { font-size: 11.5px; font-weight: 600; color: var(--text-muted); text-transform: uppercase; letter-spacing: 0.05em; }
  .input-field { padding: 9px 14px; border-radius: var(--radius-sm); border: 1px solid var(--glass-border); background: var(--bg-base); color: var(--text-primary); font-size: 13.5px; font-family: 'Inter', monospace; transition: border-color var(--transition), background var(--transition); outline: none; width: 100%; }
  .input-field:focus { border-color: var(--accent-border); background: var(--glass-hover); }
  .speed-group { display: flex; flex-direction: column; gap: 5px; }
  .speed-group label { font-size: 11.5px; font-weight: 600; color: var(--text-muted); text-transform: uppercase; letter-spacing: 0.05em; }
  .speed-select { padding: 9px 14px; border-radius: var(--radius-sm); border: 1px solid var(--glass-border); background: var(--bg-base); color: var(--text-primary); font-size: 13.5px; font-family: inherit; cursor: pointer; outline: none; transition: border-color var(--transition); }
  .speed-select:focus { border-color: var(--accent-border); }
  .btn-row { display: flex; gap: 8px; flex-wrap: wrap; }
  .btn-run { padding: 9px 20px; border-radius: var(--radius-sm); background: var(--accent); color: #fff; font-size: 13px; font-weight: 600; border: none; cursor: pointer; font-family: inherit; transition: opacity var(--transition); }
  .btn-run:hover { opacity: 0.85; }
  .btn-run:disabled { opacity: 0.4; cursor: not-allowed; }
  .btn-reset { padding: 9px 20px; border-radius: var(--radius-sm); background: var(--glass-bg); border: 1px solid var(--glass-border); color: var(--text-secondary); font-size: 13px; font-weight: 600; cursor: pointer; font-family: inherit; transition: background var(--transition); }
  .btn-reset:hover { background: var(--glass-hover); color: var(--text-primary); }

  .viz-area { min-height: 220px; display: flex; align-items: flex-end; gap: 5px; padding: 16px 0 0; position: relative; }
  .bar-wrap { display: flex; flex-direction: column; align-items: center; flex: 1; gap: 5px; }
  .bar { width: 100%; border-radius: 4px 4px 0 0; background: var(--glass-border); transition: height 0.3s cubic-bezier(0.4,0,0.2,1), background 0.2s; min-height: 4px; }
  .bar.comparing  { background: var(--warning); }
  .bar.swapping   { background: #f87171; }
  .bar.sorted     { background: var(--success); }
  .bar.pivot      { background: var(--accent); }

  .step-log { margin-top: 20px; padding: 16px; border-radius: var(--radius-sm); background: var(--bg-base); border: 1px solid var(--glass-border); min-height: 48px; font-size: 13px; color: var(--text-secondary); line-height: 1.6; }
  .step-log strong { color: var(--accent); }

  .complexity-table { width: 100%; border-collapse: collapse; font-size: 13px; margin-top: 12px; }
  .complexity-table th { text-align: left; padding: 10px 12px; border-bottom: 1px solid var(--glass-border); font-size: 11px; font-weight: 700; text-transform: uppercase; letter-spacing: 0.06em; color: var(--text-muted); }
  .complexity-table td { padding: 10px 12px; border-bottom: 1px solid var(--glass-border); color: var(--text-secondary); }
  .complexity-table td:first-child { color: var(--text-primary); font-weight: 500; }
  .complexity-table tr:last-child td { border-bottom: none; }

  @media (max-width: 680px) {
    .sort-layout { flex-direction: column; }
    .sidebar { width: 100%; }
    .sidebar-inner { position: static; }
  }
</style>

<div class="sort-layout">
  <aside class="sidebar">
    <div class="sidebar-inner glass">
      <div class="sidebar-title">Pembelajaran</div>
      <ul class="sidebar-nav">
        <span class="sidebar-section-label">Algoritma Sorting</span>
        <li>
          <a href="/Insert-Sorting">
            <span class="dot"></span>Insertion Sort
          </a>
        </li>
        <li>
          <a href="/Bubble-Sorting" class="active">
            <span class="dot"></span>Bubble Sort
          </a>
        </li>
      </ul>
    </div>
  </aside>

  <main class="sort-content">
    <div class="sort-header">
      <div class="label">Algoritma / Sorting</div>
      <h1 style="margin-top:16px;">Bubble Sort</h1>
      <p style="margin-top:10px;">
        Bubble sort membandingkan dua elemen yang berdekatan dan menukar posisinya
        apabila urutan salah. Proses diulang hingga seluruh array terurut.
      </p>
    </div>

    <div class="concept-card glass">
      <h3>Konsep Dasar</h3>
      <p>
        Pada setiap iterasi (pass), algoritma menelusuri array dari indeks pertama
        hingga akhir. Dua elemen yang berdekatan dibandingkan; jika elemen kiri
        lebih besar, keduanya ditukar. Elemen dengan nilai terbesar akan
        <strong style="color:var(--text-primary)">"mengambang"</strong> (bubble up)
        ke posisi paling akhir setelah setiap pass.
      </p>
      <div class="complexity-row">
        <div class="complexity-item">
          <span class="complexity-label">Best Case</span>
          <span class="complexity-value">O(n)</span>
        </div>
        <div class="complexity-item">
          <span class="complexity-label">Average Case</span>
          <span class="complexity-value">O(n&sup2;)</span>
        </div>
        <div class="complexity-item">
          <span class="complexity-label">Worst Case</span>
          <span class="complexity-value">O(n&sup2;)</span>
        </div>
        <div class="complexity-item">
          <span class="complexity-label">Space</span>
          <span class="complexity-value">O(1)</span>
        </div>
      </div>
    </div>

    <div class="sim-card glass">
      <div class="sim-title">Simulasi Interaktif</div>

      <div class="sim-controls">
        <div class="input-group">
          <label for="arrInput">Input Array (pisahkan dengan koma)</label>
          <input
            class="input-field"
            id="arrInput"
            type="text"
            value="64, 34, 25, 12, 22, 11, 90"
            placeholder="Contoh: 5, 2, 8, 1, 9"
          />
        </div>
        <div class="speed-group">
          <label for="speedSel">Kecepatan</label>
          <select class="speed-select" id="speedSel">
            <option value="800">Lambat</option>
            <option value="400" selected>Normal</option>
            <option value="150">Cepat</option>
          </select>
        </div>
      </div>

      <div class="btn-row" style="margin-bottom:24px;">
        <button class="btn-run" id="btnRun" onclick="startSort()">Mulai Sorting</button>
        <button class="btn-reset" onclick="resetSort()">Reset</button>
        <button class="btn-run" id="btnRandom" onclick="randomize()" style="background:var(--glass-bg);border:1px solid var(--glass-border);color:var(--text-secondary);">Acak Data</button>
      </div>

      <div class="viz-area" id="vizArea"></div>

      <div class="step-log" id="stepLog">
        Masukkan angka lalu tekan <strong>Mulai Sorting</strong> untuk melihat simulasi.
      </div>
    </div>

    <div class="concept-card glass">
      <h3>Tabel Kompleksitas</h3>
      <table class="complexity-table">
        <thead>
          <tr>
            <th>Kasus</th>
            <th>Time Complexity</th>
            <th>Keterangan</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <td>Best Case</td>
            <td>O(n)</td>
            <td>Array sudah terurut (dengan flag optimasi)</td>
          </tr>
          <tr>
            <td>Average Case</td>
            <td>O(n&sup2;)</td>
            <td>Array acak</td>
          </tr>
          <tr>
            <td>Worst Case</td>
            <td>O(n&sup2;)</td>
            <td>Array terurut terbalik</td>
          </tr>
          <tr>
            <td>Space</td>
            <td>O(1)</td>
            <td>In-place, tidak butuh memori tambahan</td>
          </tr>
        </tbody>
      </table>
    </div>
  </main>
</div>

<script>
var arr = [];
var steps = [];
var stepIdx = 0;
var timer = null;
var maxVal = 1;

function parseInput() {
  var raw = document.getElementById('arrInput').value;
  return raw.split(',').map(function(s){ return parseInt(s.trim(), 10); }).filter(function(n){ return !isNaN(n); });
}

function randomize() {
  var len = 7 + Math.floor(Math.random() * 5);
  var nums = [];
  for (var i = 0; i < len; i++) nums.push(Math.floor(Math.random() * 90) + 10);
  document.getElementById('arrInput').value = nums.join(', ');
  resetSort();
}

function buildSteps(a) {
  var steps = [];
  var arr = a.slice();
  var n = arr.length;
  steps.push({ arr: arr.slice(), i: -1, j: -1, sortedFrom: n, type: 'init', msg: 'Array awal: [' + arr.join(', ') + ']' });

  for (var i = 0; i < n - 1; i++) {
    var swapped = false;
    for (var j = 0; j < n - i - 1; j++) {
      steps.push({ arr: arr.slice(), i: i, j: j, sortedFrom: n - i, type: 'compare',
        msg: 'Pass ' + (i+1) + ': Bandingkan indeks ' + j + ' (' + arr[j] + ') dan ' + (j+1) + ' (' + arr[j+1] + ')' });
      if (arr[j] > arr[j + 1]) {
        var tmp = arr[j]; arr[j] = arr[j+1]; arr[j+1] = tmp;
        swapped = true;
        steps.push({ arr: arr.slice(), i: i, j: j, sortedFrom: n - i, type: 'swap',
          msg: arr[j] + ' < ' + arr[j+1] + ': tukar posisi ' + j + ' dan ' + (j+1) });
      }
    }
    steps.push({ arr: arr.slice(), i: i, j: -1, sortedFrom: n - i - 1, type: 'pass_done',
      msg: 'Pass ' + (i+1) + ' selesai. Elemen terbesar (' + arr[n-i-1] + ') sudah di posisi benar.' });
    if (!swapped) {
      steps.push({ arr: arr.slice(), i: i, j: -1, sortedFrom: 0, type: 'done',
        msg: 'Tidak ada pertukaran pada pass ini. Array sudah terurut.' });
      return steps;
    }
  }
  steps.push({ arr: arr.slice(), i: -1, j: -1, sortedFrom: 0, type: 'done',
    msg: 'Selesai. Array terurut: [' + arr.join(', ') + ']' });
  return steps;
}

function getMaxVal(arr) { return Math.max.apply(null, arr.map(Math.abs).concat([1])); }

function renderBars(arr, j, sortedFrom, type) {
  var area = document.getElementById('vizArea');
  var maxV = maxVal;
  var maxH = 180;
  area.innerHTML = '';
  arr.forEach(function(v, i) {
    var wrap = document.createElement('div');
    wrap.className = 'bar-wrap';

    var bar = document.createElement('div');
    var h = Math.max(10, Math.round((Math.abs(v) / maxV) * maxH));
    bar.style.height = h + 'px';

    if (type === 'done') bar.className = 'bar sorted';
    else if (i >= sortedFrom) bar.className = 'bar sorted';
    else if ((i === j || i === j + 1) && type === 'swap') bar.className = 'bar swapping';
    else if ((i === j || i === j + 1) && type === 'compare') bar.className = 'bar comparing';
    else bar.className = 'bar';

    var lbl = document.createElement('div');
    lbl.className = 'bar-label';
    lbl.textContent = v;

    wrap.appendChild(bar);
    wrap.appendChild(lbl);
    area.appendChild(wrap);
  });
}

function resetSort() {
  clearTimeout(timer);
  steps = []; stepIdx = 0;
  arr = parseInput();
  if (arr.length === 0) return;
  maxVal = getMaxVal(arr);
  renderBars(arr, -1, arr.length, 'init');
  document.getElementById('stepLog').innerHTML = 'Tekan <strong>Mulai Sorting</strong> untuk memulai.';
  document.getElementById('btnRun').disabled = false;
}

function startSort() {
  arr = parseInput();
  if (arr.length < 2) {
    document.getElementById('stepLog').innerHTML = '<strong>Masukkan minimal 2 angka.</strong>';
    return;
  }
  steps = buildSteps(arr);
  stepIdx = 0;
  maxVal = getMaxVal(arr);
  document.getElementById('btnRun').disabled = true;
  runStep();
}

function runStep() {
  if (stepIdx >= steps.length) {
    document.getElementById('btnRun').disabled = false;
    return;
  }
  var s = steps[stepIdx];
  renderBars(s.arr, s.j, s.sortedFrom, s.type);
  document.getElementById('stepLog').innerHTML = s.msg;
  stepIdx++;
  var spd = parseInt(document.getElementById('speedSel').value, 10);
  timer = setTimeout(runStep, spd);
}

window.addEventListener('DOMContentLoaded', function() { resetSort(); });
</script>
"#;
    Html(layout("Bubble Sort", "bubble", body))
}
