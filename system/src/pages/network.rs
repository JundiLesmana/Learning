use axum::response::Html;
use crate::pages::layout::layout;
use crate::pages::network_topology::topology_html;
use crate::pages::network_monitoring::monitoring_html;
use crate::pages::network_defense::defense_html;
use crate::pages::network_troubleshooting::troubleshooting_html;

pub async fn handler() -> Html<String> {
    // UI wiring script for topology editor (extracted from original monolithic file)
    let ui_wiring = r##"<script>
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
    if (v === 'table') renderAccessTable();
  }
  $('view3d').addEventListener('click', function () { setView('3d'); });
  $('view2d').addEventListener('click', function () { setView('2d'); });
  $('viewTable').addEventListener('click', function () { setView('table'); });
  $('btnResetView').addEventListener('click', function () { window.dispatchEvent(new CustomEvent('netresetview')); });

  // ---- Module tabs ----
  document.querySelectorAll('.module-tab').forEach(function (btn) {
    btn.addEventListener('click', function () {
      document.querySelectorAll('.module-tab').forEach(function (b) { b.classList.remove('active'); b.setAttribute('aria-selected', 'false'); });
      document.querySelectorAll('.module-panel').forEach(function (p) { p.classList.remove('active'); });
      btn.classList.add('active'); btn.setAttribute('aria-selected', 'true');
      var target = btn.getAttribute('data-target');
      document.getElementById(target).classList.add('active');
    });
  });

  // ---- Initial render ----
  NetApp.loadPreset('kantor');
  renderAll();
})();

// 2D SVG renderer for topology (from original)
function render2D() {
  var host = $('scene2d');
  var st = NetApp.get();
  var pos = NetApp.layout(st.devices, st.links);
  var colors = {}; Object.keys(NetApp.TYPES).forEach(function (k) { colors[k] = NetApp.TYPES[k].color; });
  var reach = NetApp.reachableFromInternet(st.devices, st.links);
  var W = 600, H = 400, cx = W / 2, cy = H / 2;
  var scaleX = 40, scaleZ = 30;
  var parts = ['<svg viewBox="0 0 ' + W + ' ' + H + '" preserveAspectRatio="xMidYMid meet" width="100%" height="100%" role="img" aria-label="Topologi jaringan 2D">'];
  st.links.forEach(function (l) {
    var pa = pos[l.a], pb = pos[l.b];
    if (!pa || !pb) return;
    var x1 = cx + pa.x * scaleX, y1 = cy + pa.z * scaleZ;
    var x2 = cx + pb.x * scaleX, y2 = cy + pb.z * scaleZ;
    var reachA = reach[l.a], reachB = reach[l.b];
    var color = (reachA && reachB) ? '#6c8eff' : '#ef4444';
    parts.push('<line x1="' + x1 + '" y1="' + y1 + '" x2="' + x2 + '" y2="' + y2 + '" stroke="' + color + '" stroke-width="2" stroke-linecap="round"></line>');
  });
  st.devices.forEach(function (d) {
    var p = pos[d.id]; if (!p) return;
    var x = cx + p.x * scaleX, y = cy + p.z * scaleZ;
    var r = d.kind === 'internet' ? 20 : (d.kind === 'router' || d.kind === 'firewall' ? 16 : 14);
    var fill = colors[d.kind] || '#9898b0';
    var isReach = reach[d.id];
    parts.push('<circle class="ts-svg-device" data-name="' + d.name + '" cx="' + x + '" cy="' + y + '" r="' + r + '" fill="' + fill + '" stroke="#fff" stroke-width="2" style="cursor:pointer;"></circle>');
    parts.push('<text class="ts-svg-label" x="' + x + '" y="' + (y + r + 14) + '" text-anchor="middle" font-size="11" fill="var(--text-primary)" pointer-events="none">' + d.name + '</text>');
  });
  parts.push('</svg>');
  host.innerHTML = parts.join('');

  host.querySelectorAll('.ts-svg-device').forEach(function (c) {
    c.addEventListener('click', function () {
      var name = c.getAttribute('data-name');
      var d = st.devices.find(function (x) { return x.name === name; });
      if (d && window.NetShowDetail) window.NetShowDetail(d);
    });
  });
}
</script>"##;

    let body = format!(r##"
<script src="https://unpkg.com/three@0.160.0/build/three.min.js"></script>
{topology_html}
{monitoring_html}
{defense_html}
{troubleshooting_html}
{ui_wiring}
"##,
        topology_html = topology_html(),
        monitoring_html = monitoring_html(),
        defense_html = defense_html(),
        troubleshooting_html = troubleshooting_html(),
        ui_wiring = ui_wiring
    );
    Html(layout("Jaringan Komputer", "network", "/Learning/", &body))
}