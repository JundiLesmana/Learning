/* Uji unit fungsi matematika murni halaman Nilai & Vektor Eigen.
   Logika ini mirror dari system/src/pages/eigen.rs (EigenMath). */
'use strict';

function gcd(a, b){ a = Math.abs(a); b = Math.abs(b); while (b){ var t = b; b = a % b; a = t; } return a || 1; }
function Fraction(n, d){
  if (d === undefined) d = 1;
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
Fraction.prototype.isZero = function(){ return this.num === 0; };
Fraction.prototype.eq = function(o){ return this.num === o.num && this.den === o.den; };
Fraction.prototype.toDec = function(){ return this.num / this.den; };

function Poly(coeffs){ this.c = coeffs.slice(); this.trim(); }
Poly.prototype.trim = function(){ while (this.c.length > 1 && this.c[this.c.length - 1].isZero()) this.c.pop(); return this; };
Poly.prototype.degree = function(){ this.trim(); return this.c.length - 1; };
Poly.prototype.lead = function(){ this.trim(); return this.c[this.c.length - 1]; };
Poly.prototype.add = function(o){
  var n = Math.max(this.c.length, o.c.length), r = [];
  for (var i = 0; i < n; i++) r.push((i < this.c.length ? this.c[i] : Fraction.ZERO()).add(i < o.c.length ? o.c[i] : Fraction.ZERO()));
  return new Poly(r);
};
Poly.prototype.sub = function(o){
  var n = Math.max(this.c.length, o.c.length), r = [];
  for (var i = 0; i < n; i++) r.push((i < this.c.length ? this.c[i] : Fraction.ZERO()).sub(i < o.c.length ? o.c[i] : Fraction.ZERO()));
  return new Poly(r);
};
Poly.prototype.mul = function(o){
  var r = [];
  for (var i = 0; i < this.c.length + o.c.length - 1; i++) r.push(Fraction.ZERO());
  for (var i = 0; i < this.c.length; i++)
    for (var j = 0; j < o.c.length; j++) r[i + j] = r[i + j].add(this.c[i].mul(o.c[j]));
  return new Poly(r);
};
Poly.prototype.scale = function(s){ return new Poly(this.c.map(function(c){ return c.mul(s); })); };
Poly.prototype.eval = function(x){
  var r = Fraction.ZERO();
  for (var i = this.c.length - 1; i >= 0; i--) r = r.mul(x).add(this.c[i]);
  return r;
};
Poly.prototype.divByLinear = function(root){
  var coeffs = this.c.slice(), b = [], acc = Fraction.ZERO();
  for (var i = coeffs.length - 1; i >= 0; i--){ acc = acc.mul(root).add(coeffs[i]); b.unshift(acc); }
  var rem = b.pop();
  return { quotient: new Poly(b), remainder: rem };
};
Poly.prototype.toLatex = function(v){
  v = v || '\u03bb';
  var terms = [], first = true;
  for (var i = this.c.length - 1; i >= 0; i--){
    var c = this.c[i];
    if (c.isZero()) continue;
    var neg = c.num < 0, absNum = neg ? -c.num : c.num;
    var showCoef = !(i > 0 && c.den === 1 && absNum === 1);
    var coef = showCoef ? (c.den === 1 ? String(absNum) : '\\frac{' + absNum + '}{' + c.den + '}') : '';
    var pow = i === 1 ? '\\' + v : (i > 1 ? '\\' + v + '^{' + i + '}' : '');
    var sign = first ? (neg ? '-' : '') : (neg ? '-' : '+');
    first = false;
    terms.push(sign + coef + pow);
  }
  return terms.length ? terms.join('') : '0';
};

function detMat(m){
  var n = m.length, a = m.map(function(r){ return r.slice(); }), det = Fraction.ONE();
  for (var col = 0; col < n; col++){
    var piv = -1;
    for (var r = col; r < n; r++) if (!a[r][col].isZero()) { piv = r; break; }
    if (piv < 0) return Fraction.ZERO();
    if (piv !== col){ var tmp = a[col]; a[col] = a[piv]; a[piv] = tmp; det = det.neg(); }
    det = det.mul(a[col][col]);
    for (var r = col + 1; r < n; r++){
      if (a[r][col].isZero()) continue;
      var factor = a[r][col].div(a[col][col]);
      for (var c = col; c < n; c++) a[r][c] = a[r][c].sub(factor.mul(a[col][c]));
    }
  }
  return det;
}
function charPoly(A){
  var n = A.length, M = [];
  for (var i = 0; i < n; i++){
    var row = [];
    for (var j = 0; j < n; j++){
      row.push(i === j ? new Poly([ A[i][j].neg(), Fraction.ONE() ]) : new Poly([ A[i][j].neg() ]));
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
    for (var r = 1; r < n; r++){ var row = []; for (var c = 0; c < n; c++) if (c !== j) row.push(M[r][c]); minor.push(row); }
    var term = M[0][j].mul(detPoly(minor));
    if (j % 2 === 1) term = term.scale(new Fraction(-1));
    res = res.add(term);
  }
  return res;
}
function Complex(re, im){ this.re = re; this.im = im === undefined ? 0 : im; }
Complex.prototype.add = function(o){ return new Complex(this.re + o.re, this.im + o.im); };
Complex.prototype.sub = function(o){ return new Complex(this.re - o.re, this.im - o.im); };
Complex.prototype.mul = function(o){ return new Complex(this.re * o.re - this.im * o.im, this.re * o.im + this.im * o.re); };
Complex.prototype.div = function(o){
  var d = o.re * o.re + o.im * o.im;
  return new Complex((this.re * o.re + this.im * o.im) / d, (this.im * o.re - this.re * o.im) / d);
};
Complex.prototype.abs = function(){ return Math.sqrt(this.re * this.re + this.im * this.im); };
function evalPolyComplex(c, x){
  var r = new Complex(0, 0);
  for (var i = c.length - 1; i >= 0; i--) r = r.mul(x).add(new Complex(c[i].toDec(), 0));
  return r;
}
function isPerfectSquareRat(f){
  var n = f.num, d = f.den;
  if (n < 0) return false;
  return Math.abs(Math.sqrt(n) - Math.round(Math.sqrt(n))) < 1e-9 && Math.abs(Math.sqrt(d) - Math.round(Math.sqrt(d))) < 1e-9;
}
function rationalRootCandidates(poly){
  var c0 = poly.c[0], lead = poly.lead(), cands = [];
  if (c0.isZero()) return cands;
  function divs(n){ n = Math.abs(n); var out = []; for (var i = 1; i <= n; i++) if (n % i === 0) out.push(i); return out; }
  var ps = divs(c0.num), qs = divs(lead.num), seen = {};
  for (var i = 0; i < ps.length; i++)
    for (var j = 0; j < qs.length; j++)
      for (var s = -1; s <= 1; s += 2){
        var r = new Fraction(s * ps[i], qs[j]);
        if (seen[r.toString()]) continue;
        seen[r.toString()] = true;
        cands.push(r);
      }
  return cands;
}
function findRoots(poly){
  poly = new Poly(poly.c.slice()).trim();
  var deg = poly.degree(), roots = [], work = poly;
  function add(kind, exact, frac, re, im){
    roots.push({ kind: kind, exact: exact, frac: frac, re: frac ? frac.toDec() : re, im: im === undefined ? 0 : im });
  }
  if (deg === 1){ add('real', true, work.c[0].neg().div(work.c[1]), null, 0); return roots; }
  if (deg === 2){
    var a = work.c[2], b = work.c[1], c = work.c[0];
    var delta = b.mul(b).sub(a.mul(c).mul(new Fraction(4, 1))), twoA = a.mul(new Fraction(2, 1)), negB = b.neg();
    if (delta.num >= 0){
      var sd;
      if (isPerfectSquareRat(delta)){ sd = new Fraction(Math.sqrt(delta.num), Math.sqrt(delta.den)); if (delta.num === 0) sd = Fraction.ZERO(); }
      if (sd){ add('real', true, negB.add(sd).div(twoA), null, 0); add('real', true, negB.sub(sd).div(twoA), null, 0); }
      else { var d = delta.toDec(), s = Math.sqrt(d); add('real', false, null, negB.toDec() / twoA.toDec() + s / twoA.toDec(), 0); add('real', false, null, negB.toDec() / twoA.toDec() - s / twoA.toDec(), 0); }
        } else { var d = delta.toDec(), s = Math.sqrt(-d), re = negB.toDec() / twoA.toDec(); add('complex', false, null, re, s / twoA.toDec()); add('complex', false, null, re, -s / twoA.toDec()); }
    return roots;
  }
  var remaining = work, tried = {}, changed = true;
  while (remaining.degree() >= 2 && changed){
    changed = false;
    var cands = rationalRootCandidates(remaining);
    for (var k = 0; k < cands.length; k++){
      var r = cands[k], key = r.toString();
      if (tried[key]) continue;
      tried[key] = true;
      var div = remaining.divByLinear(r);
      if (div.remainder.isZero()){ add('real', true, r, null, 0); remaining = div.quotient.trim(); changed = true; break; }
    }
  }
  if (remaining.degree() === 1){ add('real', true, remaining.c[0].neg().div(remaining.c[1]), null, 0); }
  else if (remaining.degree() === 2){ var sub = findRoots(remaining); for (var s = 0; s < sub.length; s++) roots.push(sub[s]); }
  else if (remaining.degree() > 2){
    var dk = durandKerner(remaining);
    for (var d = 0; d < dk.length; d++) add('complex', false, null, dk[d].re, dk[d].im);
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
    var nr = [], maxShift = 0;
    for (var i = 0; i < n; i++){
      var prod = new Complex(1, 0);
      for (var j = 0; j < n; j++) if (j !== i) prod = prod.mul(roots[i].sub(roots[j]));
      var delta = evalPolyComplex(c, roots[i]).div(prod);
      var nu = roots[i].sub(delta);
      nr.push(nu);
      maxShift = Math.max(maxShift, nu.sub(roots[i]).abs());
    }
    roots = nr;
    if (maxShift < 1e-8) break;
  }
  return roots;
}
function matMulVec(A, v){
  var n = A.length, out = [];
  for (var i = 0; i < n; i++){ var acc = Fraction.ZERO(); for (var j = 0; j < n; j++) acc = acc.add(A[i][j].mul(v[j])); out.push(acc); }
  return out;
}
function trace(A){ var t = Fraction.ZERO(); for (var i = 0; i < A.length; i++) t = t.add(A[i][i]); return t; }

function nullSpace(M){
  var n = M.length, aug = [];
  for (var i = 0; i < n; i++){ var row = M[i].slice(); row.push(Fraction.ZERO()); aug.push(row); }
  var rows = aug.length, cols = aug[0].length, a = aug.map(function(r){ return r.slice(); }), pivots = [], r = 0;
  for (var c = 0; c < cols && r < rows; c++){
    var piv = -1;
    for (var i = r; i < rows; i++) if (!a[i][c].isZero()) { piv = i; break; }
    if (piv < 0) continue;
    var tmp = a[r]; a[r] = a[piv]; a[piv] = tmp;
    var inv = new Fraction(1, 1).div(a[r][c]);
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
  var isPivot = [];
  for (var c = 0; c < n; c++) isPivot.push(false);
  for (var i = 0; i < pivots.length; i++) isPivot[pivots[i].col] = true;
  var freeCols = [];
  for (var c = 0; c < n; c++) if (!isPivot[c]) freeCols.push(c);
  var basis = [];
  for (var f = 0; f < freeCols.length; f++){
    var fc = freeCols[f], x = [];
    for (var j = 0; j < n; j++) x.push(Fraction.ZERO());
    x[fc] = Fraction.ONE();
    for (var i = pivots.length - 1; i >= 0; i--){
      var row = pivots[i].row, col = pivots[i].col, sum = Fraction.ZERO();
      for (var j = col + 1; j < n; j++) sum = sum.add(a[row][j].mul(x[j]));
      x[col] = sum.neg();
    }
    basis.push(x);
  }
  return { basis: basis, rank: pivots.length, freeCols: freeCols };
}

function matToFractions(nums){
  return nums.map(function(row){ return row.map(function(v){ return new Fraction(v, 1); }); });
}

var pass = 0, fail = 0;
function ok(cond, msg){
  if (cond) { pass++; console.log('PASS: ' + msg); }
  else { fail++; console.log('FAIL: ' + msg); }
}
function approxEq(a, b, eps){ return Math.abs(a - b) < (eps || 1e-6); }

// ---- Test 1: A = [[3,2],[-1,0]] -> char poly λ² - 3λ + 2, roots {1,2} ----
var A1 = matToFractions([[3,2],[-1,0]]);
var p1 = charPoly(A1);
ok(p1.toLatex('lambda') === '\\lambda^{2} - 3\\lambda + 2' || p1.toLatex('lambda').replace(/\s/g,'') === '\\lambda^{2}-3\\lambda+2',
   'Test1 char poly: got ' + p1.toLatex('lambda'));
var r1 = findRoots(p1);
var vals1 = r1.filter(function(x){ return x.kind === 'real'; }).map(function(x){ return x.frac.toDec(); }).sort(function(a,b){ return a - b; });
ok(vals1.length === 2 && approxEq(vals1[0], 1) && approxEq(vals1[1], 2),
   'Test1 roots {1,2}: got ' + vals1.join(','));

// ---- Test 2: E(1) basis [-1,1], E(2) basis [-2,1] ----
function eigenBasis(A, lam){
  var n = A.length;
  var M = [];
  for (var i = 0; i < n; i++){
    var row = [];
    for (var j = 0; j < n; j++){
      row.push(i === j ? A[i][j].neg().add(lam) : A[i][j].neg());
    }
    M.push(row);
  }
  return nullSpace(M).basis;
}
var b1 = eigenBasis(A1, new Fraction(1,1));
ok(b1.length === 1 && b1[0].length === 2 &&
   ((approxEq(b1[0][0].toDec(), -1) && approxEq(b1[0][1].toDec(), 1)) ||
    (approxEq(b1[0][0].toDec(), 1) && approxEq(b1[0][1].toDec(), -1))),
   'Test2 E(1) basis ~[-1,1]: got [' + b1[0].map(function(x){ return x.toDec(); }).join(',') + ']');
var b2 = eigenBasis(A1, new Fraction(2,1));
ok(b2.length === 1 &&
   ((approxEq(b2[0][0].toDec(), -2) && approxEq(b2[0][1].toDec(), 1)) ||
    (approxEq(b2[0][0].toDec(), 2) && approxEq(b2[0][1].toDec(), -1))),
   'Test2 E(2) basis ~[-2,1]: got [' + b2[0].map(function(x){ return x.toDec(); }).join(',') + ']');

// ---- Test 3: A·v = λ·v for each pair ----
function matVec(A, v){
  var out = [];
  for (var i = 0; i < A.length; i++){ var acc = new Fraction(0); for (var j = 0; j < v.length; j++) acc = acc.add(A[i][j].mul(v[j])); out.push(acc); }
  return out;
}
var pairs = [[new Fraction(1,1), b1[0]], [new Fraction(2,1), b2[0]]];
var allAvOk = true;
for (var pi = 0; pi < pairs.length; pi++){
  var lam = pairs[pi][0], v = pairs[pi][1];
  var Av = matVec(A1, v);
  var lamv = v.map(function(x){ return x.mul(lam); });
  for (var i = 0; i < v.length; i++) if (!Av[i].eq(lamv[i])) allAvOk = false;
}
ok(allAvOk, 'Test3 A·v = λ·v untuk setiap pasang (λ, v)');

// ---- Test 4: trace = jumlah eigen, det = hasil kali eigen ----
ok(trace(A1).eq(new Fraction(3,1)), 'Test4 trace(A)=3');
ok(detMat(A1).eq(new Fraction(2,1)), 'Test4 det(A)=2');
var sumR = Fraction.ZERO(), prodR = Fraction.ONE();
r1.forEach(function(x){ sumR = sumR.add(new Fraction(Math.round(x.re*1e4)/1e4, 1)); prodR = prodR.mul(new Fraction(Math.round(x.re*1e4)/1e4, 1)); });
ok(sumR.eq(trace(A1)), 'Test4 jumlah eigen = trace');
ok(prodR.eq(detMat(A1)), 'Test4 hasil kali eigen = det');

// ---- Test 5: Rotasi 90° [[0,-1],[1,0]] -> kompleks ±i ----
var Arot = matToFractions([[0,-1],[1,0]]);
var prot = charPoly(Arot);
ok(prot.toLatex('lambda').replace(/\s/g,'') === '\\lambda^{2}+1',
   'Test5 char poly rotasi: got ' + prot.toLatex('lambda'));
var rrot = findRoots(prot);
var complx = rrot.filter(function(x){ return x.kind === 'complex'; });
ok(complx.length === 2 && approxEq(Math.abs(complx[0].im), 1) && approxEq(complx[0].re, 0) && approxEq(complx[1].im, -1),
   'Test5 roots ±i: re=' + complx.map(function(x){ return x.re; }).join(',') + ' im=' + complx.map(function(x){ return x.im; }).join(','));

// ---- Test 6: [[2,1],[0,2]] -> alg mult 2, geom mult 1, not diagonalizable ----
var A6 = matToFractions([[2,1],[0,2]]);
var p6 = charPoly(A6);
var r6 = findRoots(p6);
var six = r6.filter(function(x){ return x.kind === 'real'; });
ok(six.length === 2 && six.every(function(x){ return approxEq(x.re, 2); }), 'Test6 eigenvalue = 2');
// algebraic mult of 2
var alg = r6.filter(function(x){ return approxEq(x.re, 2); }).length;
ok(alg === 2, 'Test6 multiplisitas aljabar = 2: got ' + alg);
var geom = eigenBasis(A6, new Fraction(2,1)).length;
ok(geom === 1, 'Test6 multiplisitas geometri = 1: got ' + geom);
ok(geom < alg, 'Test6 tidak dapat didiagonalisasi (geom < alg)');

console.log('\n' + pass + ' lulus, ' + fail + ' gagal.');
process.exit(fail === 0 ? 0 : 1);
