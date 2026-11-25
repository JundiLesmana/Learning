// use axum::response::Html;

pub fn topology_html() -> String {
    r##"<script>
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

  // ---- State ----
  var devices = [];
  var links = []; // { a, b }
  var subscribers = [];

  // ---- Presets ----
  var PRESETS = {
    kantor: {
      devices: [
        { name: 'Internet', kind: 'internet' },
        { name: 'Firewall', kind: 'firewall' },
        { name: 'Router-1', kind: 'router' },
        { name: 'Switch-1', kind: 'switch' },
        { name: 'PC-1', kind: 'pc' },
        { name: 'PC-2', kind: 'pc' },
        { name: 'Server', kind: 'server' },
        { name: 'AP-WiFi', kind: 'ap' }
      ],
      links: [
        { a: 'Internet', b: 'Firewall' },
        { a: 'Firewall', b: 'Router-1' },
        { a: 'Router-1', b: 'Switch-1' },
        { a: 'Switch-1', b: 'PC-1' },
        { a: 'Switch-1', b: 'PC-2' },
        { a: 'Switch-1', b: 'Server' },
        { a: 'Switch-1', b: 'AP-WiFi' }
      ]
    },
    kampus: {
      devices: [
        { name: 'Internet', kind: 'internet' },
        { name: 'Firewall', kind: 'firewall' },
        { name: 'Core-Router', kind: 'router' },
        { name: 'Dist-Switch-1', kind: 'switch' },
        { name: 'Dist-Switch-2', kind: 'switch' },
        { name: 'Access-Switch-1', kind: 'switch' },
        { name: 'Access-Switch-2', kind: 'switch' },
        { name: 'PC-Lab-1', kind: 'pc' },
        { name: 'PC-Lab-2', kind: 'pc' },
        { name: 'PC-Admin', kind: 'pc' },
        { name: 'Server-DB', kind: 'server' },
        { name: 'Server-Web', kind: 'server' },
        { name: 'AP-Kampus', kind: 'ap' }
      ],
      links: [
        { a: 'Internet', b: 'Firewall' },
        { a: 'Firewall', b: 'Core-Router' },
        { a: 'Core-Router', b: 'Dist-Switch-1' },
        { a: 'Core-Router', b: 'Dist-Switch-2' },
        { a: 'Dist-Switch-1', b: 'Access-Switch-1' },
        { a: 'Dist-Switch-1', b: 'Access-Switch-2' },
        { a: 'Access-Switch-1', b: 'PC-Lab-1' },
        { a: 'Access-Switch-1', b: 'PC-Lab-2' },
        { a: 'Access-Switch-2', b: 'PC-Admin' },
        { a: 'Core-Router', b: 'Server-DB' },
        { a: 'Core-Router', b: 'Server-Web' },
        { a: 'Dist-Switch-2', b: 'AP-Kampus' }
      ]
    },
    datacenter: {
      devices: [
        { name: 'Internet', kind: 'internet' },
        { name: 'Edge-FW', kind: 'firewall' },
        { name: 'Border-Router', kind: 'router' },
        { name: 'Core-Switch-1', kind: 'switch' },
        { name: 'Core-Switch-2', kind: 'switch' },
        { name: 'Leaf-Switch-1', kind: 'switch' },
        { name: 'Leaf-Switch-2', kind: 'switch' },
        { name: 'Server-1', kind: 'server' },
        { name: 'Server-2', kind: 'server' },
        { name: 'Server-3', kind: 'server' },
        { name: 'Database', kind: 'database' },
        { name: 'IDS', kind: 'ids' }
      ],
      links: [
        { a: 'Internet', b: 'Edge-FW' },
        { a: 'Edge-FW', b: 'Border-Router' },
        { a: 'Border-Router', b: 'Core-Switch-1' },
        { a: 'Border-Router', b: 'Core-Switch-2' },
        { a: 'Core-Switch-1', b: 'Leaf-Switch-1' },
        { a: 'Core-Switch-2', b: 'Leaf-Switch-2' },
        { a: 'Leaf-Switch-1', b: 'Server-1' },
        { a: 'Leaf-Switch-1', b: 'Server-2' },
        { a: 'Leaf-Switch-2', b: 'Server-3' },
        { a: 'Leaf-Switch-2', b: 'Database' },
        { a: 'Core-Switch-1', b: 'IDS' }
      ]
    }
  };

  // ---- Helpers ----
  function notify() {
    subscribers.forEach(function (fn) { fn(); });
  }

  function findDevice(name) {
    return devices.find(function (d) { return d.name === name; });
  }

  function deviceIndex(name) {
    return devices.findIndex(function (d) { return d.name === name; });
  }

  function hasPathToInternet(deviceName, visited) {
    if (deviceName === 'Internet') return true;
    visited = visited || new Set();
    if (visited.has(deviceName)) return false;
    visited.add(deviceName);
    var device = findDevice(deviceName);
    if (!device) return false;
    var neighbors = links.filter(function (l) { return l.a === deviceName || l.b === deviceName; });
    return neighbors.some(function (l) {
      var other = l.a === deviceName ? l.b : l.a;
      return hasPathToInternet(other, visited);
    });
  }

  function validateTopology() {
    var names = devices.map(function (d) { return d.name; });
    if (new Set(names).size !== names.length) return 'Nama perangkat harus unik';
    if (devices.length > MAX_DEVICES) return 'Maksimal ' + MAX_DEVICES + ' perangkat';
    if (!findDevice('Internet')) return 'Perangkat Internet wajib ada';
    var errors = [];
    devices.forEach(function (d) {
      if (d.name !== 'Internet' && !hasPathToInternet(d.name)) {
        errors.push(d.name + ' tidak memiliki jalur ke Internet');
      }
    });
    if (errors.length) return errors.join('; ');
    links.forEach(function (l) {
      if (!findDevice(l.a) || !findDevice(l.b)) errors.push('Link merujuk perangkat tidak ada: ' + l.a + ' - ' + l.b);
      if (l.a === l.b) errors.push('Self-link tidak diperbolehkan: ' + l.a);
    });
    if (errors.length) return errors.join('; ');
    return null;
  }

  function loadPreset(key) {
    var p = PRESETS[key];
    if (!p) return false;
    devices = p.devices.map(function (d, i) { return { id: 'd' + i, name: d.name, kind: d.kind }; });
    links = p.links.map(function (l) { return { a: l.a, b: l.b }; });
    notify();
    return true;
  }

  function reset() {
    devices = [];
    links = [];
    notify();
  }

  function addDevice(name, kind) {
    if (findDevice(name)) return { ok: false, msg: 'Nama sudah digunakan' };
    if (!TYPES[kind]) return { ok: false, msg: 'Tipe tidak valid' };
    if (devices.length >= MAX_DEVICES) return { ok: false, msg: 'Maksimal perangkat tercapai' };
    var id = 'd' + Date.now() + Math.random().toString(36).slice(2, 6);
    devices.push({ id: id, name: name, kind: kind });
    notify();
    return { ok: true };
  }

  function removeDevice(name) {
    var idx = deviceIndex(name);
    if (idx < 0) return { ok: false, msg: 'Perangkat tidak ditemukan' };
    if (name === 'Internet') return { ok: false, msg: 'Internet tidak bisa dihapus' };
    devices.splice(idx, 1);
    links = links.filter(function (l) { return l.a !== name && l.b !== name; });
    notify();
    return { ok: true };
  }

  function addLink(a, b) {
    if (a === b) return { ok: false, msg: 'Self-link tidak diperbolehkan' };
    if (!findDevice(a) || !findDevice(b)) return { ok: false, msg: 'Perangkat tidak ditemukan' };
    if (links.some(function (l) { return (l.a === a && l.b === b) || (l.a === b && l.b === a); })) {
      return { ok: false, msg: 'Link sudah ada' };
    }
    links.push({ a: a, b: b });
    notify();
    return { ok: true };
  }

  function removeLink(a, b) {
    var idx = links.findIndex(function (l) { return (l.a === a && l.b === b) || (l.a === b && l.b === a); });
    if (idx < 0) return { ok: false, msg: 'Link tidak ditemukan' };
    links.splice(idx, 1);
    notify();
    return { ok: true };
  }

  function subscribe(fn) {
    subscribers.push(fn);
    return function () {
      var i = subscribers.indexOf(fn);
      if (i >= 0) subscribers.splice(i, 1);
    };
  }

  function get() {
    return { devices: devices, links: links, types: TYPES };
  }

  // ---- Public API ----
  return {
    loadPreset: loadPreset,
    reset: reset,
    addDevice: addDevice,
    removeDevice: removeDevice,
    addLink: addLink,
    removeLink: removeLink,
    subscribe: subscribe,
    get: get,
    validate: validateTopology,
    mulberry32: mulberry32
  };
})();

