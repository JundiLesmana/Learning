use axum::response::Html;
use crate::pages::layout::layout;

pub async fn handler() -> Html<String> {
    let body = r#"
<style>
  .pointer-layout {
    display: flex;
    min-height: calc(100vh - 96px);
    max-width: 980px;
    margin: 0 auto;
    padding: 32px 24px 64px;
    gap: 24px;
  }

  /* Sidebar */
  .sidebar {
    width: 240px;
    flex-shrink: 0;
  }

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
  .pointer-content { flex: 1; min-width: 0; }

  .pointer-header { margin-bottom: 28px; }
  .pointer-header .label { margin-bottom: 14px; }
  .pointer-header h1 { margin-bottom: 8px; }
  .pointer-header p { font-size: 14px; line-height: 1.75; max-width: 560px; }

  /* Concept card */
  .concept-card {
    padding: 24px;
    margin-bottom: 24px;
    border-radius: var(--radius-lg);
  }

  .concept-card h3 { margin-bottom: 10px; }
  .concept-card p { font-size: 13.5px; line-height: 1.75; }

  .complexity-row {
    display: flex;
    gap: 20px;
    margin-top: 16px;
    flex-wrap: wrap;
  }

  .complexity-item {
    display: flex;
    flex-direction: column;
    gap: 3px;
  }

  .complexity-label {
    font-size: 10.5px;
    font-weight: 700;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.06em;
  }

  .complexity-value {
    font-size: 14px;
    font-weight: 600;
    color: var(--text-primary);
    font-variant-numeric: tabular-nums;
  }

  /* Simulation */
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
    margin-bottom: 20px;
  }

  .sim-controls {
    display: flex;
    gap: 10px;
    flex-wrap: wrap;
    align-items: flex-end;
    margin-bottom: 24px;
  }

  .input-group {
    display: flex;
    flex-direction: column;
    gap: 5px;
    flex: 1;
    min-width: 200px;
  }

  .input-group label {
    font-size: 11.5px;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .input-field {
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

  .input-field:focus {
    border-color: var(--accent-border);
    background: var(--glass-hover);
  }

  .btn-row {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
  }

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

  .btn-reset {
    padding: 9px 20px;
    border-radius: var(--radius-sm);
    background: var(--glass-bg);
    border: 1px solid var(--glass-border);
    color: var(--text-secondary);
    font-size: 13px;
    font-weight: 600;
    cursor: pointer;
    font-family: inherit;
    transition: background var(--transition);
  }

  .btn-reset:hover { background: var(--glass-hover); color: var(--text-primary); }

  /* Memory visualization */
  .memory-viz {
    position: relative;
    min-height: 200px;
    border: 1px solid var(--glass-border);
    border-radius: var(--radius-sm);
    background: var(--bg-base);
    padding: 20px;
    margin-top: 20px;
    font-family: 'Inter', monospace;
  }

  .memory-label {
    font-size: 11px;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin-bottom: 12px;
    font-weight: 600;
  }

  .memory-cell {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 12px;
    border-bottom: 1px solid var(--glass-border);
    font-size: 13px;
  }

  .memory-cell:last-child { border-bottom: none; }

  .addr-col {
    width: 80px;
    color: var(--accent);
    font-weight: 600;
  }

  .val-col {
    flex: 1;
    color: var(--text-primary);
  }

  .ptr-col {
    width: 120px;
    color: var(--success);
    font-weight: 600;
  }

  .pointer-arrow {
    color: var(--warning);
    font-weight: 700;
    margin-right: 6px;
  }

  .null-ptr {
    color: var(--text-muted);
  }

  /* String char counter */
  .counter-card {
    background: var(--bg-layer);
    border: 1px solid var(--glass-border);
    border-radius: var(--radius-lg);
    padding: 24px;
    margin-top: 24px;
  }

  .counter-title {
    font-size: 13px;
    font-weight: 700;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.06em;
    margin-bottom: 16px;
  }

  .counter-result {
    margin-top: 16px;
    padding: 16px;
    background: var(--glass-bg);
    border-radius: var(--radius-sm);
    border: 1px solid var(--glass-border);
  }

  .result-label {
    font-size: 11.5px;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin-bottom: 8px;
  }

  .result-value {
    font-size: 2rem;
    font-weight: 700;
    color: var(--accent);
  }

  .code-block {
    background: var(--bg-base);
    border: 1px solid var(--glass-border);
    border-radius: var(--radius-sm);
    padding: 16px;
    margin-top: 16px;
    font-family: 'Inter', monospace;
    font-size: 12.5px;
    overflow-x: auto;
  }

  .code-line { margin: 0; padding: 2px 0; }
  .code-keyword { color: #c678dd; }
  .code-type { color: #e06c75; }
  .code-func { color: #61afef; }
  .code-string { color: #98c379; }
  .code-comment { color: #5c6370; font-style: italic; }

  @media (max-width: 680px) {
    .pointer-layout { flex-direction: column; }
    .sidebar { width: 100%; }
    .sidebar-inner { position: static; }
  }
</style>

<div class="pointer-layout">
  <!-- Sidebar -->
  <aside class="sidebar">
    <div class="sidebar-inner glass">
      <div class="sidebar-title">Pembelajaran</div>
      <ul class="sidebar-nav">
        <span class="sidebar-section-label">Pointer & Memori</span>
        <li>
          <a href="/Learning/Pointer/" class="active">
            <span class="dot"></span>Pointer Simulation
          </a>
        </li>
        <span class="sidebar-section-label">Statistika & Probabilitas</span>
        <li>
          <a href="/Learning/Distribusi-Normal/">
            <span class="dot"></span>Distribusi Normal
          </a>
        </li>
        <li>
          <a href="/Learning/Insert-Sorting/">
            <span class="dot"></span>Insertion Sort
          </a>
        </li>
        <li>
          <a href="/Learning/Bubble-Sorting/">
            <span class="dot"></span>Bubble Sort
          </a>
        </li>
        <span class="sidebar-section-label">Jaringan Komputer</span>
        <li>
          <a href="/Learning/Jaringan-Komputer/">
            <span class="dot"></span>Jaringan Komputer
          </a>
        </li>
        <span class="sidebar-section-label">Aljabar Linear</span>
        <li>
          <a href="/Learning/Nilai-Eigen/">
            <span class="dot"></span>Nilai &amp; Vektor Eigen
          </a>
        </li>
      </ul>
    </div>
  </aside>

  <!-- Main -->
  <main class="pointer-content">

    <div class="pointer-header">
      <div class="label">Pointer & Memori</div>
      <h1 style="margin-top:16px;">Simulasi Pointer</h1>
      <p style="margin-top:10px;">
        Pointer adalah variabel yang menyimpan alamat memori dari variabel lain.
        Simulasi ini menunjukkan bagaimana pointer merujuk ke lokasi memori
        dan bagaimana nilai dapat diakses melalui dereferensi.
      </p>
    </div>

    <!-- Concept -->
    <div class="concept-card glass">
      <h3>Konsep Dasar Pointer</h3>
      <p>
        Pointer dalam Rust memungkinkan akses langsung ke alamat memori.
        Menggunakan operator <strong style="color:var(--text-primary)">*</strong> untuk
        dereferensi dan <strong style="color:var(--text-primary)">&</strong> untuk
        mengambil referensi alamat memori.
      </p>
      <div class="complexity-row">
        <div class="complexity-item">
          <span class="complexity-label">Dereference</span>
          <span class="complexity-value">*ptr</span>
        </div>
        <div class="complexity-item">
          <span class="complexity-label">Reference</span>
          <span class="complexity-value">&var</span>
        </div>
        <div class="complexity-item">
          <span class="complexity-label">Null Pointer</span>
          <span class="complexity-value">None / null</span>
        </div>
        <div class="complexity-item">
          <span class="complexity-label">Memory Address</span>
          <span class="complexity-value">0x7fff...</span>
        </div>
      </div>
    </div>

    <!-- Simulation -->
    <div class="sim-card glass">
      <div class="sim-title">Pointer Visualization</div>

      <div class="sim-controls">
        <div class="input-group">
          <label for="ptrValue">Nilai Variabel</label>
          <input
            class="input-field"
            id="ptrValue"
            type="text"
            value="42"
            placeholder="Masukkan angka"
          />
        </div>
        <div class="input-group">
          <label for="ptrName">Nama Variabel</label>
          <input
            class="input-field"
            id="ptrName"
            type="text"
            value="angka"
            placeholder="Nama variabel"
          />
        </div>
      </div>

      <div class="btn-row" style="margin-bottom:24px;">
        <button class="btn-run" id="btnRun" onclick="updatePointer()">Tampilkan Pointer</button>
        <button class="btn-reset" onclick="resetPointer()">Reset</button>
      </div>

      <div class="memory-viz" id="memoryViz">
        <div class="memory-label">Memory Layout</div>
        <div id="memoryCells"></div>
      </div>
    </div>

    <!-- String Character Counter -->
    <div class="counter-card glass">
      <div class="counter-title">String Character Counter</div>
      <p style="font-size:13px; color:var(--text-secondary); margin-bottom:16px;">
        Program ini menggunakan fungsi string <strong>.chars().count()</strong> untuk
        menghitung jumlah karakter dari array string yang diinputkan.
      </p>

      <div class="input-group">
        <label for="strArray">Array String (pisahkan dengan koma)</label>
        <input
          class="input-field"
          id="strArray"
          type="text"
          value="Rust, Pemrograman, Pointer, Belajar"
          placeholder="Contoh: hello, world, rust"
        />
      </div>

      <div class="btn-row" style="margin-bottom:16px;">
        <button class="btn-run" id="btnCount" onclick="countChars()">Hitung Karakter</button>
        <button class="btn-reset" onclick="resetCounter()">Reset</button>
      </div>

      <div class="counter-result" id="counterResult">
        <div class="result-label">Total Karakter</div>
        <div class="result-value" id="totalChars">0</div>
      </div>

      <div class="code-block" id="codeBlock">
        <div class="code-line"><span class="code-keyword">fn</span> <span class="code-func">count_chars</span>(<span class="code-keyword">arr</span>: &[<span class="code-type">&str</span>]) -> <span class="code-type">usize</span> {</div>
        <div class="code-line">&nbsp;&nbsp;<span class="code-keyword">let</span> <span class="code-keyword">mut</span> total = <span class="code-string">"0"</span>;</div>
        <div class="code-line">&nbsp;&nbsp;<span class="code-keyword">for</span> s <span class="code-keyword">in</span> arr {</div>
        <div class="code-line">&nbsp;&nbsp;&nbsp;&nbsp;total += s.<span class="code-func">chars</span>().<span class="code-func">count</span>();</div>
        <div class="code-line">&nbsp;&nbsp;}</div>
        <div class="code-line">&nbsp;&nbsp;total</div>
        <div class="code-line">}</div>
        <div class="code-line"></div>
        <div class="code-line"><span class="code-keyword">let</span> data = [<span class="code-string">"Rust"</span>, <span class="code-string">"Pemrograman"</span>];</div>
        <div class="code-line"><span class="code-keyword">let</span> result = <span class="code-func">count_chars</span>(&data);</div>
      </div>
    </div>

    <!-- Complexity table -->
    <div class="concept-card glass">
      <h3>Tabel Referensi Pointer</h3>
      <table class="complexity-table">
        <thead>
          <tr>
            <th>Operator</th>
            <th>Fungsi</th>
            <th>Contoh</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <td>&amp;</td>
            <td>Mengambil referensi / alamat memori</td>
            <td>&amp;variabel</td>
          </tr>
          <tr>
            <td>*</td>
            <td>Dereference (akses nilai via pointer)</td>
            <td>*pointer</td>
          </tr>
          <tr>
            <td>-></td>
            <td>Mengakses field via pointer (struct)</td>
            <td>ptr-&gt;field</td>
          </tr>
          <tr>
            <td>null</td>
            <td>Pointer tidak menunjuk ke mana-mana</td>
            <td>None / null</td>
          </tr>
        </tbody>
      </table>
    </div>
  </main>
</div>

<script>
var memoryCells = document.getElementById('memoryCells');

function updatePointer() {
  var val = document.getElementById('ptrValue').value;
  var name = document.getElementById('ptrName').value || 'variabel';
  var addr = '0x' + Math.floor(Math.random() * 0xfffff).toString(16).toUpperCase().padStart(5, '0');
  var ptrAddr = '0x' + Math.floor(Math.random() * 0xfffff).toString(16).toUpperCase().padStart(5, '0');

  var html = '';
  html += '<div class="memory-cell">';
  html += '<span class="addr-col">' + addr + '</span>';
  html += '<span class="val-col">' + name + ' = ' + val + '</span>';
  html += '<span class="ptr-col">-</span>';
  html += '</div>';
  html += '<div class="memory-cell">';
  html += '<span class="addr-col">' + ptrAddr + '</span>';
  html += '<span class="val-col">Pointer ke ' + name + '</span>';
  html += '<span class="ptr-col"><span class="pointer-arrow">&#9654;</span> ' + addr + '</span>';
  html += '</div>';
  memoryCells.innerHTML = html;
}

function resetPointer() {
  document.getElementById('ptrValue').value = '42';
  document.getElementById('ptrName').value = 'angka';
  memoryCells.innerHTML = '';
}

function countChars() {
  var raw = document.getElementById('strArray').value;
  var arr = raw.split(',').map(function(s){ return s.trim(); }).filter(function(s){ return s.length > 0; });
  var total = 0;
  var details = [];
  for (var i = 0; i < arr.length; i++) {
    var len = arr[i].length;
    total += len;
    details.push(arr[i] + ' = ' + len + ' karakter');
  }
  document.getElementById('totalChars').textContent = total;
  document.getElementById('counterResult').innerHTML =
    '<div class="result-label">Total Karakter</div>' +
    '<div class="result-value" id="totalChars">' + total + '</div>' +
    '<div style="margin-top:12px;font-size:12px;color:var(--text-secondary);">' +
    details.join(', ') + '</div>';

  // Update code block dynamically based on input
  var codeLines = [];
  codeLines.push('<span class="code-keyword">fn</span> <span class="code-func">count_chars</span>(<span class="code-keyword">arr</span>: &[<span class="code-type">&str</span>]) -> <span class="code-type">usize</span> {');
  codeLines.push('&nbsp;&nbsp;<span class="code-keyword">let</span> <span class="code-keyword">mut</span> total = <span class="code-string">"0"</span>;');
  codeLines.push('&nbsp;&nbsp;<span class="code-keyword">for</span> s <span class="code-keyword">in</span> arr {');
  codeLines.push('&nbsp;&nbsp;&nbsp;&nbsp;total += s.<span class="code-func">chars</span>().<span class="code-func">count</span>();');
  codeLines.push('&nbsp;&nbsp;}');
  codeLines.push('&nbsp;&nbsp;total');
  codeLines.push('}');
  codeLines.push('');
  codeLines.push('<span class="code-keyword">let</span> data = [' + arr.map(function(s){ return '<span class="code-string">"' + s + '"</span>'; }).join(', ') + '];');
  codeLines.push('<span class="code-keyword">let</span> result = <span class="code-func">count_chars</span>(&data);');
  codeLines.push('<span class="code-comment">// result = ' + total + ' karakter</span>');

  document.getElementById('codeBlock').innerHTML = codeLines.join('<br>');
}

function resetCounter() {
  document.getElementById('strArray').value = 'Rust, Pemrograman, Pointer, Belajar';
  document.getElementById('totalChars').textContent = '0';
  document.getElementById('counterResult').innerHTML =
    '<div class="result-label">Total Karakter</div>' +
    '<div class="result-value" id="totalChars">0</div>';

  // Reset code block to default
  var codeLines = [];
  codeLines.push('<span class="code-keyword">fn</span> <span class="code-func">count_chars</span>(<span class="code-keyword">arr</span>: &[<span class="code-type">&str</span>]) -> <span class="code-type">usize</span> {');
  codeLines.push('&nbsp;&nbsp;<span class="code-keyword">let</span> <span class="code-keyword">mut</span> total = <span class="code-string">"0"</span>;');
  codeLines.push('&nbsp;&nbsp;<span class="code-keyword">for</span> s <span class="code-keyword">in</span> arr {');
  codeLines.push('&nbsp;&nbsp;&nbsp;&nbsp;total += s.<span class="code-func">chars</span>().<span class="code-func">count</span>();');
  codeLines.push('&nbsp;&nbsp;}');
  codeLines.push('&nbsp;&nbsp;total');
  codeLines.push('}');
  codeLines.push('');
  codeLines.push('<span class="code-keyword">let</span> data = [<span class="code-string">"Rust"</span>, <span class="code-string">"Pemrograman"</span>, <span class="code-string">"Pointer"</span>, <span class="code-string">"Belajar"</span>];');
  codeLines.push('<span class="code-keyword">let</span> result = <span class="code-func">count_chars</span>(&data);');
  codeLines.push('<span class="code-comment">// result = 22 karakter</span>');

  document.getElementById('codeBlock').innerHTML = codeLines.join('<br>');
}

// Init on load
window.addEventListener('DOMContentLoaded', function() {
  resetPointer();
  resetCounter();
});
</script>
"#;
    Html(layout("Pointer Simulation", "pointer", "/Learning/", body))
}