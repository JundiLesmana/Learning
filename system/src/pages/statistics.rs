use axum::response::Html;
use crate::pages::layout::layout;

pub async fn handler() -> Html<String> {
    let body = r#"
<style>
  .stat-layout {
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
  .stat-content { flex: 1; min-width: 0; }

  .stat-header { margin-bottom: 28px; }
  .stat-header .label { margin-bottom: 14px; }
  .stat-header h1 { margin-bottom: 8px; }
  .stat-header p { font-size: 14px; line-height: 1.75; max-width: 560px; }

  /* Concept card */
  .concept-card {
    padding: 24px;
    margin-bottom: 24px;
    border-radius: var(--radius-lg);
  }

  .concept-card h3 { margin-bottom: 10px; }
  .concept-card p { font-size: 13.5px; line-height: 1.75; }

  /* Formula */
  .formula {
    background: var(--bg-base);
    border: 1px solid var(--glass-border);
    border-radius: var(--radius-sm);
    padding: 20px;
    margin: 16px 0;
    font-family: 'Inter', monospace;
    font-size: 16px;
    text-align: center;
    overflow-x: auto;
  }

  .formula-label {
    font-size: 11px;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin-bottom: 8px;
    display: block;
  }

  /* Simulation examples */
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

  .example-card {
    background: var(--bg-layer);
    border: 1px solid var(--glass-border);
    border-radius: var(--radius-lg);
    padding: 24px;
    margin-bottom: 20px;
  }

  .example-title {
    font-size: 14px;
    font-weight: 700;
    color: var(--text-primary);
    margin-bottom: 12px;
  }

  .example-desc {
    font-size: 13px;
    color: var(--text-secondary);
    line-height: 1.7;
    margin-bottom: 16px;
  }

  .chart-container {
    position: relative;
    height: 280px;
    width: 100%;
    margin-bottom: 16px;
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
    gap: 16px;
    margin-top: 16px;
  }

  .stat-box {
    padding: 16px;
    background: var(--glass-bg);
    border-radius: var(--radius-sm);
    border: 1px solid var(--glass-border);
    text-align: center;
  }

  .stat-num {
    font-size: 1.8rem;
    font-weight: 700;
    color: var(--accent);
  }

  .stat-label {
    font-size: 11px;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin-top: 6px;
  }

  .iot-badge {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    background: var(--accent-soft);
    border: 1px solid var(--accent-border);
    border-radius: 100px;
    color: var(--accent);
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    margin-bottom: 16px;
  }

  /* Simulation controls */
  .sim-controls {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
    gap: 12px;
    margin-bottom: 16px;
  }

  .ctrl-group {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .ctrl-label {
    font-size: 11px;
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .ctrl-input {
    padding: 8px 12px;
    border-radius: var(--radius-sm);
    border: 1px solid var(--glass-border);
    background: var(--bg-base);
    color: var(--text-primary);
    font-size: 13px;
    font-family: inherit;
    outline: none;
  }

  .ctrl-input:focus { border-color: var(--accent-border); }

  .btn-gen {
    width: 100%;
    padding: 10px;
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

  .btn-gen:hover { opacity: 0.85; }

  @media (max-width: 680px) {
    .stat-layout { flex-direction: column; }
    .sidebar { width: 100%; }
    .sidebar-inner { position: static; }
  }
</style>

<div class="stat-layout">
  <!-- Sidebar -->
  <aside class="sidebar">
    <div class="sidebar-inner glass">
      <div class="sidebar-title">Pembelajaran</div>
      <ul class="sidebar-nav">
        <span class="sidebar-section-label">Statistika & Probabilitas</span>
        <li>
          <a href="/Learning/Distribusi-Normal/" class="active">
            <span class="dot"></span>Distribusi Normal
          </a>
        </li>
        <li>
          <a href="/Learning/Pointer/">
            <span class="dot"></span>Pointer Simulation
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
      </ul>
    </div>
  </aside>

  <!-- Main -->
  <main class="stat-content">
    <div class="stat-header">
      <div class="label">Statistika dan Probabilitas</div>
      <h1 style="margin-top:16px;">Distribusi Normal</h1>
      <p style="margin-top:10px;">
        Distribusi normal (distribusi Gaussian) adalah distribusi probabilitas
        kontinu yang simetris di sekitar rata-rata, menunjukkan data di dekat
        rata-rata lebih sering terjadi dari data di luar rata-rata.
      </p>
    </div>

    <!-- Concept -->
    <div class="concept-card glass">
      <h3>Rumus Distribusi Normal</h3>
      <span class="formula-label">Fungsi Kepadatan Probabilitas (PDF)</span>
      <div class="formula">
        f(x) = (1 / (σ√(2π))) × e^(-((x-μ)² / (2σ²)))
      </div>
      <p style="font-size:12.5px; color:var(--text-muted); margin-top:8px;">
        μ = rata-rata (mean), σ = simpangan baku (standard deviation), e = bilangan Euler
      </p>

      <div class="complexity-row">
        <div class="complexity-item">
          <span class="complexity-label">68%</span>
          <span class="complexity-value">data dalam 1σ</span>
        </div>
        <div class="complexity-item">
          <span class="complexity-label">95%</span>
          <span class="complexity-value">data dalam 2σ</span>
        </div>
        <div class="complexity-item">
          <span class="complexity-label">99.7%</span>
          <span class="complexity-value">data dalam 3σ</span>
        </div>
      </div>
    </div>

    <!-- IoT Application -->
    <div class="sim-card glass">
      <div class="sim-title">Penerapan Distribusi Normal pada Internet of Things (IoT)</div>

      <span class="iot-badge">IoT Sensor Data Analysis</span>

      <!-- Contoh 1: Suhu -->
      <div class="example-card">
        <div class="example-title">Contoh 1: Suhu Ruangan Sensor IoT</div>
        <div class="example-desc">
          Sensor IoT mencatat suhu ruangan. Atur parameter distribusi lalu
          generate simulasi data suhu.
        </div>
        <div class="sim-controls">
          <div class="ctrl-group">
            <div class="ctrl-label">Rata-rata (°C)</div>
            <input class="ctrl-input" id="tempMean" type="number" value="22.5" step="0.1" />
          </div>
          <div class="ctrl-group">
            <div class="ctrl-label">Std Dev (°C)</div>
            <input class="ctrl-input" id="tempStd" type="number" value="1.8" step="0.1" />
          </div>
          <div class="ctrl-group">
            <div class="ctrl-label">Sampel</div>
            <input class="ctrl-input" id="tempCount" type="number" value="144" step="1" />
          </div>
          <button class="btn-gen" onclick="genChart('temp')">Generate</button>
        </div>
        <div class="chart-container">
          <canvas id="chartTemp"></canvas>
        </div>
        <div class="stats-grid">
          <div class="stat-box">
            <div class="stat-num" id="tempMeanVal">22.5</div>
            <div class="stat-label">Rata-rata (°C)</div>
          </div>
          <div class="stat-box">
            <div class="stat-num" id="tempStdVal">1.8</div>
            <div class="stat-label">Std Dev (°C)</div>
          </div>
          <div class="stat-box">
            <div class="stat-num" id="tempCountVal">144</div>
            <div class="stat-label">Sampel</div>
          </div>
        </div>
      </div>

      <!-- Contoh 2: Detak Jantung -->
      <div class="example-card">
        <div class="example-title">Contoh 2: Detak Jantung Pengguna Wearable</div>
        <div class="example-desc">
          Perangkat wearable IoT memantau detak jantung. Atur parameter dan
          generate simulasi data detak jantung.
        </div>
        <div class="sim-controls">
          <div class="ctrl-group">
            <div class="ctrl-label">Rata-rata (bpm)</div>
            <input class="ctrl-input" id="heartMean" type="number" value="72" step="1" />
          </div>
          <div class="ctrl-group">
            <div class="ctrl-label">Std Dev (bpm)</div>
            <input class="ctrl-input" id="heartStd" type="number" value="6" step="1" />
          </div>
          <div class="ctrl-group">
            <div class="ctrl-label">Sampel</div>
            <input class="ctrl-input" id="heartCount" type="number" value="288" step="1" />
          </div>
          <button class="btn-gen" onclick="genChart('heart')">Generate</button>
        </div>
        <div class="chart-container">
          <canvas id="chartHeart"></canvas>
        </div>
        <div class="stats-grid">
          <div class="stat-box">
            <div class="stat-num" id="heartMeanVal">72</div>
            <div class="stat-label">Rata-rata (bpm)</div>
          </div>
          <div class="stat-box">
            <div class="stat-num" id="heartStdVal">6</div>
            <div class="stat-label">Std Dev (bpm)</div>
          </div>
          <div class="stat-box">
            <div class="stat-num" id="heartCountVal">288</div>
            <div class="stat-label">Sampel</div>
          </div>
        </div>
      </div>

      <!-- Contoh 3: Tegangan -->
      <div class="example-card">
        <div class="example-title">Contoh 3: Tegangan Listrik Sensor IoT</div>
        <div class="example-desc">
          Sensor tegangan listrik mencatat fluktuasi tegangan. Atur parameter
          dan generate simulasi data tegangan.
        </div>
        <div class="sim-controls">
          <div class="ctrl-group">
            <div class="ctrl-label">Rata-rata (V)</div>
            <input class="ctrl-input" id="voltMean" type="number" value="5.0" step="0.1" />
          </div>
          <div class="ctrl-group">
            <div class="ctrl-label">Std Dev (V)</div>
            <input class="ctrl-input" id="voltStd" type="number" value="0.15" step="0.01" />
          </div>
          <div class="ctrl-group">
            <div class="ctrl-label">Sampel</div>
            <input class="ctrl-input" id="voltCount" type="number" value="500" step="1" />
          </div>
          <button class="btn-gen" onclick="genChart('volt')">Generate</button>
        </div>
        <div class="chart-container">
          <canvas id="chartVoltage"></canvas>
        </div>
        <div class="stats-grid">
          <div class="stat-box">
            <div class="stat-num" id="voltMeanVal">5.0</div>
            <div class="stat-label">Rata-rata (V)</div>
          </div>
          <div class="stat-box">
            <div class="stat-num" id="voltStdVal">0.15</div>
            <div class="stat-label">Std Dev (V)</div>
          </div>
          <div class="stat-box">
            <div class="stat-num" id="voltCountVal">500</div>
            <div class="stat-label">Sampel</div>
          </div>
        </div>
      </div>
    </div>

    <!-- Complexity table -->
    <div class="concept-card glass">
      <h3>Referensi Z-Score</h3>
      <table class="complexity-table">
        <thead>
          <tr>
            <td>Z-Score</td>
            <td>Persentil</td>
            <td>Area di bawah kurva</td>
          </tr>
        </thead>
        <tbody>
          <tr>
            <td>-3.0</td>
            <td>0.13%</td>
            <td>0.13% data di bawah -3σ</td>
          </tr>
          <tr>
            <td>-2.0</td>
            <td>2.28%</td>
            <td>2.28% data di bawah -2σ</td>
          </tr>
          <tr>
            <td>-1.0</td>
            <td>15.87%</td>
            <td>15.87% data di bawah -1σ</td>
          </tr>
          <tr>
            <td>0.0</td>
            <td>50.00%</td>
            <td>50% data di bawah mean</td>
          </tr>
          <tr>
            <td>1.0</td>
            <td>84.13%</td>
            <td>84.13% data di bawah +1σ</td>
          </tr>
          <tr>
            <td>2.0</td>
            <td>97.72%</td>
            <td>97.72% data di bawah +2σ</td>
          </tr>
          <tr>
            <td>3.0</td>
            <td>99.87%</td>
            <td>99.87% data di bawah +3σ</td>
          </tr>
        </tbody>
      </table>
    </div>
  </main>
</div>

<script src="https://cdn.jsdelivr.net/npm/chart.js"></script>

<script>
var charts = {};

function generateNormalData(mean, std, count) {
  var data = [];
  for (var i = 0; i < count; i++) {
    var u1 = Math.random();
    var u2 = Math.random();
    var z = Math.sqrt(-2 * Math.log(u1)) * Math.cos(2 * Math.PI * u2);
    data.push(mean + z * std);
  }
  return data;
}

function createHistogram(data, bins, rangeMin, rangeMax) {
  var binWidth = (rangeMax - rangeMin) / bins;
  var histogram = new Array(bins).fill(0);
  var labels = [];
  for (var i = 0; i < bins; i++) {
    labels.push((rangeMin + i * binWidth).toFixed(1));
  }
  for (var i = 0; i < data.length; i++) {
    var binIdx = Math.floor((data[i] - rangeMin) / binWidth);
    if (binIdx >= 0 && binIdx < bins) {
      histogram[binIdx]++;
    }
  }
  return { labels: labels, data: histogram };
}

function updateChart(chartId, canvasId, mean, std, count, rangeMin, rangeMax, color, label) {
  var data = generateNormalData(mean, std, count);
  var hist = createHistogram(data, 20, rangeMin, rangeMax);

  if (charts[chartId]) {
    charts[chartId].destroy();
  }

  charts[chartId] = new Chart(document.getElementById(canvasId), {
    type: 'bar',
    data: {
      labels: hist.labels,
      datasets: [{
        label: label,
        data: hist.data,
        backgroundColor: color.bg,
        borderColor: color.border,
        borderWidth: 1
      }]
    },
    options: {
      responsive: true,
      maintainAspectRatio: false,
      plugins: {
        legend: { display: false },
        title: { display: false }
      },
      scales: {
        y: { beginAtZero: true, grid: { color: 'rgba(255,255,255,0.05)' } },
        x: { grid: { display: false } }
      }
    }
  });
}

function genChart(type) {
  if (type === 'temp') {
    var mean = parseFloat(document.getElementById('tempMean').value) || 22.5;
    var std = parseFloat(document.getElementById('tempStd').value) || 1.8;
    var count = parseInt(document.getElementById('tempCount').value) || 144;
    document.getElementById('tempMeanVal').textContent = mean;
    document.getElementById('tempStdVal').textContent = std;
    document.getElementById('tempCountVal').textContent = count;
    updateChart('temp', 'chartTemp', mean, std, count, 16, 29,
      { bg: 'rgba(108, 142, 255, 0.7)', border: 'rgba(108, 142, 255, 1)' },
      'Suhu Ruangan (°C)');
  } else if (type === 'heart') {
    var mean = parseFloat(document.getElementById('heartMean').value) || 72;
    var std = parseFloat(document.getElementById('heartStd').value) || 6;
    var count = parseInt(document.getElementById('heartCount').value) || 288;
    document.getElementById('heartMeanVal').textContent = mean;
    document.getElementById('heartStdVal').textContent = std;
    document.getElementById('heartCountVal').textContent = count;
    updateChart('heart', 'chartHeart', mean, std, count, 50, 95,
      { bg: 'rgba(255, 107, 107, 0.7)', border: 'rgba(255, 107, 107, 1)' },
      'Detak Jantung (bpm)');
  } else if (type === 'volt') {
    var mean = parseFloat(document.getElementById('voltMean').value) || 5.0;
    var std = parseFloat(document.getElementById('voltStd').value) || 0.15;
    var count = parseInt(document.getElementById('voltCount').value) || 500;
    document.getElementById('voltMeanVal').textContent = mean;
    document.getElementById('voltStdVal').textContent = std;
    document.getElementById('voltCountVal').textContent = count;
    updateChart('volt', 'chartVoltage', mean, std, count, 4.4, 5.6,
      { bg: 'rgba(168, 230, 207, 0.7)', border: 'rgba(168, 230, 207, 1)' },
      'Tegangan (V)');
  }
}

window.addEventListener('DOMContentLoaded', function() {
  genChart('temp');
  genChart('heart');
  genChart('volt');
});
</script>
"#;
    Html(layout("Distribusi Normal", "statistics", "/Learning/", body))
}