// OrbitControls implementation (for Three.js 3D scenes)
(function () {
  'use strict';
  if (typeof window.THREE === 'undefined') {
    console.warn('Three.js not loaded yet, OrbitControls will be defined when Three.js loads');
  }

  var OrbitControls = function (object, domElement) {
    this.object = object;
    this.domElement = domElement || document;
    this.enabled = true;
    this.target = new THREE.Vector3();
    this.enableDamping = false;
    this.dampingFactor = 0.05;
    this.enableZoom = true;
    this.zoomSpeed = 1.0;
    this.enableRotate = true;
    this.rotateSpeed = 1.0;
    this.enablePan = true;
    this.panSpeed = 1.0;
    this.minDistance = 0;
    this.maxDistance = Infinity;
    this.minZoom = 0;
    this.maxZoom = Infinity;
    this.minPolarAngle = 0;
    this.maxPolarAngle = Math.PI;
    this.autoRotate = false;
    this.autoRotateSpeed = 2.0;
    this.keys = { LEFT: 37, UP: 38, RIGHT: 39, BOTTOM: 40 };
    this.mouseButtons = { LEFT: THREE.MOUSE.LEFT, MIDDLE: THREE.MOUSE.MIDDLE, RIGHT: THREE.MOUSE.RIGHT };

    var scope = this;
    var EPS = 0.000001;
    var rotateStart = new THREE.Vector2();
    var rotateEnd = new THREE.Vector2();
    var rotateDelta = new THREE.Vector2();
    var panStart = new THREE.Vector2();
    var panEnd = new THREE.Vector2();
    var panDelta = new THREE.Vector2();
    var dollyStart = new THREE.Vector2();
    var dollyEnd = new THREE.Vector2();
    var dollyDelta = new THREE.Vector2();
    var STATE = { NONE: -1, ROTATE: 0, DOLLY: 1, PAN: 2, TOUCH_ROTATE: 3, TOUCH_DOLLY_PAN: 4 };
    var state = STATE.NONE;

    function getZoomScale() { return Math.pow(0.95, scope.zoomSpeed); }

    function rotateLeft(angle) { scope.target.x += Math.sin(angle) * scope.object.position.distanceTo(scope.target); scope.target.z += Math.cos(angle) * scope.object.position.distanceTo(scope.target); }
    function rotateUp(angle) { scope.target.y += angle * scope.object.position.distanceTo(scope.target); }

    var lastPosition = new THREE.Vector3();
    var lastQuaternion = new THREE.Quaternion();

    this.update = function () {
      var offset = new THREE.Vector3().subVectors(scope.object.position, scope.target);
      if (scope.enableDamping) {
        offset.multiplyScalar(1 - scope.dampingFactor);
      }
      scope.object.position.copy(scope.target).add(offset);
      scope.object.lookAt(scope.target);
      if (scope.autoRotate && state === STATE.NONE) {
        rotateLeft(getAutoRotationAngle());
      }
      if (lastPosition.distanceToSquared(scope.object.position) > EPS || 8 * (1 - lastQuaternion.dot(scope.object.quaternion)) > EPS) {
        scope.dispatchEvent({ type: 'change' });
        lastPosition.copy(scope.object.position);
        lastQuaternion.copy(scope.object.quaternion);
      }
      return true;
    };

    function getAutoRotationAngle() { return 2 * Math.PI / 60 / 60 * scope.autoRotateSpeed; }

    function onMouseDown(event) {
      if (!scope.enabled) return;
      event.preventDefault();
      if (event.button === scope.mouseButtons.LEFT) {
        if (scope.enableRotate === false) return;
        rotateStart.set(event.clientX, event.clientY);
        state = STATE.ROTATE;
      } else if (event.button === scope.mouseButtons.MIDDLE) {
        if (scope.enableZoom === false) return;
        dollyStart.set(event.clientX, event.clientY);
        state = STATE.DOLLY;
      } else if (event.button === scope.mouseButtons.RIGHT) {
        if (scope.enablePan === false) return;
        panStart.set(event.clientX, event.clientY);
        state = STATE.PAN;
      }
      if (state !== STATE.NONE) {
        document.addEventListener('mousemove', onMouseMove, false);
        document.addEventListener('mouseup', onMouseUp, false);
        scope.dispatchEvent({ type: 'start' });
      }
    }

    function onMouseMove(event) {
      if (!scope.enabled) return;
      event.preventDefault();
      if (state === STATE.ROTATE) {
        rotateEnd.set(event.clientX, event.clientY);
        rotateDelta.subVectors(rotateEnd, rotateStart);
        var element = scope.domElement === document ? scope.domElement.body : scope.domElement;
        var theta = 2 * Math.PI * rotateDelta.x / element.clientWidth * scope.rotateSpeed;
        var phi = 2 * Math.PI * rotateDelta.y / element.clientHeight * scope.rotateSpeed;
        rotateLeft(theta);
        rotateUp(phi);
        rotateStart.copy(rotateEnd);
        scope.update();
      } else if (state === STATE.DOLLY) {
        dollyEnd.set(event.clientX, event.clientY);
        dollyDelta.subVectors(dollyEnd, dollyStart);
        if (dollyDelta.y > 0) { scope.dollyOut(getZoomScale()); } else if (dollyDelta.y < 0) { scope.dollyIn(getZoomScale()); }
        dollyStart.copy(dollyEnd);
        scope.update();
      } else if (state === STATE.PAN) {
        panEnd.set(event.clientX, event.clientY);
        panDelta.subVectors(panEnd, panStart);
        pan(panDelta.x, panDelta.y);
        panStart.copy(panEnd);
        scope.update();
      }
    }

    function onMouseUp(event) {
      if (!scope.enabled) return;
      document.removeEventListener('mousemove', onMouseMove, false);
      document.removeEventListener('mouseup', onMouseUp, false);
      scope.dispatchEvent({ type: 'end' });
      state = STATE.NONE;
    }

    function onMouseWheel(event) {
      if (!scope.enabled || !scope.enableZoom) return;
      event.preventDefault();
      if (event.deltaY < 0) { scope.dollyIn(getZoomScale()); } else if (event.deltaY > 0) { scope.dollyOut(getZoomScale()); }
      scope.update();
    }

    function pan(deltaX, deltaY) {
      var offset = new THREE.Vector3();
      var element = scope.domElement === document ? scope.domElement.body : scope.domElement;
      offset.set(deltaX * scope.panSpeed / element.clientWidth, deltaY * scope.panSpeed / element.clientHeight, 0);
      offset.applyQuaternion(scope.object.quaternion);
      scope.object.position.add(offset);
      scope.target.add(offset);
    }

    this.dollyIn = function (dollyScale) {
      var offset = new THREE.Vector3().subVectors(scope.object.position, scope.target);
      offset.multiplyScalar(dollyScale);
      scope.object.position.copy(scope.target).add(offset);
      if (scope.object.position.distanceTo(scope.target) < scope.minDistance) {
        scope.object.position.setLength(scope.minDistance);
      }
    };

    this.dollyOut = function (dollyScale) {
      var offset = new THREE.Vector3().subVectors(scope.object.position, scope.target);
      offset.multiplyScalar(1 / dollyScale);
      scope.object.position.copy(scope.target).add(offset);
      if (scope.object.position.distanceTo(scope.target) > scope.maxDistance) {
        scope.object.position.setLength(scope.maxDistance);
      }
    };

    function onKeyDown(event) {
      if (!scope.enabled) return;
      switch (event.keyCode) {
        case scope.keys.UP: scope.pan(0, scope.panSpeed); scope.update(); break;
        case scope.keys.BOTTOM: scope.pan(0, -scope.panSpeed); scope.update(); break;
        case scope.keys.LEFT: scope.pan(scope.panSpeed, 0); scope.update(); break;
        case scope.keys.RIGHT: scope.pan(-scope.panSpeed, 0); scope.update(); break;
      }
    }

    function touchStart(event) {
      if (!scope.enabled) return;
      event.preventDefault();
      switch (event.touches.length) {
        case 1:
          if (scope.enableRotate === false) return;
          rotateStart.set(event.touches[0].pageX, event.touches[0].pageY);
          state = STATE.TOUCH_ROTATE;
          break;
        case 2:
          if (scope.enableZoom === false && scope.enablePan === false) return;
          var dx = event.touches[0].pageX - event.touches[1].pageX;
          var dy = event.touches[0].pageY - event.touches[1].pageY;
          var distance = Math.sqrt(dx * dx + dy * dy);
          dollyStart.set(0, distance);
          state = STATE.TOUCH_DOLLY_PAN;
          break;
        default: state = STATE.NONE;
      }
      if (state !== STATE.NONE) { scope.dispatchEvent({ type: 'start' }); }
    }

    function touchMove(event) {
      if (!scope.enabled) return;
      event.preventDefault();
      event.stopPropagation();
      switch (event.touches.length) {
        case 1:
          if (state !== STATE.TOUCH_ROTATE) return;
          rotateEnd.set(event.touches[0].pageX, event.touches[0].pageY);
          rotateDelta.subVectors(rotateEnd, rotateStart);
          var element = scope.domElement === document ? scope.domElement.body : scope.domElement;
          var theta = 2 * Math.PI * rotateDelta.x / element.clientWidth * scope.rotateSpeed;
          var phi = 2 * Math.PI * rotateDelta.y / element.clientHeight * scope.rotateSpeed;
          rotateLeft(theta);
          rotateUp(phi);
          rotateStart.copy(rotateEnd);
          scope.update();
          break;
        case 2:
          if (state !== STATE.TOUCH_DOLLY_PAN) return;
          var dx = event.touches[0].pageX - event.touches[1].pageX;
          var dy = event.touches[0].pageY - event.touches[1].pageY;
          var distance = Math.sqrt(dx * dx + dy * dy);
          dollyEnd.set(0, distance);
          dollyDelta.subVectors(dollyEnd, dollyStart);
          if (dollyDelta.y > 0) { scope.dollyOut(getZoomScale()); } else { scope.dollyIn(getZoomScale()); }
          dollyStart.copy(dollyEnd);
          scope.update();
          break;
      }
    }

    function touchEnd(event) {
      if (!scope.enabled) return;
      scope.dispatchEvent({ type: 'end' });
      state = STATE.NONE;
    }

    function contextMenu(event) { event.preventDefault(); }

    scope.domElement.addEventListener('contextmenu', contextMenu, false);
    scope.domElement.addEventListener('mousedown', onMouseDown, false);
    scope.domElement.addEventListener('wheel', onMouseWheel, false);
    scope.domElement.addEventListener('touchstart', touchStart, false);
    scope.domElement.addEventListener('touchend', touchEnd, false);
    scope.domElement.addEventListener('touchmove', touchMove, false);
    window.addEventListener('keydown', onKeyDown, false);

    this.dispose = function () {
      scope.domElement.removeEventListener('contextmenu', contextMenu, false);
      scope.domElement.removeEventListener('mousedown', onMouseDown, false);
      scope.domElement.removeEventListener('wheel', onMouseWheel, false);
      scope.domElement.removeEventListener('touchstart', touchStart, false);
      scope.domElement.removeEventListener('touchend', touchEnd, false);
      scope.domElement.removeEventListener('touchmove', touchMove, false);
      window.removeEventListener('keydown', onKeyDown, false);
    };

    this.update();
  };

  OrbitControls.prototype = Object.create(THREE.EventDispatcher.prototype);
  OrbitControls.prototype.constructor = OrbitControls;

  window.OrbitControls = OrbitControls;
})();
</script>"##.to_string()
}