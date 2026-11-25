// Unit tests for Defense in Depth pure functions
// Run in browser console after loading the Jaringan Komputer page

(function () {
  'use strict';

  // ---- Test utilities ----
  var passed = 0, failed = 0;
  function assert(cond, msg) {
    if (cond) { passed++; console.log('✓ ' + msg); }
    else { failed++; console.error('✗ ' + msg); }
  }
  function assertApprox(actual, expected, tolerance, msg) {
    var diff = Math.abs(actual - expected);
    if (diff <= tolerance) { passed++; console.log('✓ ' + msg + ' (' + actual + ' ≈ ' + expected + ')'); }
    else { failed++; console.error('✗ ' + msg + ': got ' + actual + ', expected ~' + expected + ' (tol ' + tolerance + ')'); }
  }

  // ---- Recreate pure functions for testing (copied from DefenseSim) ----
  function mulberry32(seed) {
    var a = seed >>> 0;
    return function () {
      a |= 0; a = (a + 0x6D2B79F5) | 0;
      var t = Math.imul(a ^ (a >>> 15), 1 | a);
      t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
      return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
    };
  }

  var LAYERS = [
    { id: 'L1', name: 'Kebijakan & Edukasi Pengguna', color: '#6c8eff', defaultStrength: 70, active: true, controls: [], canDetect: false },
    { id: 'L2', name: 'Keamanan Fisik', color: '#ef4444', defaultStrength: 70, active: true, controls: [], canDetect: false },
    { id: 'L3', name: 'Perimeter (Firewall)', color: '#3a6bdf', defaultStrength: 70, active: true, controls: [], canDetect: false },
    { id: 'L4', name: 'Jaringan (Segmentasi, IDS/IPS)', color: '#22d3ee', defaultStrength: 70, active: true, controls: [], canDetect: true, detectBase: 0.5 },
    { id: 'L5', name: 'Host/Endpoint (AV, Patch)', color: '#a78bfa', defaultStrength: 70, active: true, controls: [], canDetect: true, detectBase: 0.4 },
    { id: 'L6', name: 'Aplikasi (WAF, MFA, Validasi)', color: '#fbbf24', defaultStrength: 70, active: true, controls: [], canDetect: false },
    { id: 'L7', name: 'Data (Enkripsi, Backup)', color: '#34d399', defaultStrength: 70, active: true, controls: [], canDetect: false }
  ];

  var DEFAULT_EFFECTIVENESS = {
    'Phishing':            [0.60, 0.00, 0.30, 0.10, 0.40, 0.10, 0.10],
    'Malware/Ransomware':  [0.30, 0.00, 0.40, 0.40, 0.70, 0.20, 0.50],
    'DDoS':                [0.00, 0.00, 0.70, 0.40, 0.00, 0.30, 0.00],
    'Brute-force':         [0.30, 0.00, 0.30, 0.30, 0.30, 0.60, 0.20],
    'SQL Injection':       [0.05, 0.00, 0.30, 0.20, 0.05, 0.80, 0.40],
    'Insider Threat':      [0.40, 0.30, 0.00, 0.40, 0.30, 0.30, 0.50],
    'Pencurian Fisik':     [0.30, 0.80, 0.00, 0.00, 0.30, 0.00, 0.60]
  };

  function blockProbability(layer, attack, effectiveness) {
    if (!layer.active) return 0;
    var eff = effectiveness[attack];
    if (!eff) return 0;
    var layerIdx = LAYERS.indexOf(layer);
    if (layerIdx < 0) return 0;
    return eff[layerIdx] * (layer.strength / 100);
  }

  function analyticalResidualRisk(layers, attack, effectiveness, startLayer, endLayer) {
    var details = [];
    var risk = 1.0;
    for (var i = startLayer; i <= endLayer; i++) {
      var layer = layers[i];
      var p = blockProbability(layer, attack, effectiveness);
      var pass = 1 - p;
      risk *= pass;
      details.push({ layer: layer.id, name: layer.name, active: layer.active, strength: layer.strength, effectiveness: effectiveness[attack] ? effectiveness[attack][i] : 0, blockProb: p, passProb: pass });
    }
    return { risk: risk, riskPercent: risk * 100, details: details };
  }

  function runSimulation(layers, attack, effectiveness, startLayer, endLayer, N, seed) {
    var rng = mulberry32(seed);
    var blockedAt = [];
    var penetrated = 0;
    var detected = 0;
    var totalLayersPassed = 0;
    var trials = [];
    for (var trial = 0; trial < N; trial++) {
      var stopped = false;
      var layersPassed = 0;
      for (var i = startLayer; i <= endLayer; i++) {
        var layer = layers[i];
        var p = blockProbability(layer, attack, effectiveness);
        if (rng() < p) {
          blockedAt[i] = (blockedAt[i] || 0) + 1;
          stopped = true;
          if (trials.length < 60) trials.push({ stoppedAt: i, penetrated: false });
          break;
        }
        layersPassed++;
        if (layer.canDetect && layer.active && rng() < layer.detectBase * (layer.strength / 100)) {
          detected++;
        }
      }
      if (!stopped) {
        penetrated++;
        totalLayersPassed += layersPassed;
        if (trials.length < 60) trials.push({ stoppedAt: -1, penetrated: true, layersPassed: layersPassed });
      }
    }
    return { blockedAt: blockedAt, penetrated: penetrated, penetratedPercent: (penetrated / N) * 100, blockedPercent: 100 - (penetrated / N) * 100, detected: detected, avgLayersPassed: penetrated > 0 ? totalLayersPassed / penetrated : 0, trials: trials };
  }

  function calculateDetection(simResult, layers, startLayer, endLayer, baseDelayMinutes) {
    var delayMinutes = simResult.avgLayersPassed * baseDelayMinutes;
    return { detectedCount: simResult.detected, detectedPercent: simResult.penetrated > 0 ? (simResult.detected / (simResult.penetrated + simResult.detected)) * 100 : 0, attackerDelayMinutes: delayMinutes };
  }

  function findStrongestLayer(layers, attack, effectiveness, startLayer, endLayer) {
    var bestIdx = startLayer, bestP = -1;
    for (var i = startLayer; i <= endLayer; i++) {
      var p = blockProbability(layers[i], attack, effectiveness);
      if (p > bestP) { bestP = p; bestIdx = i; }
    }
    return { index: bestIdx, blockProb: bestP };
  }

  function runComparators(layers, attack, effectiveness, startLayer, endLayer, N, seed, baseDelayMinutes) {
    var strongest = findStrongestLayer(layers, attack, effectiveness, startLayer, endLayer);
    var onlyStrongest = LAYERS.map(function (l, i) { return { active: i === strongest.index, strength: i === strongest.index ? layers[i].strength : 0 }; });
    var simA = runSimulation(onlyStrongest, attack, effectiveness, startLayer, endLayer, N, seed);
    var riskA = analyticalResidualRisk(onlyStrongest, attack, effectiveness, startLayer, endLayer).riskPercent;
    var simB = runSimulation(layers, attack, effectiveness, startLayer, endLayer, N, seed);
    var riskB = analyticalResidualRisk(layers, attack, effectiveness, startLayer, endLayer).riskPercent;
    var withoutStrongest = layers.map(function (l, i) { return { active: i !== strongest.index && l.active, strength: i !== strongest.index ? l.strength : 0 }; });
    var simC = runSimulation(withoutStrongest, attack, effectiveness, startLayer, endLayer, N, seed);
    var riskC = analyticalResidualRisk(withoutStrongest, attack, effectiveness, startLayer, endLayer).riskPercent;
    return {
      onlyStrongest: { riskPercent: simA.penetratedPercent, simulated: simA, analytical: riskA, layer: strongest.index },
      allLayers: { riskPercent: simB.penetratedPercent, simulated: simB, analytical: riskB },
      withoutStrongest: { riskPercent: simC.penetratedPercent, simulated: simC, analytical: riskC, layer: strongest.index }
    };
  }

  function analyzeWeakPoints(layers, attack, effectiveness, startLayer, endLayer, simResult) {
    var weak = [];
    var contributions = [];
    var baseRisk = analyticalResidualRisk(layers, attack, effectiveness, startLayer, endLayer).risk;
    for (var i = startLayer; i <= endLayer; i++) {
      if (!layers[i].active) {
        weak.push({ type: 'inactive', layer: i, message: LAYERS[i].name + ' nonaktif' });
        contributions.push({ layer: i, marginalIncrease: 0, message: 'Nonaktif' });
        continue;
      }
      var without = layers.map(function (l, idx) { return { active: idx !== i ? l.active : false, strength: l.strength }; });
      var riskWithout = analyticalResidualRisk(without, attack, effectiveness, startLayer, endLayer).risk;
      var marginal = (riskWithout - baseRisk) * 100;
      contributions.push({ layer: i, marginalIncrease: marginal, message: '+ ' + marginal.toFixed(2) + ' pp risiko' });
      if (marginal < 0.5) weak.push({ type: 'low-contrib', layer: i, message: LAYERS[i].name + ' kontribusi marginal < 0.5 pp' });
    }
    for (var i = startLayer; i <= endLayer; i++) {
      if (!layers[i].active) continue;
      var p = blockProbability(layers[i], attack, effectiveness);
      if (p < 0.15) weak.push({ type: 'low-effectiveness', layer: i, attack: attack, message: attack + ' x ' + LAYERS[i].name + ': peluang blokir efektif ' + (p * 100).toFixed(1) + '% (< 15%)' });
    }
    var recommendations = weak.map(function (w) {
      if (w.type === 'inactive') return 'Aktifkan ' + LAYERS[w.layer].name + ': ' + LAYERS[w.layer].controls.slice(0, 3).join(', ');
      else if (w.type === 'low-contrib') return 'Tingkatkan kekuatan ' + LAYERS[w.layer].name;
      else return 'Perkuat ' + LAYERS[w.layer].name + ' terhadap ' + w.attack;
    });
    return { weakPoints: weak, contributions: contributions, recommendations: recommendations };
  }

  function postureScore(residualRiskPercent) { return Math.max(0, Math.min(100, Math.round(100 * (1 - residualRiskPercent / 100)))); }

  // ---- Test Cases ----

  console.log('=== Defense in Depth Pure Function Tests ===\n');

  // Test 1: All layers inactive -> 100% residual risk
  (function () {
    var layers = LAYERS.map(function (l) { return { active: false, strength: 0 }; });
    var res = analyticalResidualRisk(layers, 'Phishing', DEFAULT_EFFECTIVENESS, 0, 6);
    assertApprox(res.riskPercent, 100, 0.01, 'Test 1: All layers inactive -> 100% risk');
  })();

  // Test 2: Adding/activating a layer never increases residual risk (monotonic)
  (function () {
    var layers1 = LAYERS.map(function (l) { return { active: false, strength: 0 }; });
    layers1[2] = { active: true, strength: 70 }; // L3 only
    var res1 = analyticalResidualRisk(layers1, 'Phishing', DEFAULT_EFFECTIVENESS, 0, 6).riskPercent;

    var layers2 = LAYERS.map(function (l) { return { active: false, strength: 0 }; });
    layers2[2] = { active: true, strength: 70 };
    layers2[4] = { active: true, strength: 70 }; // L3 + L5
    var res2 = analyticalResidualRisk(layers2, 'Phishing', DEFAULT_EFFECTIVENESS, 0, 6).riskPercent;

    assert(res2 <= res1 + 0.001, 'Test 2: Adding layer L5 does not increase risk (' + res2.toFixed(4) + ' <= ' + res1.toFixed(4) + ')');
  })();

  // Test 3: Same seed + same input = identical simulation results
  (function () {
    var layers = LAYERS.map(function (l) { return { active: true, strength: 70 }; });
    var sim1 = runSimulation(layers, 'Phishing', DEFAULT_EFFECTIVENESS, 0, 6, 1000, 42);
    var sim2 = runSimulation(layers, 'Phishing', DEFAULT_EFFECTIVENESS, 0, 6, 1000, 42);
    assert(sim1.penetrated === sim2.penetrated && sim1.detected === sim2.detected, 'Test 3: Same seed produces identical results');
  })();

  // Test 4: Different seeds produce different results (usually)
  (function () {
    var layers = LAYERS.map(function (l) { return { active: true, strength: 70 }; });
    var sim1 = runSimulation(layers, 'Phishing', DEFAULT_EFFECTIVENESS, 0, 6, 1000, 42);
    var sim2 = runSimulation(layers, 'Phishing', DEFAULT_EFFECTIVENESS, 0, 6, 1000, 43);
    // Very unlikely to be identical with different seeds
    assert(sim1.penetrated !== sim2.penetrated || sim1.detected !== sim2.detected, 'Test 4: Different seeds produce different results');
  })();

  // Test 5: Large N simulation approaches analytical (within ±2pp)
  (function () {
    var layers = LAYERS.map(function (l) { return { active: true, strength: 70 }; });
    var ana = analyticalResidualRisk(layers, 'Phishing', DEFAULT_EFFECTIVENESS, 0, 6);
    var sim = runSimulation(layers, 'Phishing', DEFAULT_EFFECTIVENESS, 0, 6, 10000, 999);
    var diff = Math.abs(sim.penetratedPercent - ana.riskPercent);
    assertApprox(sim.penetratedPercent, ana.riskPercent, 2.0, 'Test 5: Large N sim ≈ analytical (diff=' + diff.toFixed(2) + 'pp)');
  })();

  // Test 6: Entry point "Fisik" (physical) starts at L2 (index 1)
  (function () {
    var layers = LAYERS.map(function (l) { return { active: true, strength: 70 }; });
    var res = analyticalResidualRisk(layers, 'Pencurian Fisik', DEFAULT_EFFECTIVENESS, 1, 6); // L2..L7
    // L1 effectiveness for Pencurian Fisik is 0.30, L2 is 0.80
    // With L1 skipped, risk should be higher than if L1 included
    var resWithL1 = analyticalResidualRisk(layers, 'Pencurian Fisik', DEFAULT_EFFECTIVENESS, 0, 6);
    assert(res.riskPercent > resWithL1.riskPercent, 'Test 6: Physical entry (L2..L7) has higher risk than Internet (L1..L7)');
  })();

  // Test 7: Entry point "Insider" starts at L4 (index 3)
  (function () {
    var layers = LAYERS.map(function (l) { return { active: true, strength: 70 }; });
    var res = analyticalResidualRisk(layers, 'Insider Threat', DEFAULT_EFFECTIVENESS, 3, 6); // L4..L7
    var resWithL1 = analyticalResidualRisk(layers, 'Insider Threat', DEFAULT_EFFECTIVENESS, 0, 6);
    assert(res.riskPercent > resWithL1.riskPercent, 'Test 7: Insider entry (L4..L7) has higher risk than Internet (L1..L7)');
  })();

  // Test 8: Target "Endpoint" ends at L5 (index 4)
  (function () {
    var layers = LAYERS.map(function (l) { return { active: true, strength: 70 }; });
    var res = analyticalResidualRisk(layers, 'Malware/Ransomware', DEFAULT_EFFECTIVENESS, 0, 4); // L1..L5
    var resToL7 = analyticalResidualRisk(layers, 'Malware/Ransomware', DEFAULT_EFFECTIVENESS, 0, 6);
    assert(res.riskPercent > resToL7.riskPercent, 'Test 8: Endpoint target (L1..L5) has higher risk than Data target (L1..L7)');
  })();

  // Test 9: All layers active gives risk <= strongest single layer
  (function () {
    var layers = LAYERS.map(function (l) { return { active: true, strength: 70 }; });
    var comp = runComparators(layers, 'Phishing', DEFAULT_EFFECTIVENESS, 0, 6, 1000, 42, 5);
    assert(comp.allLayers.riskPercent <= comp.onlyStrongest.riskPercent + 0.01, 'Test 9: Defense in depth risk (' + comp.allLayers.riskPercent.toFixed(2) + '%) <= Single layer risk (' + comp.onlyStrongest.riskPercent.toFixed(2) + '%)');
  })();

  // Test 10: Disabling strongest layer doesn't make risk 100% if others active
  (function () {
    var layers = LAYERS.map(function (l) { return { active: true, strength: 70 }; });
    var comp = runComparators(layers, 'Phishing', DEFAULT_EFFECTIVENESS, 0, 6, 1000, 42, 5);
    assert(comp.withoutStrongest.riskPercent < 100, 'Test 10: Without strongest layer, risk < 100% (' + comp.withoutStrongest.riskPercent.toFixed(2) + '%)');
  })();

  // Test 11: Blocked counts + penetrated = N
  (function () {
    var layers = LAYERS.map(function (l) { return { active: true, strength: 70 }; });
    var sim = runSimulation(layers, 'Phishing', DEFAULT_EFFECTIVENESS, 0, 6, 1000, 42);
    var totalBlocked = sim.blockedAt.reduce(function (a, b) { return a + (b || 0); }, 0);
    assert(totalBlocked + sim.penetrated === 1000, 'Test 11: Blocked (' + totalBlocked + ') + Penetrated (' + sim.penetrated + ') = N (1000)');
  })();

  // Test 12: Edge case - strength 100 with effectiveness 1 -> risk 0%
  (function () {
    var layers = LAYERS.map(function (l) { return { active: true, strength: 100 }; });
    var eff = { 'TestAttack': [1, 1, 1, 1, 1, 1, 1] };
    var res = analyticalResidualRisk(layers, 'TestAttack', eff, 0, 6);
    assertApprox(res.riskPercent, 0, 0.01, 'Test 12: Strength 100 + eff 1.0 -> 0% risk');
  })();

  // Test 13: Edge case - all effectiveness 0 -> risk 100%
  (function () {
    var layers = LAYERS.map(function (l) { return { active: true, strength: 100 }; });
    var eff = { 'TestAttack': [0, 0, 0, 0, 0, 0, 0] };
    var res = analyticalResidualRisk(layers, 'TestAttack', eff, 0, 6);
    assertApprox(res.riskPercent, 100, 0.01, 'Test 13: All effectiveness 0 -> 100% risk');
  })();

  // Test 14: Detection probability formula
  (function () {
    var layers = LAYERS.map(function (l) { return { active: true, strength: 70 }; });
    var sim = runSimulation(layers, 'Phishing', DEFAULT_EFFECTIVENESS, 0, 6, 10000, 123);
    var det = calculateDetection(sim, layers, 0, 6, 5);
    // IDS at L4: 0.5 * 0.7 = 0.35 per passed attack
    // EDR at L5: 0.4 * 0.7 = 0.28 per passed attack
    // Should have some detections
    assert(det.detectedCount > 0, 'Test 14: Detection count > 0 (' + det.detectedCount + ')');
  })();

  // Test 15: Weak points identifies inactive layers
  (function () {
    var layers = LAYERS.map(function (l) { return { active: true, strength: 70 }; });
    layers[2].active = false; // L3 inactive
    var sim = runSimulation(layers, 'Phishing', DEFAULT_EFFECTIVENESS, 0, 6, 1000, 42);
    var wp = analyzeWeakPoints(layers, 'Phishing', DEFAULT_EFFECTIVENESS, 0, 6, sim);
    var hasInactive = wp.weakPoints.some(function (w) { return w.type === 'inactive' && w.layer === 2; });
    assert(hasInactive, 'Test 15: Weak points identifies inactive L3');
  })();

  // Test 16: Marginal contribution calculation
  (function () {
    var layers = LAYERS.map(function (l) { return { active: true, strength: 70 }; });
    var sim = runSimulation(layers, 'Phishing', DEFAULT_EFFECTIVENESS, 0, 6, 1000, 42);
    var wp = analyzeWeakPoints(layers, 'Phishing', DEFAULT_EFFECTIVENESS, 0, 6, sim);
    // All marginal increases should be >= 0 (disabling a layer never reduces risk)
    var allNonNeg = wp.contributions.every(function (c) { return c.marginalIncrease >= -0.001; });
    assert(allNonNeg, 'Test 16: All marginal contributions >= 0');
  })();

  // Test 17: Posture score formula
  (function () {
    assert(postureScore(0) === 100, 'Test 17a: 0% risk -> 100 score');
    assert(postureScore(50) === 50, 'Test 17b: 50% risk -> 50 score');
    assert(postureScore(100) === 0, 'Test 17c: 100% risk -> 0 score');
    assert(postureScore(150) === 0, 'Test 17d: >100% risk clamped to 0');
  })();

  // Test 18: Mulberry32 deterministic
  (function () {
    var rng1 = mulberry32(42);
    var rng2 = mulberry32(42);
    var same = true;
    for (var i = 0; i < 100; i++) { if (rng1() !== rng2()) { same = false; break; } }
    assert(same, 'Test 18: mulberry32 with same seed produces identical sequence');
  })();

  // Test 19: Block probability formula p = eff * (strength/100) when active
  (function () {
    var layer = { active: true, strength: 50 };
    var eff = { 'Test': [0.8, 0, 0, 0, 0, 0, 0] };
    // Manually test L1
    var p = blockProbability(layer, 'Test', eff);
    assertApprox(p, 0.4, 0.001, 'Test 19: p = 0.8 * (50/100) = 0.4');
  })();

  // Test 20: Block probability is 0 when inactive
  (function () {
    var layer = { active: false, strength: 100 };
    var eff = { 'Test': [1, 0, 0, 0, 0, 0, 0] };
    var p = blockProbability(layer, 'Test', eff);
    assert(p === 0, 'Test 20: Inactive layer -> p = 0');
  })();

  // ---- Summary ----
  console.log('\n=== Test Summary ===');
  console.log('Passed: ' + passed);
  console.log('Failed: ' + failed);
  console.log('Total: ' + (passed + failed));
  if (failed === 0) console.log('\n✓ All tests passed!');
  else console.log('\n✗ ' + failed + ' test(s) failed.');

})();