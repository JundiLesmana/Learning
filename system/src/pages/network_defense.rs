// use axum::response::Html;

pub fn defense_html() -> String {
    r##"<div class="module-panel" id="mod2" role="tabpanel">
      <div class="sim-card glass">
        <div class="sim-title">Modul 2 &mdash; Defense in Depth</div>
        <div class="mon-question">
          <b>Soal:</b> Mengapa konsep defense in depth dianggap penting dalam pengamanan jaringan?
        </div>

        <div class="error-box" id="defErr" role="alert"></div>

        <!-- Input Section -->
        <div class="sim-title" style="margin-top:16px;">Konfigurasi Lapisan Pertahanan</div>
        <div class="sim-controls" style="margin-bottom:16px;">
          <div class="input-group small">
            <label for="defPreset">Preset Konfigurasi</label>
            <select class="input-field" id="defPreset">
              <option value="full">Defense in Depth Penuh</option>
              <option value="standard">Standar Kantor</option>
              <option value="firewall">Hanya Firewall</option>
              <option value="none">Tanpa Pertahanan</option>
            </select>
          </div>
          <div class="input-group small">
            <label for="defAttack">Skenario Serangan</label>
            <select class="input-field" id="defAttack"></select>
          </div>
          <div class="input-group small">
            <label for="defEntry">Titik Masuk</label>
            <select class="input-field" id="defEntry"></select>
          </div>
          <div class="input-group small">
            <label for="defTarget">Aset Target</label>
            <select class="input-field" id="defTarget"></select>
          </div>
        </div>

        <div class="sim-controls" style="margin-bottom:16px;">
          <div class="input-group small"><label for="defTrials">Jumlah Percobaan (N)</label><input class="input-field" id="defTrials" type="number" min="10" max="10000" value="1000" /></div>
          <div class="input-group small"><label for="defSeed">Seed</label><input class="input-field" id="defSeed" type="number" value="42" /></div>
          <div class="input-group small"><label for="defBaseDelay">Tunda Dasar (menit/lapisan)</label><input class="input-field" id="defBaseDelay" type="number" min="1" max="60" value="5" /></div>
          <div style="flex:0 0 auto;display:flex;align-items:flex-end;"><button class="btn-ghost" id="defRandomSeed" style="height:38px;">Seed Acak</button></div>
        </div>

        <div class="btn-row" style="margin-bottom:20px;">
          <button class="btn-run" id="defRun">Jalankan Simulasi</button>
          <button class="btn-ghost" id="defExample">Contoh</button>
          <button class="btn-reset" id="defReset">Reset</button>
          <button class="btn-ghost" id="defExportJson">Ekspor JSON</button>
          <button class="btn-ghost" id="defCopyText">Salin Teks</button>
        </div>

        <!-- Layer Config Table -->
        <div class="sim-title" style="margin-top:16px;">Detail Lapisan (klik untuk edit kekuatan/aktif)</div>
        <div style="overflow-x:auto;">
          <table class="data-table" id="defLayerTable">
            <thead>
              <tr>
                <th>Lapisan</th><th>Nama</th><th>Aktif</th><th>Kekuatan (0-100)</th><th>Kontrol Contoh</th><th>Deteksi</th>
              </tr>
            </thead>
            <tbody></tbody>
          </table>
        </div>

        <!-- Effectiveness Matrix Heatmap -->
        <div class="sim-title" style="margin-top:20px;">Matriks Efektivitas (Serangan × Lapisan) — nilai 0-1, editable</div>
        <p class="hint" style="margin-bottom:10px;">Angka adalah asumsi edukatif, bukan data empiris. Edit sel, lalu klik "Kembalikan default" untuk reset.</p>
        <div style="overflow-x:auto;">
          <table class="data-table" id="defMatrixTable">
            <thead>
              <tr><th>Serangan</th><th>L1</th><th>L2</th><th>L3</th><th>L4</th><th>L5</th><th>L6</th><th>L7</th></tr>
            </thead>
            <tbody></tbody>
          </table>
        </div>
        <div class="btn-row" style="margin-top:8px;"><button class="btn-ghost" id="defMatrixReset">Kembalikan Default</button></div>

        <!-- 3D/2D Scene -->
        <div class="sim-title" style="margin-top:20px;">Diorama Benteng Berlapis</div>
        <div class="scene-toolbar">
          <button class="btn-ghost active" id="defView3d">3D</button>
          <button class="btn-ghost" id="defView2d">Mode 2D (SVG)</button>
          <button class="btn-ghost" id="defViewTable">Tabel Aksesibel</button>
          <span class="spacer"></span>
          <button class="btn-ghost" id="defAnimPlay" style="display:none;">Play</button>
          <button class="btn-ghost" id="defAnimPause" style="display:none;">Pause</button>
          <button class="btn-ghost" id="defAnimRestart" style="display:none;">Ulangi</button>
          <div class="input-group small" style="min-width:140px;display:none;" id="defSpeedWrap"><label for="defAnimSpeed">Kecepatan</label><select class="input-field" id="defAnimSpeed"><option value="0.5">0.5x</option><option value="1" selected>1x</option><option value="2">2x</option><option value="4">4x</option></select></div>
        </div>

        <div class="scene-wrap">
          <div id="defScene3d"></div>
          <div id="defScene2d" class="scene-hidden"></div>
          <div class="label-layer" id="defLabelLayer"></div>
        </div>

        <div class="legend" aria-hidden="true" id="defLegend"></div>

        <div class="detail-panel" id="defDetailPanel">
          <h4 id="defDetailTitle">Detail Lapisan</h4>
          <div class="detail-grid" id="defDetailGrid"></div>
        </div>

        <div id="defTableWrap" style="display:none; margin-top:16px; overflow-x:auto;">
          <table class="data-table" id="defAccessTable">
            <thead><tr><th>Lapisan</th><th>Nama</th><th>Aktif</th><th>Kekuatan</th><th>Peluang Blokir</th><th>Tertahan (Sim)</th></tr></thead>
            <tbody></tbody>
          </table>
        </div>

        <!-- Outputs -->
        <div class="sim-title" style="margin-top:20px;">A. Grafik Funnel — Serangan Bertahan per Lapisan</div>
        <div class="chart-wrap"><svg id="defFunnelChart" viewBox="0 0 100 42" preserveAspectRatio="none" role="img" aria-label="Funnel chart"></svg></div>

        <div class="sim-title" style="margin-top:20px;">B. Papan Skor</div>
        <div class="kpi-row" id="defScoreboard"></div>

        <div class="sim-title" style="margin-top:20px;">C. Tabel Pembanding (Single Layer vs Defense in Depth vs What-if)</div>
        <div style="overflow-x:auto;">
          <table class="data-table" id="defComparatorTable">
            <thead><tr><th>Skenario</th><th>Risiko Residual (%)</th><th>Tertahan</th><th>Menembus</th><th>Terdeteksi</th><th>Waktu Tunda (mnt)</th><th>Penurunan Risiko vs Single</th></tr></thead>
            <tbody></tbody>
          </table>
        </div>

        <div class="sim-title" style="margin-top:20px;">D. Heatmap Peluang Blokir Efektif p(i,s) — Serangan Terpilih</div>
        <div style="overflow-x:auto;">
          <table class="data-table" id="defHeatmapTable">
            <thead><tr><th>Lapisan</th><th>Efektivitas Dasar</th><th>Kekuatan</th><th>Aktif</th><th>p(i,s)</th></tr></thead>
            <tbody></tbody>
          </table>
        </div>

        <div class="sim-title" style="margin-top:20px;">E. Titik Lemah & Saran Perbaikan</div>
        <div class="analysis-box" id="defWeakPanel"></div>

        <div class="sim-title" style="margin-top:20px;">F. Mengapa Defense in Depth Penting? (Analisis dari Hasil Simulasi)</div>
        <div class="analysis-box" id="defAnalysis"></div>
        <div class="btn-row" style="margin-top:12px;"><button class="btn-run" id="defCopyAnswer">Salin Jawaban (Esai)</button></div>
      </div>
    </div>

<script>
/* =====================================================================
   Modul 2 — Defense in Depth: Pure Simulation Logic
   All outputs computed from inputs only. Deterministic with seeded RNG.
   ===================================================================== */
(function () {
  'use strict';

  // ---- Seeded RNG (mulberry32) ----
  function mulberry32(seed) {
    var a = seed >>> 0;
    return function () {
      a |= 0; a = (a + 0x6D2B79F5) | 0;
      var t = Math.imul(a ^ (a >>> 15), 1 | a);
      t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
      return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
    };
  }

  // ---- Layer definitions (L1..L7, outer to inner) ----
  var LAYERS = [
    { id: 'L1', name: 'Kebijakan & Edukasi Pengguna', color: '#6c8eff', defaultStrength: 70, active: true,
      controls: ['Kebijakan kata sandi', 'Pelatihan phishing', 'Prosedur insiden', 'Manajemen akses'],
      canDetect: false },
    { id: 'L2', name: 'Keamanan Fisik', color: '#ef4444', defaultStrength: 70, active: true,
      controls: ['Akses ruang server', 'CCTV', 'Kunci elektronik', 'Log kunjungan'],
      canDetect: false },
    { id: 'L3', name: 'Perimeter (Firewall)', color: '#3a6bdf', defaultStrength: 70, active: true,
      controls: ['Firewall stateful', 'DPI/IPS', 'Geo-blocking', 'DDoS mitigation'],
      canDetect: false },
    { id: 'L4', name: 'Jaringan (Segmentasi, IDS/IPS)', color: '#22d3ee', defaultStrength: 70, active: true,
      controls: ['VLAN segmentasi', 'IDS/IPS', 'NAC', 'Micro-segmentasi'],
      canDetect: true, detectBase: 0.5 }, // IDS detection
    { id: 'L5', name: 'Host/Endpoint (AV, Patch)', color: '#a78bfa', defaultStrength: 70, active: true,
      controls: ['Antivirus/EDR', 'Patch management', 'Hardening', 'App control'],
      canDetect: true, detectBase: 0.4 }, // EDR detection
    { id: 'L6', name: 'Aplikasi (WAF, MFA, Validasi)', color: '#fbbf24', defaultStrength: 70, active: true,
      controls: ['WAF', 'Validasi input', 'MFA', 'Rate limiting', 'API security'],
      canDetect: false },
    { id: 'L7', name: 'Data (Enkripsi, Backup)', color: '#34d399', defaultStrength: 70, active: true,
      controls: ['Enkripsi at-rest/in-transit', 'Backup immutable', 'DLP', 'Key management'],
      canDetect: false }
  ];

  // ---- Default effectiveness matrix (attack x layer) ----
  // Rows: attacks, Cols: L1..L7 (0-1)
  var DEFAULT_EFFECTIVENESS = {
    'Phishing':            [0.60, 0.00, 0.30, 0.10, 0.40, 0.10, 0.10],
    'Malware/Ransomware':  [0.30, 0.00, 0.40, 0.40, 0.70, 0.20, 0.50],
    'DDoS':                [0.00, 0.00, 0.70, 0.40, 0.00, 0.30, 0.00],
    'Brute-force':         [0.30, 0.00, 0.30, 0.30, 0.30, 0.60, 0.20],
    'SQL Injection':       [0.05, 0.00, 0.30, 0.20, 0.05, 0.80, 0.40],
    'Insider Threat':      [0.40, 0.30, 0.00, 0.40, 0.30, 0.30, 0.50],
    'Pencurian Fisik':     [0.30, 0.80, 0.00, 0.00, 0.30, 0.00, 0.60]
  };
  var ATTACK_NAMES = Object.keys(DEFAULT_EFFECTIVENESS);

  // ---- Entry point definitions ----
  var ENTRY_POINTS = {
    'internet': { label: 'Luar (Internet)', startLayer: 0 }, // L1..L7
    'physical': { label: 'Fisik', startLayer: 1 },            // L2..L7
    'insider':  { label: 'Dalam jaringan/Insider', startLayer: 3 } // L4..L7
  };

  // ---- Target asset definitions ----
  var TARGETS = {
    'data': { label: 'Data/Database', endLayer: 6 },      // up to L7 (index 6)
    'app':  { label: 'Server Aplikasi', endLayer: 5 },    // up to L6
    'endpoint': { label: 'Endpoint Pengguna', endLayer: 4 } // up to L5
  };

  // ---- Preset configurations ----
  var PRESETS = {
    'none': {
      name: 'Tanpa Pertahanan',
      layers: LAYERS.map(function (l) { return { active: false, strength: 0 }; })
    },
    'firewall': {
      name: 'Hanya Firewall',
      layers: LAYERS.map(function (l, i) { return { active: i === 2, strength: i === 2 ? 80 : 0 }; })
    },
    'standard': {
      name: 'Standar Kantor',
      layers: LAYERS.map(function (l, i) {
        var active = [1,2,4].indexOf(i) >= 0; // L2, L3, L5
        return { active: active, strength: active ? 70 : 0 };
      })
    },
    'full': {
      name: 'Defense in Depth Penuh',
      layers: LAYERS.map(function () { return { active: true, strength: 70 }; })
    }
  };

  // ---- Pure calculation functions ----

  // 1. Block probability for layer i against attack s
  function blockProbability(layer, attack, effectiveness, layerIdx) {
    if (!layer.active) return 0;
    var eff = effectiveness[attack];
    if (!eff) return 0;
    if (layerIdx < 0 || layerIdx >= eff.length) return 0;
    return eff[layerIdx] * (layer.strength / 100);
  }

  // 2. Analytical residual risk = product of (1 - p) for traversed layers
  function analyticalResidualRisk(layers, attack, effectiveness, startLayer, endLayer) {
    var details = [];
    var risk = 1.0;
    for (var i = startLayer; i <= endLayer; i++) {
      var layer = layers[i];
      var p = blockProbability(layer, attack, effectiveness, i);
      var pass = 1 - p;
      risk *= pass;
      details.push({
        layer: layer.id,
        name: layer.name,
        active: layer.active,
        strength: layer.strength,
        effectiveness: effectiveness[attack] ? effectiveness[attack][i] : 0,
        blockProb: p,
        passProb: pass
      });
    }
    return { risk: risk, riskPercent: risk * 100, details: details };
  }

  // 3. Simulation: run N trials with seeded RNG
  function runSimulation(layers, attack, effectiveness, startLayer, endLayer, N, seed) {
    var rng = mulberry32(seed);
    var blockedAt = []; // count per layer index
    var penetrated = 0;
    var detected = 0;
    var totalLayersPassed = 0; // for attacker delay
    var trials = []; // for visualization (max 60)

    for (var trial = 0; trial < N; trial++) {
      var stopped = false;
      var layersPassed = 0;
      for (var i = startLayer; i <= endLayer; i++) {
        var layer = layers[i];
        var p = blockProbability(layer, attack, effectiveness, i);
        if (rng() < p) {
          blockedAt[i] = (blockedAt[i] || 0) + 1;
          stopped = true;
          if (trials.length < 60) {
            trials.push({ stoppedAt: i, penetrated: false });
          }
          break;
        }
        layersPassed++;
        // Detection check (IDS at L4, EDR at L5)
        if (layer.canDetect && layer.active && rng() < layer.detectBase * (layer.strength / 100)) {
          detected++;
        }
      }
      if (!stopped) {
        penetrated++;
        totalLayersPassed += layersPassed;
        if (trials.length < 60) {
          trials.push({ stoppedAt: -1, penetrated: true, layersPassed: layersPassed });
        }
      } else {
        // find where it stopped for delay calc
        for (var i = startLayer; i <= endLayer; i++) {
          if (blockedAt[i] && blockedAt[i] > 0) { /* already counted */ break; }
        }
      }
    }

    return {
      blockedAt: blockedAt,
      penetrated: penetrated,
      penetratedPercent: (penetrated / N) * 100,
      blockedPercent: 100 - (penetrated / N) * 100,
      detected: detected,
      avgLayersPassed: penetrated > 0 ? totalLayersPassed / penetrated : 0,
      trials: trials
    };
  }

  // 4. Detection & attacker delay
  function calculateDetection(simResult, layers, startLayer, endLayer, baseDelayMinutes) {
    var delayMinutes = simResult.avgLayersPassed * baseDelayMinutes;
    return {
      detectedCount: simResult.detected,
      detectedPercent: simResult.penetrated > 0 ? (simResult.detected / (simResult.penetrated + simResult.detected)) * 100 : 0,
      attackerDelayMinutes: delayMinutes
    };
  }

  // 5. Automatic comparators
  function findStrongestLayer(layers, attack, effectiveness, startLayer, endLayer) {
    var bestIdx = startLayer, bestP = -1;
    for (var i = startLayer; i <= endLayer; i++) {
      var p = blockProbability(layers[i], attack, effectiveness, i);
      if (p > bestP) { bestP = p; bestIdx = i; }
    }
    return { index: bestIdx, blockProb: bestP };
  }

  function runComparators(layers, attack, effectiveness, startLayer, endLayer, N, seed, baseDelayMinutes) {
    var results = {};

    // a) Only strongest layer
    var strongest = findStrongestLayer(layers, attack, effectiveness, startLayer, endLayer);
    var onlyStrongest = LAYERS.map(function (l, i) {
      return { active: i === strongest.index, strength: i === strongest.index ? layers[i].strength : 0 };
    });
    var simA = runSimulation(onlyStrongest, attack, effectiveness, startLayer, endLayer, N, seed);
    var riskA = analyticalResidualRisk(onlyStrongest, attack, effectiveness, startLayer, endLayer).riskPercent;
    results.onlyStrongest = { riskPercent: riskPenetrated(simA), simulated: simA, analytical: riskA, layer: strongest.index };

    // b) All layers as configured
    var simB = runSimulation(layers, attack, effectiveness, startLayer, endLayer, N, seed);
    var riskB = analyticalResidualRisk(layers, attack, effectiveness, startLayer, endLayer).riskPercent;
    results.allLayers = { riskPercent: riskPenetrated(simB), simulated: simB, analytical: riskB };

    // c) All layers but strongest disabled
    var withoutStrongest = layers.map(function (l, i) {
      return { active: i !== strongest.index && l.active, strength: i !== strongest.index ? l.strength : 0 };
    });
    var simC = runSimulation(withoutStrongest, attack, effectiveness, startLayer, endLayer, N, seed);
    var riskC = analyticalResidualRisk(withoutStrongest, attack, effectiveness, startLayer, endLayer).riskPercent;
    results.withoutStrongest = { riskPercent: riskPenetrated(simC), simulated: simC, analytical: riskC, layer: strongest.index };

    return results;
  }

  function riskPenetrated(sim) { return sim.penetratedPercent; }

  // 6. Weak points analysis
  function analyzeWeakPoints(layers, attack, effectiveness, startLayer, endLayer, simResult) {
    var weak = [];
    var contributions = [];

    // Layer contributions (marginal risk increase if disabled)
    var baseRisk = analyticalResidualRisk(layers, attack, effectiveness, startLayer, endLayer).risk;
    for (var i = startLayer; i <= endLayer; i++) {
      if (!layers[i].active) {
        weak.push({ type: 'inactive', layer: i, message: LAYERS[i].name + ' nonaktif' });
        contributions.push({ layer: i, marginalIncrease: 0, message: 'Nonaktif' });
        continue;
      }
      var without = layers.map(function (l, idx) {
        return { active: idx !== i ? l.active : false, strength: l.strength };
      });
      var riskWithout = analyticalResidualRisk(without, attack, effectiveness, startLayer, endLayer).risk;
      var marginal = (riskWithout - baseRisk) * 100; // percentage points
      contributions.push({ layer: i, marginalIncrease: marginal, message: '+ ' + marginal.toFixed(2) + ' pp risiko' });
      if (marginal < 0.5) {
        weak.push({ type: 'low-contrib', layer: i, message: LAYERS[i].name + ' kontribusi marginal < 0.5 pp' });
      }
    }

    // Attack x layer combinations with effective p < 0.15
    for (var i = startLayer; i <= endLayer; i++) {
      if (!layers[i].active) continue;
      var p = blockProbability(layers[i], attack, effectiveness, i);
      if (p < 0.15) {
        weak.push({
          type: 'low-effectiveness',
          layer: i,
          attack: attack,
          message: attack + ' x ' + LAYERS[i].name + ': peluang blokir efektif ' + (p * 100).toFixed(1) + '% (< 15%)'
        });
      }
    }

    // Recommendations from control dictionary
    var recommendations = weak.map(function (w) {
      if (w.type === 'inactive') {
        return 'Aktifkan ' + LAYERS[w.layer].name + ': ' + LAYERS[w.layer].controls.slice(0, 3).join(', ');
      } else if (w.type === 'low-contrib') {
        return 'Tingkatkan kekuatan ' + LAYERS[w.layer].name + ' atau tambah kontrol: ' + LAYERS[w.layer].controls.slice(0, 2).join(', ');
      } else {
        return 'Perkuat ' + LAYERS[w.layer].name + ' terhadap ' + w.attack + ': ' + LAYERS[w.layer].controls.slice(0, 2).join(', ');
      }
    });

    return { weakPoints: weak, contributions: contributions, recommendations: recommendations };
  }

  // 7. Security posture score
  function postureScore(residualRiskPercent) {
    return Math.max(0, Math.min(100, Math.round(100 * (1 - residualRiskPercent / 100))));
  }

  // 8. Generate analysis text (built from actual simulation numbers)
  function generateAnalysisText(params, simResult, analytical, comparators, weakPoints, detection) {
    var parts = [];
    var attack = params.attack;
    var entry = params.entryPoint;
    var target = params.target;
    var N = params.trials;

    // 1) No single point of failure
    var riskAll = comparators.allLayers.riskPercent;
    var riskWithout = comparators.withoutStrongest.riskPercent;
    var strongestName = LAYERS[comparators.withoutStrongest.layer].name;
    parts.push({
      title: 'Tidak ada Single Point of Failure',
      text: 'Dengan semua lapisan aktif, risiko residual ' + riskAll.toFixed(2) + '%. ' +
        'Jika lapisan terkuat (' + strongestName + ') gagal, risiko naik jadi ' + riskWithout.toFixed(2) + '% ' +
        '(bukan 100%), karena lapisan lain masih melindungi. Selisih: ' + (riskWithout - riskAll).toFixed(2) + ' pp.'
    });

    // 2) Controls covering each other's weaknesses
    var weakLayer = null;
    for (var i = params.startLayer; i <= params.endLayer; i++) {
      var p = blockProbability(params.layers[i], attack, params.effectiveness, i);
      if (p < 0.15 && params.layers[i].active) { weakLayer = i; break; }
    }
    var coveringLayer = null;
    if (weakLayer !== null) {
      for (var i = params.startLayer; i <= params.endLayer; i++) {
        if (i === weakLayer) continue;
        var p2 = blockProbability(params.layers[i], attack, params.effectiveness, i);
        if (p2 > 0.3) { coveringLayer = i; break; }
      }
    }
    if (weakLayer !== null && coveringLayer !== null) {
      parts.push({
        title: 'Kontrol Saling Menutup Kelemahan',
        text: LAYERS[weakLayer].name + ' lemah terhadap ' + attack + ' (blokir ' + (blockProbability(params.layers[weakLayer], attack, params.effectiveness, weakLayer) * 100).toFixed(1) + '%), ' +
          'tapi ' + LAYERS[coveringLayer].name + ' menutupinya (blokir ' + (blockProbability(params.layers[coveringLayer], attack, params.effectiveness, coveringLayer) * 100).toFixed(1) + '%).'
      });
    } else if (weakLayer !== null) {
      parts.push({
        title: 'Kontrol Saling Menutup Kelemahan',
        text: LAYERS[weakLayer].name + ' lemah terhadap ' + attack + ' (blokir ' + (blockProbability(params.layers[weakLayer], attack, params.effectiveness, weakLayer) * 100).toFixed(1) + '%), ' +
          'dan tidak ada lapisan lain yang cukup kuat menutupinya. Perlu diperkuat.'
      });
    } else {
      parts.push({
        title: 'Kontrol Saling Menutup Kelemahan',
        text: 'Semua lapisan yang dilalui memiliki peluang blokir efektif ≥ 15% terhadap ' + attack + '. Tidak ditemukan celah besar.'
      });
    }

    // 3) Slowing attacker for detection
    parts.push({
      title: 'Memperlambat Penyerang untuk Deteksi',
      text: 'Rata-rata penyerang melewati ' + simResult.avgLayersPassed.toFixed(1) + ' lapisan sebelum tertahan/menembus. ' +
        'Dengan tunda dasar ' + params.baseDelay + ' menit/lapisan, waktu tunda penyerang ≈ ' + detection.attackerDelayMinutes.toFixed(1) + ' menit. ' +
        'IDS (L4) + EDR (L5) mendeteksi ' + detection.detectedCount + ' dari ' + N + ' percobaan (' + detection.detectedPercent.toFixed(1) + '%).'
    });

    // 4) Reducing impact (blocked before asset)
    parts.push({
      title: 'Mengurangi Dampak Kerusakan',
      text: 'Dari ' + N + ' percobaan, ' + (N - simResult.penetrated) + ' tertahan sebelum mencapai aset target (' + TARGETS[target].label + '). ' +
        'Hanya ' + simResult.penetrated + ' (' + simResult.penetratedPercent.toFixed(2) + '%) yang menembus seluruh pertahanan.'
    });

    // 5) Single layer not enough
    var riskSingle = comparators.onlyStrongest.riskPercent;
    parts.push({
      title: 'Satu Lapisan Saja Tidak Cukup',
      text: 'Hanya lapisan terkuat (' + LAYERS[comparators.onlyStrongest.layer].name + '): risiko ' + riskSingle.toFixed(2) + '%. ' +
        'Semua lapisan (defense in depth): risiko ' + riskAll.toFixed(2) + '%. ' +
        'Pengurangan risiko relatif: ' + ((riskSingle - riskAll) / riskSingle * 100).toFixed(1) + '%.'
    });

    // Conclusion paragraph
    var conclusion = 'Kesimpulannya, defense in depth penting karena: (1) tidak ada single point of failure — kegagalan ' + strongestName +
      ' hanya menaikkan risiko ' + (riskWithout - riskAll).toFixed(2) + ' pp, bukan 100%; (2) lapisan saling menutup kelemahan, mis. ' +
      (weakLayer !== null ? LAYERS[weakLayer].name + ' ditutupi ' + (coveringLayer !== null ? LAYERS[coveringLayer].name : 'tidak ada') : 'tidak ada celah besar') +
      '; (3) memperlambat penyerang ' + detection.attackerDelayMinutes.toFixed(1) + ' menit rata-rata sehingga ' + detection.detectedCount + ' serangan terdeteksi; ' +
      '(4) ' + (N - simResult.penetrated) + '/' + N + ' serangan tertahan sebelum aset; (5) satu lapisan saja risikonya ' +
      riskSingle.toFixed(2) + '% vs defense in depth ' + riskAll.toFixed(2) + '%. ' +
      'Skor postur keamanan: ' + postureScore(riskAll) + '/100.';

    parts.push({ title: 'Kesimpulan', text: conclusion, isConclusion: true });

    return parts;
  }

  // Export to window for use by UI
  window.DefenseSim = {
    LAYERS: LAYERS,
    ATTACK_NAMES: ATTACK_NAMES,
    DEFAULT_EFFECTIVENESS: DEFAULT_EFFECTIVENESS,
    ENTRY_POINTS: ENTRY_POINTS,
    TARGETS: TARGETS,
    PRESETS: PRESETS,
    mulberry32: mulberry32,
    blockProbability: blockProbability,
    analyticalResidualRisk: analyticalResidualRisk,
    runSimulation: runSimulation,
    calculateDetection: calculateDetection,
    runComparators: runComparators,
    analyzeWeakPoints: analyzeWeakPoints,
    postureScore: postureScore,
    generateAnalysisText: generateAnalysisText
  };
})();
</script>

<script>
/* =====================================================================
   Modul 2 — Defense in Depth: UI Logic & 3D/2D Visualization
   Integrates with DefenseSim pure functions and NetApp topology store.
   ===================================================================== */
(function () {
  'use strict';

  var $ = function (id) { return document.getElementById(id); };
  var DS = window.DefenseSim;
  var NetApp = window.NetApp;

  // ---- State ----
  var state = {
    layers: DS.LAYERS.map(function (l) { return { active: l.active, strength: l.defaultStrength }; }),
    effectiveness: JSON.parse(JSON.stringify(DS.DEFAULT_EFFECTIVENESS)),
    attack: 'Phishing',
    entryPoint: 'internet',
    target: 'data',
    trials: 1000,
    seed: 42,
    baseDelay: 5,
    animating: false,
    animSpeed: 1,
    animTrials: [],
    animIndex: 0,
    animRaf: null,
    lastResults: null
  };

  // ---- Helpers ----
  function showError(msgs) {
    var box = $('defErr');
    if (!msgs || !msgs.length) { box.classList.remove('show'); box.innerHTML = ''; return; }
    var html = '<strong>Input tidak valid:</strong><ul>';
    msgs.forEach(function (m) { html += '<li>' + m + '</li>'; });
    box.innerHTML = html + '</ul>'; box.classList.add('show');
  }

  function hexCss(n) { return '#' + ('000000' + (n >>> 0).toString(16)).slice(-6); }

  function colorClass(p) {
    if (p >= 0.6) return 'high';
    if (p >= 0.3) return 'med';
    if (p > 0) return 'low';
    return 'zero';
  }

  // ---- Input validation ----
  function validate() {
    var errs = [];
    if (state.trials < 10 || state.trials > 10000) errs.push('N harus 10-10.000');
    if (isNaN(state.seed)) errs.push('Seed harus angka');
    if (state.baseDelay < 1 || state.baseDelay > 60) errs.push('Tunda dasar 1-60 menit');
    // Matrix values
    for (var atk in state.effectiveness) {
      state.effectiveness[atk].forEach(function (v, i) {
        if (v < 0 || v > 1) errs.push('Matriks ' + atk + ' L' + (i+1) + ' di luar 0-1');
      });
    }
    // Layer strengths
    state.layers.forEach(function (l, i) {
      if (l.strength < 0 || l.strength > 100) errs.push('Kekuatan L' + (i+1) + ' di luar 0-100');
    });
    // Entry/target logic
    var ep = DS.ENTRY_POINTS[state.entryPoint];
    var tg = DS.TARGETS[state.target];
    if (ep.startLayer > tg.endLayer) {
      errs.push('Titik masuk ' + ep.label + ' melewati lapisan setelah target ' + tg.label + ' — tidak masuk akal');
    }
    return errs;
  }

  // ---- Get traversed layer range ----
  function getLayerRange() {
    var ep = DS.ENTRY_POINTS[state.entryPoint];
    var tg = DS.TARGETS[state.target];
    return { start: ep.startLayer, end: tg.endLayer };
  }

  // ---- Populate dropdowns ----
  function populateDropdowns() {
    // Attacks
    var atkSel = $('defAttack');
    atkSel.innerHTML = '';
    DS.ATTACK_NAMES.forEach(function (a) {
      var o = document.createElement('option'); o.value = a; o.textContent = a;
      if (a === state.attack) o.selected = true;
      atkSel.appendChild(o);
    });
    // Entry points
    var epSel = $('defEntry');
    epSel.innerHTML = '';
    Object.keys(DS.ENTRY_POINTS).forEach(function (k) {
      var o = document.createElement('option'); o.value = k; o.textContent = DS.ENTRY_POINTS[k].label;
      if (k === state.entryPoint) o.selected = true;
      epSel.appendChild(o);
    });
    // Targets
    var tgSel = $('defTarget');
    tgSel.innerHTML = '';
    Object.keys(DS.TARGETS).forEach(function (k) {
      var o = document.createElement('option'); o.value = k; o.textContent = DS.TARGETS[k].label;
      if (k === state.target) o.selected = true;
      tgSel.appendChild(o);
    });
  }

  // ---- Render layer config table ----
  function renderLayerTable() {
    var tb = $('defLayerTable').querySelector('tbody');
    tb.innerHTML = '';
    state.layers.forEach(function (l, i) {
      var L = DS.LAYERS[i];
      var tr = document.createElement('tr');
      var activeChecked = l.active ? 'checked' : '';
      var colorDot = '<span class="layer-color" style="background:' + L.color + '"></span>';
      tr.innerHTML =
        '<td>' + colorDot + ' L' + (i+1) + '</td>' +
        '<td>' + L.name + '</td>' +
        '<td><input type="checkbox" data-idx="' + i + '" ' + activeChecked + ' /></td>' +
        '<td class="def-layer-row">' +
          '<input type="range" data-idx="' + i + '" min="0" max="100" value="' + l.strength + '" ' + (l.active ? '' : 'disabled') + ' />' +
          '<span class="strength-val">' + l.strength + '</span>' +
        '</td>' +
        '<td style="font-size:11px;color:var(--text-secondary);max-width:280px;">' + L.controls.slice(0, 3).join(', ') + '</td>' +
        '<td>' + (L.canDetect ? '<span class="status-pill waspada" style="font-size:10px;padding:2px 8px;">IDS/EDR</span>' : '<span style="color:var(--text-muted);font-size:11px;">—</span>') + '</td>';
      tb.appendChild(tr);
    });
    // Event listeners
    tb.querySelectorAll('input[type="checkbox"]').forEach(function (cb) {
      cb.addEventListener('change', function () {
        var idx = +cb.getAttribute('data-idx');
        state.layers[idx].active = cb.checked;
        var range = tb.querySelector('input[type="range"][data-idx="' + idx + '"]');
        range.disabled = !cb.checked;
        if (!cb.checked) state.layers[idx].strength = 0;
        renderLayerTable(); // re-render to update strength display
        renderAllOutputs();
      });
    });
    tb.querySelectorAll('input[type="range"]').forEach(function (rg) {
      rg.addEventListener('input', function () {
        var idx = +rg.getAttribute('data-idx');
        state.layers[idx].strength = +rg.value;
        tb.querySelector('.strength-val').textContent = rg.value; // note: this only updates first, fix below
        // Update all strength displays
        tb.querySelectorAll('input[type="range"]').forEach(function (r) {
          var id = +r.getAttribute('data-idx');
          var sv = tb.querySelector('input[type="range"][data-idx="' + id + '"]').parentNode.querySelector('.strength-val');
          if (sv) sv.textContent = r.value;
        });
        renderAllOutputs();
      });
    });
  }

  // ---- Render effectiveness matrix heatmap ----
  function renderMatrixTable() {
    var tb = $('defMatrixTable').querySelector('tbody');
    tb.innerHTML = '';
    DS.ATTACK_NAMES.forEach(function (atk) {
      var tr = document.createElement('tr');
      var html = '<td style="font-weight:600;">' + atk + '</td>';
      state.effectiveness[atk].forEach(function (v, i) {
        var cls = colorClass(v);
        html += '<td><input type="number" class="' + cls + '" min="0" max="1" step="0.01" value="' + v.toFixed(2) + '" data-atk="' + atk + '" data-idx="' + i + '" style="width:50px;text-align:center;" /></td>';
      });
      tr.innerHTML = html;
      tb.appendChild(tr);
    });
    tb.querySelectorAll('input').forEach(function (inp) {
      inp.addEventListener('change', function () {
        var atk = inp.getAttribute('data-atk');
        var idx = +inp.getAttribute('data-idx');
        var v = +inp.value;
        if (isNaN(v) || v < 0 || v > 1) { showError(['Nilai matriks harus 0-1']); inp.value = state.effectiveness[atk][idx].toFixed(2); return; }
        state.effectiveness[atk][idx] = v;
        inp.className = colorClass(v);
        showError(null);
        renderAllOutputs();
      });
    });
  }

  // ---- Apply preset ----
  function applyPreset(key) {
    var preset = DS.PRESETS[key];
    if (!preset) return;
    preset.layers.forEach(function (l, i) {
      state.layers[i].active = l.active;
      state.layers[i].strength = l.strength;
    });
    renderLayerTable();
    renderAllOutputs();
  }

  // ---- Main simulation runner ----
  function runSimulation() {
    var errs = validate();
    if (errs.length) { showError(errs); return null; }
    showError(null);

    var range = getLayerRange();
    var layers = state.layers;
    var attack = state.attack;
    var effectiveness = state.effectiveness;
    var N = state.trials;
    var seed = state.seed;
    var baseDelay = state.baseDelay;

    // Analytical
    var analytical = DS.analyticalResidualRisk(layers, attack, effectiveness, range.start, range.end);
    // Simulation
    var sim = DS.runSimulation(layers, attack, effectiveness, range.start, range.end, N, seed);
    // Detection & delay
    var detection = DS.calculateDetection(sim, layers, range.start, range.end, baseDelay);
    // Comparators
    var comparators = DS.runComparators(layers, attack, effectiveness, range.start, range.end, N, seed, baseDelay);
    // Weak points
    var weakPoints = DS.analyzeWeakPoints(layers, attack, effectiveness, range.start, range.end, sim);
    // Posture score
    var score = DS.postureScore(analytical.riskPercent);
    // Analysis text
    var analysisParams = {
      layers: layers, attack: attack, effectiveness: effectiveness,
      startLayer: range.start, endLayer: range.end,
      entryPoint: state.entryPoint, target: state.target,
      trials: N, baseDelay: baseDelay
    };
    var analysis = DS.generateAnalysisText(analysisParams, sim, analytical, comparators, weakPoints, detection);

    var results = {
      analytical: analytical,
      sim: sim,
      detection: detection,
      comparators: comparators,
      weakPoints: weakPoints,
      score: score,
      analysis: analysis,
      params: analysisParams,
      range: range
    };
    state.lastResults = results;
    return results;
  }

  // ---- Render all outputs ----
  function renderAllOutputs() {
    var results = runSimulation();
    if (!results) return;

    renderFunnelChart(results);
    renderScoreboard(results);
    renderComparatorTable(results);
    renderHeatmapTable(results);
    renderWeakPanel(results);
    renderAnalysis(results);
    render3DScene(results);
    render2DScene(results);
    renderAccessTable(results);
    updateLegend(results);
    // Animation controls
    prepareAnimation(results);
  }

  // ---- A. Funnel Chart ----
  function renderFunnelChart(results) {
    var svg = $('defFunnelChart');
    var details = results.analytical.details;
    var sim = results.sim;
    // Use analytical pass counts for funnel
    var width = 100, height = 42;
    var parts = [];
    var y = 4;
    var barH = 4;
    var gap = 1.5;
    var passed = 1.0;
    details.forEach(function (d, i) {
      var passCount = passed * d.passProb;
      var blockCount = passed - passCount;
      var passW = (passCount / 1.0) * width;
      var blockW = (blockCount / 1.0) * width;
      passed = passCount;

      parts.push('<g>');
      // Passed segment (left)
      parts.push('<rect class="def-funnel-bar-seg" x="' + (width - passW) + '" y="' + y + '" width="' + passW + '" height="' + barH + '" fill="var(--accent)" />');
      // Blocked segment (right)
      if (blockW > 0) {
        parts.push('<rect class="def-funnel-bar-seg" x="' + (width - passW - blockW) + '" y="' + y + '" width="' + blockW + '" height="' + barH + '" fill="#ef4444" />');
      }
      // Label
      parts.push('<text x="2" y="' + (y + barH - 1) + '" font-size="2.8" fill="var(--text-secondary)" font-family="Inter,sans-serif">' + d.layer + '</text>');
      // Values
      parts.push('<text x="' + (width - passW - blockW/2) + '" y="' + (y + barH - 1) + '" font-size="2.5" fill="#fff" font-weight="700" text-anchor="middle" font-family="Inter,monospace">' + (blockCount * 100).toFixed(1) + '%</text>');
      parts.push('<text x="' + (width - passW/2) + '" y="' + (y + barH - 1) + '" font-size="2.5" fill="var(--text-primary)" font-weight="700" text-anchor="middle" font-family="Inter,monospace">' + (passCount * 100).toFixed(1) + '%</text>');
      parts.push('</g>');
      y += barH + gap;
    });
    // Final penetrated
    parts.push('<g>');
    parts.push('<rect x="' + (width - passed * width) + '" y="' + y + '" width="' + (passed * width) + '" height="' + barH + '" fill="#6c8eff" />');
    parts.push('<text x="2" y="' + (y + barH - 1) + '" font-size="2.8" fill="var(--text-secondary)" font-family="Inter,sans-serif">Target</text>');
    parts.push('<text x="' + (width - passed * width / 2) + '" y="' + (y + barH - 1) + '" font-size="2.5" fill="#fff" font-weight="700" text-anchor="middle" font-family="Inter,monospace">' + (passed * 100).toFixed(2) + '%</text>');
    parts.push('</g>');

    svg.innerHTML = parts.join('');
  }

  // ---- B. Scoreboard ----
  function renderScoreboard(results) {
    var wrap = $('defScoreboard');
    var sim = results.sim;
    var det = results.detection;
    var an = results.analytical;
    var items = [
      { label: 'Risiko Residual', value: an.riskPercent.toFixed(2) + '%', sub: 'Analitik: ∏(1-p)' },
      { label: 'Skor Postur', value: results.score + '/100', sub: '100 × (1 - risiko)' },
      { label: 'Serangan Tertahan', value: (results.trials - sim.penetrated).toLocaleString(), sub: sim.blockedPercent.toFixed(1) + '% dari ' + results.trials },
      { label: 'Menembus Target', value: sim.penetrated.toLocaleString(), sub: sim.penetratedPercent.toFixed(2) + '%' },
      { label: 'Terdeteksi (IDS/EDR)', value: det.detectedCount.toLocaleString(), sub: det.detectedPercent.toFixed(1) + '%' },
      { label: 'Waktu Tunda Penyerang', value: det.attackerDelayMinutes.toFixed(1) + ' menit', sub: 'Rata-rata ' + sim.avgLayersPassed.toFixed(1) + ' lapisan × ' + state.baseDelay + ' mnt' }
    ];
    wrap.innerHTML = items.map(function (it) {
      return '<div class="def-kpi"><div class="kl">' + it.label + '</div><div class="kv">' + it.value + '</div><div class="ks">' + it.sub + '</div></div>';
    }).join('');
  }

  // ---- C. Comparator Table ----
  function renderComparatorTable(results) {
    var tb = $('defComparatorTable').querySelector('tbody');
    var comp = results.comparators;
    var simB = comp.allLayers.simulated;
    var riskAll = comp.allLayers.riskPercent;
    var rows = [
      {
        name: 'a) Hanya lapisan terkuat (' + DS.LAYERS[comp.onlyStrongest.layer].name + ')',
        risk: comp.onlyStrongest.riskPercent.toFixed(2),
        blocked: comp.onlyStrongest.simulated.blockedAt.reduce(function(a,b){return a+b;},0),
        penetrated: comp.onlyStrongest.simulated.penetrated,
        detected: comp.onlyStrongest.simulated.detected,
        delay: (comp.onlyStrongest.simulated.avgLayersPassed * state.baseDelay).toFixed(1),
        reduction: '— (baseline)'
      },
      {
        name: 'b) Semua lapisan aktif (Defense in Depth)',
        risk: comp.allLayers.riskPercent.toFixed(2),
        blocked: simB.blockedAt.reduce(function(a,b){return a+b;},0),
        penetrated: simB.penetrated,
        detected: simB.detected,
        delay: (simB.avgLayersPassed * state.baseDelay).toFixed(1),
        reduction: ((comp.onlyStrongest.riskPercent - riskAll) / comp.onlyStrongest.riskPercent * 100).toFixed(1) + '%'
      },
      {
        name: 'c) Semua aktif tapi ' + DS.LAYERS[comp.withoutStrongest.layer].name + ' gagal',
        risk: comp.withoutStrongest.riskPercent.toFixed(2),
        blocked: comp.withoutStrongest.simulated.blockedAt.reduce(function(a,b){return a+b;},0),
        penetrated: comp.withoutStrongest.simulated.penetrated,
        detected: comp.withoutStrongest.simulated.detected,
        delay: (comp.withoutStrongest.simulated.avgLayersPassed * state.baseDelay).toFixed(1),
        reduction: ((comp.onlyStrongest.riskPercent - comp.withoutStrongest.riskPercent) / comp.onlyStrongest.riskPercent * 100).toFixed(1) + '%'
      }
    ];
    tb.innerHTML = rows.map(function (r) {
      return '<tr><td>' + r.name + '</td><td>' + r.risk + '%</td><td>' + r.blocked + '</td><td>' + r.penetrated + '</td><td>' + r.detected + '</td><td>' + r.delay + '</td><td>' + r.reduction + '</td></tr>';
    }).join('');
  }

  // ---- D. Heatmap Table ----
  function renderHeatmapTable(results) {
    var tb = $('defHeatmapTable').querySelector('tbody');
    var range = results.range;
    var attack = state.attack;
    tb.innerHTML = '';
    for (var i = range.start; i <= range.end; i++) {
      var layer = state.layers[i];
      var eff = state.effectiveness[attack][i];
      var p = layer.active ? eff * (layer.strength / 100) : 0;
      var cls = colorClass(p);
      var tr = document.createElement('tr');
      tr.innerHTML =
        '<td>' + DS.LAYERS[i].id + ' ' + DS.LAYERS[i].name + '</td>' +
        '<td class="def-heatmap-cell ' + cls + '">' + eff.toFixed(2) + '</td>' +
        '<td class="def-heatmap-cell ' + cls + '">' + layer.strength + '</td>' +
        '<td class="def-heatmap-cell ' + cls + '">' + (layer.active ? 'Ya' : 'Tidak') + '</td>' +
        '<td class="def-heatmap-cell ' + cls + '"><b>' + (p * 100).toFixed(2) + '%</b></td>';
      tb.appendChild(tr);
    }
  }

  // ---- E. Weak Points Panel ----
  function renderWeakPanel(results) {
    var box = $('defWeakPanel');
    var wp = results.weakPoints;
    var html = '';
    if (!wp.weakPoints.length) {
      html = '<div class="empty-note">Tidak ditemukan titik lemah signifikan pada konfigurasi ini.</div>';
    } else {
      wp.weakPoints.forEach(function (w, idx) {
        var rec = wp.recommendations[idx] || '';
        html += '<div class="def-weak-item ' + w.type + '">';
        html += '<div class="weak-title">' + w.message + '</div>';
        if (rec) html += '<div class="weak-rec">Saran: ' + rec + '</div>';
        html += '</div>';
      });
    }
    // Marginal contributions
    html += '<h4 style="margin-top:16px;margin-bottom:8px;font-size:12px;text-transform:uppercase;letter-spacing:0.05em;color:var(--accent);">Kontribusi Marginal (pp risiko naik jika lapisan dimatikan)</h4>';
    html += '<div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(180px,1fr));gap:8px;">';
    wp.contributions.forEach(function (c) {
      var L = DS.LAYERS[c.layer];
      html += '<div class="def-detail-item"><div class="dl">' + L.id + '</div><div class="dv" style="color:' + (c.marginalIncrease > 1 ? '#ef4444' : (c.marginalIncrease > 0.1 ? '#fbbf24' : 'var(--success)')) + '">' + c.marginalIncrease.toFixed(2) + ' pp</div></div>';
    });
    html += '</div>';
    box.innerHTML = html;
  }

  // ---- F. Analysis Text ----
  function renderAnalysis(results) {
    var box = $('defAnalysis');
    var html = '';
    results.analysis.forEach(function (sec) {
      if (sec.isConclusion) {
        html += '<div class="def-analysis-section concl">' + sec.text + '</div>';
      } else {
        html += '<div class="def-analysis-section"><h4>' + sec.title + '</h4><p>' + sec.text + '</p></div>';
      }
    });
    box.innerHTML = html;
    // Store plain text for copy
    box._plain = results.analysis.map(function (s) { return s.title + ': ' + s.text.replace(/<[^>]+>/g, ''); }).join('\n\n');
  }

  // ---- 3D Scene (benteng berlapis) ----
  var defTHREE = null, defScene = null, defCamera = null, defRenderer = null, defControls = null;
  var defWallGroups = [], defPacketMeshes = [], defHumanMeshes = {}, defRaf = null, defClock = null;
  var defAnimPlaying = false, defAnimSpeed = 1, defAnimTrials = [], defAnimTrialIdx = 0, defAnimPacketIdx = 0;
  var defLabelLayer = $('defLabelLayer');
  var defReduced = window.matchMedia && window.matchMedia('(prefers-reduced-motion: reduce)').matches;

  function defLoadThree() {
    // Three.js and OrbitControls are loaded via script tags above
    // Just wait for them to be available
    return new Promise(function (resolve) {
      var checkCount = 0;
      function check() {
        checkCount++;
        if (window.THREE && window.OrbitControls) {
          defTHREE = window.THREE;
          defControls = window.OrbitControls;
          resolve();
        } else if (checkCount > 200) { // 20 seconds timeout
          console.error('Three.js or OrbitControls not loaded after timeout');
          resolve(); // Resolve anyway to not block
        } else {
          setTimeout(check, 100);
        }
      }
      check();
    });
  }

  function defColor(status) {
    if (status === 'active') return 0x4ade80;
    if (status === 'blocking') return 0xfbbf24;
    if (status === 'breached') return 0xef4444;
    return 0x55556a;
  }

  function defMat(color, opts) {
    return new defTHREE.MeshStandardMaterial(Object.assign({ color: color, roughness: 0.7, metalness: 0.15, flatShading: true }, opts || {}));
  }

  function defBuildWall(layer, height, thickness, status) {
    var L = DS.LAYERS[layer];
    var g = new defTHREE.Group();
    // Concentric cylinder wall
    var radius = 0.8 + layer * 1.1; // L0=0.8, L6=7.4
    var geom = new defTHREE.CylinderGeometry(radius, radius, height, 32, 1, true);
    var mat = defMat(L.color, { transparent: true, opacity: layer === 0 ? 0.6 : 0.5 });
    var wall = new defTHREE.Mesh(geom, mat);
    wall.position.y = height / 2;
    if (status === 'inactive') { mat.opacity = 0.15; mat.transparent = true; }
    if (status === 'blocking') { mat.emissive = new defTHREE.Color(L.color); mat.emissiveIntensity = 0.6; }
    if (status === 'breached') { mat.color.setHex(0xef4444); mat.emissive.setHex(0xef4444); mat.emissiveIntensity = 0.8; }
    g.add(wall);
    g.userData = { layer: layer, wall: wall, radius: radius };
    return g;
  }

  function defBuildTarget() {
    var g = new defTHREE.Group();
    // Database cylinder
    var db = new defTHREE.Mesh(new defTHREE.CylinderGeometry(0.6, 0.6, 1.2, 20), defMat(0x34d399));
    db.position.y = 0.6;
    g.add(db);
    var cap = new defTHREE.Mesh(new defTHREE.CylinderGeometry(0.65, 0.65, 0.1, 20), defMat(0xdddddd));
    cap.position.y = 1.25; g.add(cap);
    cap = cap.clone(); cap.position.y = -0.05; g.add(cap);
    // Glow
    var glow = new defTHREE.Mesh(new defTHREE.SphereGeometry(1.0, 16, 16), defMat(0x34d399, { transparent: true, opacity: 0.1, emissive: 0x34d399, emissiveIntensity: 0.5 }));
    g.add(glow);
    g.userData.isTarget = true;
    return g;
  }

  function defBuildAttacker(position) {
    var g = new defTHREE.Group();
    // Hoodie figure
    var body = new defTHREE.Mesh(new defTHREE.BoxGeometry(0.5, 0.7, 0.3), defMat(0x1a1a1a));
    body.position.y = 0.35; g.add(body);
    var head = new defTHREE.Mesh(new defTHREE.SphereGeometry(0.22, 16, 16), defMat(0xf2c9a0));
    head.position.y = 0.95; g.add(head);
    var hood = new defTHREE.Mesh(new defTHREE.SphereGeometry(0.28, 16, 16), defMat(0x181818));
    hood.position.y = 1.0; g.add(hood);
    var laptop = new defTHREE.Mesh(new defTHREE.BoxGeometry(0.45, 0.3, 0.05), defMat(0xcc2222));
    laptop.position.set(0, 0.4, 0.35); g.add(laptop);
    g.position.copy(position);
    g.userData = { role: 'attacker' };
    return g;
  }

  function defBuildAdmin(position) {
    var g = new defTHREE.Group();
    var body = new defTHREE.Mesh(new defTHREE.BoxGeometry(0.5, 0.7, 0.3), defMat(0x2b6cff));
    body.position.y = 0.35; g.add(body);
    var head = new defTHREE.Mesh(new defTHREE.SphereGeometry(0.22, 16, 16), defMat(0xf2c9a0));
    head.position.y = 0.95; g.add(head);
    var glasses = new defTHREE.Mesh(new defTHREE.BoxGeometry(0.34, 0.08, 0.05), defMat(0x111111));
    glasses.position.set(0, 0.98, 0.2); g.add(glasses);
    var tablet = new defTHREE.Mesh(new defTHREE.BoxGeometry(0.3, 0.2, 0.03), defMat(0x88aaff));
    tablet.position.set(0.4, 0.4, 0.2); g.add(tablet);
    g.position.copy(position);
    g.userData = { role: 'admin' };
    return g;
  }

  function defBuildUser(position) {
    var g = new defTHREE.Group();
    var body = new defTHREE.Mesh(new defTHREE.BoxGeometry(0.5, 0.7, 0.3), defMat(0x4ade80));
    body.position.y = 0.35; g.add(body);
    var head = new defTHREE.Mesh(new defTHREE.SphereGeometry(0.22, 16, 16), defMat(0xf2c9a0));
    head.position.y = 0.95; g.add(head);
    g.position.copy(position);
    g.userData = { role: 'user' };
    return g;
  }

  function defInit3D() {
    var host = $('defScene3d');
    defScene = new defTHREE.Scene();
    defScene.background = new defTHREE.Color(0x0a0a0f);

    defCamera = new defTHREE.PerspectiveCamera(40, host.clientWidth / host.clientHeight, 0.1, 200);
    defCamera.position.set(12, 14, 18);

    defRenderer = new defTHREE.WebGLRenderer({ antialias: true, alpha: false });
    defRenderer.setPixelRatio(Math.min(window.devicePixelRatio || 1, 2));
    defRenderer.setSize(host.clientWidth, host.clientHeight);
    host.appendChild(defRenderer.domElement);

    defControls = new (window.OrbitControls || defControls)(defCamera, defRenderer.domElement);
    defControls.enableDamping = true;
    defControls.dampingFactor = 0.08;
    defControls.minDistance = 8; defControls.maxDistance = 40;
    defControls.minPolarAngle = 0.25; defControls.maxPolarAngle = 1.4;
    defControls.enablePan = false;
    defControls.target.set(0, 1, 0);

    // Lights
    defScene.add(new defTHREE.AmbientLight(0xffffff, 0.5));
    var key = new defTHREE.DirectionalLight(0xffffff, 0.8); key.position.set(10, 20, 12); defScene.add(key);
    var fill = new defTHREE.DirectionalLight(0x88aaff, 0.3); fill.position.set(-12, 10, -10); defScene.add(fill);

    // Floor
    var floor = new defTHREE.Mesh(new defTHREE.CylinderGeometry(18, 18, 0.6, 12), defMat(0x12121a, { roughness: 0.9 }));
    floor.position.y = -0.4; defScene.add(floor);
    var grid = new defTHREE.GridHelper(36, 36, 0x333344, 0x222233);
    grid.position.y = -0.1; defScene.add(grid);

    defClock = new defTHREE.Clock();
    defRenderer.domElement.addEventListener('pointerdown', defOnPointerDown);
    window.addEventListener('resize', defOnResize);
    window.addEventListener('beforeunload', defDispose);
  }

  function defBuildScene(results) {
    if (!defScene) return;
    // Clear existing
    defWallGroups.forEach(function (g) { defScene.remove(g); });
    defWallGroups = [];
    defPacketMeshes = [];
    Object.values(defHumanMeshes).forEach(function (h) { defScene.remove(h); });
    defHumanMeshes = {};
    if (defLabelLayer) { defLabelLayer.innerHTML = ''; }

    var range = results.range;
    var layers = state.layers;

    // Build walls L1..L7 (only traversed ones visible, others faded)
    for (var i = 0; i < 7; i++) {
      var inRange = i >= range.start && i <= range.end;
      var active = layers[i].active;
      var height = inRange ? 3.0 : 1.5;
      var status = !active ? 'inactive' : 'active';
      var wall = defBuildWall(i, height, 0.15, status);
      wall.position.y = 0;
      defScene.add(wall);
      defWallGroups.push(wall);

      // Label
      var el = document.createElement('div');
      el.className = 'float-label';
      el.innerHTML = '<span class="fl-dot" style="background:' + DS.LAYERS[i].color + '"></span>' + DS.LAYERS[i].id + ' ' + DS.LAYERS[i].name + (active ? '' : ' (nonaktif)');
      defLabelLayer.appendChild(el);
      wall.userData.labelEl = el;
    }

    // Target at center
    var target = defBuildTarget();
    defScene.add(target);
    defWallGroups.push(target);

    // Attacker at entry point
    var ep = DS.ENTRY_POINTS[state.entryPoint];
    var attackerRadius = 0.8 + (ep.startLayer === 0 ? 0 : ep.startLayer) * 1.1 + 2.5;
    var attackerPos = new defTHREE.Vector3(attackerRadius, 0, 0);
    var attacker = defBuildAttacker(attackerPos);
    defScene.add(attacker);
    defHumanMeshes.attacker = attacker;

    // Admin near target
    var admin = defBuildAdmin(new defTHREE.Vector3(-1.5, 0, 1.5));
    defScene.add(admin);
    defHumanMeshes.admin = admin;

    // User outside
    var user = defBuildUser(new defTHREE.Vector3(0, 0, attackerRadius + 1));
    defScene.add(user);
    defHumanMeshes.user = user;
  }

  function defOnResize() {
    if (!defRenderer) return;
    var host = $('defScene3d');
    if (!host.clientWidth) return;
    defCamera.aspect = host.clientWidth / host.clientHeight;
    defCamera.updateProjectionMatrix();
    defRenderer.setSize(host.clientWidth, host.clientHeight);
  }

  function defOnPointerDown(evt) {
    var rect = defRenderer.domElement.getBoundingClientRect();
    var mouse = new defTHREE.Vector2(
      ((evt.clientX - rect.left) / rect.width) * 2 - 1,
      -((evt.clientY - rect.top) / rect.height) * 2 + 1
    );
    var raycaster = new defTHREE.Raycaster();
    raycaster.setFromCamera(mouse, defCamera);
    var hits = raycaster.intersectObjects(defWallGroups, true);
    if (!hits.length) return;
    var obj = hits[0].object;
    while (obj && !obj.userData.layer && !obj.userData.isTarget) obj = obj.parent;
    if (!obj) return;
    if (obj.userData.isTarget) {
      showDetail({ layer: -1, name: 'Target (Data/Database)', active: true, strength: 100, blockProb: 0, blockedCount: 0 });
      return;
    }
    var layer = obj.userData.layer;
    var L = DS.LAYERS[layer];
    var range = getLayerRange();
    var p = state.layers[layer].active ? state.effectiveness[state.attack][layer] * (state.layers[layer].strength / 100) : 0;
    var sim = state.lastResults?.sim;
    var blocked = sim ? (sim.blockedAt[layer] || 0) : 0;
    showDetail({ layer: layer, name: L.name, active: state.layers[layer].active, strength: state.layers[layer].strength, blockProb: p, blockedCount: blocked });
  }

  function showDetail(info) {
    var title = $('defDetailTitle'), grid = $('defDetailGrid');
    if (info.layer === -1) {
      title.textContent = info.name;
      grid.innerHTML = '<div class="def-detail-item"><div class="dl">Aset Target</div><div class="dv">Data/Database</div></div>';
      $('defDetailPanel').classList.add('show');
      return;
    }
    title.textContent = info.layer + ' — ' + info.name;
    var pct = (info.blockProb * 100).toFixed(2);
    var items = [
      ['Status', info.active ? 'Aktif' : 'Nonaktif'],
      ['Kekuatan', info.strength + '/100'],
      ['Efektivitas Dasar vs ' + state.attack, (state.effectiveness[state.attack][info.layer] * 100).toFixed(1) + '%'],
      ['Peluang Blokir p(i,s)', pct + '%'],
      ['Tertahan (Simulasi)', info.blockedCount]
    ];
    grid.innerHTML = items.map(function (it) {
      return '<div class="def-detail-item"><div class="dl">' + it[0] + '</div><div class="dv">' + it[1] + '</div></div>';
    }).join('');
    $('defDetailPanel').classList.add('show');
  }

  function defAnimate() {
    defRaf = requestAnimationFrame(defAnimate);
    if (!defAnimPlaying || defReduced) {
      if (defControls && typeof defControls.update === 'function') defControls.update();
      if (defRenderer && defScene && defCamera) defRenderer.render(defScene, defCamera);
      defUpdateLabels();
      return;
    }
    var dt = defClock.getDelta() * defAnimSpeed;
    // Animate packets
    defPacketMeshes.forEach(function (pk) {
      pk.t += dt * pk.speed;
      if (pk.t > 1) {
        pk.t = 1;
        pk.mesh.visible = false;
      }
      var pt = pk.curve.getPoint(pk.t);
      pk.mesh.position.copy(pt);
    });
    if (defControls && typeof defControls.update === 'function') defControls.update();
    if (defRenderer && defScene && defCamera) defRenderer.render(defScene, defCamera);
    defUpdateLabels();
  }

  function defUpdateLabels() {
    if (!defCamera || !defRenderer) return;
    var host = $('defScene3d');
    var w = host.clientWidth, h = host.clientHeight;
    var tmp = new defTHREE.Vector3();
    defWallGroups.forEach(function (g) {
      if (!g.userData.labelEl) return;
      tmp.set(g.position.x, g.position.y + 2.0, g.position.z);
      tmp.project(defCamera);
      var x = (tmp.x * 0.5 + 0.5) * w;
      var y = (-tmp.y * 0.5 + 0.5) * h;
      g.userData.labelEl.style.left = x + 'px';
      g.userData.labelEl.style.top = y + 'px';
      g.userData.labelEl.style.display = (tmp.z > 1) ? 'none' : 'block';
    });
    // Human labels
    Object.values(defHumanMeshes).forEach(function (h) {
      if (!h.userData.labelEl) return;
      tmp.set(h.position.x, h.position.y + 1.5, h.position.z);
      tmp.project(defCamera);
      h.userData.labelEl.style.left = (tmp.x * 0.5 + 0.5) * w + 'px';
      h.userData.labelEl.style.top = (-tmp.y * 0.5 + 0.5) * h + 'px';
    });
  }

  function defDispose() {
    if (defRaf) cancelAnimationFrame(defRaf);
    if (defRenderer) { defRenderer.dispose(); if (defRenderer.domElement.parentNode) defRenderer.domElement.parentNode.removeChild(defRenderer.domElement); }
    window.removeEventListener('resize', defOnResize);
  }

  // ---- 2D SVG Fallback ----
  function render2DScene(results) {
    var host = $('defScene2d');
    var range = results.range;
    var layers = state.layers;
    var W = 100, H = 100, cx = 50, cy = 50;
    var parts = ['<svg viewBox="0 0 ' + W + ' ' + H + '" preserveAspectRatio="xMidYMid meet" role="img" aria-label="Diorama benteng berlapis 2D">'];
    // Rings
    for (var i = 6; i >= 0; i--) {
      var inRange = i >= range.start && i <= range.end;
      var r = 8 + i * 6;
      var active = layers[i].active;
      var color = DS.LAYERS[i].color;
      var cls = 'def-svg-ring' + (active && inRange ? '' : ' inactive');
      parts.push('<circle class="' + cls + '" cx="' + cx + '" cy="' + cy + '" r="' + r + '" fill="none" stroke="' + color + '" stroke-width="' + (inRange ? 3 : 1.5) + '" data-layer="' + i + '" style="cursor:pointer;"></circle>');
      // Label
      var angle = -Math.PI / 2;
      var lx = cx + (r + 3) * Math.cos(angle);
      var ly = cy + (r + 3) * Math.sin(angle);
      parts.push('<text class="def-svg-label" x="' + lx + '" y="' + ly + '" text-anchor="middle" fill="var(--text-secondary)" font-size="3" pointer-events="none">' + DS.LAYERS[i].id + '</text>');
    }
    // Target
    parts.push('<circle cx="' + cx + '" cy="' + cy + '" r="5" fill="#34d399" opacity="0.3"></circle>');
    parts.push('<circle cx="' + cx + '" cy="' + cy + '" r="4" fill="#34d399"></circle>');
    parts.push('<text class="def-svg-label" x="' + cx + '" y="' + (cy + 1) + '" text-anchor="middle" fill="#fff" font-size="3" font-weight="700">TARGET</text>');
    // Attacker
    var ep = DS.ENTRY_POINTS[state.entryPoint];
    var ar = 8 + (ep.startLayer === 0 ? 0 : ep.startLayer) * 6 + 10;
    parts.push('<circle cx="' + (cx + ar) + '" cy="' + cy + '" r="3" fill="#cc2222"></circle>');
    parts.push('<text class="def-svg-label" x="' + (cx + ar) + '" y="' + (cy - 5) + '" text-anchor="middle" fill="#cc2222" font-size="2.5">ATTACKER</text>');
    // Admin
    parts.push('<circle cx="' + (cx - 6) + '" cy="' + (cy + 6) + '" r="2.5" fill="#2b6cff"></circle>');
    parts.push('<text class="def-svg-label" x="' + (cx - 6) + '" y="' + (cy + 10) + '" text-anchor="middle" fill="#2b6cff" font-size="2.5">ADMIN</text>');
    parts.push('</svg>');
    host.innerHTML = parts.join('');

    // Click handlers
    host.querySelectorAll('circle[data-layer]').forEach(function (c) {
      c.addEventListener('click', function () {
        var layer = +c.getAttribute('data-layer');
        var L = DS.LAYERS[layer];
        var p = state.layers[layer].active ? state.effectiveness[state.attack][layer] * (state.layers[layer].strength / 100) : 0;
        var sim = state.lastResults?.sim;
        var blocked = sim ? (sim.blockedAt[layer] || 0) : 0;
        showDetail({ layer: layer, name: L.name, active: state.layers[layer].active, strength: state.layers[layer].strength, blockProb: p, blockedCount: blocked });
      });
    });
  }

  // ---- Accessible Table ----
  function renderAccessTable(results) {
    var tb = $('defAccessTable').querySelector('tbody');
    var range = results.range;
    var sim = results.sim;
    tb.innerHTML = '';
    for (var i = range.start; i <= range.end; i++) {
      var layer = state.layers[i];
      var p = layer.active ? state.effectiveness[state.attack][i] * (layer.strength / 100) : 0;
      var blocked = sim ? (sim.blockedAt[i] || 0) : 0;
      var tr = document.createElement('tr');
      tr.innerHTML =
        '<td>' + DS.LAYERS[i].id + '</td>' +
        '<td>' + DS.LAYERS[i].name + '</td>' +
        '<td>' + (layer.active ? 'Ya' : 'Tidak') + '</td>' +
        '<td>' + layer.strength + '</td>' +
        '<td>' + (p * 100).toFixed(2) + '%</td>' +
        '<td>' + blocked + '</td>';
      tb.appendChild(tr);
    }
  }

  // ---- Legend ----
  function updateLegend(results) {
    var leg = $('defLegend');
    leg.innerHTML =
      '<span><i class="dot" style="background:#4ade80"></i> Lapisan aktif</span>' +
      '<span><i class="dot" style="background:#fbbf24"></i> Sedang memblokir</span>' +
      '<span><i class="dot" style="background:#ef4444"></i> Tertembus</span>' +
      '<span><i class="dot" style="background:#55556a"></i> Nonaktif</span>' +
      '<span><i class="dot" style="background:#cc2222"></i> Penyerang</span>' +
      '<span><i class="dot" style="background:#2b6cff"></i> Admin</span>' +
      '<span><i class="dot" style="background:#4ade80"></i> Pengguna</span>' +
      '<span><i class="dot" style="background:#34d399"></i> Target</span>';
  }

  // ---- Animation Preparation ----
  function prepareAnimation(results) {
    defAnimTrials = results.sim.trials.slice();
    defAnimTrialIdx = 0;
    defAnimPacketIdx = 0;
    defPacketMeshes = [];

    // Show animation controls
    $('defAnimPlay').style.display = 'inline-flex';
    $('defAnimPause').style.display = 'inline-flex';
    $('defAnimRestart').style.display = 'inline-flex';
    $('defSpeedWrap').style.display = 'flex';

    if (!defScene) return; // 3D not loaded yet

    // Build packet paths for each trial
    var range = results.range;
    defPacketMeshes = [];
    defAnimTrials.forEach(function (trial, ti) {
      if (ti >= 60) return; // max 60 visualizations
      var curvePoints = [];
      var startR = 0.8 + (range.start === 0 ? 0 : range.start) * 1.1 + 2.5;
      var startPos = new defTHREE.Vector3(startR, 0.5, 0);
      curvePoints.push(startPos);
      // Through each layer
      for (var i = range.start; i <= range.end; i++) {
        var r = 0.8 + i * 1.1;
        var angle = 0;
        var pos = new defTHREE.Vector3(r * Math.cos(angle), 0.5 + (ti % 3) * 0.3, r * Math.sin(angle));
        curvePoints.push(pos);
      }
      // Target
      curvePoints.push(new defTHREE.Vector3(0, 0.5, 0));
      var curve = new defTHREE.CatmullRomCurve3(curvePoints);
      var cube = new defTHREE.Mesh(new defTHREE.BoxGeometry(0.18, 0.18, 0.18), defMat(0xef4444, { emissive: 0xef4444, emissiveIntensity: 0.8 }));
      defScene.add(cube);
      var rng = DS.mulberry32(state.seed + ti * 17);
      defPacketMeshes.push({
        mesh: cube, curve: curve, t: 0, speed: 0.3 + (ti % 5) * 0.1,
        trial: trial, stopped: false
      });
    });
  }

  function startAnimation() { defAnimPlaying = true; $('defAnimPlay').textContent = 'Playing...'; }
  function pauseAnimation() { defAnimPlaying = false; $('defAnimPlay').textContent = 'Play'; }
  function restartAnimation() {
    defPacketMeshes.forEach(function (pk) { pk.t = 0; pk.mesh.visible = true; });
    defAnimTrialIdx = 0;
    defAnimPlaying = true;
    $('defAnimPlay').textContent = 'Playing...';
  }

  function setAnimSpeed(v) { defAnimSpeed = v; }

  // ---- View Switching ----
  var defView = '3d';
  function defSetView(v) {
    defView = v;
    $('defView3d').classList.toggle('active', v === '3d');
    $('defView2d').classList.toggle('active', v === '2d');
    $('defViewTable').classList.toggle('active', v === 'table');
    $('defScene3d').classList.toggle('scene-hidden', v !== '3d');
    $('defScene2d').classList.toggle('scene-hidden', v !== '2d');
    $('defLabelLayer').classList.toggle('scene-hidden', v === 'table');
    $('defTableWrap').style.display = v === 'table' ? 'block' : 'none';
    if (v === '2d' && state.lastResults) render2DScene(state.lastResults);
    if (v === '3d' && state.lastResults && !defTHREE) {
      defLoadThree().then(function () { defInit3D(); defBuildScene(state.lastResults); defAnimate(); });
    }
  }

  // ---- Export / Copy ----
  function exportJson() {
    var out = {
      layers: state.layers,
      effectiveness: state.effectiveness,
      attack: state.attack,
      entryPoint: state.entryPoint,
      target: state.target,
      trials: state.trials,
      seed: state.seed,
      baseDelay: state.baseDelay,
      results: state.lastResults
    };
    var blob = new Blob([JSON.stringify(out, null, 2)], { type: 'application/json' });
    var url = URL.createObjectURL(blob);
    var a = document.createElement('a'); a.href = url; a.download = 'defense_in_depth.json'; document.body.appendChild(a); a.click();
    setTimeout(function () { URL.revokeObjectURL(url); a.remove(); }, 100);
  }

  function copyAnswer() {
    var txt = 'Mengapa konsep defense in depth dianggap penting dalam pengamanan jaringan?\n\n' +
      ($('defAnalysis')._plain || '') +
      '\n\n---\nSkor Postur: ' + (state.lastResults?.score || 0) + '/100\nRisiko Residual: ' + (state.lastResults?.analytical?.riskPercent?.toFixed(2) || 0) + '%';
    if (navigator.clipboard && navigator.clipboard.writeText) {
      navigator.clipboard.writeText(txt).then(function () {
        var btn = $('defCopyAnswer'); btn.textContent = 'Tersalin!'; setTimeout(function () { btn.textContent = 'Salin Jawaban (Esai)'; }, 1500);
      });
    } else {
      var blob = new Blob([txt], { type: 'text/plain' });
      var url = URL.createObjectURL(blob);
      var a = document.createElement('a'); a.href = url; a.download = 'jawaban_defense_in_depth.txt'; document.body.appendChild(a); a.click();
      setTimeout(function () { URL.revokeObjectURL(url); a.remove(); }, 100);
    }
  }

  function exampleConfig() {
    // Set a good example: Phishing, Internet entry, Data target, full defense
    state.attack = 'Phishing';
    state.entryPoint = 'internet';
    state.target = 'data';
    state.trials = 1000;
    state.seed = 12345;
    state.baseDelay = 5;
    applyPreset('full');
    $('defAttack').value = state.attack;
    $('defEntry').value = state.entryPoint;
    $('defTarget').value = state.target;
    $('defTrials').value = state.trials;
    $('defSeed').value = state.seed;
    $('defBaseDelay').value = state.baseDelay;
    renderAllOutputs();
  }

  function resetAll() {
    state.layers = DS.LAYERS.map(function (l) { return { active: l.active, strength: l.defaultStrength }; });
    state.effectiveness = JSON.parse(JSON.stringify(DS.DEFAULT_EFFECTIVENESS));
    state.attack = 'Phishing';
    state.entryPoint = 'internet';
    state.target = 'data';
    state.trials = 1000;
    state.seed = 42;
    state.baseDelay = 5;
    $('defAttack').value = state.attack;
    $('defEntry').value = state.entryPoint;
    $('defTarget').value = state.target;
    $('defTrials').value = state.trials;
    $('defSeed').value = state.seed;
    $('defBaseDelay').value = state.baseDelay;
    renderLayerTable();
    renderMatrixTable();
    renderAllOutputs();
  }

  function randomSeed() {
    state.seed = Math.floor(Math.random() * 0xFFFFFFFF);
    $('defSeed').value = state.seed;
    renderAllOutputs();
  }

  function resetMatrix() {
    state.effectiveness = JSON.parse(JSON.stringify(DS.DEFAULT_EFFECTIVENESS));
    renderMatrixTable();
    renderAllOutputs();
  }

  // ---- Event Wiring ----
  function wireEvents() {
    $('defPreset').addEventListener('change', function () { applyPreset(this.value); });
    $('defAttack').addEventListener('change', function () { state.attack = this.value; renderAllOutputs(); });
    $('defEntry').addEventListener('change', function () { state.entryPoint = this.value; renderAllOutputs(); });
    $('defTarget').addEventListener('change', function () { state.target = this.value; renderAllOutputs(); });
    $('defTrials').addEventListener('change', function () { state.trials = Math.max(10, Math.min(10000, +this.value || 1000)); renderAllOutputs(); });
    $('defSeed').addEventListener('change', function () { state.seed = +this.value || 0; renderAllOutputs(); });
    $('defBaseDelay').addEventListener('change', function () { state.baseDelay = Math.max(1, Math.min(60, +this.value || 5)); renderAllOutputs(); });
    $('defRandomSeed').addEventListener('click', randomSeed);
    $('defRun').addEventListener('click', renderAllOutputs);
    $('defExample').addEventListener('click', exampleConfig);
    $('defReset').addEventListener('click', resetAll);
    $('defExportJson').addEventListener('click', exportJson);
    $('defCopyText').addEventListener('click', copyAnswer);
    $('defMatrixReset').addEventListener('click', resetMatrix);
    $('defView3d').addEventListener('click', function () { defSetView('3d'); });
    $('defView2d').addEventListener('click', function () { defSetView('2d'); });
    $('defViewTable').addEventListener('click', function () { defSetView('table'); });
    $('defAnimPlay').addEventListener('click', startAnimation);
    $('defAnimPause').addEventListener('click', pauseAnimation);
    $('defAnimRestart').addEventListener('click', restartAnimation);
    $('defAnimSpeed').addEventListener('change', function () { setAnimSpeed(+this.value); });
  }

  // ---- Init ----
  populateDropdowns();
  renderLayerTable();
  renderMatrixTable();
  wireEvents();

  // Initial run
  renderAllOutputs();

  // Load 3D on first view
  if (document.readyState === 'complete' || document.readyState === 'interactive') {
    setTimeout(function () { defLoadThree().then(function () { defInit3D(); if (state.lastResults) defBuildScene(state.lastResults); defAnimate(); }); }, 400);
  } else {
    window.addEventListener('DOMContentLoaded', function () {
      setTimeout(function () { defLoadThree().then(function () { defInit3D(); if (state.lastResults) defBuildScene(state.lastResults); defAnimate(); }); }, 400);
    });
  }

  // Re-render when topology changes (device list doesn't affect Module 2 directly, but keep for consistency)
  NetApp.subscribe(function () { /* no-op for now */ });

  window.DefApp = { state: state, runSimulation: runSimulation, renderAllOutputs: renderAllOutputs };
})();
</script>"##.to_string()
}