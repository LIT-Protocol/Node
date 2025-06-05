var jwt = (() => {
  var hm = (t, e) => () => (t && (e = t((t = 0))), e);
  var R = (t, e) => () => (e || t((e = { exports: {} }).exports, e), e.exports);
  var A,
    S = hm(() => {
      A = { env: { NODE_DEBUG: !1 } };
    });
  var Bf = R((GB, Kc) => {
    S();
    var Ra = function (t, e) {
      Error.call(this, t),
        Error.captureStackTrace &&
          Error.captureStackTrace(this, this.constructor),
        (this.name = 'JsonWebTokenError'),
        (this.message = t),
        e && (this.inner = e);
    };
    Ra.prototype = Object.create(Error.prototype);
    Ra.prototype.constructor = Ra;
    Kc.exports = Ra;
  });
  var i0 = R(($B, Gc) => {
    S();
    var Vc = Bf(),
      qa = function (t, e) {
        Vc.call(this, t), (this.name = 'NotBeforeError'), (this.date = e);
      };
    qa.prototype = Object.create(Vc.prototype);
    qa.prototype.constructor = qa;
    Gc.exports = qa;
  });
  var n0 = R((JB, $c) => {
    S();
    var Wc = Bf(),
      Ta = function (t, e) {
        Wc.call(this, t),
          (this.name = 'TokenExpiredError'),
          (this.expiredAt = e);
      };
    Ta.prototype = Object.create(Wc.prototype);
    Ta.prototype.constructor = Ta;
    $c.exports = Ta;
  });
  var Xc = R((Pa) => {
    'use strict';
    S();
    Pa.byteLength = cm;
    Pa.toByteArray = lm;
    Pa.fromByteArray = bm;
    var Sr = [],
      qt = [],
      um = typeof Uint8Array < 'u' ? Uint8Array : Array,
      f0 = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/';
    for (Fi = 0, Zc = f0.length; Fi < Zc; ++Fi)
      (Sr[Fi] = f0[Fi]), (qt[f0.charCodeAt(Fi)] = Fi);
    var Fi, Zc;
    qt['-'.charCodeAt(0)] = 62;
    qt['_'.charCodeAt(0)] = 63;
    function Jc(t) {
      var e = t.length;
      if (e % 4 > 0)
        throw new Error('Invalid string. Length must be a multiple of 4');
      var r = t.indexOf('=');
      r === -1 && (r = e);
      var n = r === e ? 0 : 4 - (r % 4);
      return [r, n];
    }
    function cm(t) {
      var e = Jc(t),
        r = e[0],
        n = e[1];
      return ((r + n) * 3) / 4 - n;
    }
    function dm(t, e, r) {
      return ((e + r) * 3) / 4 - r;
    }
    function lm(t) {
      var e,
        r = Jc(t),
        n = r[0],
        i = r[1],
        a = new um(dm(t, n, i)),
        h = 0,
        v = i > 0 ? n - 4 : n,
        g;
      for (g = 0; g < v; g += 4)
        (e =
          (qt[t.charCodeAt(g)] << 18) |
          (qt[t.charCodeAt(g + 1)] << 12) |
          (qt[t.charCodeAt(g + 2)] << 6) |
          qt[t.charCodeAt(g + 3)]),
          (a[h++] = (e >> 16) & 255),
          (a[h++] = (e >> 8) & 255),
          (a[h++] = e & 255);
      return (
        i === 2 &&
          ((e = (qt[t.charCodeAt(g)] << 2) | (qt[t.charCodeAt(g + 1)] >> 4)),
          (a[h++] = e & 255)),
        i === 1 &&
          ((e =
            (qt[t.charCodeAt(g)] << 10) |
            (qt[t.charCodeAt(g + 1)] << 4) |
            (qt[t.charCodeAt(g + 2)] >> 2)),
          (a[h++] = (e >> 8) & 255),
          (a[h++] = e & 255)),
        a
      );
    }
    function pm(t) {
      return (
        Sr[(t >> 18) & 63] + Sr[(t >> 12) & 63] + Sr[(t >> 6) & 63] + Sr[t & 63]
      );
    }
    function vm(t, e, r) {
      for (var n, i = [], a = e; a < r; a += 3)
        (n =
          ((t[a] << 16) & 16711680) +
          ((t[a + 1] << 8) & 65280) +
          (t[a + 2] & 255)),
          i.push(pm(n));
      return i.join('');
    }
    function bm(t) {
      for (
        var e, r = t.length, n = r % 3, i = [], a = 16383, h = 0, v = r - n;
        h < v;
        h += a
      )
        i.push(vm(t, h, h + a > v ? v : h + a));
      return (
        n === 1
          ? ((e = t[r - 1]), i.push(Sr[e >> 2] + Sr[(e << 4) & 63] + '=='))
          : n === 2 &&
            ((e = (t[r - 2] << 8) + t[r - 1]),
            i.push(Sr[e >> 10] + Sr[(e >> 4) & 63] + Sr[(e << 2) & 63] + '=')),
        i.join('')
      );
    }
  });
  var Yc = R((a0) => {
    S();
    a0.read = function (t, e, r, n, i) {
      var a,
        h,
        v = i * 8 - n - 1,
        g = (1 << v) - 1,
        M = g >> 1,
        x = -7,
        E = r ? i - 1 : 0,
        I = r ? -1 : 1,
        q = t[e + E];
      for (
        E += I, a = q & ((1 << -x) - 1), q >>= -x, x += v;
        x > 0;
        a = a * 256 + t[e + E], E += I, x -= 8
      );
      for (
        h = a & ((1 << -x) - 1), a >>= -x, x += n;
        x > 0;
        h = h * 256 + t[e + E], E += I, x -= 8
      );
      if (a === 0) a = 1 - M;
      else {
        if (a === g) return h ? NaN : (q ? -1 : 1) * (1 / 0);
        (h = h + Math.pow(2, n)), (a = a - M);
      }
      return (q ? -1 : 1) * h * Math.pow(2, a - n);
    };
    a0.write = function (t, e, r, n, i, a) {
      var h,
        v,
        g,
        M = a * 8 - i - 1,
        x = (1 << M) - 1,
        E = x >> 1,
        I = i === 23 ? Math.pow(2, -24) - Math.pow(2, -77) : 0,
        q = n ? 0 : a - 1,
        k = n ? 1 : -1,
        L = e < 0 || (e === 0 && 1 / e < 0) ? 1 : 0;
      for (
        e = Math.abs(e),
          isNaN(e) || e === 1 / 0
            ? ((v = isNaN(e) ? 1 : 0), (h = x))
            : ((h = Math.floor(Math.log(e) / Math.LN2)),
              e * (g = Math.pow(2, -h)) < 1 && (h--, (g *= 2)),
              h + E >= 1 ? (e += I / g) : (e += I * Math.pow(2, 1 - E)),
              e * g >= 2 && (h++, (g /= 2)),
              h + E >= x
                ? ((v = 0), (h = x))
                : h + E >= 1
                  ? ((v = (e * g - 1) * Math.pow(2, i)), (h = h + E))
                  : ((v = e * Math.pow(2, E - 1) * Math.pow(2, i)), (h = 0)));
        i >= 8;
        t[r + q] = v & 255, q += k, v /= 256, i -= 8
      );
      for (
        h = (h << i) | v, M += i;
        M > 0;
        t[r + q] = h & 255, q += k, h /= 256, M -= 8
      );
      t[r + q - k] |= L * 128;
    };
  });
  var Et = R((Bn) => {
    'use strict';
    S();
    var o0 = Xc(),
      En = Yc(),
      Qc =
        typeof Symbol == 'function' && typeof Symbol.for == 'function'
          ? Symbol.for('nodejs.util.inspect.custom')
          : null;
    Bn.Buffer = C;
    Bn.SlowBuffer = xm;
    Bn.INSPECT_MAX_BYTES = 50;
    var ka = 2147483647;
    Bn.kMaxLength = ka;
    C.TYPED_ARRAY_SUPPORT = ym();
    !C.TYPED_ARRAY_SUPPORT &&
      typeof console < 'u' &&
      typeof console.error == 'function' &&
      console.error(
        'This browser lacks typed array (Uint8Array) support which is required by `buffer` v5.x. Use `buffer` v4.x if you require old browser support.'
      );
    function ym() {
      try {
        let t = new Uint8Array(1),
          e = {
            foo: function () {
              return 42;
            },
          };
        return (
          Object.setPrototypeOf(e, Uint8Array.prototype),
          Object.setPrototypeOf(t, e),
          t.foo() === 42
        );
      } catch {
        return !1;
      }
    }
    Object.defineProperty(C.prototype, 'parent', {
      enumerable: !0,
      get: function () {
        if (!!C.isBuffer(this)) return this.buffer;
      },
    });
    Object.defineProperty(C.prototype, 'offset', {
      enumerable: !0,
      get: function () {
        if (!!C.isBuffer(this)) return this.byteOffset;
      },
    });
    function Fr(t) {
      if (t > ka)
        throw new RangeError(
          'The value "' + t + '" is invalid for option "size"'
        );
      let e = new Uint8Array(t);
      return Object.setPrototypeOf(e, C.prototype), e;
    }
    function C(t, e, r) {
      if (typeof t == 'number') {
        if (typeof e == 'string')
          throw new TypeError(
            'The "string" argument must be of type string. Received type number'
          );
        return c0(t);
      }
      return id(t, e, r);
    }
    C.poolSize = 8192;
    function id(t, e, r) {
      if (typeof t == 'string') return gm(t, e);
      if (ArrayBuffer.isView(t)) return wm(t);
      if (t == null)
        throw new TypeError(
          'The first argument must be one of type string, Buffer, ArrayBuffer, Array, or Array-like Object. Received type ' +
            typeof t
        );
      if (
        Er(t, ArrayBuffer) ||
        (t && Er(t.buffer, ArrayBuffer)) ||
        (typeof SharedArrayBuffer < 'u' &&
          (Er(t, SharedArrayBuffer) || (t && Er(t.buffer, SharedArrayBuffer))))
      )
        return h0(t, e, r);
      if (typeof t == 'number')
        throw new TypeError(
          'The "value" argument must not be of type number. Received type number'
        );
      let n = t.valueOf && t.valueOf();
      if (n != null && n !== t) return C.from(n, e, r);
      let i = _m(t);
      if (i) return i;
      if (
        typeof Symbol < 'u' &&
        Symbol.toPrimitive != null &&
        typeof t[Symbol.toPrimitive] == 'function'
      )
        return C.from(t[Symbol.toPrimitive]('string'), e, r);
      throw new TypeError(
        'The first argument must be one of type string, Buffer, ArrayBuffer, Array, or Array-like Object. Received type ' +
          typeof t
      );
    }
    C.from = function (t, e, r) {
      return id(t, e, r);
    };
    Object.setPrototypeOf(C.prototype, Uint8Array.prototype);
    Object.setPrototypeOf(C, Uint8Array);
    function nd(t) {
      if (typeof t != 'number')
        throw new TypeError('"size" argument must be of type number');
      if (t < 0)
        throw new RangeError(
          'The value "' + t + '" is invalid for option "size"'
        );
    }
    function mm(t, e, r) {
      return (
        nd(t),
        t <= 0
          ? Fr(t)
          : e !== void 0
            ? typeof r == 'string'
              ? Fr(t).fill(e, r)
              : Fr(t).fill(e)
            : Fr(t)
      );
    }
    C.alloc = function (t, e, r) {
      return mm(t, e, r);
    };
    function c0(t) {
      return nd(t), Fr(t < 0 ? 0 : d0(t) | 0);
    }
    C.allocUnsafe = function (t) {
      return c0(t);
    };
    C.allocUnsafeSlow = function (t) {
      return c0(t);
    };
    function gm(t, e) {
      if (
        ((typeof e != 'string' || e === '') && (e = 'utf8'), !C.isEncoding(e))
      )
        throw new TypeError('Unknown encoding: ' + e);
      let r = fd(t, e) | 0,
        n = Fr(r),
        i = n.write(t, e);
      return i !== r && (n = n.slice(0, i)), n;
    }
    function s0(t) {
      let e = t.length < 0 ? 0 : d0(t.length) | 0,
        r = Fr(e);
      for (let n = 0; n < e; n += 1) r[n] = t[n] & 255;
      return r;
    }
    function wm(t) {
      if (Er(t, Uint8Array)) {
        let e = new Uint8Array(t);
        return h0(e.buffer, e.byteOffset, e.byteLength);
      }
      return s0(t);
    }
    function h0(t, e, r) {
      if (e < 0 || t.byteLength < e)
        throw new RangeError('"offset" is outside of buffer bounds');
      if (t.byteLength < e + (r || 0))
        throw new RangeError('"length" is outside of buffer bounds');
      let n;
      return (
        e === void 0 && r === void 0
          ? (n = new Uint8Array(t))
          : r === void 0
            ? (n = new Uint8Array(t, e))
            : (n = new Uint8Array(t, e, r)),
        Object.setPrototypeOf(n, C.prototype),
        n
      );
    }
    function _m(t) {
      if (C.isBuffer(t)) {
        let e = d0(t.length) | 0,
          r = Fr(e);
        return r.length === 0 || t.copy(r, 0, 0, e), r;
      }
      if (t.length !== void 0)
        return typeof t.length != 'number' || p0(t.length) ? Fr(0) : s0(t);
      if (t.type === 'Buffer' && Array.isArray(t.data)) return s0(t.data);
    }
    function d0(t) {
      if (t >= ka)
        throw new RangeError(
          'Attempt to allocate Buffer larger than maximum size: 0x' +
            ka.toString(16) +
            ' bytes'
        );
      return t | 0;
    }
    function xm(t) {
      return +t != t && (t = 0), C.alloc(+t);
    }
    C.isBuffer = function (e) {
      return e != null && e._isBuffer === !0 && e !== C.prototype;
    };
    C.compare = function (e, r) {
      if (
        (Er(e, Uint8Array) && (e = C.from(e, e.offset, e.byteLength)),
        Er(r, Uint8Array) && (r = C.from(r, r.offset, r.byteLength)),
        !C.isBuffer(e) || !C.isBuffer(r))
      )
        throw new TypeError(
          'The "buf1", "buf2" arguments must be one of type Buffer or Uint8Array'
        );
      if (e === r) return 0;
      let n = e.length,
        i = r.length;
      for (let a = 0, h = Math.min(n, i); a < h; ++a)
        if (e[a] !== r[a]) {
          (n = e[a]), (i = r[a]);
          break;
        }
      return n < i ? -1 : i < n ? 1 : 0;
    };
    C.isEncoding = function (e) {
      switch (String(e).toLowerCase()) {
        case 'hex':
        case 'utf8':
        case 'utf-8':
        case 'ascii':
        case 'latin1':
        case 'binary':
        case 'base64':
        case 'ucs2':
        case 'ucs-2':
        case 'utf16le':
        case 'utf-16le':
          return !0;
        default:
          return !1;
      }
    };
    C.concat = function (e, r) {
      if (!Array.isArray(e))
        throw new TypeError('"list" argument must be an Array of Buffers');
      if (e.length === 0) return C.alloc(0);
      let n;
      if (r === void 0) for (r = 0, n = 0; n < e.length; ++n) r += e[n].length;
      let i = C.allocUnsafe(r),
        a = 0;
      for (n = 0; n < e.length; ++n) {
        let h = e[n];
        if (Er(h, Uint8Array))
          a + h.length > i.length
            ? (C.isBuffer(h) || (h = C.from(h)), h.copy(i, a))
            : Uint8Array.prototype.set.call(i, h, a);
        else if (C.isBuffer(h)) h.copy(i, a);
        else throw new TypeError('"list" argument must be an Array of Buffers');
        a += h.length;
      }
      return i;
    };
    function fd(t, e) {
      if (C.isBuffer(t)) return t.length;
      if (ArrayBuffer.isView(t) || Er(t, ArrayBuffer)) return t.byteLength;
      if (typeof t != 'string')
        throw new TypeError(
          'The "string" argument must be one of type string, Buffer, or ArrayBuffer. Received type ' +
            typeof t
        );
      let r = t.length,
        n = arguments.length > 2 && arguments[2] === !0;
      if (!n && r === 0) return 0;
      let i = !1;
      for (;;)
        switch (e) {
          case 'ascii':
          case 'latin1':
          case 'binary':
            return r;
          case 'utf8':
          case 'utf-8':
            return u0(t).length;
          case 'ucs2':
          case 'ucs-2':
          case 'utf16le':
          case 'utf-16le':
            return r * 2;
          case 'hex':
            return r >>> 1;
          case 'base64':
            return pd(t).length;
          default:
            if (i) return n ? -1 : u0(t).length;
            (e = ('' + e).toLowerCase()), (i = !0);
        }
    }
    C.byteLength = fd;
    function Mm(t, e, r) {
      let n = !1;
      if (
        ((e === void 0 || e < 0) && (e = 0),
        e > this.length ||
          ((r === void 0 || r > this.length) && (r = this.length), r <= 0) ||
          ((r >>>= 0), (e >>>= 0), r <= e))
      )
        return '';
      for (t || (t = 'utf8'); ; )
        switch (t) {
          case 'hex':
            return km(this, e, r);
          case 'utf8':
          case 'utf-8':
            return od(this, e, r);
          case 'ascii':
            return Tm(this, e, r);
          case 'latin1':
          case 'binary':
            return Pm(this, e, r);
          case 'base64':
            return Rm(this, e, r);
          case 'ucs2':
          case 'ucs-2':
          case 'utf16le':
          case 'utf-16le':
            return Om(this, e, r);
          default:
            if (n) throw new TypeError('Unknown encoding: ' + t);
            (t = (t + '').toLowerCase()), (n = !0);
        }
    }
    C.prototype._isBuffer = !0;
    function ji(t, e, r) {
      let n = t[e];
      (t[e] = t[r]), (t[r] = n);
    }
    C.prototype.swap16 = function () {
      let e = this.length;
      if (e % 2 !== 0)
        throw new RangeError('Buffer size must be a multiple of 16-bits');
      for (let r = 0; r < e; r += 2) ji(this, r, r + 1);
      return this;
    };
    C.prototype.swap32 = function () {
      let e = this.length;
      if (e % 4 !== 0)
        throw new RangeError('Buffer size must be a multiple of 32-bits');
      for (let r = 0; r < e; r += 4) ji(this, r, r + 3), ji(this, r + 1, r + 2);
      return this;
    };
    C.prototype.swap64 = function () {
      let e = this.length;
      if (e % 8 !== 0)
        throw new RangeError('Buffer size must be a multiple of 64-bits');
      for (let r = 0; r < e; r += 8)
        ji(this, r, r + 7),
          ji(this, r + 1, r + 6),
          ji(this, r + 2, r + 5),
          ji(this, r + 3, r + 4);
      return this;
    };
    C.prototype.toString = function () {
      let e = this.length;
      return e === 0
        ? ''
        : arguments.length === 0
          ? od(this, 0, e)
          : Mm.apply(this, arguments);
    };
    C.prototype.toLocaleString = C.prototype.toString;
    C.prototype.equals = function (e) {
      if (!C.isBuffer(e)) throw new TypeError('Argument must be a Buffer');
      return this === e ? !0 : C.compare(this, e) === 0;
    };
    C.prototype.inspect = function () {
      let e = '',
        r = Bn.INSPECT_MAX_BYTES;
      return (
        (e = this.toString('hex', 0, r)
          .replace(/(.{2})/g, '$1 ')
          .trim()),
        this.length > r && (e += ' ... '),
        '<Buffer ' + e + '>'
      );
    };
    Qc && (C.prototype[Qc] = C.prototype.inspect);
    C.prototype.compare = function (e, r, n, i, a) {
      if (
        (Er(e, Uint8Array) && (e = C.from(e, e.offset, e.byteLength)),
        !C.isBuffer(e))
      )
        throw new TypeError(
          'The "target" argument must be one of type Buffer or Uint8Array. Received type ' +
            typeof e
        );
      if (
        (r === void 0 && (r = 0),
        n === void 0 && (n = e ? e.length : 0),
        i === void 0 && (i = 0),
        a === void 0 && (a = this.length),
        r < 0 || n > e.length || i < 0 || a > this.length)
      )
        throw new RangeError('out of range index');
      if (i >= a && r >= n) return 0;
      if (i >= a) return -1;
      if (r >= n) return 1;
      if (((r >>>= 0), (n >>>= 0), (i >>>= 0), (a >>>= 0), this === e))
        return 0;
      let h = a - i,
        v = n - r,
        g = Math.min(h, v),
        M = this.slice(i, a),
        x = e.slice(r, n);
      for (let E = 0; E < g; ++E)
        if (M[E] !== x[E]) {
          (h = M[E]), (v = x[E]);
          break;
        }
      return h < v ? -1 : v < h ? 1 : 0;
    };
    function ad(t, e, r, n, i) {
      if (t.length === 0) return -1;
      if (
        (typeof r == 'string'
          ? ((n = r), (r = 0))
          : r > 2147483647
            ? (r = 2147483647)
            : r < -2147483648 && (r = -2147483648),
        (r = +r),
        p0(r) && (r = i ? 0 : t.length - 1),
        r < 0 && (r = t.length + r),
        r >= t.length)
      ) {
        if (i) return -1;
        r = t.length - 1;
      } else if (r < 0)
        if (i) r = 0;
        else return -1;
      if ((typeof e == 'string' && (e = C.from(e, n)), C.isBuffer(e)))
        return e.length === 0 ? -1 : ed(t, e, r, n, i);
      if (typeof e == 'number')
        return (
          (e = e & 255),
          typeof Uint8Array.prototype.indexOf == 'function'
            ? i
              ? Uint8Array.prototype.indexOf.call(t, e, r)
              : Uint8Array.prototype.lastIndexOf.call(t, e, r)
            : ed(t, [e], r, n, i)
        );
      throw new TypeError('val must be string, number or Buffer');
    }
    function ed(t, e, r, n, i) {
      let a = 1,
        h = t.length,
        v = e.length;
      if (
        n !== void 0 &&
        ((n = String(n).toLowerCase()),
        n === 'ucs2' || n === 'ucs-2' || n === 'utf16le' || n === 'utf-16le')
      ) {
        if (t.length < 2 || e.length < 2) return -1;
        (a = 2), (h /= 2), (v /= 2), (r /= 2);
      }
      function g(x, E) {
        return a === 1 ? x[E] : x.readUInt16BE(E * a);
      }
      let M;
      if (i) {
        let x = -1;
        for (M = r; M < h; M++)
          if (g(t, M) === g(e, x === -1 ? 0 : M - x)) {
            if ((x === -1 && (x = M), M - x + 1 === v)) return x * a;
          } else x !== -1 && (M -= M - x), (x = -1);
      } else
        for (r + v > h && (r = h - v), M = r; M >= 0; M--) {
          let x = !0;
          for (let E = 0; E < v; E++)
            if (g(t, M + E) !== g(e, E)) {
              x = !1;
              break;
            }
          if (x) return M;
        }
      return -1;
    }
    C.prototype.includes = function (e, r, n) {
      return this.indexOf(e, r, n) !== -1;
    };
    C.prototype.indexOf = function (e, r, n) {
      return ad(this, e, r, n, !0);
    };
    C.prototype.lastIndexOf = function (e, r, n) {
      return ad(this, e, r, n, !1);
    };
    function Sm(t, e, r, n) {
      r = Number(r) || 0;
      let i = t.length - r;
      n ? ((n = Number(n)), n > i && (n = i)) : (n = i);
      let a = e.length;
      n > a / 2 && (n = a / 2);
      let h;
      for (h = 0; h < n; ++h) {
        let v = parseInt(e.substr(h * 2, 2), 16);
        if (p0(v)) return h;
        t[r + h] = v;
      }
      return h;
    }
    function Em(t, e, r, n) {
      return Oa(u0(e, t.length - r), t, r, n);
    }
    function Am(t, e, r, n) {
      return Oa(Lm(e), t, r, n);
    }
    function Bm(t, e, r, n) {
      return Oa(pd(e), t, r, n);
    }
    function Im(t, e, r, n) {
      return Oa(Fm(e, t.length - r), t, r, n);
    }
    C.prototype.write = function (e, r, n, i) {
      if (r === void 0) (i = 'utf8'), (n = this.length), (r = 0);
      else if (n === void 0 && typeof r == 'string')
        (i = r), (n = this.length), (r = 0);
      else if (isFinite(r))
        (r = r >>> 0),
          isFinite(n)
            ? ((n = n >>> 0), i === void 0 && (i = 'utf8'))
            : ((i = n), (n = void 0));
      else
        throw new Error(
          'Buffer.write(string, encoding, offset[, length]) is no longer supported'
        );
      let a = this.length - r;
      if (
        ((n === void 0 || n > a) && (n = a),
        (e.length > 0 && (n < 0 || r < 0)) || r > this.length)
      )
        throw new RangeError('Attempt to write outside buffer bounds');
      i || (i = 'utf8');
      let h = !1;
      for (;;)
        switch (i) {
          case 'hex':
            return Sm(this, e, r, n);
          case 'utf8':
          case 'utf-8':
            return Em(this, e, r, n);
          case 'ascii':
          case 'latin1':
          case 'binary':
            return Am(this, e, r, n);
          case 'base64':
            return Bm(this, e, r, n);
          case 'ucs2':
          case 'ucs-2':
          case 'utf16le':
          case 'utf-16le':
            return Im(this, e, r, n);
          default:
            if (h) throw new TypeError('Unknown encoding: ' + i);
            (i = ('' + i).toLowerCase()), (h = !0);
        }
    };
    C.prototype.toJSON = function () {
      return {
        type: 'Buffer',
        data: Array.prototype.slice.call(this._arr || this, 0),
      };
    };
    function Rm(t, e, r) {
      return e === 0 && r === t.length
        ? o0.fromByteArray(t)
        : o0.fromByteArray(t.slice(e, r));
    }
    function od(t, e, r) {
      r = Math.min(t.length, r);
      let n = [],
        i = e;
      for (; i < r; ) {
        let a = t[i],
          h = null,
          v = a > 239 ? 4 : a > 223 ? 3 : a > 191 ? 2 : 1;
        if (i + v <= r) {
          let g, M, x, E;
          switch (v) {
            case 1:
              a < 128 && (h = a);
              break;
            case 2:
              (g = t[i + 1]),
                (g & 192) === 128 &&
                  ((E = ((a & 31) << 6) | (g & 63)), E > 127 && (h = E));
              break;
            case 3:
              (g = t[i + 1]),
                (M = t[i + 2]),
                (g & 192) === 128 &&
                  (M & 192) === 128 &&
                  ((E = ((a & 15) << 12) | ((g & 63) << 6) | (M & 63)),
                  E > 2047 && (E < 55296 || E > 57343) && (h = E));
              break;
            case 4:
              (g = t[i + 1]),
                (M = t[i + 2]),
                (x = t[i + 3]),
                (g & 192) === 128 &&
                  (M & 192) === 128 &&
                  (x & 192) === 128 &&
                  ((E =
                    ((a & 15) << 18) |
                    ((g & 63) << 12) |
                    ((M & 63) << 6) |
                    (x & 63)),
                  E > 65535 && E < 1114112 && (h = E));
          }
        }
        h === null
          ? ((h = 65533), (v = 1))
          : h > 65535 &&
            ((h -= 65536),
            n.push(((h >>> 10) & 1023) | 55296),
            (h = 56320 | (h & 1023))),
          n.push(h),
          (i += v);
      }
      return qm(n);
    }
    var td = 4096;
    function qm(t) {
      let e = t.length;
      if (e <= td) return String.fromCharCode.apply(String, t);
      let r = '',
        n = 0;
      for (; n < e; )
        r += String.fromCharCode.apply(String, t.slice(n, (n += td)));
      return r;
    }
    function Tm(t, e, r) {
      let n = '';
      r = Math.min(t.length, r);
      for (let i = e; i < r; ++i) n += String.fromCharCode(t[i] & 127);
      return n;
    }
    function Pm(t, e, r) {
      let n = '';
      r = Math.min(t.length, r);
      for (let i = e; i < r; ++i) n += String.fromCharCode(t[i]);
      return n;
    }
    function km(t, e, r) {
      let n = t.length;
      (!e || e < 0) && (e = 0), (!r || r < 0 || r > n) && (r = n);
      let i = '';
      for (let a = e; a < r; ++a) i += jm[t[a]];
      return i;
    }
    function Om(t, e, r) {
      let n = t.slice(e, r),
        i = '';
      for (let a = 0; a < n.length - 1; a += 2)
        i += String.fromCharCode(n[a] + n[a + 1] * 256);
      return i;
    }
    C.prototype.slice = function (e, r) {
      let n = this.length;
      (e = ~~e),
        (r = r === void 0 ? n : ~~r),
        e < 0 ? ((e += n), e < 0 && (e = 0)) : e > n && (e = n),
        r < 0 ? ((r += n), r < 0 && (r = 0)) : r > n && (r = n),
        r < e && (r = e);
      let i = this.subarray(e, r);
      return Object.setPrototypeOf(i, C.prototype), i;
    };
    function nt(t, e, r) {
      if (t % 1 !== 0 || t < 0) throw new RangeError('offset is not uint');
      if (t + e > r)
        throw new RangeError('Trying to access beyond buffer length');
    }
    C.prototype.readUintLE = C.prototype.readUIntLE = function (e, r, n) {
      (e = e >>> 0), (r = r >>> 0), n || nt(e, r, this.length);
      let i = this[e],
        a = 1,
        h = 0;
      for (; ++h < r && (a *= 256); ) i += this[e + h] * a;
      return i;
    };
    C.prototype.readUintBE = C.prototype.readUIntBE = function (e, r, n) {
      (e = e >>> 0), (r = r >>> 0), n || nt(e, r, this.length);
      let i = this[e + --r],
        a = 1;
      for (; r > 0 && (a *= 256); ) i += this[e + --r] * a;
      return i;
    };
    C.prototype.readUint8 = C.prototype.readUInt8 = function (e, r) {
      return (e = e >>> 0), r || nt(e, 1, this.length), this[e];
    };
    C.prototype.readUint16LE = C.prototype.readUInt16LE = function (e, r) {
      return (
        (e = e >>> 0), r || nt(e, 2, this.length), this[e] | (this[e + 1] << 8)
      );
    };
    C.prototype.readUint16BE = C.prototype.readUInt16BE = function (e, r) {
      return (
        (e = e >>> 0), r || nt(e, 2, this.length), (this[e] << 8) | this[e + 1]
      );
    };
    C.prototype.readUint32LE = C.prototype.readUInt32LE = function (e, r) {
      return (
        (e = e >>> 0),
        r || nt(e, 4, this.length),
        (this[e] | (this[e + 1] << 8) | (this[e + 2] << 16)) +
          this[e + 3] * 16777216
      );
    };
    C.prototype.readUint32BE = C.prototype.readUInt32BE = function (e, r) {
      return (
        (e = e >>> 0),
        r || nt(e, 4, this.length),
        this[e] * 16777216 +
          ((this[e + 1] << 16) | (this[e + 2] << 8) | this[e + 3])
      );
    };
    C.prototype.readBigUInt64LE = bi(function (e) {
      (e = e >>> 0), An(e, 'offset');
      let r = this[e],
        n = this[e + 7];
      (r === void 0 || n === void 0) && If(e, this.length - 8);
      let i =
          r + this[++e] * 2 ** 8 + this[++e] * 2 ** 16 + this[++e] * 2 ** 24,
        a = this[++e] + this[++e] * 2 ** 8 + this[++e] * 2 ** 16 + n * 2 ** 24;
      return BigInt(i) + (BigInt(a) << BigInt(32));
    });
    C.prototype.readBigUInt64BE = bi(function (e) {
      (e = e >>> 0), An(e, 'offset');
      let r = this[e],
        n = this[e + 7];
      (r === void 0 || n === void 0) && If(e, this.length - 8);
      let i =
          r * 2 ** 24 + this[++e] * 2 ** 16 + this[++e] * 2 ** 8 + this[++e],
        a = this[++e] * 2 ** 24 + this[++e] * 2 ** 16 + this[++e] * 2 ** 8 + n;
      return (BigInt(i) << BigInt(32)) + BigInt(a);
    });
    C.prototype.readIntLE = function (e, r, n) {
      (e = e >>> 0), (r = r >>> 0), n || nt(e, r, this.length);
      let i = this[e],
        a = 1,
        h = 0;
      for (; ++h < r && (a *= 256); ) i += this[e + h] * a;
      return (a *= 128), i >= a && (i -= Math.pow(2, 8 * r)), i;
    };
    C.prototype.readIntBE = function (e, r, n) {
      (e = e >>> 0), (r = r >>> 0), n || nt(e, r, this.length);
      let i = r,
        a = 1,
        h = this[e + --i];
      for (; i > 0 && (a *= 256); ) h += this[e + --i] * a;
      return (a *= 128), h >= a && (h -= Math.pow(2, 8 * r)), h;
    };
    C.prototype.readInt8 = function (e, r) {
      return (
        (e = e >>> 0),
        r || nt(e, 1, this.length),
        this[e] & 128 ? (255 - this[e] + 1) * -1 : this[e]
      );
    };
    C.prototype.readInt16LE = function (e, r) {
      (e = e >>> 0), r || nt(e, 2, this.length);
      let n = this[e] | (this[e + 1] << 8);
      return n & 32768 ? n | 4294901760 : n;
    };
    C.prototype.readInt16BE = function (e, r) {
      (e = e >>> 0), r || nt(e, 2, this.length);
      let n = this[e + 1] | (this[e] << 8);
      return n & 32768 ? n | 4294901760 : n;
    };
    C.prototype.readInt32LE = function (e, r) {
      return (
        (e = e >>> 0),
        r || nt(e, 4, this.length),
        this[e] | (this[e + 1] << 8) | (this[e + 2] << 16) | (this[e + 3] << 24)
      );
    };
    C.prototype.readInt32BE = function (e, r) {
      return (
        (e = e >>> 0),
        r || nt(e, 4, this.length),
        (this[e] << 24) | (this[e + 1] << 16) | (this[e + 2] << 8) | this[e + 3]
      );
    };
    C.prototype.readBigInt64LE = bi(function (e) {
      (e = e >>> 0), An(e, 'offset');
      let r = this[e],
        n = this[e + 7];
      (r === void 0 || n === void 0) && If(e, this.length - 8);
      let i =
        this[e + 4] + this[e + 5] * 2 ** 8 + this[e + 6] * 2 ** 16 + (n << 24);
      return (
        (BigInt(i) << BigInt(32)) +
        BigInt(
          r + this[++e] * 2 ** 8 + this[++e] * 2 ** 16 + this[++e] * 2 ** 24
        )
      );
    });
    C.prototype.readBigInt64BE = bi(function (e) {
      (e = e >>> 0), An(e, 'offset');
      let r = this[e],
        n = this[e + 7];
      (r === void 0 || n === void 0) && If(e, this.length - 8);
      let i = (r << 24) + this[++e] * 2 ** 16 + this[++e] * 2 ** 8 + this[++e];
      return (
        (BigInt(i) << BigInt(32)) +
        BigInt(
          this[++e] * 2 ** 24 + this[++e] * 2 ** 16 + this[++e] * 2 ** 8 + n
        )
      );
    });
    C.prototype.readFloatLE = function (e, r) {
      return (
        (e = e >>> 0), r || nt(e, 4, this.length), En.read(this, e, !0, 23, 4)
      );
    };
    C.prototype.readFloatBE = function (e, r) {
      return (
        (e = e >>> 0), r || nt(e, 4, this.length), En.read(this, e, !1, 23, 4)
      );
    };
    C.prototype.readDoubleLE = function (e, r) {
      return (
        (e = e >>> 0), r || nt(e, 8, this.length), En.read(this, e, !0, 52, 8)
      );
    };
    C.prototype.readDoubleBE = function (e, r) {
      return (
        (e = e >>> 0), r || nt(e, 8, this.length), En.read(this, e, !1, 52, 8)
      );
    };
    function wt(t, e, r, n, i, a) {
      if (!C.isBuffer(t))
        throw new TypeError('"buffer" argument must be a Buffer instance');
      if (e > i || e < a)
        throw new RangeError('"value" argument is out of bounds');
      if (r + n > t.length) throw new RangeError('Index out of range');
    }
    C.prototype.writeUintLE = C.prototype.writeUIntLE = function (e, r, n, i) {
      if (((e = +e), (r = r >>> 0), (n = n >>> 0), !i)) {
        let v = Math.pow(2, 8 * n) - 1;
        wt(this, e, r, n, v, 0);
      }
      let a = 1,
        h = 0;
      for (this[r] = e & 255; ++h < n && (a *= 256); )
        this[r + h] = (e / a) & 255;
      return r + n;
    };
    C.prototype.writeUintBE = C.prototype.writeUIntBE = function (e, r, n, i) {
      if (((e = +e), (r = r >>> 0), (n = n >>> 0), !i)) {
        let v = Math.pow(2, 8 * n) - 1;
        wt(this, e, r, n, v, 0);
      }
      let a = n - 1,
        h = 1;
      for (this[r + a] = e & 255; --a >= 0 && (h *= 256); )
        this[r + a] = (e / h) & 255;
      return r + n;
    };
    C.prototype.writeUint8 = C.prototype.writeUInt8 = function (e, r, n) {
      return (
        (e = +e),
        (r = r >>> 0),
        n || wt(this, e, r, 1, 255, 0),
        (this[r] = e & 255),
        r + 1
      );
    };
    C.prototype.writeUint16LE = C.prototype.writeUInt16LE = function (e, r, n) {
      return (
        (e = +e),
        (r = r >>> 0),
        n || wt(this, e, r, 2, 65535, 0),
        (this[r] = e & 255),
        (this[r + 1] = e >>> 8),
        r + 2
      );
    };
    C.prototype.writeUint16BE = C.prototype.writeUInt16BE = function (e, r, n) {
      return (
        (e = +e),
        (r = r >>> 0),
        n || wt(this, e, r, 2, 65535, 0),
        (this[r] = e >>> 8),
        (this[r + 1] = e & 255),
        r + 2
      );
    };
    C.prototype.writeUint32LE = C.prototype.writeUInt32LE = function (e, r, n) {
      return (
        (e = +e),
        (r = r >>> 0),
        n || wt(this, e, r, 4, 4294967295, 0),
        (this[r + 3] = e >>> 24),
        (this[r + 2] = e >>> 16),
        (this[r + 1] = e >>> 8),
        (this[r] = e & 255),
        r + 4
      );
    };
    C.prototype.writeUint32BE = C.prototype.writeUInt32BE = function (e, r, n) {
      return (
        (e = +e),
        (r = r >>> 0),
        n || wt(this, e, r, 4, 4294967295, 0),
        (this[r] = e >>> 24),
        (this[r + 1] = e >>> 16),
        (this[r + 2] = e >>> 8),
        (this[r + 3] = e & 255),
        r + 4
      );
    };
    function sd(t, e, r, n, i) {
      ld(e, n, i, t, r, 7);
      let a = Number(e & BigInt(4294967295));
      (t[r++] = a),
        (a = a >> 8),
        (t[r++] = a),
        (a = a >> 8),
        (t[r++] = a),
        (a = a >> 8),
        (t[r++] = a);
      let h = Number((e >> BigInt(32)) & BigInt(4294967295));
      return (
        (t[r++] = h),
        (h = h >> 8),
        (t[r++] = h),
        (h = h >> 8),
        (t[r++] = h),
        (h = h >> 8),
        (t[r++] = h),
        r
      );
    }
    function hd(t, e, r, n, i) {
      ld(e, n, i, t, r, 7);
      let a = Number(e & BigInt(4294967295));
      (t[r + 7] = a),
        (a = a >> 8),
        (t[r + 6] = a),
        (a = a >> 8),
        (t[r + 5] = a),
        (a = a >> 8),
        (t[r + 4] = a);
      let h = Number((e >> BigInt(32)) & BigInt(4294967295));
      return (
        (t[r + 3] = h),
        (h = h >> 8),
        (t[r + 2] = h),
        (h = h >> 8),
        (t[r + 1] = h),
        (h = h >> 8),
        (t[r] = h),
        r + 8
      );
    }
    C.prototype.writeBigUInt64LE = bi(function (e, r = 0) {
      return sd(this, e, r, BigInt(0), BigInt('0xffffffffffffffff'));
    });
    C.prototype.writeBigUInt64BE = bi(function (e, r = 0) {
      return hd(this, e, r, BigInt(0), BigInt('0xffffffffffffffff'));
    });
    C.prototype.writeIntLE = function (e, r, n, i) {
      if (((e = +e), (r = r >>> 0), !i)) {
        let g = Math.pow(2, 8 * n - 1);
        wt(this, e, r, n, g - 1, -g);
      }
      let a = 0,
        h = 1,
        v = 0;
      for (this[r] = e & 255; ++a < n && (h *= 256); )
        e < 0 && v === 0 && this[r + a - 1] !== 0 && (v = 1),
          (this[r + a] = (((e / h) >> 0) - v) & 255);
      return r + n;
    };
    C.prototype.writeIntBE = function (e, r, n, i) {
      if (((e = +e), (r = r >>> 0), !i)) {
        let g = Math.pow(2, 8 * n - 1);
        wt(this, e, r, n, g - 1, -g);
      }
      let a = n - 1,
        h = 1,
        v = 0;
      for (this[r + a] = e & 255; --a >= 0 && (h *= 256); )
        e < 0 && v === 0 && this[r + a + 1] !== 0 && (v = 1),
          (this[r + a] = (((e / h) >> 0) - v) & 255);
      return r + n;
    };
    C.prototype.writeInt8 = function (e, r, n) {
      return (
        (e = +e),
        (r = r >>> 0),
        n || wt(this, e, r, 1, 127, -128),
        e < 0 && (e = 255 + e + 1),
        (this[r] = e & 255),
        r + 1
      );
    };
    C.prototype.writeInt16LE = function (e, r, n) {
      return (
        (e = +e),
        (r = r >>> 0),
        n || wt(this, e, r, 2, 32767, -32768),
        (this[r] = e & 255),
        (this[r + 1] = e >>> 8),
        r + 2
      );
    };
    C.prototype.writeInt16BE = function (e, r, n) {
      return (
        (e = +e),
        (r = r >>> 0),
        n || wt(this, e, r, 2, 32767, -32768),
        (this[r] = e >>> 8),
        (this[r + 1] = e & 255),
        r + 2
      );
    };
    C.prototype.writeInt32LE = function (e, r, n) {
      return (
        (e = +e),
        (r = r >>> 0),
        n || wt(this, e, r, 4, 2147483647, -2147483648),
        (this[r] = e & 255),
        (this[r + 1] = e >>> 8),
        (this[r + 2] = e >>> 16),
        (this[r + 3] = e >>> 24),
        r + 4
      );
    };
    C.prototype.writeInt32BE = function (e, r, n) {
      return (
        (e = +e),
        (r = r >>> 0),
        n || wt(this, e, r, 4, 2147483647, -2147483648),
        e < 0 && (e = 4294967295 + e + 1),
        (this[r] = e >>> 24),
        (this[r + 1] = e >>> 16),
        (this[r + 2] = e >>> 8),
        (this[r + 3] = e & 255),
        r + 4
      );
    };
    C.prototype.writeBigInt64LE = bi(function (e, r = 0) {
      return sd(
        this,
        e,
        r,
        -BigInt('0x8000000000000000'),
        BigInt('0x7fffffffffffffff')
      );
    });
    C.prototype.writeBigInt64BE = bi(function (e, r = 0) {
      return hd(
        this,
        e,
        r,
        -BigInt('0x8000000000000000'),
        BigInt('0x7fffffffffffffff')
      );
    });
    function ud(t, e, r, n, i, a) {
      if (r + n > t.length) throw new RangeError('Index out of range');
      if (r < 0) throw new RangeError('Index out of range');
    }
    function cd(t, e, r, n, i) {
      return (
        (e = +e),
        (r = r >>> 0),
        i || ud(t, e, r, 4, 34028234663852886e22, -34028234663852886e22),
        En.write(t, e, r, n, 23, 4),
        r + 4
      );
    }
    C.prototype.writeFloatLE = function (e, r, n) {
      return cd(this, e, r, !0, n);
    };
    C.prototype.writeFloatBE = function (e, r, n) {
      return cd(this, e, r, !1, n);
    };
    function dd(t, e, r, n, i) {
      return (
        (e = +e),
        (r = r >>> 0),
        i || ud(t, e, r, 8, 17976931348623157e292, -17976931348623157e292),
        En.write(t, e, r, n, 52, 8),
        r + 8
      );
    }
    C.prototype.writeDoubleLE = function (e, r, n) {
      return dd(this, e, r, !0, n);
    };
    C.prototype.writeDoubleBE = function (e, r, n) {
      return dd(this, e, r, !1, n);
    };
    C.prototype.copy = function (e, r, n, i) {
      if (!C.isBuffer(e)) throw new TypeError('argument should be a Buffer');
      if (
        (n || (n = 0),
        !i && i !== 0 && (i = this.length),
        r >= e.length && (r = e.length),
        r || (r = 0),
        i > 0 && i < n && (i = n),
        i === n || e.length === 0 || this.length === 0)
      )
        return 0;
      if (r < 0) throw new RangeError('targetStart out of bounds');
      if (n < 0 || n >= this.length) throw new RangeError('Index out of range');
      if (i < 0) throw new RangeError('sourceEnd out of bounds');
      i > this.length && (i = this.length),
        e.length - r < i - n && (i = e.length - r + n);
      let a = i - n;
      return (
        this === e && typeof Uint8Array.prototype.copyWithin == 'function'
          ? this.copyWithin(r, n, i)
          : Uint8Array.prototype.set.call(e, this.subarray(n, i), r),
        a
      );
    };
    C.prototype.fill = function (e, r, n, i) {
      if (typeof e == 'string') {
        if (
          (typeof r == 'string'
            ? ((i = r), (r = 0), (n = this.length))
            : typeof n == 'string' && ((i = n), (n = this.length)),
          i !== void 0 && typeof i != 'string')
        )
          throw new TypeError('encoding must be a string');
        if (typeof i == 'string' && !C.isEncoding(i))
          throw new TypeError('Unknown encoding: ' + i);
        if (e.length === 1) {
          let h = e.charCodeAt(0);
          ((i === 'utf8' && h < 128) || i === 'latin1') && (e = h);
        }
      } else
        typeof e == 'number'
          ? (e = e & 255)
          : typeof e == 'boolean' && (e = Number(e));
      if (r < 0 || this.length < r || this.length < n)
        throw new RangeError('Out of range index');
      if (n <= r) return this;
      (r = r >>> 0), (n = n === void 0 ? this.length : n >>> 0), e || (e = 0);
      let a;
      if (typeof e == 'number') for (a = r; a < n; ++a) this[a] = e;
      else {
        let h = C.isBuffer(e) ? e : C.from(e, i),
          v = h.length;
        if (v === 0)
          throw new TypeError(
            'The value "' + e + '" is invalid for argument "value"'
          );
        for (a = 0; a < n - r; ++a) this[a + r] = h[a % v];
      }
      return this;
    };
    var Sn = {};
    function l0(t, e, r) {
      Sn[t] = class extends r {
        constructor() {
          super(),
            Object.defineProperty(this, 'message', {
              value: e.apply(this, arguments),
              writable: !0,
              configurable: !0,
            }),
            (this.name = `${this.name} [${t}]`),
            this.stack,
            delete this.name;
        }
        get code() {
          return t;
        }
        set code(i) {
          Object.defineProperty(this, 'code', {
            configurable: !0,
            enumerable: !0,
            value: i,
            writable: !0,
          });
        }
        toString() {
          return `${this.name} [${t}]: ${this.message}`;
        }
      };
    }
    l0(
      'ERR_BUFFER_OUT_OF_BOUNDS',
      function (t) {
        return t
          ? `${t} is outside of buffer bounds`
          : 'Attempt to access memory outside buffer bounds';
      },
      RangeError
    );
    l0(
      'ERR_INVALID_ARG_TYPE',
      function (t, e) {
        return `The "${t}" argument must be of type number. Received type ${typeof e}`;
      },
      TypeError
    );
    l0(
      'ERR_OUT_OF_RANGE',
      function (t, e, r) {
        let n = `The value of "${t}" is out of range.`,
          i = r;
        return (
          Number.isInteger(r) && Math.abs(r) > 2 ** 32
            ? (i = rd(String(r)))
            : typeof r == 'bigint' &&
              ((i = String(r)),
              (r > BigInt(2) ** BigInt(32) || r < -(BigInt(2) ** BigInt(32))) &&
                (i = rd(i)),
              (i += 'n')),
          (n += ` It must be ${e}. Received ${i}`),
          n
        );
      },
      RangeError
    );
    function rd(t) {
      let e = '',
        r = t.length,
        n = t[0] === '-' ? 1 : 0;
      for (; r >= n + 4; r -= 3) e = `_${t.slice(r - 3, r)}${e}`;
      return `${t.slice(0, r)}${e}`;
    }
    function Cm(t, e, r) {
      An(e, 'offset'),
        (t[e] === void 0 || t[e + r] === void 0) && If(e, t.length - (r + 1));
    }
    function ld(t, e, r, n, i, a) {
      if (t > r || t < e) {
        let h = typeof e == 'bigint' ? 'n' : '',
          v;
        throw (
          (a > 3
            ? e === 0 || e === BigInt(0)
              ? (v = `>= 0${h} and < 2${h} ** ${(a + 1) * 8}${h}`)
              : (v = `>= -(2${h} ** ${(a + 1) * 8 - 1}${h}) and < 2 ** ${(a + 1) * 8 - 1}${h}`)
            : (v = `>= ${e}${h} and <= ${r}${h}`),
          new Sn.ERR_OUT_OF_RANGE('value', v, t))
        );
      }
      Cm(n, i, a);
    }
    function An(t, e) {
      if (typeof t != 'number')
        throw new Sn.ERR_INVALID_ARG_TYPE(e, 'number', t);
    }
    function If(t, e, r) {
      throw Math.floor(t) !== t
        ? (An(t, r), new Sn.ERR_OUT_OF_RANGE(r || 'offset', 'an integer', t))
        : e < 0
          ? new Sn.ERR_BUFFER_OUT_OF_BOUNDS()
          : new Sn.ERR_OUT_OF_RANGE(
              r || 'offset',
              `>= ${r ? 1 : 0} and <= ${e}`,
              t
            );
    }
    var Nm = /[^+/0-9A-Za-z-_]/g;
    function Dm(t) {
      if (((t = t.split('=')[0]), (t = t.trim().replace(Nm, '')), t.length < 2))
        return '';
      for (; t.length % 4 !== 0; ) t = t + '=';
      return t;
    }
    function u0(t, e) {
      e = e || 1 / 0;
      let r,
        n = t.length,
        i = null,
        a = [];
      for (let h = 0; h < n; ++h) {
        if (((r = t.charCodeAt(h)), r > 55295 && r < 57344)) {
          if (!i) {
            if (r > 56319) {
              (e -= 3) > -1 && a.push(239, 191, 189);
              continue;
            } else if (h + 1 === n) {
              (e -= 3) > -1 && a.push(239, 191, 189);
              continue;
            }
            i = r;
            continue;
          }
          if (r < 56320) {
            (e -= 3) > -1 && a.push(239, 191, 189), (i = r);
            continue;
          }
          r = (((i - 55296) << 10) | (r - 56320)) + 65536;
        } else i && (e -= 3) > -1 && a.push(239, 191, 189);
        if (((i = null), r < 128)) {
          if ((e -= 1) < 0) break;
          a.push(r);
        } else if (r < 2048) {
          if ((e -= 2) < 0) break;
          a.push((r >> 6) | 192, (r & 63) | 128);
        } else if (r < 65536) {
          if ((e -= 3) < 0) break;
          a.push((r >> 12) | 224, ((r >> 6) & 63) | 128, (r & 63) | 128);
        } else if (r < 1114112) {
          if ((e -= 4) < 0) break;
          a.push(
            (r >> 18) | 240,
            ((r >> 12) & 63) | 128,
            ((r >> 6) & 63) | 128,
            (r & 63) | 128
          );
        } else throw new Error('Invalid code point');
      }
      return a;
    }
    function Lm(t) {
      let e = [];
      for (let r = 0; r < t.length; ++r) e.push(t.charCodeAt(r) & 255);
      return e;
    }
    function Fm(t, e) {
      let r,
        n,
        i,
        a = [];
      for (let h = 0; h < t.length && !((e -= 2) < 0); ++h)
        (r = t.charCodeAt(h)),
          (n = r >> 8),
          (i = r % 256),
          a.push(i),
          a.push(n);
      return a;
    }
    function pd(t) {
      return o0.toByteArray(Dm(t));
    }
    function Oa(t, e, r, n) {
      let i;
      for (i = 0; i < n && !(i + r >= e.length || i >= t.length); ++i)
        e[i + r] = t[i];
      return i;
    }
    function Er(t, e) {
      return (
        t instanceof e ||
        (t != null &&
          t.constructor != null &&
          t.constructor.name != null &&
          t.constructor.name === e.name)
      );
    }
    function p0(t) {
      return t !== t;
    }
    var jm = (function () {
      let t = '0123456789abcdef',
        e = new Array(256);
      for (let r = 0; r < 16; ++r) {
        let n = r * 16;
        for (let i = 0; i < 16; ++i) e[n + i] = t[r] + t[i];
      }
      return e;
    })();
    function bi(t) {
      return typeof BigInt > 'u' ? Um : t;
    }
    function Um() {
      throw new Error('BigInt not supported');
    }
  });
  var Ie = R((v0, bd) => {
    S();
    var Ca = Et(),
      Ar = Ca.Buffer;
    function vd(t, e) {
      for (var r in t) e[r] = t[r];
    }
    Ar.from && Ar.alloc && Ar.allocUnsafe && Ar.allocUnsafeSlow
      ? (bd.exports = Ca)
      : (vd(Ca, v0), (v0.Buffer = Ui));
    function Ui(t, e, r) {
      return Ar(t, e, r);
    }
    Ui.prototype = Object.create(Ar.prototype);
    vd(Ar, Ui);
    Ui.from = function (t, e, r) {
      if (typeof t == 'number')
        throw new TypeError('Argument must not be a number');
      return Ar(t, e, r);
    };
    Ui.alloc = function (t, e, r) {
      if (typeof t != 'number')
        throw new TypeError('Argument must be a number');
      var n = Ar(t);
      return (
        e !== void 0
          ? typeof r == 'string'
            ? n.fill(e, r)
            : n.fill(e)
          : n.fill(0),
        n
      );
    };
    Ui.allocUnsafe = function (t) {
      if (typeof t != 'number')
        throw new TypeError('Argument must be a number');
      return Ar(t);
    };
    Ui.allocUnsafeSlow = function (t) {
      if (typeof t != 'number')
        throw new TypeError('Argument must be a number');
      return Ca.SlowBuffer(t);
    };
  });
  var La = R((aI, b0) => {
    'use strict';
    S();
    var In = typeof Reflect == 'object' ? Reflect : null,
      yd =
        In && typeof In.apply == 'function'
          ? In.apply
          : function (e, r, n) {
              return Function.prototype.apply.call(e, r, n);
            },
      Na;
    In && typeof In.ownKeys == 'function'
      ? (Na = In.ownKeys)
      : Object.getOwnPropertySymbols
        ? (Na = function (e) {
            return Object.getOwnPropertyNames(e).concat(
              Object.getOwnPropertySymbols(e)
            );
          })
        : (Na = function (e) {
            return Object.getOwnPropertyNames(e);
          });
    function zm(t) {
      console && console.warn && console.warn(t);
    }
    var gd =
      Number.isNaN ||
      function (e) {
        return e !== e;
      };
    function He() {
      He.init.call(this);
    }
    b0.exports = He;
    b0.exports.once = Gm;
    He.EventEmitter = He;
    He.prototype._events = void 0;
    He.prototype._eventsCount = 0;
    He.prototype._maxListeners = void 0;
    var md = 10;
    function Da(t) {
      if (typeof t != 'function')
        throw new TypeError(
          'The "listener" argument must be of type Function. Received type ' +
            typeof t
        );
    }
    Object.defineProperty(He, 'defaultMaxListeners', {
      enumerable: !0,
      get: function () {
        return md;
      },
      set: function (t) {
        if (typeof t != 'number' || t < 0 || gd(t))
          throw new RangeError(
            'The value of "defaultMaxListeners" is out of range. It must be a non-negative number. Received ' +
              t +
              '.'
          );
        md = t;
      },
    });
    He.init = function () {
      (this._events === void 0 ||
        this._events === Object.getPrototypeOf(this)._events) &&
        ((this._events = Object.create(null)), (this._eventsCount = 0)),
        (this._maxListeners = this._maxListeners || void 0);
    };
    He.prototype.setMaxListeners = function (e) {
      if (typeof e != 'number' || e < 0 || gd(e))
        throw new RangeError(
          'The value of "n" is out of range. It must be a non-negative number. Received ' +
            e +
            '.'
        );
      return (this._maxListeners = e), this;
    };
    function wd(t) {
      return t._maxListeners === void 0
        ? He.defaultMaxListeners
        : t._maxListeners;
    }
    He.prototype.getMaxListeners = function () {
      return wd(this);
    };
    He.prototype.emit = function (e) {
      for (var r = [], n = 1; n < arguments.length; n++) r.push(arguments[n]);
      var i = e === 'error',
        a = this._events;
      if (a !== void 0) i = i && a.error === void 0;
      else if (!i) return !1;
      if (i) {
        var h;
        if ((r.length > 0 && (h = r[0]), h instanceof Error)) throw h;
        var v = new Error(
          'Unhandled error.' + (h ? ' (' + h.message + ')' : '')
        );
        throw ((v.context = h), v);
      }
      var g = a[e];
      if (g === void 0) return !1;
      if (typeof g == 'function') yd(g, this, r);
      else
        for (var M = g.length, x = Ed(g, M), n = 0; n < M; ++n)
          yd(x[n], this, r);
      return !0;
    };
    function _d(t, e, r, n) {
      var i, a, h;
      if (
        (Da(r),
        (a = t._events),
        a === void 0
          ? ((a = t._events = Object.create(null)), (t._eventsCount = 0))
          : (a.newListener !== void 0 &&
              (t.emit('newListener', e, r.listener ? r.listener : r),
              (a = t._events)),
            (h = a[e])),
        h === void 0)
      )
        (h = a[e] = r), ++t._eventsCount;
      else if (
        (typeof h == 'function'
          ? (h = a[e] = n ? [r, h] : [h, r])
          : n
            ? h.unshift(r)
            : h.push(r),
        (i = wd(t)),
        i > 0 && h.length > i && !h.warned)
      ) {
        h.warned = !0;
        var v = new Error(
          'Possible EventEmitter memory leak detected. ' +
            h.length +
            ' ' +
            String(e) +
            ' listeners added. Use emitter.setMaxListeners() to increase limit'
        );
        (v.name = 'MaxListenersExceededWarning'),
          (v.emitter = t),
          (v.type = e),
          (v.count = h.length),
          zm(v);
      }
      return t;
    }
    He.prototype.addListener = function (e, r) {
      return _d(this, e, r, !1);
    };
    He.prototype.on = He.prototype.addListener;
    He.prototype.prependListener = function (e, r) {
      return _d(this, e, r, !0);
    };
    function Hm() {
      if (!this.fired)
        return (
          this.target.removeListener(this.type, this.wrapFn),
          (this.fired = !0),
          arguments.length === 0
            ? this.listener.call(this.target)
            : this.listener.apply(this.target, arguments)
        );
    }
    function xd(t, e, r) {
      var n = { fired: !1, wrapFn: void 0, target: t, type: e, listener: r },
        i = Hm.bind(n);
      return (i.listener = r), (n.wrapFn = i), i;
    }
    He.prototype.once = function (e, r) {
      return Da(r), this.on(e, xd(this, e, r)), this;
    };
    He.prototype.prependOnceListener = function (e, r) {
      return Da(r), this.prependListener(e, xd(this, e, r)), this;
    };
    He.prototype.removeListener = function (e, r) {
      var n, i, a, h, v;
      if ((Da(r), (i = this._events), i === void 0)) return this;
      if (((n = i[e]), n === void 0)) return this;
      if (n === r || n.listener === r)
        --this._eventsCount === 0
          ? (this._events = Object.create(null))
          : (delete i[e],
            i.removeListener &&
              this.emit('removeListener', e, n.listener || r));
      else if (typeof n != 'function') {
        for (a = -1, h = n.length - 1; h >= 0; h--)
          if (n[h] === r || n[h].listener === r) {
            (v = n[h].listener), (a = h);
            break;
          }
        if (a < 0) return this;
        a === 0 ? n.shift() : Km(n, a),
          n.length === 1 && (i[e] = n[0]),
          i.removeListener !== void 0 && this.emit('removeListener', e, v || r);
      }
      return this;
    };
    He.prototype.off = He.prototype.removeListener;
    He.prototype.removeAllListeners = function (e) {
      var r, n, i;
      if (((n = this._events), n === void 0)) return this;
      if (n.removeListener === void 0)
        return (
          arguments.length === 0
            ? ((this._events = Object.create(null)), (this._eventsCount = 0))
            : n[e] !== void 0 &&
              (--this._eventsCount === 0
                ? (this._events = Object.create(null))
                : delete n[e]),
          this
        );
      if (arguments.length === 0) {
        var a = Object.keys(n),
          h;
        for (i = 0; i < a.length; ++i)
          (h = a[i]), h !== 'removeListener' && this.removeAllListeners(h);
        return (
          this.removeAllListeners('removeListener'),
          (this._events = Object.create(null)),
          (this._eventsCount = 0),
          this
        );
      }
      if (((r = n[e]), typeof r == 'function')) this.removeListener(e, r);
      else if (r !== void 0)
        for (i = r.length - 1; i >= 0; i--) this.removeListener(e, r[i]);
      return this;
    };
    function Md(t, e, r) {
      var n = t._events;
      if (n === void 0) return [];
      var i = n[e];
      return i === void 0
        ? []
        : typeof i == 'function'
          ? r
            ? [i.listener || i]
            : [i]
          : r
            ? Vm(i)
            : Ed(i, i.length);
    }
    He.prototype.listeners = function (e) {
      return Md(this, e, !0);
    };
    He.prototype.rawListeners = function (e) {
      return Md(this, e, !1);
    };
    He.listenerCount = function (t, e) {
      return typeof t.listenerCount == 'function'
        ? t.listenerCount(e)
        : Sd.call(t, e);
    };
    He.prototype.listenerCount = Sd;
    function Sd(t) {
      var e = this._events;
      if (e !== void 0) {
        var r = e[t];
        if (typeof r == 'function') return 1;
        if (r !== void 0) return r.length;
      }
      return 0;
    }
    He.prototype.eventNames = function () {
      return this._eventsCount > 0 ? Na(this._events) : [];
    };
    function Ed(t, e) {
      for (var r = new Array(e), n = 0; n < e; ++n) r[n] = t[n];
      return r;
    }
    function Km(t, e) {
      for (; e + 1 < t.length; e++) t[e] = t[e + 1];
      t.pop();
    }
    function Vm(t) {
      for (var e = new Array(t.length), r = 0; r < e.length; ++r)
        e[r] = t[r].listener || t[r];
      return e;
    }
    function Gm(t, e) {
      return new Promise(function (r, n) {
        function i(h) {
          t.removeListener(e, a), n(h);
        }
        function a() {
          typeof t.removeListener == 'function' && t.removeListener('error', i),
            r([].slice.call(arguments));
        }
        Ad(t, e, a, { once: !0 }), e !== 'error' && Wm(t, i, { once: !0 });
      });
    }
    function Wm(t, e, r) {
      typeof t.on == 'function' && Ad(t, 'error', e, r);
    }
    function Ad(t, e, r, n) {
      if (typeof t.on == 'function') n.once ? t.once(e, r) : t.on(e, r);
      else if (typeof t.addEventListener == 'function')
        t.addEventListener(e, function i(a) {
          n.once && t.removeEventListener(e, i), r(a);
        });
      else
        throw new TypeError(
          'The "emitter" argument must be of type EventEmitter. Received type ' +
            typeof t
        );
    }
  });
  var qe = R((sI, y0) => {
    S();
    typeof Object.create == 'function'
      ? (y0.exports = function (e, r) {
          r &&
            ((e.super_ = r),
            (e.prototype = Object.create(r.prototype, {
              constructor: {
                value: e,
                enumerable: !1,
                writable: !0,
                configurable: !0,
              },
            })));
        })
      : (y0.exports = function (e, r) {
          if (r) {
            e.super_ = r;
            var n = function () {};
            (n.prototype = r.prototype),
              (e.prototype = new n()),
              (e.prototype.constructor = e);
          }
        });
  });
  var m0 = R((uI, Bd) => {
    S();
    Bd.exports = La().EventEmitter;
  });
  var g0 = R((dI, Id) => {
    'use strict';
    S();
    Id.exports = function () {
      if (
        typeof Symbol != 'function' ||
        typeof Object.getOwnPropertySymbols != 'function'
      )
        return !1;
      if (typeof Symbol.iterator == 'symbol') return !0;
      var e = {},
        r = Symbol('test'),
        n = Object(r);
      if (
        typeof r == 'string' ||
        Object.prototype.toString.call(r) !== '[object Symbol]' ||
        Object.prototype.toString.call(n) !== '[object Symbol]'
      )
        return !1;
      var i = 42;
      e[r] = i;
      for (r in e) return !1;
      if (
        (typeof Object.keys == 'function' && Object.keys(e).length !== 0) ||
        (typeof Object.getOwnPropertyNames == 'function' &&
          Object.getOwnPropertyNames(e).length !== 0)
      )
        return !1;
      var a = Object.getOwnPropertySymbols(e);
      if (
        a.length !== 1 ||
        a[0] !== r ||
        !Object.prototype.propertyIsEnumerable.call(e, r)
      )
        return !1;
      if (typeof Object.getOwnPropertyDescriptor == 'function') {
        var h = Object.getOwnPropertyDescriptor(e, r);
        if (h.value !== i || h.enumerable !== !0) return !1;
      }
      return !0;
    };
  });
  var Rf = R((pI, Rd) => {
    'use strict';
    S();
    var $m = g0();
    Rd.exports = function () {
      return $m() && !!Symbol.toStringTag;
    };
  });
  var Pd = R((bI, Td) => {
    'use strict';
    S();
    var qd = typeof Symbol < 'u' && Symbol,
      Zm = g0();
    Td.exports = function () {
      return typeof qd != 'function' ||
        typeof Symbol != 'function' ||
        typeof qd('foo') != 'symbol' ||
        typeof Symbol('bar') != 'symbol'
        ? !1
        : Zm();
    };
  });
  var Od = R((mI, kd) => {
    'use strict';
    S();
    var Jm = 'Function.prototype.bind called on incompatible ',
      w0 = Array.prototype.slice,
      Xm = Object.prototype.toString,
      Ym = '[object Function]';
    kd.exports = function (e) {
      var r = this;
      if (typeof r != 'function' || Xm.call(r) !== Ym)
        throw new TypeError(Jm + r);
      for (
        var n = w0.call(arguments, 1),
          i,
          a = function () {
            if (this instanceof i) {
              var x = r.apply(this, n.concat(w0.call(arguments)));
              return Object(x) === x ? x : this;
            } else return r.apply(e, n.concat(w0.call(arguments)));
          },
          h = Math.max(0, r.length - n.length),
          v = [],
          g = 0;
        g < h;
        g++
      )
        v.push('$' + g);
      if (
        ((i = Function(
          'binder',
          'return function (' +
            v.join(',') +
            '){ return binder.apply(this,arguments); }'
        )(a)),
        r.prototype)
      ) {
        var M = function () {};
        (M.prototype = r.prototype),
          (i.prototype = new M()),
          (M.prototype = null);
      }
      return i;
    };
  });
  var Fa = R((wI, Cd) => {
    'use strict';
    S();
    var Qm = Od();
    Cd.exports = Function.prototype.bind || Qm;
  });
  var Dd = R((xI, Nd) => {
    'use strict';
    S();
    var e4 = Fa();
    Nd.exports = e4.call(Function.call, Object.prototype.hasOwnProperty);
  });
  var za = R((SI, Ud) => {
    'use strict';
    S();
    var je,
      kn = SyntaxError,
      jd = Function,
      Tn = TypeError,
      _0 = function (t) {
        try {
          return jd('"use strict"; return (' + t + ').constructor;')();
        } catch {}
      },
      zi = Object.getOwnPropertyDescriptor;
    if (zi)
      try {
        zi({}, '');
      } catch {
        zi = null;
      }
    var x0 = function () {
        throw new Tn();
      },
      t4 = zi
        ? (function () {
            try {
              return arguments.callee, x0;
            } catch {
              try {
                return zi(arguments, 'callee').get;
              } catch {
                return x0;
              }
            }
          })()
        : x0,
      Rn = Pd()(),
      yi =
        Object.getPrototypeOf ||
        function (t) {
          return t.__proto__;
        },
      qn = {},
      r4 = typeof Uint8Array > 'u' ? je : yi(Uint8Array),
      Pn = {
        '%AggregateError%': typeof AggregateError > 'u' ? je : AggregateError,
        '%Array%': Array,
        '%ArrayBuffer%': typeof ArrayBuffer > 'u' ? je : ArrayBuffer,
        '%ArrayIteratorPrototype%': Rn ? yi([][Symbol.iterator]()) : je,
        '%AsyncFromSyncIteratorPrototype%': je,
        '%AsyncFunction%': qn,
        '%AsyncGenerator%': qn,
        '%AsyncGeneratorFunction%': qn,
        '%AsyncIteratorPrototype%': qn,
        '%Atomics%': typeof Atomics > 'u' ? je : Atomics,
        '%BigInt%': typeof BigInt > 'u' ? je : BigInt,
        '%Boolean%': Boolean,
        '%DataView%': typeof DataView > 'u' ? je : DataView,
        '%Date%': Date,
        '%decodeURI%': decodeURI,
        '%decodeURIComponent%': decodeURIComponent,
        '%encodeURI%': encodeURI,
        '%encodeURIComponent%': encodeURIComponent,
        '%Error%': Error,
        '%eval%': eval,
        '%EvalError%': EvalError,
        '%Float32Array%': typeof Float32Array > 'u' ? je : Float32Array,
        '%Float64Array%': typeof Float64Array > 'u' ? je : Float64Array,
        '%FinalizationRegistry%':
          typeof FinalizationRegistry > 'u' ? je : FinalizationRegistry,
        '%Function%': jd,
        '%GeneratorFunction%': qn,
        '%Int8Array%': typeof Int8Array > 'u' ? je : Int8Array,
        '%Int16Array%': typeof Int16Array > 'u' ? je : Int16Array,
        '%Int32Array%': typeof Int32Array > 'u' ? je : Int32Array,
        '%isFinite%': isFinite,
        '%isNaN%': isNaN,
        '%IteratorPrototype%': Rn ? yi(yi([][Symbol.iterator]())) : je,
        '%JSON%': typeof JSON == 'object' ? JSON : je,
        '%Map%': typeof Map > 'u' ? je : Map,
        '%MapIteratorPrototype%':
          typeof Map > 'u' || !Rn ? je : yi(new Map()[Symbol.iterator]()),
        '%Math%': Math,
        '%Number%': Number,
        '%Object%': Object,
        '%parseFloat%': parseFloat,
        '%parseInt%': parseInt,
        '%Promise%': typeof Promise > 'u' ? je : Promise,
        '%Proxy%': typeof Proxy > 'u' ? je : Proxy,
        '%RangeError%': RangeError,
        '%ReferenceError%': ReferenceError,
        '%Reflect%': typeof Reflect > 'u' ? je : Reflect,
        '%RegExp%': RegExp,
        '%Set%': typeof Set > 'u' ? je : Set,
        '%SetIteratorPrototype%':
          typeof Set > 'u' || !Rn ? je : yi(new Set()[Symbol.iterator]()),
        '%SharedArrayBuffer%':
          typeof SharedArrayBuffer > 'u' ? je : SharedArrayBuffer,
        '%String%': String,
        '%StringIteratorPrototype%': Rn ? yi(''[Symbol.iterator]()) : je,
        '%Symbol%': Rn ? Symbol : je,
        '%SyntaxError%': kn,
        '%ThrowTypeError%': t4,
        '%TypedArray%': r4,
        '%TypeError%': Tn,
        '%Uint8Array%': typeof Uint8Array > 'u' ? je : Uint8Array,
        '%Uint8ClampedArray%':
          typeof Uint8ClampedArray > 'u' ? je : Uint8ClampedArray,
        '%Uint16Array%': typeof Uint16Array > 'u' ? je : Uint16Array,
        '%Uint32Array%': typeof Uint32Array > 'u' ? je : Uint32Array,
        '%URIError%': URIError,
        '%WeakMap%': typeof WeakMap > 'u' ? je : WeakMap,
        '%WeakRef%': typeof WeakRef > 'u' ? je : WeakRef,
        '%WeakSet%': typeof WeakSet > 'u' ? je : WeakSet,
      },
      i4 = function t(e) {
        var r;
        if (e === '%AsyncFunction%') r = _0('async function () {}');
        else if (e === '%GeneratorFunction%') r = _0('function* () {}');
        else if (e === '%AsyncGeneratorFunction%')
          r = _0('async function* () {}');
        else if (e === '%AsyncGenerator%') {
          var n = t('%AsyncGeneratorFunction%');
          n && (r = n.prototype);
        } else if (e === '%AsyncIteratorPrototype%') {
          var i = t('%AsyncGenerator%');
          i && (r = yi(i.prototype));
        }
        return (Pn[e] = r), r;
      },
      Ld = {
        '%ArrayBufferPrototype%': ['ArrayBuffer', 'prototype'],
        '%ArrayPrototype%': ['Array', 'prototype'],
        '%ArrayProto_entries%': ['Array', 'prototype', 'entries'],
        '%ArrayProto_forEach%': ['Array', 'prototype', 'forEach'],
        '%ArrayProto_keys%': ['Array', 'prototype', 'keys'],
        '%ArrayProto_values%': ['Array', 'prototype', 'values'],
        '%AsyncFunctionPrototype%': ['AsyncFunction', 'prototype'],
        '%AsyncGenerator%': ['AsyncGeneratorFunction', 'prototype'],
        '%AsyncGeneratorPrototype%': [
          'AsyncGeneratorFunction',
          'prototype',
          'prototype',
        ],
        '%BooleanPrototype%': ['Boolean', 'prototype'],
        '%DataViewPrototype%': ['DataView', 'prototype'],
        '%DatePrototype%': ['Date', 'prototype'],
        '%ErrorPrototype%': ['Error', 'prototype'],
        '%EvalErrorPrototype%': ['EvalError', 'prototype'],
        '%Float32ArrayPrototype%': ['Float32Array', 'prototype'],
        '%Float64ArrayPrototype%': ['Float64Array', 'prototype'],
        '%FunctionPrototype%': ['Function', 'prototype'],
        '%Generator%': ['GeneratorFunction', 'prototype'],
        '%GeneratorPrototype%': ['GeneratorFunction', 'prototype', 'prototype'],
        '%Int8ArrayPrototype%': ['Int8Array', 'prototype'],
        '%Int16ArrayPrototype%': ['Int16Array', 'prototype'],
        '%Int32ArrayPrototype%': ['Int32Array', 'prototype'],
        '%JSONParse%': ['JSON', 'parse'],
        '%JSONStringify%': ['JSON', 'stringify'],
        '%MapPrototype%': ['Map', 'prototype'],
        '%NumberPrototype%': ['Number', 'prototype'],
        '%ObjectPrototype%': ['Object', 'prototype'],
        '%ObjProto_toString%': ['Object', 'prototype', 'toString'],
        '%ObjProto_valueOf%': ['Object', 'prototype', 'valueOf'],
        '%PromisePrototype%': ['Promise', 'prototype'],
        '%PromiseProto_then%': ['Promise', 'prototype', 'then'],
        '%Promise_all%': ['Promise', 'all'],
        '%Promise_reject%': ['Promise', 'reject'],
        '%Promise_resolve%': ['Promise', 'resolve'],
        '%RangeErrorPrototype%': ['RangeError', 'prototype'],
        '%ReferenceErrorPrototype%': ['ReferenceError', 'prototype'],
        '%RegExpPrototype%': ['RegExp', 'prototype'],
        '%SetPrototype%': ['Set', 'prototype'],
        '%SharedArrayBufferPrototype%': ['SharedArrayBuffer', 'prototype'],
        '%StringPrototype%': ['String', 'prototype'],
        '%SymbolPrototype%': ['Symbol', 'prototype'],
        '%SyntaxErrorPrototype%': ['SyntaxError', 'prototype'],
        '%TypedArrayPrototype%': ['TypedArray', 'prototype'],
        '%TypeErrorPrototype%': ['TypeError', 'prototype'],
        '%Uint8ArrayPrototype%': ['Uint8Array', 'prototype'],
        '%Uint8ClampedArrayPrototype%': ['Uint8ClampedArray', 'prototype'],
        '%Uint16ArrayPrototype%': ['Uint16Array', 'prototype'],
        '%Uint32ArrayPrototype%': ['Uint32Array', 'prototype'],
        '%URIErrorPrototype%': ['URIError', 'prototype'],
        '%WeakMapPrototype%': ['WeakMap', 'prototype'],
        '%WeakSetPrototype%': ['WeakSet', 'prototype'],
      },
      qf = Fa(),
      ja = Dd(),
      n4 = qf.call(Function.call, Array.prototype.concat),
      f4 = qf.call(Function.apply, Array.prototype.splice),
      Fd = qf.call(Function.call, String.prototype.replace),
      Ua = qf.call(Function.call, String.prototype.slice),
      a4 = qf.call(Function.call, RegExp.prototype.exec),
      o4 =
        /[^%.[\]]+|\[(?:(-?\d+(?:\.\d+)?)|(["'])((?:(?!\2)[^\\]|\\.)*?)\2)\]|(?=(?:\.|\[\])(?:\.|\[\]|%$))/g,
      s4 = /\\(\\)?/g,
      h4 = function (e) {
        var r = Ua(e, 0, 1),
          n = Ua(e, -1);
        if (r === '%' && n !== '%')
          throw new kn('invalid intrinsic syntax, expected closing `%`');
        if (n === '%' && r !== '%')
          throw new kn('invalid intrinsic syntax, expected opening `%`');
        var i = [];
        return (
          Fd(e, o4, function (a, h, v, g) {
            i[i.length] = v ? Fd(g, s4, '$1') : h || a;
          }),
          i
        );
      },
      u4 = function (e, r) {
        var n = e,
          i;
        if ((ja(Ld, n) && ((i = Ld[n]), (n = '%' + i[0] + '%')), ja(Pn, n))) {
          var a = Pn[n];
          if ((a === qn && (a = i4(n)), typeof a > 'u' && !r))
            throw new Tn(
              'intrinsic ' +
                e +
                ' exists, but is not available. Please file an issue!'
            );
          return { alias: i, name: n, value: a };
        }
        throw new kn('intrinsic ' + e + ' does not exist!');
      };
    Ud.exports = function (e, r) {
      if (typeof e != 'string' || e.length === 0)
        throw new Tn('intrinsic name must be a non-empty string');
      if (arguments.length > 1 && typeof r != 'boolean')
        throw new Tn('"allowMissing" argument must be a boolean');
      if (a4(/^%?[^%]*%?$/, e) === null)
        throw new kn(
          '`%` may not be present anywhere but at the beginning and end of the intrinsic name'
        );
      var n = h4(e),
        i = n.length > 0 ? n[0] : '',
        a = u4('%' + i + '%', r),
        h = a.name,
        v = a.value,
        g = !1,
        M = a.alias;
      M && ((i = M[0]), f4(n, n4([0, 1], M)));
      for (var x = 1, E = !0; x < n.length; x += 1) {
        var I = n[x],
          q = Ua(I, 0, 1),
          k = Ua(I, -1);
        if (
          (q === '"' ||
            q === "'" ||
            q === '`' ||
            k === '"' ||
            k === "'" ||
            k === '`') &&
          q !== k
        )
          throw new kn('property names with quotes must have matching quotes');
        if (
          ((I === 'constructor' || !E) && (g = !0),
          (i += '.' + I),
          (h = '%' + i + '%'),
          ja(Pn, h))
        )
          v = Pn[h];
        else if (v != null) {
          if (!(I in v)) {
            if (!r)
              throw new Tn(
                'base intrinsic for ' +
                  e +
                  ' exists, but the property is not available.'
              );
            return;
          }
          if (zi && x + 1 >= n.length) {
            var L = zi(v, I);
            (E = !!L),
              E && 'get' in L && !('originalValue' in L.get)
                ? (v = L.get)
                : (v = v[I]);
          } else (E = ja(v, I)), (v = v[I]);
          E && !g && (Pn[h] = v);
        }
      }
      return v;
    };
  });
  var Wd = R((AI, Ha) => {
    'use strict';
    S();
    var M0 = Fa(),
      On = za(),
      Kd = On('%Function.prototype.apply%'),
      Vd = On('%Function.prototype.call%'),
      Gd = On('%Reflect.apply%', !0) || M0.call(Vd, Kd),
      zd = On('%Object.getOwnPropertyDescriptor%', !0),
      Hi = On('%Object.defineProperty%', !0),
      c4 = On('%Math.max%');
    if (Hi)
      try {
        Hi({}, 'a', { value: 1 });
      } catch {
        Hi = null;
      }
    Ha.exports = function (e) {
      var r = Gd(M0, Vd, arguments);
      if (zd && Hi) {
        var n = zd(r, 'length');
        n.configurable &&
          Hi(r, 'length', {
            value: 1 + c4(0, e.length - (arguments.length - 1)),
          });
      }
      return r;
    };
    var Hd = function () {
      return Gd(M0, Kd, arguments);
    };
    Hi ? Hi(Ha.exports, 'apply', { value: Hd }) : (Ha.exports.apply = Hd);
  });
  var Ka = R((II, Jd) => {
    'use strict';
    S();
    var $d = za(),
      Zd = Wd(),
      d4 = Zd($d('String.prototype.indexOf'));
    Jd.exports = function (e, r) {
      var n = $d(e, !!r);
      return typeof n == 'function' && d4(e, '.prototype.') > -1 ? Zd(n) : n;
    };
  });
  var Qd = R((qI, Yd) => {
    'use strict';
    S();
    var l4 = Rf()(),
      p4 = Ka(),
      S0 = p4('Object.prototype.toString'),
      Va = function (e) {
        return l4 && e && typeof e == 'object' && Symbol.toStringTag in e
          ? !1
          : S0(e) === '[object Arguments]';
      },
      Xd = function (e) {
        return Va(e)
          ? !0
          : e !== null &&
              typeof e == 'object' &&
              typeof e.length == 'number' &&
              e.length >= 0 &&
              S0(e) !== '[object Array]' &&
              S0(e.callee) === '[object Function]';
      },
      v4 = (function () {
        return Va(arguments);
      })();
    Va.isLegacyArguments = Xd;
    Yd.exports = v4 ? Va : Xd;
  });
  var rl = R((PI, tl) => {
    'use strict';
    S();
    var b4 = Object.prototype.toString,
      y4 = Function.prototype.toString,
      m4 = /^\s*(?:function)?\*/,
      el = Rf()(),
      E0 = Object.getPrototypeOf,
      g4 = function () {
        if (!el) return !1;
        try {
          return Function('return function*() {}')();
        } catch {}
      },
      A0;
    tl.exports = function (e) {
      if (typeof e != 'function') return !1;
      if (m4.test(y4.call(e))) return !0;
      if (!el) {
        var r = b4.call(e);
        return r === '[object GeneratorFunction]';
      }
      if (!E0) return !1;
      if (typeof A0 > 'u') {
        var n = g4();
        A0 = n ? E0(n) : !1;
      }
      return E0(e) === A0;
    };
  });
  var al = R((OI, fl) => {
    'use strict';
    S();
    var nl = Function.prototype.toString,
      Cn = typeof Reflect == 'object' && Reflect !== null && Reflect.apply,
      I0,
      Ga;
    if (typeof Cn == 'function' && typeof Object.defineProperty == 'function')
      try {
        (I0 = Object.defineProperty({}, 'length', {
          get: function () {
            throw Ga;
          },
        })),
          (Ga = {}),
          Cn(
            function () {
              throw 42;
            },
            null,
            I0
          );
      } catch (t) {
        t !== Ga && (Cn = null);
      }
    else Cn = null;
    var w4 = /^\s*class\b/,
      R0 = function (e) {
        try {
          var r = nl.call(e);
          return w4.test(r);
        } catch {
          return !1;
        }
      },
      B0 = function (e) {
        try {
          return R0(e) ? !1 : (nl.call(e), !0);
        } catch {
          return !1;
        }
      },
      Wa = Object.prototype.toString,
      _4 = '[object Object]',
      x4 = '[object Function]',
      M4 = '[object GeneratorFunction]',
      S4 = '[object HTMLAllCollection]',
      E4 = '[object HTML document.all class]',
      A4 = '[object HTMLCollection]',
      B4 = typeof Symbol == 'function' && !!Symbol.toStringTag,
      I4 = !(0 in [,]),
      q0 = function () {
        return !1;
      };
    typeof document == 'object' &&
      ((il = document.all),
      Wa.call(il) === Wa.call(document.all) &&
        (q0 = function (e) {
          if ((I4 || !e) && (typeof e > 'u' || typeof e == 'object'))
            try {
              var r = Wa.call(e);
              return (
                (r === S4 || r === E4 || r === A4 || r === _4) && e('') == null
              );
            } catch {}
          return !1;
        }));
    var il;
    fl.exports = Cn
      ? function (e) {
          if (q0(e)) return !0;
          if (!e || (typeof e != 'function' && typeof e != 'object')) return !1;
          try {
            Cn(e, null, I0);
          } catch (r) {
            if (r !== Ga) return !1;
          }
          return !R0(e) && B0(e);
        }
      : function (e) {
          if (q0(e)) return !0;
          if (!e || (typeof e != 'function' && typeof e != 'object')) return !1;
          if (B4) return B0(e);
          if (R0(e)) return !1;
          var r = Wa.call(e);
          return r !== x4 && r !== M4 && !/^\[object HTML/.test(r) ? !1 : B0(e);
        };
  });
  var T0 = R((NI, sl) => {
    'use strict';
    S();
    var R4 = al(),
      q4 = Object.prototype.toString,
      ol = Object.prototype.hasOwnProperty,
      T4 = function (e, r, n) {
        for (var i = 0, a = e.length; i < a; i++)
          ol.call(e, i) && (n == null ? r(e[i], i, e) : r.call(n, e[i], i, e));
      },
      P4 = function (e, r, n) {
        for (var i = 0, a = e.length; i < a; i++)
          n == null ? r(e.charAt(i), i, e) : r.call(n, e.charAt(i), i, e);
      },
      k4 = function (e, r, n) {
        for (var i in e)
          ol.call(e, i) && (n == null ? r(e[i], i, e) : r.call(n, e[i], i, e));
      },
      O4 = function (e, r, n) {
        if (!R4(r)) throw new TypeError('iterator must be a function');
        var i;
        arguments.length >= 3 && (i = n),
          q4.call(e) === '[object Array]'
            ? T4(e, r, i)
            : typeof e == 'string'
              ? P4(e, r, i)
              : k4(e, r, i);
      };
    sl.exports = O4;
  });
  var k0 = R((LI, hl) => {
    'use strict';
    S();
    var P0 = [
        'BigInt64Array',
        'BigUint64Array',
        'Float32Array',
        'Float64Array',
        'Int16Array',
        'Int32Array',
        'Int8Array',
        'Uint16Array',
        'Uint32Array',
        'Uint8Array',
        'Uint8ClampedArray',
      ],
      C4 = globalThis;
    hl.exports = function () {
      for (var e = [], r = 0; r < P0.length; r++)
        typeof C4[P0[r]] == 'function' && (e[e.length] = P0[r]);
      return e;
    };
  });
  var O0 = R((jI, ul) => {
    'use strict';
    S();
    var N4 = za(),
      $a = N4('%Object.getOwnPropertyDescriptor%', !0);
    if ($a)
      try {
        $a([], 'length');
      } catch {
        $a = null;
      }
    ul.exports = $a;
  });
  var D0 = R((zI, vl) => {
    'use strict';
    S();
    var cl = T0(),
      D4 = k0(),
      N0 = Ka(),
      L4 = N0('Object.prototype.toString'),
      dl = Rf()(),
      Za = O0(),
      F4 = globalThis,
      ll = D4(),
      j4 =
        N0('Array.prototype.indexOf', !0) ||
        function (e, r) {
          for (var n = 0; n < e.length; n += 1) if (e[n] === r) return n;
          return -1;
        },
      U4 = N0('String.prototype.slice'),
      pl = {},
      C0 = Object.getPrototypeOf;
    dl &&
      Za &&
      C0 &&
      cl(ll, function (t) {
        var e = new F4[t]();
        if (Symbol.toStringTag in e) {
          var r = C0(e),
            n = Za(r, Symbol.toStringTag);
          if (!n) {
            var i = C0(r);
            n = Za(i, Symbol.toStringTag);
          }
          pl[t] = n.get;
        }
      });
    var z4 = function (e) {
      var r = !1;
      return (
        cl(pl, function (n, i) {
          if (!r)
            try {
              r = n.call(e) === i;
            } catch {}
        }),
        r
      );
    };
    vl.exports = function (e) {
      if (!e || typeof e != 'object') return !1;
      if (!dl || !(Symbol.toStringTag in e)) {
        var r = U4(L4(e), 8, -1);
        return j4(ll, r) > -1;
      }
      return Za ? z4(e) : !1;
    };
  });
  var xl = R((KI, _l) => {
    'use strict';
    S();
    var yl = T0(),
      H4 = k0(),
      ml = Ka(),
      L0 = O0(),
      K4 = ml('Object.prototype.toString'),
      gl = Rf()(),
      bl = globalThis,
      V4 = H4(),
      G4 = ml('String.prototype.slice'),
      wl = {},
      F0 = Object.getPrototypeOf;
    gl &&
      L0 &&
      F0 &&
      yl(V4, function (t) {
        if (typeof bl[t] == 'function') {
          var e = new bl[t]();
          if (Symbol.toStringTag in e) {
            var r = F0(e),
              n = L0(r, Symbol.toStringTag);
            if (!n) {
              var i = F0(r);
              n = L0(i, Symbol.toStringTag);
            }
            wl[t] = n.get;
          }
        }
      });
    var W4 = function (e) {
        var r = !1;
        return (
          yl(wl, function (n, i) {
            if (!r)
              try {
                var a = n.call(e);
                a === i && (r = a);
              } catch {}
          }),
          r
        );
      },
      $4 = D0();
    _l.exports = function (e) {
      return $4(e)
        ? !gl || !(Symbol.toStringTag in e)
          ? G4(K4(e), 8, -1)
          : W4(e)
        : !1;
    };
  });
  var Nl = R((Ce) => {
    'use strict';
    S();
    var Z4 = Qd(),
      J4 = rl(),
      ur = xl(),
      Ml = D0();
    function Nn(t) {
      return t.call.bind(t);
    }
    var Sl = typeof BigInt < 'u',
      El = typeof Symbol < 'u',
      Tt = Nn(Object.prototype.toString),
      X4 = Nn(Number.prototype.valueOf),
      Y4 = Nn(String.prototype.valueOf),
      Q4 = Nn(Boolean.prototype.valueOf);
    Sl && (Al = Nn(BigInt.prototype.valueOf));
    var Al;
    El && (Bl = Nn(Symbol.prototype.valueOf));
    var Bl;
    function Pf(t, e) {
      if (typeof t != 'object') return !1;
      try {
        return e(t), !0;
      } catch {
        return !1;
      }
    }
    Ce.isArgumentsObject = Z4;
    Ce.isGeneratorFunction = J4;
    Ce.isTypedArray = Ml;
    function eg(t) {
      return (
        (typeof Promise < 'u' && t instanceof Promise) ||
        (t !== null &&
          typeof t == 'object' &&
          typeof t.then == 'function' &&
          typeof t.catch == 'function')
      );
    }
    Ce.isPromise = eg;
    function tg(t) {
      return typeof ArrayBuffer < 'u' && ArrayBuffer.isView
        ? ArrayBuffer.isView(t)
        : Ml(t) || Rl(t);
    }
    Ce.isArrayBufferView = tg;
    function rg(t) {
      return ur(t) === 'Uint8Array';
    }
    Ce.isUint8Array = rg;
    function ig(t) {
      return ur(t) === 'Uint8ClampedArray';
    }
    Ce.isUint8ClampedArray = ig;
    function ng(t) {
      return ur(t) === 'Uint16Array';
    }
    Ce.isUint16Array = ng;
    function fg(t) {
      return ur(t) === 'Uint32Array';
    }
    Ce.isUint32Array = fg;
    function ag(t) {
      return ur(t) === 'Int8Array';
    }
    Ce.isInt8Array = ag;
    function og(t) {
      return ur(t) === 'Int16Array';
    }
    Ce.isInt16Array = og;
    function sg(t) {
      return ur(t) === 'Int32Array';
    }
    Ce.isInt32Array = sg;
    function hg(t) {
      return ur(t) === 'Float32Array';
    }
    Ce.isFloat32Array = hg;
    function ug(t) {
      return ur(t) === 'Float64Array';
    }
    Ce.isFloat64Array = ug;
    function cg(t) {
      return ur(t) === 'BigInt64Array';
    }
    Ce.isBigInt64Array = cg;
    function dg(t) {
      return ur(t) === 'BigUint64Array';
    }
    Ce.isBigUint64Array = dg;
    function Ja(t) {
      return Tt(t) === '[object Map]';
    }
    Ja.working = typeof Map < 'u' && Ja(new Map());
    function lg(t) {
      return typeof Map > 'u' ? !1 : Ja.working ? Ja(t) : t instanceof Map;
    }
    Ce.isMap = lg;
    function Xa(t) {
      return Tt(t) === '[object Set]';
    }
    Xa.working = typeof Set < 'u' && Xa(new Set());
    function pg(t) {
      return typeof Set > 'u' ? !1 : Xa.working ? Xa(t) : t instanceof Set;
    }
    Ce.isSet = pg;
    function Ya(t) {
      return Tt(t) === '[object WeakMap]';
    }
    Ya.working = typeof WeakMap < 'u' && Ya(new WeakMap());
    function vg(t) {
      return typeof WeakMap > 'u'
        ? !1
        : Ya.working
          ? Ya(t)
          : t instanceof WeakMap;
    }
    Ce.isWeakMap = vg;
    function U0(t) {
      return Tt(t) === '[object WeakSet]';
    }
    U0.working = typeof WeakSet < 'u' && U0(new WeakSet());
    function bg(t) {
      return U0(t);
    }
    Ce.isWeakSet = bg;
    function Qa(t) {
      return Tt(t) === '[object ArrayBuffer]';
    }
    Qa.working = typeof ArrayBuffer < 'u' && Qa(new ArrayBuffer());
    function Il(t) {
      return typeof ArrayBuffer > 'u'
        ? !1
        : Qa.working
          ? Qa(t)
          : t instanceof ArrayBuffer;
    }
    Ce.isArrayBuffer = Il;
    function eo(t) {
      return Tt(t) === '[object DataView]';
    }
    eo.working =
      typeof ArrayBuffer < 'u' &&
      typeof DataView < 'u' &&
      eo(new DataView(new ArrayBuffer(1), 0, 1));
    function Rl(t) {
      return typeof DataView > 'u'
        ? !1
        : eo.working
          ? eo(t)
          : t instanceof DataView;
    }
    Ce.isDataView = Rl;
    var j0 = typeof SharedArrayBuffer < 'u' ? SharedArrayBuffer : void 0;
    function Tf(t) {
      return Tt(t) === '[object SharedArrayBuffer]';
    }
    function ql(t) {
      return typeof j0 > 'u'
        ? !1
        : (typeof Tf.working > 'u' && (Tf.working = Tf(new j0())),
          Tf.working ? Tf(t) : t instanceof j0);
    }
    Ce.isSharedArrayBuffer = ql;
    function yg(t) {
      return Tt(t) === '[object AsyncFunction]';
    }
    Ce.isAsyncFunction = yg;
    function mg(t) {
      return Tt(t) === '[object Map Iterator]';
    }
    Ce.isMapIterator = mg;
    function gg(t) {
      return Tt(t) === '[object Set Iterator]';
    }
    Ce.isSetIterator = gg;
    function wg(t) {
      return Tt(t) === '[object Generator]';
    }
    Ce.isGeneratorObject = wg;
    function _g(t) {
      return Tt(t) === '[object WebAssembly.Module]';
    }
    Ce.isWebAssemblyCompiledModule = _g;
    function Tl(t) {
      return Pf(t, X4);
    }
    Ce.isNumberObject = Tl;
    function Pl(t) {
      return Pf(t, Y4);
    }
    Ce.isStringObject = Pl;
    function kl(t) {
      return Pf(t, Q4);
    }
    Ce.isBooleanObject = kl;
    function Ol(t) {
      return Sl && Pf(t, Al);
    }
    Ce.isBigIntObject = Ol;
    function Cl(t) {
      return El && Pf(t, Bl);
    }
    Ce.isSymbolObject = Cl;
    function xg(t) {
      return Tl(t) || Pl(t) || kl(t) || Ol(t) || Cl(t);
    }
    Ce.isBoxedPrimitive = xg;
    function Mg(t) {
      return typeof Uint8Array < 'u' && (Il(t) || ql(t));
    }
    Ce.isAnyArrayBuffer = Mg;
    ['isProxy', 'isExternal', 'isModuleNamespaceObject'].forEach(function (t) {
      Object.defineProperty(Ce, t, {
        enumerable: !1,
        value: function () {
          throw new Error(t + ' is not supported in userland');
        },
      });
    });
  });
  var Ll = R(($I, Dl) => {
    S();
    Dl.exports = function (e) {
      return (
        e &&
        typeof e == 'object' &&
        typeof e.copy == 'function' &&
        typeof e.fill == 'function' &&
        typeof e.readUInt8 == 'function'
      );
    };
  });
  var Gi = R((Ne) => {
    S();
    var Fl =
        Object.getOwnPropertyDescriptors ||
        function (e) {
          for (var r = Object.keys(e), n = {}, i = 0; i < r.length; i++)
            n[r[i]] = Object.getOwnPropertyDescriptor(e, r[i]);
          return n;
        },
      Sg = /%[sdj%]/g;
    Ne.format = function (t) {
      if (!oo(t)) {
        for (var e = [], r = 0; r < arguments.length; r++)
          e.push(mi(arguments[r]));
        return e.join(' ');
      }
      for (
        var r = 1,
          n = arguments,
          i = n.length,
          a = String(t).replace(Sg, function (v) {
            if (v === '%%') return '%';
            if (r >= i) return v;
            switch (v) {
              case '%s':
                return String(n[r++]);
              case '%d':
                return Number(n[r++]);
              case '%j':
                try {
                  return JSON.stringify(n[r++]);
                } catch {
                  return '[Circular]';
                }
              default:
                return v;
            }
          }),
          h = n[r];
        r < i;
        h = n[++r]
      )
        ao(h) || !Dn(h) ? (a += ' ' + h) : (a += ' ' + mi(h));
      return a;
    };
    Ne.deprecate = function (t, e) {
      if (typeof A < 'u' && A.noDeprecation === !0) return t;
      if (typeof A > 'u')
        return function () {
          return Ne.deprecate(t, e).apply(this, arguments);
        };
      var r = !1;
      function n() {
        if (!r) {
          if (A.throwDeprecation) throw new Error(e);
          A.traceDeprecation ? console.trace(e) : console.error(e), (r = !0);
        }
        return t.apply(this, arguments);
      }
      return n;
    };
    var to = {},
      jl = /^$/;
    A.env.NODE_DEBUG &&
      ((ro = A.env.NODE_DEBUG),
      (ro = ro
        .replace(/[|\\{}()[\]^$+?.]/g, '\\$&')
        .replace(/\*/g, '.*')
        .replace(/,/g, '$|^')
        .toUpperCase()),
      (jl = new RegExp('^' + ro + '$', 'i')));
    var ro;
    Ne.debuglog = function (t) {
      if (((t = t.toUpperCase()), !to[t]))
        if (jl.test(t)) {
          var e = A.pid;
          to[t] = function () {
            var r = Ne.format.apply(Ne, arguments);
            console.error('%s %d: %s', t, e, r);
          };
        } else to[t] = function () {};
      return to[t];
    };
    function mi(t, e) {
      var r = { seen: [], stylize: Ag };
      return (
        arguments.length >= 3 && (r.depth = arguments[2]),
        arguments.length >= 4 && (r.colors = arguments[3]),
        V0(e) ? (r.showHidden = e) : e && Ne._extend(r, e),
        Vi(r.showHidden) && (r.showHidden = !1),
        Vi(r.depth) && (r.depth = 2),
        Vi(r.colors) && (r.colors = !1),
        Vi(r.customInspect) && (r.customInspect = !0),
        r.colors && (r.stylize = Eg),
        no(r, t, r.depth)
      );
    }
    Ne.inspect = mi;
    mi.colors = {
      bold: [1, 22],
      italic: [3, 23],
      underline: [4, 24],
      inverse: [7, 27],
      white: [37, 39],
      grey: [90, 39],
      black: [30, 39],
      blue: [34, 39],
      cyan: [36, 39],
      green: [32, 39],
      magenta: [35, 39],
      red: [31, 39],
      yellow: [33, 39],
    };
    mi.styles = {
      special: 'cyan',
      number: 'yellow',
      boolean: 'yellow',
      undefined: 'grey',
      null: 'bold',
      string: 'green',
      date: 'magenta',
      regexp: 'red',
    };
    function Eg(t, e) {
      var r = mi.styles[e];
      return r
        ? '\x1B[' + mi.colors[r][0] + 'm' + t + '\x1B[' + mi.colors[r][1] + 'm'
        : t;
    }
    function Ag(t, e) {
      return t;
    }
    function Bg(t) {
      var e = {};
      return (
        t.forEach(function (r, n) {
          e[r] = !0;
        }),
        e
      );
    }
    function no(t, e, r) {
      if (
        t.customInspect &&
        e &&
        io(e.inspect) &&
        e.inspect !== Ne.inspect &&
        !(e.constructor && e.constructor.prototype === e)
      ) {
        var n = e.inspect(r, t);
        return oo(n) || (n = no(t, n, r)), n;
      }
      var i = Ig(t, e);
      if (i) return i;
      var a = Object.keys(e),
        h = Bg(a);
      if (
        (t.showHidden && (a = Object.getOwnPropertyNames(e)),
        Of(e) && (a.indexOf('message') >= 0 || a.indexOf('description') >= 0))
      )
        return z0(e);
      if (a.length === 0) {
        if (io(e)) {
          var v = e.name ? ': ' + e.name : '';
          return t.stylize('[Function' + v + ']', 'special');
        }
        if (kf(e))
          return t.stylize(RegExp.prototype.toString.call(e), 'regexp');
        if (fo(e)) return t.stylize(Date.prototype.toString.call(e), 'date');
        if (Of(e)) return z0(e);
      }
      var g = '',
        M = !1,
        x = ['{', '}'];
      if ((Ul(e) && ((M = !0), (x = ['[', ']'])), io(e))) {
        var E = e.name ? ': ' + e.name : '';
        g = ' [Function' + E + ']';
      }
      if (
        (kf(e) && (g = ' ' + RegExp.prototype.toString.call(e)),
        fo(e) && (g = ' ' + Date.prototype.toUTCString.call(e)),
        Of(e) && (g = ' ' + z0(e)),
        a.length === 0 && (!M || e.length == 0))
      )
        return x[0] + g + x[1];
      if (r < 0)
        return kf(e)
          ? t.stylize(RegExp.prototype.toString.call(e), 'regexp')
          : t.stylize('[Object]', 'special');
      t.seen.push(e);
      var I;
      return (
        M
          ? (I = Rg(t, e, r, h, a))
          : (I = a.map(function (q) {
              return K0(t, e, r, h, q, M);
            })),
        t.seen.pop(),
        qg(I, g, x)
      );
    }
    function Ig(t, e) {
      if (Vi(e)) return t.stylize('undefined', 'undefined');
      if (oo(e)) {
        var r =
          "'" +
          JSON.stringify(e)
            .replace(/^"|"$/g, '')
            .replace(/'/g, "\\'")
            .replace(/\\"/g, '"') +
          "'";
        return t.stylize(r, 'string');
      }
      if (zl(e)) return t.stylize('' + e, 'number');
      if (V0(e)) return t.stylize('' + e, 'boolean');
      if (ao(e)) return t.stylize('null', 'null');
    }
    function z0(t) {
      return '[' + Error.prototype.toString.call(t) + ']';
    }
    function Rg(t, e, r, n, i) {
      for (var a = [], h = 0, v = e.length; h < v; ++h)
        Hl(e, String(h)) ? a.push(K0(t, e, r, n, String(h), !0)) : a.push('');
      return (
        i.forEach(function (g) {
          g.match(/^\d+$/) || a.push(K0(t, e, r, n, g, !0));
        }),
        a
      );
    }
    function K0(t, e, r, n, i, a) {
      var h, v, g;
      if (
        ((g = Object.getOwnPropertyDescriptor(e, i) || { value: e[i] }),
        g.get
          ? g.set
            ? (v = t.stylize('[Getter/Setter]', 'special'))
            : (v = t.stylize('[Getter]', 'special'))
          : g.set && (v = t.stylize('[Setter]', 'special')),
        Hl(n, i) || (h = '[' + i + ']'),
        v ||
          (t.seen.indexOf(g.value) < 0
            ? (ao(r) ? (v = no(t, g.value, null)) : (v = no(t, g.value, r - 1)),
              v.indexOf(`
`) > -1 &&
                (a
                  ? (v = v
                      .split(
                        `
`
                      )
                      .map(function (M) {
                        return '  ' + M;
                      })
                      .join(
                        `
`
                      )
                      .slice(2))
                  : (v =
                      `
` +
                      v
                        .split(
                          `
`
                        )
                        .map(function (M) {
                          return '   ' + M;
                        }).join(`
`))))
            : (v = t.stylize('[Circular]', 'special'))),
        Vi(h))
      ) {
        if (a && i.match(/^\d+$/)) return v;
        (h = JSON.stringify('' + i)),
          h.match(/^"([a-zA-Z_][a-zA-Z_0-9]*)"$/)
            ? ((h = h.slice(1, -1)), (h = t.stylize(h, 'name')))
            : ((h = h
                .replace(/'/g, "\\'")
                .replace(/\\"/g, '"')
                .replace(/(^"|"$)/g, "'")),
              (h = t.stylize(h, 'string')));
      }
      return h + ': ' + v;
    }
    function qg(t, e, r) {
      var n = 0,
        i = t.reduce(function (a, h) {
          return (
            n++,
            h.indexOf(`
`) >= 0 && n++,
            a + h.replace(/\u001b\[\d\d?m/g, '').length + 1
          );
        }, 0);
      return i > 60
        ? r[0] +
            (e === ''
              ? ''
              : e +
                `
 `) +
            ' ' +
            t.join(`,
  `) +
            ' ' +
            r[1]
        : r[0] + e + ' ' + t.join(', ') + ' ' + r[1];
    }
    Ne.types = Nl();
    function Ul(t) {
      return Array.isArray(t);
    }
    Ne.isArray = Ul;
    function V0(t) {
      return typeof t == 'boolean';
    }
    Ne.isBoolean = V0;
    function ao(t) {
      return t === null;
    }
    Ne.isNull = ao;
    function Tg(t) {
      return t == null;
    }
    Ne.isNullOrUndefined = Tg;
    function zl(t) {
      return typeof t == 'number';
    }
    Ne.isNumber = zl;
    function oo(t) {
      return typeof t == 'string';
    }
    Ne.isString = oo;
    function Pg(t) {
      return typeof t == 'symbol';
    }
    Ne.isSymbol = Pg;
    function Vi(t) {
      return t === void 0;
    }
    Ne.isUndefined = Vi;
    function kf(t) {
      return Dn(t) && G0(t) === '[object RegExp]';
    }
    Ne.isRegExp = kf;
    Ne.types.isRegExp = kf;
    function Dn(t) {
      return typeof t == 'object' && t !== null;
    }
    Ne.isObject = Dn;
    function fo(t) {
      return Dn(t) && G0(t) === '[object Date]';
    }
    Ne.isDate = fo;
    Ne.types.isDate = fo;
    function Of(t) {
      return Dn(t) && (G0(t) === '[object Error]' || t instanceof Error);
    }
    Ne.isError = Of;
    Ne.types.isNativeError = Of;
    function io(t) {
      return typeof t == 'function';
    }
    Ne.isFunction = io;
    function kg(t) {
      return (
        t === null ||
        typeof t == 'boolean' ||
        typeof t == 'number' ||
        typeof t == 'string' ||
        typeof t == 'symbol' ||
        typeof t > 'u'
      );
    }
    Ne.isPrimitive = kg;
    Ne.isBuffer = Ll();
    function G0(t) {
      return Object.prototype.toString.call(t);
    }
    function H0(t) {
      return t < 10 ? '0' + t.toString(10) : t.toString(10);
    }
    var Og = [
      'Jan',
      'Feb',
      'Mar',
      'Apr',
      'May',
      'Jun',
      'Jul',
      'Aug',
      'Sep',
      'Oct',
      'Nov',
      'Dec',
    ];
    function Cg() {
      var t = new Date(),
        e = [H0(t.getHours()), H0(t.getMinutes()), H0(t.getSeconds())].join(
          ':'
        );
      return [t.getDate(), Og[t.getMonth()], e].join(' ');
    }
    Ne.log = function () {
      console.log('%s - %s', Cg(), Ne.format.apply(Ne, arguments));
    };
    Ne.inherits = qe();
    Ne._extend = function (t, e) {
      if (!e || !Dn(e)) return t;
      for (var r = Object.keys(e), n = r.length; n--; ) t[r[n]] = e[r[n]];
      return t;
    };
    function Hl(t, e) {
      return Object.prototype.hasOwnProperty.call(t, e);
    }
    var Ki = typeof Symbol < 'u' ? Symbol('util.promisify.custom') : void 0;
    Ne.promisify = function (e) {
      if (typeof e != 'function')
        throw new TypeError('The "original" argument must be of type Function');
      if (Ki && e[Ki]) {
        var r = e[Ki];
        if (typeof r != 'function')
          throw new TypeError(
            'The "util.promisify.custom" argument must be of type Function'
          );
        return (
          Object.defineProperty(r, Ki, {
            value: r,
            enumerable: !1,
            writable: !1,
            configurable: !0,
          }),
          r
        );
      }
      function r() {
        for (
          var n,
            i,
            a = new Promise(function (g, M) {
              (n = g), (i = M);
            }),
            h = [],
            v = 0;
          v < arguments.length;
          v++
        )
          h.push(arguments[v]);
        h.push(function (g, M) {
          g ? i(g) : n(M);
        });
        try {
          e.apply(this, h);
        } catch (g) {
          i(g);
        }
        return a;
      }
      return (
        Object.setPrototypeOf(r, Object.getPrototypeOf(e)),
        Ki &&
          Object.defineProperty(r, Ki, {
            value: r,
            enumerable: !1,
            writable: !1,
            configurable: !0,
          }),
        Object.defineProperties(r, Fl(e))
      );
    };
    Ne.promisify.custom = Ki;
    function Ng(t, e) {
      if (!t) {
        var r = new Error('Promise was rejected with a falsy value');
        (r.reason = t), (t = r);
      }
      return e(t);
    }
    function Dg(t) {
      if (typeof t != 'function')
        throw new TypeError('The "original" argument must be of type Function');
      function e() {
        for (var r = [], n = 0; n < arguments.length; n++) r.push(arguments[n]);
        var i = r.pop();
        if (typeof i != 'function')
          throw new TypeError('The last argument must be of type Function');
        var a = this,
          h = function () {
            return i.apply(a, arguments);
          };
        t.apply(this, r).then(
          function (v) {
            A.nextTick(h.bind(null, null, v));
          },
          function (v) {
            A.nextTick(Ng.bind(null, v, h));
          }
        );
      }
      return (
        Object.setPrototypeOf(e, Object.getPrototypeOf(t)),
        Object.defineProperties(e, Fl(t)),
        e
      );
    }
    Ne.callbackify = Dg;
  });
  var Wl = R((YI, Gl) => {
    'use strict';
    S();
    function Kl(t, e) {
      var r = Object.keys(t);
      if (Object.getOwnPropertySymbols) {
        var n = Object.getOwnPropertySymbols(t);
        e &&
          (n = n.filter(function (i) {
            return Object.getOwnPropertyDescriptor(t, i).enumerable;
          })),
          r.push.apply(r, n);
      }
      return r;
    }
    function Lg(t) {
      for (var e = 1; e < arguments.length; e++) {
        var r = arguments[e] != null ? arguments[e] : {};
        e % 2
          ? Kl(Object(r), !0).forEach(function (n) {
              Fg(t, n, r[n]);
            })
          : Object.getOwnPropertyDescriptors
            ? Object.defineProperties(t, Object.getOwnPropertyDescriptors(r))
            : Kl(Object(r)).forEach(function (n) {
                Object.defineProperty(
                  t,
                  n,
                  Object.getOwnPropertyDescriptor(r, n)
                );
              });
      }
      return t;
    }
    function Fg(t, e, r) {
      return (
        e in t
          ? Object.defineProperty(t, e, {
              value: r,
              enumerable: !0,
              configurable: !0,
              writable: !0,
            })
          : (t[e] = r),
        t
      );
    }
    function jg(t, e) {
      if (!(t instanceof e))
        throw new TypeError('Cannot call a class as a function');
    }
    function Vl(t, e) {
      for (var r = 0; r < e.length; r++) {
        var n = e[r];
        (n.enumerable = n.enumerable || !1),
          (n.configurable = !0),
          'value' in n && (n.writable = !0),
          Object.defineProperty(t, n.key, n);
      }
    }
    function Ug(t, e, r) {
      return e && Vl(t.prototype, e), r && Vl(t, r), t;
    }
    var zg = Et(),
      so = zg.Buffer,
      Hg = Gi(),
      W0 = Hg.inspect,
      Kg = (W0 && W0.custom) || 'inspect';
    function Vg(t, e, r) {
      so.prototype.copy.call(t, e, r);
    }
    Gl.exports = (function () {
      function t() {
        jg(this, t), (this.head = null), (this.tail = null), (this.length = 0);
      }
      return (
        Ug(t, [
          {
            key: 'push',
            value: function (r) {
              var n = { data: r, next: null };
              this.length > 0 ? (this.tail.next = n) : (this.head = n),
                (this.tail = n),
                ++this.length;
            },
          },
          {
            key: 'unshift',
            value: function (r) {
              var n = { data: r, next: this.head };
              this.length === 0 && (this.tail = n),
                (this.head = n),
                ++this.length;
            },
          },
          {
            key: 'shift',
            value: function () {
              if (this.length !== 0) {
                var r = this.head.data;
                return (
                  this.length === 1
                    ? (this.head = this.tail = null)
                    : (this.head = this.head.next),
                  --this.length,
                  r
                );
              }
            },
          },
          {
            key: 'clear',
            value: function () {
              (this.head = this.tail = null), (this.length = 0);
            },
          },
          {
            key: 'join',
            value: function (r) {
              if (this.length === 0) return '';
              for (var n = this.head, i = '' + n.data; (n = n.next); )
                i += r + n.data;
              return i;
            },
          },
          {
            key: 'concat',
            value: function (r) {
              if (this.length === 0) return so.alloc(0);
              for (var n = so.allocUnsafe(r >>> 0), i = this.head, a = 0; i; )
                Vg(i.data, n, a), (a += i.data.length), (i = i.next);
              return n;
            },
          },
          {
            key: 'consume',
            value: function (r, n) {
              var i;
              return (
                r < this.head.data.length
                  ? ((i = this.head.data.slice(0, r)),
                    (this.head.data = this.head.data.slice(r)))
                  : r === this.head.data.length
                    ? (i = this.shift())
                    : (i = n ? this._getString(r) : this._getBuffer(r)),
                i
              );
            },
          },
          {
            key: 'first',
            value: function () {
              return this.head.data;
            },
          },
          {
            key: '_getString',
            value: function (r) {
              var n = this.head,
                i = 1,
                a = n.data;
              for (r -= a.length; (n = n.next); ) {
                var h = n.data,
                  v = r > h.length ? h.length : r;
                if (
                  (v === h.length ? (a += h) : (a += h.slice(0, r)),
                  (r -= v),
                  r === 0)
                ) {
                  v === h.length
                    ? (++i,
                      n.next
                        ? (this.head = n.next)
                        : (this.head = this.tail = null))
                    : ((this.head = n), (n.data = h.slice(v)));
                  break;
                }
                ++i;
              }
              return (this.length -= i), a;
            },
          },
          {
            key: '_getBuffer',
            value: function (r) {
              var n = so.allocUnsafe(r),
                i = this.head,
                a = 1;
              for (i.data.copy(n), r -= i.data.length; (i = i.next); ) {
                var h = i.data,
                  v = r > h.length ? h.length : r;
                if ((h.copy(n, n.length - r, 0, v), (r -= v), r === 0)) {
                  v === h.length
                    ? (++a,
                      i.next
                        ? (this.head = i.next)
                        : (this.head = this.tail = null))
                    : ((this.head = i), (i.data = h.slice(v)));
                  break;
                }
                ++a;
              }
              return (this.length -= a), n;
            },
          },
          {
            key: Kg,
            value: function (r, n) {
              return W0(this, Lg({}, n, { depth: 0, customInspect: !1 }));
            },
          },
        ]),
        t
      );
    })();
  });
  var Z0 = R((eR, Zl) => {
    'use strict';
    S();
    function Gg(t, e) {
      var r = this,
        n = this._readableState && this._readableState.destroyed,
        i = this._writableState && this._writableState.destroyed;
      return n || i
        ? (e
            ? e(t)
            : t &&
              (this._writableState
                ? this._writableState.errorEmitted ||
                  ((this._writableState.errorEmitted = !0),
                  A.nextTick($0, this, t))
                : A.nextTick($0, this, t)),
          this)
        : (this._readableState && (this._readableState.destroyed = !0),
          this._writableState && (this._writableState.destroyed = !0),
          this._destroy(t || null, function (a) {
            !e && a
              ? r._writableState
                ? r._writableState.errorEmitted
                  ? A.nextTick(ho, r)
                  : ((r._writableState.errorEmitted = !0), A.nextTick($l, r, a))
                : A.nextTick($l, r, a)
              : e
                ? (A.nextTick(ho, r), e(a))
                : A.nextTick(ho, r);
          }),
          this);
    }
    function $l(t, e) {
      $0(t, e), ho(t);
    }
    function ho(t) {
      (t._writableState && !t._writableState.emitClose) ||
        (t._readableState && !t._readableState.emitClose) ||
        t.emit('close');
    }
    function Wg() {
      this._readableState &&
        ((this._readableState.destroyed = !1),
        (this._readableState.reading = !1),
        (this._readableState.ended = !1),
        (this._readableState.endEmitted = !1)),
        this._writableState &&
          ((this._writableState.destroyed = !1),
          (this._writableState.ended = !1),
          (this._writableState.ending = !1),
          (this._writableState.finalCalled = !1),
          (this._writableState.prefinished = !1),
          (this._writableState.finished = !1),
          (this._writableState.errorEmitted = !1));
    }
    function $0(t, e) {
      t.emit('error', e);
    }
    function $g(t, e) {
      var r = t._readableState,
        n = t._writableState;
      (r && r.autoDestroy) || (n && n.autoDestroy)
        ? t.destroy(e)
        : t.emit('error', e);
    }
    Zl.exports = { destroy: Gg, undestroy: Wg, errorOrDestroy: $g };
  });
  var Wi = R((rR, Yl) => {
    'use strict';
    S();
    function Zg(t, e) {
      (t.prototype = Object.create(e.prototype)),
        (t.prototype.constructor = t),
        (t.__proto__ = e);
    }
    var Xl = {};
    function Pt(t, e, r) {
      r || (r = Error);
      function n(a, h, v) {
        return typeof e == 'string' ? e : e(a, h, v);
      }
      var i = (function (a) {
        Zg(h, a);
        function h(v, g, M) {
          return a.call(this, n(v, g, M)) || this;
        }
        return h;
      })(r);
      (i.prototype.name = r.name), (i.prototype.code = t), (Xl[t] = i);
    }
    function Jl(t, e) {
      if (Array.isArray(t)) {
        var r = t.length;
        return (
          (t = t.map(function (n) {
            return String(n);
          })),
          r > 2
            ? 'one of '
                .concat(e, ' ')
                .concat(t.slice(0, r - 1).join(', '), ', or ') + t[r - 1]
            : r === 2
              ? 'one of '.concat(e, ' ').concat(t[0], ' or ').concat(t[1])
              : 'of '.concat(e, ' ').concat(t[0])
        );
      } else return 'of '.concat(e, ' ').concat(String(t));
    }
    function Jg(t, e, r) {
      return t.substr(!r || r < 0 ? 0 : +r, e.length) === e;
    }
    function Xg(t, e, r) {
      return (
        (r === void 0 || r > t.length) && (r = t.length),
        t.substring(r - e.length, r) === e
      );
    }
    function Yg(t, e, r) {
      return (
        typeof r != 'number' && (r = 0),
        r + e.length > t.length ? !1 : t.indexOf(e, r) !== -1
      );
    }
    Pt(
      'ERR_INVALID_OPT_VALUE',
      function (t, e) {
        return 'The value "' + e + '" is invalid for option "' + t + '"';
      },
      TypeError
    );
    Pt(
      'ERR_INVALID_ARG_TYPE',
      function (t, e, r) {
        var n;
        typeof e == 'string' && Jg(e, 'not ')
          ? ((n = 'must not be'), (e = e.replace(/^not /, '')))
          : (n = 'must be');
        var i;
        if (Xg(t, ' argument'))
          i = 'The '.concat(t, ' ').concat(n, ' ').concat(Jl(e, 'type'));
        else {
          var a = Yg(t, '.') ? 'property' : 'argument';
          i = 'The "'
            .concat(t, '" ')
            .concat(a, ' ')
            .concat(n, ' ')
            .concat(Jl(e, 'type'));
        }
        return (i += '. Received type '.concat(typeof r)), i;
      },
      TypeError
    );
    Pt('ERR_STREAM_PUSH_AFTER_EOF', 'stream.push() after EOF');
    Pt('ERR_METHOD_NOT_IMPLEMENTED', function (t) {
      return 'The ' + t + ' method is not implemented';
    });
    Pt('ERR_STREAM_PREMATURE_CLOSE', 'Premature close');
    Pt('ERR_STREAM_DESTROYED', function (t) {
      return 'Cannot call ' + t + ' after a stream was destroyed';
    });
    Pt('ERR_MULTIPLE_CALLBACK', 'Callback called multiple times');
    Pt('ERR_STREAM_CANNOT_PIPE', 'Cannot pipe, not readable');
    Pt('ERR_STREAM_WRITE_AFTER_END', 'write after end');
    Pt(
      'ERR_STREAM_NULL_VALUES',
      'May not write null values to stream',
      TypeError
    );
    Pt(
      'ERR_UNKNOWN_ENCODING',
      function (t) {
        return 'Unknown encoding: ' + t;
      },
      TypeError
    );
    Pt(
      'ERR_STREAM_UNSHIFT_AFTER_END_EVENT',
      'stream.unshift() after end event'
    );
    Yl.exports.codes = Xl;
  });
  var J0 = R((nR, Ql) => {
    'use strict';
    S();
    var Qg = Wi().codes.ERR_INVALID_OPT_VALUE;
    function e8(t, e, r) {
      return t.highWaterMark != null ? t.highWaterMark : e ? t[r] : null;
    }
    function t8(t, e, r, n) {
      var i = e8(e, n, r);
      if (i != null) {
        if (!(isFinite(i) && Math.floor(i) === i) || i < 0) {
          var a = n ? r : 'highWaterMark';
          throw new Qg(a, i);
        }
        return Math.floor(i);
      }
      return t.objectMode ? 16 : 16 * 1024;
    }
    Ql.exports = { getHighWaterMark: t8 };
  });
  var t1 = R((aR, e1) => {
    S();
    e1.exports = r8;
    function r8(t, e) {
      if (X0('noDeprecation')) return t;
      var r = !1;
      function n() {
        if (!r) {
          if (X0('throwDeprecation')) throw new Error(e);
          X0('traceDeprecation') ? console.trace(e) : console.warn(e), (r = !0);
        }
        return t.apply(this, arguments);
      }
      return n;
    }
    function X0(t) {
      try {
        if (!globalThis.localStorage) return !1;
      } catch {
        return !1;
      }
      var e = globalThis.localStorage[t];
      return e == null ? !1 : String(e).toLowerCase() === 'true';
    }
  });
  var lo = R((sR, o1) => {
    'use strict';
    S();
    o1.exports = Xe;
    function i1(t) {
      var e = this;
      (this.next = null),
        (this.entry = null),
        (this.finish = function () {
          R8(e, t);
        });
    }
    var Ln;
    Xe.WritableState = Nf;
    var i8 = { deprecate: t1() },
      n1 = m0(),
      co = Et().Buffer,
      n8 = globalThis.Uint8Array || function () {};
    function f8(t) {
      return co.from(t);
    }
    function a8(t) {
      return co.isBuffer(t) || t instanceof n8;
    }
    var Q0 = Z0(),
      o8 = J0(),
      s8 = o8.getHighWaterMark,
      gi = Wi().codes,
      h8 = gi.ERR_INVALID_ARG_TYPE,
      u8 = gi.ERR_METHOD_NOT_IMPLEMENTED,
      c8 = gi.ERR_MULTIPLE_CALLBACK,
      d8 = gi.ERR_STREAM_CANNOT_PIPE,
      l8 = gi.ERR_STREAM_DESTROYED,
      p8 = gi.ERR_STREAM_NULL_VALUES,
      v8 = gi.ERR_STREAM_WRITE_AFTER_END,
      b8 = gi.ERR_UNKNOWN_ENCODING,
      Fn = Q0.errorOrDestroy;
    qe()(Xe, n1);
    function y8() {}
    function Nf(t, e, r) {
      (Ln = Ln || wi()),
        (t = t || {}),
        typeof r != 'boolean' && (r = e instanceof Ln),
        (this.objectMode = !!t.objectMode),
        r && (this.objectMode = this.objectMode || !!t.writableObjectMode),
        (this.highWaterMark = s8(this, t, 'writableHighWaterMark', r)),
        (this.finalCalled = !1),
        (this.needDrain = !1),
        (this.ending = !1),
        (this.ended = !1),
        (this.finished = !1),
        (this.destroyed = !1);
      var n = t.decodeStrings === !1;
      (this.decodeStrings = !n),
        (this.defaultEncoding = t.defaultEncoding || 'utf8'),
        (this.length = 0),
        (this.writing = !1),
        (this.corked = 0),
        (this.sync = !0),
        (this.bufferProcessing = !1),
        (this.onwrite = function (i) {
          S8(e, i);
        }),
        (this.writecb = null),
        (this.writelen = 0),
        (this.bufferedRequest = null),
        (this.lastBufferedRequest = null),
        (this.pendingcb = 0),
        (this.prefinished = !1),
        (this.errorEmitted = !1),
        (this.emitClose = t.emitClose !== !1),
        (this.autoDestroy = !!t.autoDestroy),
        (this.bufferedRequestCount = 0),
        (this.corkedRequestsFree = new i1(this));
    }
    Nf.prototype.getBuffer = function () {
      for (var e = this.bufferedRequest, r = []; e; ) r.push(e), (e = e.next);
      return r;
    };
    (function () {
      try {
        Object.defineProperty(Nf.prototype, 'buffer', {
          get: i8.deprecate(
            function () {
              return this.getBuffer();
            },
            '_writableState.buffer is deprecated. Use _writableState.getBuffer instead.',
            'DEP0003'
          ),
        });
      } catch {}
    })();
    var uo;
    typeof Symbol == 'function' &&
    Symbol.hasInstance &&
    typeof Function.prototype[Symbol.hasInstance] == 'function'
      ? ((uo = Function.prototype[Symbol.hasInstance]),
        Object.defineProperty(Xe, Symbol.hasInstance, {
          value: function (e) {
            return uo.call(this, e)
              ? !0
              : this !== Xe
                ? !1
                : e && e._writableState instanceof Nf;
          },
        }))
      : (uo = function (e) {
          return e instanceof this;
        });
    function Xe(t) {
      Ln = Ln || wi();
      var e = this instanceof Ln;
      if (!e && !uo.call(Xe, this)) return new Xe(t);
      (this._writableState = new Nf(t, this, e)),
        (this.writable = !0),
        t &&
          (typeof t.write == 'function' && (this._write = t.write),
          typeof t.writev == 'function' && (this._writev = t.writev),
          typeof t.destroy == 'function' && (this._destroy = t.destroy),
          typeof t.final == 'function' && (this._final = t.final)),
        n1.call(this);
    }
    Xe.prototype.pipe = function () {
      Fn(this, new d8());
    };
    function m8(t, e) {
      var r = new v8();
      Fn(t, r), A.nextTick(e, r);
    }
    function g8(t, e, r, n) {
      var i;
      return (
        r === null
          ? (i = new p8())
          : typeof r != 'string' &&
            !e.objectMode &&
            (i = new h8('chunk', ['string', 'Buffer'], r)),
        i ? (Fn(t, i), A.nextTick(n, i), !1) : !0
      );
    }
    Xe.prototype.write = function (t, e, r) {
      var n = this._writableState,
        i = !1,
        a = !n.objectMode && a8(t);
      return (
        a && !co.isBuffer(t) && (t = f8(t)),
        typeof e == 'function' && ((r = e), (e = null)),
        a ? (e = 'buffer') : e || (e = n.defaultEncoding),
        typeof r != 'function' && (r = y8),
        n.ending
          ? m8(this, r)
          : (a || g8(this, n, t, r)) &&
            (n.pendingcb++, (i = _8(this, n, a, t, e, r))),
        i
      );
    };
    Xe.prototype.cork = function () {
      this._writableState.corked++;
    };
    Xe.prototype.uncork = function () {
      var t = this._writableState;
      t.corked &&
        (t.corked--,
        !t.writing &&
          !t.corked &&
          !t.bufferProcessing &&
          t.bufferedRequest &&
          f1(this, t));
    };
    Xe.prototype.setDefaultEncoding = function (e) {
      if (
        (typeof e == 'string' && (e = e.toLowerCase()),
        !(
          [
            'hex',
            'utf8',
            'utf-8',
            'ascii',
            'binary',
            'base64',
            'ucs2',
            'ucs-2',
            'utf16le',
            'utf-16le',
            'raw',
          ].indexOf((e + '').toLowerCase()) > -1
        ))
      )
        throw new b8(e);
      return (this._writableState.defaultEncoding = e), this;
    };
    Object.defineProperty(Xe.prototype, 'writableBuffer', {
      enumerable: !1,
      get: function () {
        return this._writableState && this._writableState.getBuffer();
      },
    });
    function w8(t, e, r) {
      return (
        !t.objectMode &&
          t.decodeStrings !== !1 &&
          typeof e == 'string' &&
          (e = co.from(e, r)),
        e
      );
    }
    Object.defineProperty(Xe.prototype, 'writableHighWaterMark', {
      enumerable: !1,
      get: function () {
        return this._writableState.highWaterMark;
      },
    });
    function _8(t, e, r, n, i, a) {
      if (!r) {
        var h = w8(e, n, i);
        n !== h && ((r = !0), (i = 'buffer'), (n = h));
      }
      var v = e.objectMode ? 1 : n.length;
      e.length += v;
      var g = e.length < e.highWaterMark;
      if ((g || (e.needDrain = !0), e.writing || e.corked)) {
        var M = e.lastBufferedRequest;
        (e.lastBufferedRequest = {
          chunk: n,
          encoding: i,
          isBuf: r,
          callback: a,
          next: null,
        }),
          M
            ? (M.next = e.lastBufferedRequest)
            : (e.bufferedRequest = e.lastBufferedRequest),
          (e.bufferedRequestCount += 1);
      } else Y0(t, e, !1, v, n, i, a);
      return g;
    }
    function Y0(t, e, r, n, i, a, h) {
      (e.writelen = n),
        (e.writecb = h),
        (e.writing = !0),
        (e.sync = !0),
        e.destroyed
          ? e.onwrite(new l8('write'))
          : r
            ? t._writev(i, e.onwrite)
            : t._write(i, a, e.onwrite),
        (e.sync = !1);
    }
    function x8(t, e, r, n, i) {
      --e.pendingcb,
        r
          ? (A.nextTick(i, n),
            A.nextTick(Cf, t, e),
            (t._writableState.errorEmitted = !0),
            Fn(t, n))
          : (i(n), (t._writableState.errorEmitted = !0), Fn(t, n), Cf(t, e));
    }
    function M8(t) {
      (t.writing = !1),
        (t.writecb = null),
        (t.length -= t.writelen),
        (t.writelen = 0);
    }
    function S8(t, e) {
      var r = t._writableState,
        n = r.sync,
        i = r.writecb;
      if (typeof i != 'function') throw new c8();
      if ((M8(r), e)) x8(t, r, n, e, i);
      else {
        var a = a1(r) || t.destroyed;
        !a && !r.corked && !r.bufferProcessing && r.bufferedRequest && f1(t, r),
          n ? A.nextTick(r1, t, r, a, i) : r1(t, r, a, i);
      }
    }
    function r1(t, e, r, n) {
      r || E8(t, e), e.pendingcb--, n(), Cf(t, e);
    }
    function E8(t, e) {
      e.length === 0 && e.needDrain && ((e.needDrain = !1), t.emit('drain'));
    }
    function f1(t, e) {
      e.bufferProcessing = !0;
      var r = e.bufferedRequest;
      if (t._writev && r && r.next) {
        var n = e.bufferedRequestCount,
          i = new Array(n),
          a = e.corkedRequestsFree;
        a.entry = r;
        for (var h = 0, v = !0; r; )
          (i[h] = r), r.isBuf || (v = !1), (r = r.next), (h += 1);
        (i.allBuffers = v),
          Y0(t, e, !0, e.length, i, '', a.finish),
          e.pendingcb++,
          (e.lastBufferedRequest = null),
          a.next
            ? ((e.corkedRequestsFree = a.next), (a.next = null))
            : (e.corkedRequestsFree = new i1(e)),
          (e.bufferedRequestCount = 0);
      } else {
        for (; r; ) {
          var g = r.chunk,
            M = r.encoding,
            x = r.callback,
            E = e.objectMode ? 1 : g.length;
          if (
            (Y0(t, e, !1, E, g, M, x),
            (r = r.next),
            e.bufferedRequestCount--,
            e.writing)
          )
            break;
        }
        r === null && (e.lastBufferedRequest = null);
      }
      (e.bufferedRequest = r), (e.bufferProcessing = !1);
    }
    Xe.prototype._write = function (t, e, r) {
      r(new u8('_write()'));
    };
    Xe.prototype._writev = null;
    Xe.prototype.end = function (t, e, r) {
      var n = this._writableState;
      return (
        typeof t == 'function'
          ? ((r = t), (t = null), (e = null))
          : typeof e == 'function' && ((r = e), (e = null)),
        t != null && this.write(t, e),
        n.corked && ((n.corked = 1), this.uncork()),
        n.ending || I8(this, n, r),
        this
      );
    };
    Object.defineProperty(Xe.prototype, 'writableLength', {
      enumerable: !1,
      get: function () {
        return this._writableState.length;
      },
    });
    function a1(t) {
      return (
        t.ending &&
        t.length === 0 &&
        t.bufferedRequest === null &&
        !t.finished &&
        !t.writing
      );
    }
    function A8(t, e) {
      t._final(function (r) {
        e.pendingcb--,
          r && Fn(t, r),
          (e.prefinished = !0),
          t.emit('prefinish'),
          Cf(t, e);
      });
    }
    function B8(t, e) {
      !e.prefinished &&
        !e.finalCalled &&
        (typeof t._final == 'function' && !e.destroyed
          ? (e.pendingcb++, (e.finalCalled = !0), A.nextTick(A8, t, e))
          : ((e.prefinished = !0), t.emit('prefinish')));
    }
    function Cf(t, e) {
      var r = a1(e);
      if (
        r &&
        (B8(t, e),
        e.pendingcb === 0 &&
          ((e.finished = !0), t.emit('finish'), e.autoDestroy))
      ) {
        var n = t._readableState;
        (!n || (n.autoDestroy && n.endEmitted)) && t.destroy();
      }
      return r;
    }
    function I8(t, e, r) {
      (e.ending = !0),
        Cf(t, e),
        r && (e.finished ? A.nextTick(r) : t.once('finish', r)),
        (e.ended = !0),
        (t.writable = !1);
    }
    function R8(t, e, r) {
      var n = t.entry;
      for (t.entry = null; n; ) {
        var i = n.callback;
        e.pendingcb--, i(r), (n = n.next);
      }
      e.corkedRequestsFree.next = t;
    }
    Object.defineProperty(Xe.prototype, 'destroyed', {
      enumerable: !1,
      get: function () {
        return this._writableState === void 0
          ? !1
          : this._writableState.destroyed;
      },
      set: function (e) {
        !this._writableState || (this._writableState.destroyed = e);
      },
    });
    Xe.prototype.destroy = Q0.destroy;
    Xe.prototype._undestroy = Q0.undestroy;
    Xe.prototype._destroy = function (t, e) {
      e(t);
    };
  });
  var wi = R((uR, h1) => {
    'use strict';
    S();
    var q8 =
      Object.keys ||
      function (t) {
        var e = [];
        for (var r in t) e.push(r);
        return e;
      };
    h1.exports = Br;
    var s1 = bo(),
      th = lo();
    qe()(Br, s1);
    for (eh = q8(th.prototype), po = 0; po < eh.length; po++)
      (vo = eh[po]), Br.prototype[vo] || (Br.prototype[vo] = th.prototype[vo]);
    var eh, vo, po;
    function Br(t) {
      if (!(this instanceof Br)) return new Br(t);
      s1.call(this, t),
        th.call(this, t),
        (this.allowHalfOpen = !0),
        t &&
          (t.readable === !1 && (this.readable = !1),
          t.writable === !1 && (this.writable = !1),
          t.allowHalfOpen === !1 &&
            ((this.allowHalfOpen = !1), this.once('end', T8)));
    }
    Object.defineProperty(Br.prototype, 'writableHighWaterMark', {
      enumerable: !1,
      get: function () {
        return this._writableState.highWaterMark;
      },
    });
    Object.defineProperty(Br.prototype, 'writableBuffer', {
      enumerable: !1,
      get: function () {
        return this._writableState && this._writableState.getBuffer();
      },
    });
    Object.defineProperty(Br.prototype, 'writableLength', {
      enumerable: !1,
      get: function () {
        return this._writableState.length;
      },
    });
    function T8() {
      this._writableState.ended || A.nextTick(P8, this);
    }
    function P8(t) {
      t.end();
    }
    Object.defineProperty(Br.prototype, 'destroyed', {
      enumerable: !1,
      get: function () {
        return this._readableState === void 0 || this._writableState === void 0
          ? !1
          : this._readableState.destroyed && this._writableState.destroyed;
      },
      set: function (e) {
        this._readableState === void 0 ||
          this._writableState === void 0 ||
          ((this._readableState.destroyed = e),
          (this._writableState.destroyed = e));
      },
    });
  });
  var yo = R((c1) => {
    'use strict';
    S();
    var ih = Ie().Buffer,
      u1 =
        ih.isEncoding ||
        function (t) {
          switch (((t = '' + t), t && t.toLowerCase())) {
            case 'hex':
            case 'utf8':
            case 'utf-8':
            case 'ascii':
            case 'binary':
            case 'base64':
            case 'ucs2':
            case 'ucs-2':
            case 'utf16le':
            case 'utf-16le':
            case 'raw':
              return !0;
            default:
              return !1;
          }
        };
    function k8(t) {
      if (!t) return 'utf8';
      for (var e; ; )
        switch (t) {
          case 'utf8':
          case 'utf-8':
            return 'utf8';
          case 'ucs2':
          case 'ucs-2':
          case 'utf16le':
          case 'utf-16le':
            return 'utf16le';
          case 'latin1':
          case 'binary':
            return 'latin1';
          case 'base64':
          case 'ascii':
          case 'hex':
            return t;
          default:
            if (e) return;
            (t = ('' + t).toLowerCase()), (e = !0);
        }
    }
    function O8(t) {
      var e = k8(t);
      if (typeof e != 'string' && (ih.isEncoding === u1 || !u1(t)))
        throw new Error('Unknown encoding: ' + t);
      return e || t;
    }
    c1.StringDecoder = Df;
    function Df(t) {
      this.encoding = O8(t);
      var e;
      switch (this.encoding) {
        case 'utf16le':
          (this.text = j8), (this.end = U8), (e = 4);
          break;
        case 'utf8':
          (this.fillLast = D8), (e = 4);
          break;
        case 'base64':
          (this.text = z8), (this.end = H8), (e = 3);
          break;
        default:
          (this.write = K8), (this.end = V8);
          return;
      }
      (this.lastNeed = 0),
        (this.lastTotal = 0),
        (this.lastChar = ih.allocUnsafe(e));
    }
    Df.prototype.write = function (t) {
      if (t.length === 0) return '';
      var e, r;
      if (this.lastNeed) {
        if (((e = this.fillLast(t)), e === void 0)) return '';
        (r = this.lastNeed), (this.lastNeed = 0);
      } else r = 0;
      return r < t.length
        ? e
          ? e + this.text(t, r)
          : this.text(t, r)
        : e || '';
    };
    Df.prototype.end = F8;
    Df.prototype.text = L8;
    Df.prototype.fillLast = function (t) {
      if (this.lastNeed <= t.length)
        return (
          t.copy(
            this.lastChar,
            this.lastTotal - this.lastNeed,
            0,
            this.lastNeed
          ),
          this.lastChar.toString(this.encoding, 0, this.lastTotal)
        );
      t.copy(this.lastChar, this.lastTotal - this.lastNeed, 0, t.length),
        (this.lastNeed -= t.length);
    };
    function rh(t) {
      return t <= 127
        ? 0
        : t >> 5 === 6
          ? 2
          : t >> 4 === 14
            ? 3
            : t >> 3 === 30
              ? 4
              : t >> 6 === 2
                ? -1
                : -2;
    }
    function C8(t, e, r) {
      var n = e.length - 1;
      if (n < r) return 0;
      var i = rh(e[n]);
      return i >= 0
        ? (i > 0 && (t.lastNeed = i - 1), i)
        : --n < r || i === -2
          ? 0
          : ((i = rh(e[n])),
            i >= 0
              ? (i > 0 && (t.lastNeed = i - 2), i)
              : --n < r || i === -2
                ? 0
                : ((i = rh(e[n])),
                  i >= 0
                    ? (i > 0 && (i === 2 ? (i = 0) : (t.lastNeed = i - 3)), i)
                    : 0));
    }
    function N8(t, e, r) {
      if ((e[0] & 192) !== 128) return (t.lastNeed = 0), '\uFFFD';
      if (t.lastNeed > 1 && e.length > 1) {
        if ((e[1] & 192) !== 128) return (t.lastNeed = 1), '\uFFFD';
        if (t.lastNeed > 2 && e.length > 2 && (e[2] & 192) !== 128)
          return (t.lastNeed = 2), '\uFFFD';
      }
    }
    function D8(t) {
      var e = this.lastTotal - this.lastNeed,
        r = N8(this, t, e);
      if (r !== void 0) return r;
      if (this.lastNeed <= t.length)
        return (
          t.copy(this.lastChar, e, 0, this.lastNeed),
          this.lastChar.toString(this.encoding, 0, this.lastTotal)
        );
      t.copy(this.lastChar, e, 0, t.length), (this.lastNeed -= t.length);
    }
    function L8(t, e) {
      var r = C8(this, t, e);
      if (!this.lastNeed) return t.toString('utf8', e);
      this.lastTotal = r;
      var n = t.length - (r - this.lastNeed);
      return t.copy(this.lastChar, 0, n), t.toString('utf8', e, n);
    }
    function F8(t) {
      var e = t && t.length ? this.write(t) : '';
      return this.lastNeed ? e + '\uFFFD' : e;
    }
    function j8(t, e) {
      if ((t.length - e) % 2 === 0) {
        var r = t.toString('utf16le', e);
        if (r) {
          var n = r.charCodeAt(r.length - 1);
          if (n >= 55296 && n <= 56319)
            return (
              (this.lastNeed = 2),
              (this.lastTotal = 4),
              (this.lastChar[0] = t[t.length - 2]),
              (this.lastChar[1] = t[t.length - 1]),
              r.slice(0, -1)
            );
        }
        return r;
      }
      return (
        (this.lastNeed = 1),
        (this.lastTotal = 2),
        (this.lastChar[0] = t[t.length - 1]),
        t.toString('utf16le', e, t.length - 1)
      );
    }
    function U8(t) {
      var e = t && t.length ? this.write(t) : '';
      if (this.lastNeed) {
        var r = this.lastTotal - this.lastNeed;
        return e + this.lastChar.toString('utf16le', 0, r);
      }
      return e;
    }
    function z8(t, e) {
      var r = (t.length - e) % 3;
      return r === 0
        ? t.toString('base64', e)
        : ((this.lastNeed = 3 - r),
          (this.lastTotal = 3),
          r === 1
            ? (this.lastChar[0] = t[t.length - 1])
            : ((this.lastChar[0] = t[t.length - 2]),
              (this.lastChar[1] = t[t.length - 1])),
          t.toString('base64', e, t.length - r));
    }
    function H8(t) {
      var e = t && t.length ? this.write(t) : '';
      return this.lastNeed
        ? e + this.lastChar.toString('base64', 0, 3 - this.lastNeed)
        : e;
    }
    function K8(t) {
      return t.toString(this.encoding);
    }
    function V8(t) {
      return t && t.length ? this.write(t) : '';
    }
  });
  var Lf = R((pR, p1) => {
    'use strict';
    S();
    var d1 = Wi().codes.ERR_STREAM_PREMATURE_CLOSE;
    function G8(t) {
      var e = !1;
      return function () {
        if (!e) {
          e = !0;
          for (var r = arguments.length, n = new Array(r), i = 0; i < r; i++)
            n[i] = arguments[i];
          t.apply(this, n);
        }
      };
    }
    function W8() {}
    function $8(t) {
      return t.setHeader && typeof t.abort == 'function';
    }
    function l1(t, e, r) {
      if (typeof e == 'function') return l1(t, null, e);
      e || (e = {}), (r = G8(r || W8));
      var n = e.readable || (e.readable !== !1 && t.readable),
        i = e.writable || (e.writable !== !1 && t.writable),
        a = function () {
          t.writable || v();
        },
        h = t._writableState && t._writableState.finished,
        v = function () {
          (i = !1), (h = !0), n || r.call(t);
        },
        g = t._readableState && t._readableState.endEmitted,
        M = function () {
          (n = !1), (g = !0), i || r.call(t);
        },
        x = function (k) {
          r.call(t, k);
        },
        E = function () {
          var k;
          if (n && !g)
            return (
              (!t._readableState || !t._readableState.ended) && (k = new d1()),
              r.call(t, k)
            );
          if (i && !h)
            return (
              (!t._writableState || !t._writableState.ended) && (k = new d1()),
              r.call(t, k)
            );
        },
        I = function () {
          t.req.on('finish', v);
        };
      return (
        $8(t)
          ? (t.on('complete', v),
            t.on('abort', E),
            t.req ? I() : t.on('request', I))
          : i && !t._writableState && (t.on('end', a), t.on('close', a)),
        t.on('end', M),
        t.on('finish', v),
        e.error !== !1 && t.on('error', x),
        t.on('close', E),
        function () {
          t.removeListener('complete', v),
            t.removeListener('abort', E),
            t.removeListener('request', I),
            t.req && t.req.removeListener('finish', v),
            t.removeListener('end', a),
            t.removeListener('close', a),
            t.removeListener('finish', v),
            t.removeListener('end', M),
            t.removeListener('error', x),
            t.removeListener('close', E);
        }
      );
    }
    p1.exports = l1;
  });
  var b1 = R((bR, v1) => {
    'use strict';
    S();
    var mo;
    function _i(t, e, r) {
      return (
        e in t
          ? Object.defineProperty(t, e, {
              value: r,
              enumerable: !0,
              configurable: !0,
              writable: !0,
            })
          : (t[e] = r),
        t
      );
    }
    var Z8 = Lf(),
      xi = Symbol('lastResolve'),
      $i = Symbol('lastReject'),
      Ff = Symbol('error'),
      go = Symbol('ended'),
      Zi = Symbol('lastPromise'),
      nh = Symbol('handlePromise'),
      Ji = Symbol('stream');
    function Mi(t, e) {
      return { value: t, done: e };
    }
    function J8(t) {
      var e = t[xi];
      if (e !== null) {
        var r = t[Ji].read();
        r !== null &&
          ((t[Zi] = null), (t[xi] = null), (t[$i] = null), e(Mi(r, !1)));
      }
    }
    function X8(t) {
      A.nextTick(J8, t);
    }
    function Y8(t, e) {
      return function (r, n) {
        t.then(function () {
          if (e[go]) {
            r(Mi(void 0, !0));
            return;
          }
          e[nh](r, n);
        }, n);
      };
    }
    var Q8 = Object.getPrototypeOf(function () {}),
      e5 = Object.setPrototypeOf(
        ((mo = {
          get stream() {
            return this[Ji];
          },
          next: function () {
            var e = this,
              r = this[Ff];
            if (r !== null) return Promise.reject(r);
            if (this[go]) return Promise.resolve(Mi(void 0, !0));
            if (this[Ji].destroyed)
              return new Promise(function (h, v) {
                A.nextTick(function () {
                  e[Ff] ? v(e[Ff]) : h(Mi(void 0, !0));
                });
              });
            var n = this[Zi],
              i;
            if (n) i = new Promise(Y8(n, this));
            else {
              var a = this[Ji].read();
              if (a !== null) return Promise.resolve(Mi(a, !1));
              i = new Promise(this[nh]);
            }
            return (this[Zi] = i), i;
          },
        }),
        _i(mo, Symbol.asyncIterator, function () {
          return this;
        }),
        _i(mo, 'return', function () {
          var e = this;
          return new Promise(function (r, n) {
            e[Ji].destroy(null, function (i) {
              if (i) {
                n(i);
                return;
              }
              r(Mi(void 0, !0));
            });
          });
        }),
        mo),
        Q8
      ),
      t5 = function (e) {
        var r,
          n = Object.create(
            e5,
            ((r = {}),
            _i(r, Ji, { value: e, writable: !0 }),
            _i(r, xi, { value: null, writable: !0 }),
            _i(r, $i, { value: null, writable: !0 }),
            _i(r, Ff, { value: null, writable: !0 }),
            _i(r, go, { value: e._readableState.endEmitted, writable: !0 }),
            _i(r, nh, {
              value: function (a, h) {
                var v = n[Ji].read();
                v
                  ? ((n[Zi] = null),
                    (n[xi] = null),
                    (n[$i] = null),
                    a(Mi(v, !1)))
                  : ((n[xi] = a), (n[$i] = h));
              },
              writable: !0,
            }),
            r)
          );
        return (
          (n[Zi] = null),
          Z8(e, function (i) {
            if (i && i.code !== 'ERR_STREAM_PREMATURE_CLOSE') {
              var a = n[$i];
              a !== null &&
                ((n[Zi] = null), (n[xi] = null), (n[$i] = null), a(i)),
                (n[Ff] = i);
              return;
            }
            var h = n[xi];
            h !== null &&
              ((n[Zi] = null),
              (n[xi] = null),
              (n[$i] = null),
              h(Mi(void 0, !0))),
              (n[go] = !0);
          }),
          e.on('readable', X8.bind(null, n)),
          n
        );
      };
    v1.exports = t5;
  });
  var m1 = R((mR, y1) => {
    S();
    y1.exports = function () {
      throw new Error('Readable.from is not available in the browser');
    };
  });
  var bo = R((_R, I1) => {
    'use strict';
    S();
    I1.exports = Ue;
    var jn;
    Ue.ReadableState = x1;
    var wR = La().EventEmitter,
      _1 = function (e, r) {
        return e.listeners(r).length;
      },
      Uf = m0(),
      wo = Et().Buffer,
      r5 = globalThis.Uint8Array || function () {};
    function i5(t) {
      return wo.from(t);
    }
    function n5(t) {
      return wo.isBuffer(t) || t instanceof r5;
    }
    var fh = Gi(),
      Oe;
    fh && fh.debuglog ? (Oe = fh.debuglog('stream')) : (Oe = function () {});
    var f5 = Wl(),
      dh = Z0(),
      a5 = J0(),
      o5 = a5.getHighWaterMark,
      _o = Wi().codes,
      s5 = _o.ERR_INVALID_ARG_TYPE,
      h5 = _o.ERR_STREAM_PUSH_AFTER_EOF,
      u5 = _o.ERR_METHOD_NOT_IMPLEMENTED,
      c5 = _o.ERR_STREAM_UNSHIFT_AFTER_END_EVENT,
      Un,
      ah,
      oh;
    qe()(Ue, Uf);
    var jf = dh.errorOrDestroy,
      sh = ['error', 'close', 'destroy', 'pause', 'resume'];
    function d5(t, e, r) {
      if (typeof t.prependListener == 'function')
        return t.prependListener(e, r);
      !t._events || !t._events[e]
        ? t.on(e, r)
        : Array.isArray(t._events[e])
          ? t._events[e].unshift(r)
          : (t._events[e] = [r, t._events[e]]);
    }
    function x1(t, e, r) {
      (jn = jn || wi()),
        (t = t || {}),
        typeof r != 'boolean' && (r = e instanceof jn),
        (this.objectMode = !!t.objectMode),
        r && (this.objectMode = this.objectMode || !!t.readableObjectMode),
        (this.highWaterMark = o5(this, t, 'readableHighWaterMark', r)),
        (this.buffer = new f5()),
        (this.length = 0),
        (this.pipes = null),
        (this.pipesCount = 0),
        (this.flowing = null),
        (this.ended = !1),
        (this.endEmitted = !1),
        (this.reading = !1),
        (this.sync = !0),
        (this.needReadable = !1),
        (this.emittedReadable = !1),
        (this.readableListening = !1),
        (this.resumeScheduled = !1),
        (this.paused = !0),
        (this.emitClose = t.emitClose !== !1),
        (this.autoDestroy = !!t.autoDestroy),
        (this.destroyed = !1),
        (this.defaultEncoding = t.defaultEncoding || 'utf8'),
        (this.awaitDrain = 0),
        (this.readingMore = !1),
        (this.decoder = null),
        (this.encoding = null),
        t.encoding &&
          (Un || (Un = yo().StringDecoder),
          (this.decoder = new Un(t.encoding)),
          (this.encoding = t.encoding));
    }
    function Ue(t) {
      if (((jn = jn || wi()), !(this instanceof Ue))) return new Ue(t);
      var e = this instanceof jn;
      (this._readableState = new x1(t, this, e)),
        (this.readable = !0),
        t &&
          (typeof t.read == 'function' && (this._read = t.read),
          typeof t.destroy == 'function' && (this._destroy = t.destroy)),
        Uf.call(this);
    }
    Object.defineProperty(Ue.prototype, 'destroyed', {
      enumerable: !1,
      get: function () {
        return this._readableState === void 0
          ? !1
          : this._readableState.destroyed;
      },
      set: function (e) {
        !this._readableState || (this._readableState.destroyed = e);
      },
    });
    Ue.prototype.destroy = dh.destroy;
    Ue.prototype._undestroy = dh.undestroy;
    Ue.prototype._destroy = function (t, e) {
      e(t);
    };
    Ue.prototype.push = function (t, e) {
      var r = this._readableState,
        n;
      return (
        r.objectMode
          ? (n = !0)
          : typeof t == 'string' &&
            ((e = e || r.defaultEncoding),
            e !== r.encoding && ((t = wo.from(t, e)), (e = '')),
            (n = !0)),
        M1(this, t, e, !1, n)
      );
    };
    Ue.prototype.unshift = function (t) {
      return M1(this, t, null, !0, !1);
    };
    function M1(t, e, r, n, i) {
      Oe('readableAddChunk', e);
      var a = t._readableState;
      if (e === null) (a.reading = !1), v5(t, a);
      else {
        var h;
        if ((i || (h = l5(a, e)), h)) jf(t, h);
        else if (a.objectMode || (e && e.length > 0))
          if (
            (typeof e != 'string' &&
              !a.objectMode &&
              Object.getPrototypeOf(e) !== wo.prototype &&
              (e = i5(e)),
            n)
          )
            a.endEmitted ? jf(t, new c5()) : hh(t, a, e, !0);
          else if (a.ended) jf(t, new h5());
          else {
            if (a.destroyed) return !1;
            (a.reading = !1),
              a.decoder && !r
                ? ((e = a.decoder.write(e)),
                  a.objectMode || e.length !== 0 ? hh(t, a, e, !1) : ch(t, a))
                : hh(t, a, e, !1);
          }
        else n || ((a.reading = !1), ch(t, a));
      }
      return !a.ended && (a.length < a.highWaterMark || a.length === 0);
    }
    function hh(t, e, r, n) {
      e.flowing && e.length === 0 && !e.sync
        ? ((e.awaitDrain = 0), t.emit('data', r))
        : ((e.length += e.objectMode ? 1 : r.length),
          n ? e.buffer.unshift(r) : e.buffer.push(r),
          e.needReadable && xo(t)),
        ch(t, e);
    }
    function l5(t, e) {
      var r;
      return (
        !n5(e) &&
          typeof e != 'string' &&
          e !== void 0 &&
          !t.objectMode &&
          (r = new s5('chunk', ['string', 'Buffer', 'Uint8Array'], e)),
        r
      );
    }
    Ue.prototype.isPaused = function () {
      return this._readableState.flowing === !1;
    };
    Ue.prototype.setEncoding = function (t) {
      Un || (Un = yo().StringDecoder);
      var e = new Un(t);
      (this._readableState.decoder = e),
        (this._readableState.encoding = this._readableState.decoder.encoding);
      for (var r = this._readableState.buffer.head, n = ''; r !== null; )
        (n += e.write(r.data)), (r = r.next);
      return (
        this._readableState.buffer.clear(),
        n !== '' && this._readableState.buffer.push(n),
        (this._readableState.length = n.length),
        this
      );
    };
    var g1 = 1073741824;
    function p5(t) {
      return (
        t >= g1
          ? (t = g1)
          : (t--,
            (t |= t >>> 1),
            (t |= t >>> 2),
            (t |= t >>> 4),
            (t |= t >>> 8),
            (t |= t >>> 16),
            t++),
        t
      );
    }
    function w1(t, e) {
      return t <= 0 || (e.length === 0 && e.ended)
        ? 0
        : e.objectMode
          ? 1
          : t !== t
            ? e.flowing && e.length
              ? e.buffer.head.data.length
              : e.length
            : (t > e.highWaterMark && (e.highWaterMark = p5(t)),
              t <= e.length
                ? t
                : e.ended
                  ? e.length
                  : ((e.needReadable = !0), 0));
    }
    Ue.prototype.read = function (t) {
      Oe('read', t), (t = parseInt(t, 10));
      var e = this._readableState,
        r = t;
      if (
        (t !== 0 && (e.emittedReadable = !1),
        t === 0 &&
          e.needReadable &&
          ((e.highWaterMark !== 0
            ? e.length >= e.highWaterMark
            : e.length > 0) ||
            e.ended))
      )
        return (
          Oe('read: emitReadable', e.length, e.ended),
          e.length === 0 && e.ended ? uh(this) : xo(this),
          null
        );
      if (((t = w1(t, e)), t === 0 && e.ended))
        return e.length === 0 && uh(this), null;
      var n = e.needReadable;
      Oe('need readable', n),
        (e.length === 0 || e.length - t < e.highWaterMark) &&
          ((n = !0), Oe('length less than watermark', n)),
        e.ended || e.reading
          ? ((n = !1), Oe('reading or ended', n))
          : n &&
            (Oe('do read'),
            (e.reading = !0),
            (e.sync = !0),
            e.length === 0 && (e.needReadable = !0),
            this._read(e.highWaterMark),
            (e.sync = !1),
            e.reading || (t = w1(r, e)));
      var i;
      return (
        t > 0 ? (i = A1(t, e)) : (i = null),
        i === null
          ? ((e.needReadable = e.length <= e.highWaterMark), (t = 0))
          : ((e.length -= t), (e.awaitDrain = 0)),
        e.length === 0 &&
          (e.ended || (e.needReadable = !0), r !== t && e.ended && uh(this)),
        i !== null && this.emit('data', i),
        i
      );
    };
    function v5(t, e) {
      if ((Oe('onEofChunk'), !e.ended)) {
        if (e.decoder) {
          var r = e.decoder.end();
          r &&
            r.length &&
            (e.buffer.push(r), (e.length += e.objectMode ? 1 : r.length));
        }
        (e.ended = !0),
          e.sync
            ? xo(t)
            : ((e.needReadable = !1),
              e.emittedReadable || ((e.emittedReadable = !0), S1(t)));
      }
    }
    function xo(t) {
      var e = t._readableState;
      Oe('emitReadable', e.needReadable, e.emittedReadable),
        (e.needReadable = !1),
        e.emittedReadable ||
          (Oe('emitReadable', e.flowing),
          (e.emittedReadable = !0),
          A.nextTick(S1, t));
    }
    function S1(t) {
      var e = t._readableState;
      Oe('emitReadable_', e.destroyed, e.length, e.ended),
        !e.destroyed &&
          (e.length || e.ended) &&
          (t.emit('readable'), (e.emittedReadable = !1)),
        (e.needReadable =
          !e.flowing && !e.ended && e.length <= e.highWaterMark),
        lh(t);
    }
    function ch(t, e) {
      e.readingMore || ((e.readingMore = !0), A.nextTick(b5, t, e));
    }
    function b5(t, e) {
      for (
        ;
        !e.reading &&
        !e.ended &&
        (e.length < e.highWaterMark || (e.flowing && e.length === 0));

      ) {
        var r = e.length;
        if ((Oe('maybeReadMore read 0'), t.read(0), r === e.length)) break;
      }
      e.readingMore = !1;
    }
    Ue.prototype._read = function (t) {
      jf(this, new u5('_read()'));
    };
    Ue.prototype.pipe = function (t, e) {
      var r = this,
        n = this._readableState;
      switch (n.pipesCount) {
        case 0:
          n.pipes = t;
          break;
        case 1:
          n.pipes = [n.pipes, t];
          break;
        default:
          n.pipes.push(t);
          break;
      }
      (n.pipesCount += 1), Oe('pipe count=%d opts=%j', n.pipesCount, e);
      var i = (!e || e.end !== !1) && t !== A.stdout && t !== A.stderr,
        a = i ? v : L;
      n.endEmitted ? A.nextTick(a) : r.once('end', a), t.on('unpipe', h);
      function h(xe, U) {
        Oe('onunpipe'),
          xe === r && U && U.hasUnpiped === !1 && ((U.hasUnpiped = !0), x());
      }
      function v() {
        Oe('onend'), t.end();
      }
      var g = y5(r);
      t.on('drain', g);
      var M = !1;
      function x() {
        Oe('cleanup'),
          t.removeListener('close', q),
          t.removeListener('finish', k),
          t.removeListener('drain', g),
          t.removeListener('error', I),
          t.removeListener('unpipe', h),
          r.removeListener('end', v),
          r.removeListener('end', L),
          r.removeListener('data', E),
          (M = !0),
          n.awaitDrain &&
            (!t._writableState || t._writableState.needDrain) &&
            g();
      }
      r.on('data', E);
      function E(xe) {
        Oe('ondata');
        var U = t.write(xe);
        Oe('dest.write', U),
          U === !1 &&
            (((n.pipesCount === 1 && n.pipes === t) ||
              (n.pipesCount > 1 && B1(n.pipes, t) !== -1)) &&
              !M &&
              (Oe('false write response, pause', n.awaitDrain), n.awaitDrain++),
            r.pause());
      }
      function I(xe) {
        Oe('onerror', xe),
          L(),
          t.removeListener('error', I),
          _1(t, 'error') === 0 && jf(t, xe);
      }
      d5(t, 'error', I);
      function q() {
        t.removeListener('finish', k), L();
      }
      t.once('close', q);
      function k() {
        Oe('onfinish'), t.removeListener('close', q), L();
      }
      t.once('finish', k);
      function L() {
        Oe('unpipe'), r.unpipe(t);
      }
      return t.emit('pipe', r), n.flowing || (Oe('pipe resume'), r.resume()), t;
    };
    function y5(t) {
      return function () {
        var r = t._readableState;
        Oe('pipeOnDrain', r.awaitDrain),
          r.awaitDrain && r.awaitDrain--,
          r.awaitDrain === 0 && _1(t, 'data') && ((r.flowing = !0), lh(t));
      };
    }
    Ue.prototype.unpipe = function (t) {
      var e = this._readableState,
        r = { hasUnpiped: !1 };
      if (e.pipesCount === 0) return this;
      if (e.pipesCount === 1)
        return t && t !== e.pipes
          ? this
          : (t || (t = e.pipes),
            (e.pipes = null),
            (e.pipesCount = 0),
            (e.flowing = !1),
            t && t.emit('unpipe', this, r),
            this);
      if (!t) {
        var n = e.pipes,
          i = e.pipesCount;
        (e.pipes = null), (e.pipesCount = 0), (e.flowing = !1);
        for (var a = 0; a < i; a++)
          n[a].emit('unpipe', this, { hasUnpiped: !1 });
        return this;
      }
      var h = B1(e.pipes, t);
      return h === -1
        ? this
        : (e.pipes.splice(h, 1),
          (e.pipesCount -= 1),
          e.pipesCount === 1 && (e.pipes = e.pipes[0]),
          t.emit('unpipe', this, r),
          this);
    };
    Ue.prototype.on = function (t, e) {
      var r = Uf.prototype.on.call(this, t, e),
        n = this._readableState;
      return (
        t === 'data'
          ? ((n.readableListening = this.listenerCount('readable') > 0),
            n.flowing !== !1 && this.resume())
          : t === 'readable' &&
            !n.endEmitted &&
            !n.readableListening &&
            ((n.readableListening = n.needReadable = !0),
            (n.flowing = !1),
            (n.emittedReadable = !1),
            Oe('on readable', n.length, n.reading),
            n.length ? xo(this) : n.reading || A.nextTick(m5, this)),
        r
      );
    };
    Ue.prototype.addListener = Ue.prototype.on;
    Ue.prototype.removeListener = function (t, e) {
      var r = Uf.prototype.removeListener.call(this, t, e);
      return t === 'readable' && A.nextTick(E1, this), r;
    };
    Ue.prototype.removeAllListeners = function (t) {
      var e = Uf.prototype.removeAllListeners.apply(this, arguments);
      return (t === 'readable' || t === void 0) && A.nextTick(E1, this), e;
    };
    function E1(t) {
      var e = t._readableState;
      (e.readableListening = t.listenerCount('readable') > 0),
        e.resumeScheduled && !e.paused
          ? (e.flowing = !0)
          : t.listenerCount('data') > 0 && t.resume();
    }
    function m5(t) {
      Oe('readable nexttick read 0'), t.read(0);
    }
    Ue.prototype.resume = function () {
      var t = this._readableState;
      return (
        t.flowing ||
          (Oe('resume'), (t.flowing = !t.readableListening), g5(this, t)),
        (t.paused = !1),
        this
      );
    };
    function g5(t, e) {
      e.resumeScheduled || ((e.resumeScheduled = !0), A.nextTick(w5, t, e));
    }
    function w5(t, e) {
      Oe('resume', e.reading),
        e.reading || t.read(0),
        (e.resumeScheduled = !1),
        t.emit('resume'),
        lh(t),
        e.flowing && !e.reading && t.read(0);
    }
    Ue.prototype.pause = function () {
      return (
        Oe('call pause flowing=%j', this._readableState.flowing),
        this._readableState.flowing !== !1 &&
          (Oe('pause'), (this._readableState.flowing = !1), this.emit('pause')),
        (this._readableState.paused = !0),
        this
      );
    };
    function lh(t) {
      var e = t._readableState;
      for (Oe('flow', e.flowing); e.flowing && t.read() !== null; );
    }
    Ue.prototype.wrap = function (t) {
      var e = this,
        r = this._readableState,
        n = !1;
      t.on('end', function () {
        if ((Oe('wrapped end'), r.decoder && !r.ended)) {
          var h = r.decoder.end();
          h && h.length && e.push(h);
        }
        e.push(null);
      }),
        t.on('data', function (h) {
          if (
            (Oe('wrapped data'),
            r.decoder && (h = r.decoder.write(h)),
            !(r.objectMode && h == null) &&
              !(!r.objectMode && (!h || !h.length)))
          ) {
            var v = e.push(h);
            v || ((n = !0), t.pause());
          }
        });
      for (var i in t)
        this[i] === void 0 &&
          typeof t[i] == 'function' &&
          (this[i] = (function (v) {
            return function () {
              return t[v].apply(t, arguments);
            };
          })(i));
      for (var a = 0; a < sh.length; a++)
        t.on(sh[a], this.emit.bind(this, sh[a]));
      return (
        (this._read = function (h) {
          Oe('wrapped _read', h), n && ((n = !1), t.resume());
        }),
        this
      );
    };
    typeof Symbol == 'function' &&
      (Ue.prototype[Symbol.asyncIterator] = function () {
        return ah === void 0 && (ah = b1()), ah(this);
      });
    Object.defineProperty(Ue.prototype, 'readableHighWaterMark', {
      enumerable: !1,
      get: function () {
        return this._readableState.highWaterMark;
      },
    });
    Object.defineProperty(Ue.prototype, 'readableBuffer', {
      enumerable: !1,
      get: function () {
        return this._readableState && this._readableState.buffer;
      },
    });
    Object.defineProperty(Ue.prototype, 'readableFlowing', {
      enumerable: !1,
      get: function () {
        return this._readableState.flowing;
      },
      set: function (e) {
        this._readableState && (this._readableState.flowing = e);
      },
    });
    Ue._fromList = A1;
    Object.defineProperty(Ue.prototype, 'readableLength', {
      enumerable: !1,
      get: function () {
        return this._readableState.length;
      },
    });
    function A1(t, e) {
      if (e.length === 0) return null;
      var r;
      return (
        e.objectMode
          ? (r = e.buffer.shift())
          : !t || t >= e.length
            ? (e.decoder
                ? (r = e.buffer.join(''))
                : e.buffer.length === 1
                  ? (r = e.buffer.first())
                  : (r = e.buffer.concat(e.length)),
              e.buffer.clear())
            : (r = e.buffer.consume(t, e.decoder)),
        r
      );
    }
    function uh(t) {
      var e = t._readableState;
      Oe('endReadable', e.endEmitted),
        e.endEmitted || ((e.ended = !0), A.nextTick(_5, e, t));
    }
    function _5(t, e) {
      if (
        (Oe('endReadableNT', t.endEmitted, t.length),
        !t.endEmitted &&
          t.length === 0 &&
          ((t.endEmitted = !0),
          (e.readable = !1),
          e.emit('end'),
          t.autoDestroy))
      ) {
        var r = e._writableState;
        (!r || (r.autoDestroy && r.finished)) && e.destroy();
      }
    }
    typeof Symbol == 'function' &&
      (Ue.from = function (t, e) {
        return oh === void 0 && (oh = m1()), oh(Ue, t, e);
      });
    function B1(t, e) {
      for (var r = 0, n = t.length; r < n; r++) if (t[r] === e) return r;
      return -1;
    }
  });
  var Eo = R((MR, q1) => {
    'use strict';
    S();
    q1.exports = jr;
    var Mo = Wi().codes,
      x5 = Mo.ERR_METHOD_NOT_IMPLEMENTED,
      M5 = Mo.ERR_MULTIPLE_CALLBACK,
      S5 = Mo.ERR_TRANSFORM_ALREADY_TRANSFORMING,
      E5 = Mo.ERR_TRANSFORM_WITH_LENGTH_0,
      So = wi();
    qe()(jr, So);
    function A5(t, e) {
      var r = this._transformState;
      r.transforming = !1;
      var n = r.writecb;
      if (n === null) return this.emit('error', new M5());
      (r.writechunk = null),
        (r.writecb = null),
        e != null && this.push(e),
        n(t);
      var i = this._readableState;
      (i.reading = !1),
        (i.needReadable || i.length < i.highWaterMark) &&
          this._read(i.highWaterMark);
    }
    function jr(t) {
      if (!(this instanceof jr)) return new jr(t);
      So.call(this, t),
        (this._transformState = {
          afterTransform: A5.bind(this),
          needTransform: !1,
          transforming: !1,
          writecb: null,
          writechunk: null,
          writeencoding: null,
        }),
        (this._readableState.needReadable = !0),
        (this._readableState.sync = !1),
        t &&
          (typeof t.transform == 'function' && (this._transform = t.transform),
          typeof t.flush == 'function' && (this._flush = t.flush)),
        this.on('prefinish', B5);
    }
    function B5() {
      var t = this;
      typeof this._flush == 'function' && !this._readableState.destroyed
        ? this._flush(function (e, r) {
            R1(t, e, r);
          })
        : R1(this, null, null);
    }
    jr.prototype.push = function (t, e) {
      return (
        (this._transformState.needTransform = !1),
        So.prototype.push.call(this, t, e)
      );
    };
    jr.prototype._transform = function (t, e, r) {
      r(new x5('_transform()'));
    };
    jr.prototype._write = function (t, e, r) {
      var n = this._transformState;
      if (
        ((n.writecb = r),
        (n.writechunk = t),
        (n.writeencoding = e),
        !n.transforming)
      ) {
        var i = this._readableState;
        (n.needTransform || i.needReadable || i.length < i.highWaterMark) &&
          this._read(i.highWaterMark);
      }
    };
    jr.prototype._read = function (t) {
      var e = this._transformState;
      e.writechunk !== null && !e.transforming
        ? ((e.transforming = !0),
          this._transform(e.writechunk, e.writeencoding, e.afterTransform))
        : (e.needTransform = !0);
    };
    jr.prototype._destroy = function (t, e) {
      So.prototype._destroy.call(this, t, function (r) {
        e(r);
      });
    };
    function R1(t, e, r) {
      if (e) return t.emit('error', e);
      if ((r != null && t.push(r), t._writableState.length)) throw new E5();
      if (t._transformState.transforming) throw new S5();
      return t.push(null);
    }
  });
  var ph = R((ER, P1) => {
    'use strict';
    S();
    P1.exports = zf;
    var T1 = Eo();
    qe()(zf, T1);
    function zf(t) {
      if (!(this instanceof zf)) return new zf(t);
      T1.call(this, t);
    }
    zf.prototype._transform = function (t, e, r) {
      r(null, t);
    };
  });
  var bh = R((BR, N1) => {
    'use strict';
    S();
    var vh;
    function I5(t) {
      var e = !1;
      return function () {
        e || ((e = !0), t.apply(void 0, arguments));
      };
    }
    var C1 = Wi().codes,
      R5 = C1.ERR_MISSING_ARGS,
      q5 = C1.ERR_STREAM_DESTROYED;
    function k1(t) {
      if (t) throw t;
    }
    function T5(t) {
      return t.setHeader && typeof t.abort == 'function';
    }
    function P5(t, e, r, n) {
      n = I5(n);
      var i = !1;
      t.on('close', function () {
        i = !0;
      }),
        vh === void 0 && (vh = Lf()),
        vh(t, { readable: e, writable: r }, function (h) {
          if (h) return n(h);
          (i = !0), n();
        });
      var a = !1;
      return function (h) {
        if (!i && !a) {
          if (((a = !0), T5(t))) return t.abort();
          if (typeof t.destroy == 'function') return t.destroy();
          n(h || new q5('pipe'));
        }
      };
    }
    function O1(t) {
      t();
    }
    function k5(t, e) {
      return t.pipe(e);
    }
    function O5(t) {
      return !t.length || typeof t[t.length - 1] != 'function' ? k1 : t.pop();
    }
    function C5() {
      for (var t = arguments.length, e = new Array(t), r = 0; r < t; r++)
        e[r] = arguments[r];
      var n = O5(e);
      if ((Array.isArray(e[0]) && (e = e[0]), e.length < 2))
        throw new R5('streams');
      var i,
        a = e.map(function (h, v) {
          var g = v < e.length - 1,
            M = v > 0;
          return P5(h, g, M, function (x) {
            i || (i = x), x && a.forEach(O1), !g && (a.forEach(O1), n(i));
          });
        });
      return e.reduce(k5);
    }
    N1.exports = C5;
  });
  var Hf = R((RR, D1) => {
    S();
    D1.exports = kt;
    var yh = La().EventEmitter,
      N5 = qe();
    N5(kt, yh);
    kt.Readable = bo();
    kt.Writable = lo();
    kt.Duplex = wi();
    kt.Transform = Eo();
    kt.PassThrough = ph();
    kt.finished = Lf();
    kt.pipeline = bh();
    kt.Stream = kt;
    function kt() {
      yh.call(this);
    }
    kt.prototype.pipe = function (t, e) {
      var r = this;
      function n(x) {
        t.writable && t.write(x) === !1 && r.pause && r.pause();
      }
      r.on('data', n);
      function i() {
        r.readable && r.resume && r.resume();
      }
      t.on('drain', i),
        !t._isStdio &&
          (!e || e.end !== !1) &&
          (r.on('end', h), r.on('close', v));
      var a = !1;
      function h() {
        a || ((a = !0), t.end());
      }
      function v() {
        a || ((a = !0), typeof t.destroy == 'function' && t.destroy());
      }
      function g(x) {
        if ((M(), yh.listenerCount(this, 'error') === 0)) throw x;
      }
      r.on('error', g), t.on('error', g);
      function M() {
        r.removeListener('data', n),
          t.removeListener('drain', i),
          r.removeListener('end', h),
          r.removeListener('close', v),
          r.removeListener('error', g),
          t.removeListener('error', g),
          r.removeListener('end', M),
          r.removeListener('close', M),
          t.removeListener('close', M);
      }
      return (
        r.on('end', M), r.on('close', M), t.on('close', M), t.emit('pipe', r), t
      );
    };
  });
  var mh = R((TR, L1) => {
    S();
    var Ao = Ie().Buffer,
      D5 = Hf(),
      L5 = Gi();
    function Bo(t) {
      if (
        ((this.buffer = null), (this.writable = !0), (this.readable = !0), !t)
      )
        return (this.buffer = Ao.alloc(0)), this;
      if (typeof t.pipe == 'function')
        return (this.buffer = Ao.alloc(0)), t.pipe(this), this;
      if (t.length || typeof t == 'object')
        return (
          (this.buffer = t),
          (this.writable = !1),
          A.nextTick(
            function () {
              this.emit('end', t), (this.readable = !1), this.emit('close');
            }.bind(this)
          ),
          this
        );
      throw new TypeError('Unexpected data type (' + typeof t + ')');
    }
    L5.inherits(Bo, D5);
    Bo.prototype.write = function (e) {
      (this.buffer = Ao.concat([this.buffer, Ao.from(e)])),
        this.emit('data', e);
    };
    Bo.prototype.end = function (e) {
      e && this.write(e),
        this.emit('end', e),
        this.emit('close'),
        (this.writable = !1),
        (this.readable = !1);
    };
    L1.exports = Bo;
  });
  var j1 = R((kR, F1) => {
    'use strict';
    S();
    var Kf = Et().Buffer,
      gh = Et().SlowBuffer;
    F1.exports = Io;
    function Io(t, e) {
      if (!Kf.isBuffer(t) || !Kf.isBuffer(e) || t.length !== e.length)
        return !1;
      for (var r = 0, n = 0; n < t.length; n++) r |= t[n] ^ e[n];
      return r === 0;
    }
    Io.install = function () {
      Kf.prototype.equal = gh.prototype.equal = function (e) {
        return Io(this, e);
      };
    };
    var F5 = Kf.prototype.equal,
      j5 = gh.prototype.equal;
    Io.restore = function () {
      (Kf.prototype.equal = F5), (gh.prototype.equal = j5);
    };
  });
  var Xi = R((CR, _h) => {
    'use strict';
    S();
    var wh = 65536,
      U5 = 4294967295;
    function z5() {
      throw new Error(`Secure random number generation is not supported by this browser.
Use Chrome, Firefox or Internet Explorer 11`);
    }
    var H5 = Ie().Buffer,
      Ro = globalThis.crypto || globalThis.msCrypto;
    Ro && Ro.getRandomValues ? (_h.exports = K5) : (_h.exports = z5);
    function K5(t, e) {
      if (t > U5) throw new RangeError('requested too many random bytes');
      var r = H5.allocUnsafe(t);
      if (t > 0)
        if (t > wh)
          for (var n = 0; n < t; n += wh)
            Ro.getRandomValues(r.slice(n, n + wh));
        else Ro.getRandomValues(r);
      return typeof e == 'function'
        ? A.nextTick(function () {
            e(null, r);
          })
        : r;
    }
  });
  var xh = R((Ot, U1) => {
    S();
    Ot = U1.exports = bo();
    Ot.Stream = Ot;
    Ot.Readable = Ot;
    Ot.Writable = lo();
    Ot.Duplex = wi();
    Ot.Transform = Eo();
    Ot.PassThrough = ph();
    Ot.finished = Lf();
    Ot.pipeline = bh();
  });
  var Mh = R((LR, H1) => {
    'use strict';
    S();
    var qo = Ie().Buffer,
      z1 = xh().Transform,
      V5 = qe();
    function G5(t, e) {
      if (!qo.isBuffer(t) && typeof t != 'string')
        throw new TypeError(e + ' must be a string or a buffer');
    }
    function Si(t) {
      z1.call(this),
        (this._block = qo.allocUnsafe(t)),
        (this._blockSize = t),
        (this._blockOffset = 0),
        (this._length = [0, 0, 0, 0]),
        (this._finalized = !1);
    }
    V5(Si, z1);
    Si.prototype._transform = function (t, e, r) {
      var n = null;
      try {
        this.update(t, e);
      } catch (i) {
        n = i;
      }
      r(n);
    };
    Si.prototype._flush = function (t) {
      var e = null;
      try {
        this.push(this.digest());
      } catch (r) {
        e = r;
      }
      t(e);
    };
    Si.prototype.update = function (t, e) {
      if ((G5(t, 'Data'), this._finalized))
        throw new Error('Digest already called');
      qo.isBuffer(t) || (t = qo.from(t, e));
      for (
        var r = this._block, n = 0;
        this._blockOffset + t.length - n >= this._blockSize;

      ) {
        for (var i = this._blockOffset; i < this._blockSize; ) r[i++] = t[n++];
        this._update(), (this._blockOffset = 0);
      }
      for (; n < t.length; ) r[this._blockOffset++] = t[n++];
      for (var a = 0, h = t.length * 8; h > 0; ++a)
        (this._length[a] += h),
          (h = (this._length[a] / 4294967296) | 0),
          h > 0 && (this._length[a] -= 4294967296 * h);
      return this;
    };
    Si.prototype._update = function () {
      throw new Error('_update is not implemented');
    };
    Si.prototype.digest = function (t) {
      if (this._finalized) throw new Error('Digest already called');
      this._finalized = !0;
      var e = this._digest();
      t !== void 0 && (e = e.toString(t)),
        this._block.fill(0),
        (this._blockOffset = 0);
      for (var r = 0; r < 4; ++r) this._length[r] = 0;
      return e;
    };
    Si.prototype._digest = function () {
      throw new Error('_digest is not implemented');
    };
    H1.exports = Si;
  });
  var ko = R((jR, V1) => {
    'use strict';
    S();
    var W5 = qe(),
      K1 = Mh(),
      $5 = Ie().Buffer,
      Z5 = new Array(16);
    function To() {
      K1.call(this, 64),
        (this._a = 1732584193),
        (this._b = 4023233417),
        (this._c = 2562383102),
        (this._d = 271733878);
    }
    W5(To, K1);
    To.prototype._update = function () {
      for (var t = Z5, e = 0; e < 16; ++e)
        t[e] = this._block.readInt32LE(e * 4);
      var r = this._a,
        n = this._b,
        i = this._c,
        a = this._d;
      (r = dt(r, n, i, a, t[0], 3614090360, 7)),
        (a = dt(a, r, n, i, t[1], 3905402710, 12)),
        (i = dt(i, a, r, n, t[2], 606105819, 17)),
        (n = dt(n, i, a, r, t[3], 3250441966, 22)),
        (r = dt(r, n, i, a, t[4], 4118548399, 7)),
        (a = dt(a, r, n, i, t[5], 1200080426, 12)),
        (i = dt(i, a, r, n, t[6], 2821735955, 17)),
        (n = dt(n, i, a, r, t[7], 4249261313, 22)),
        (r = dt(r, n, i, a, t[8], 1770035416, 7)),
        (a = dt(a, r, n, i, t[9], 2336552879, 12)),
        (i = dt(i, a, r, n, t[10], 4294925233, 17)),
        (n = dt(n, i, a, r, t[11], 2304563134, 22)),
        (r = dt(r, n, i, a, t[12], 1804603682, 7)),
        (a = dt(a, r, n, i, t[13], 4254626195, 12)),
        (i = dt(i, a, r, n, t[14], 2792965006, 17)),
        (n = dt(n, i, a, r, t[15], 1236535329, 22)),
        (r = lt(r, n, i, a, t[1], 4129170786, 5)),
        (a = lt(a, r, n, i, t[6], 3225465664, 9)),
        (i = lt(i, a, r, n, t[11], 643717713, 14)),
        (n = lt(n, i, a, r, t[0], 3921069994, 20)),
        (r = lt(r, n, i, a, t[5], 3593408605, 5)),
        (a = lt(a, r, n, i, t[10], 38016083, 9)),
        (i = lt(i, a, r, n, t[15], 3634488961, 14)),
        (n = lt(n, i, a, r, t[4], 3889429448, 20)),
        (r = lt(r, n, i, a, t[9], 568446438, 5)),
        (a = lt(a, r, n, i, t[14], 3275163606, 9)),
        (i = lt(i, a, r, n, t[3], 4107603335, 14)),
        (n = lt(n, i, a, r, t[8], 1163531501, 20)),
        (r = lt(r, n, i, a, t[13], 2850285829, 5)),
        (a = lt(a, r, n, i, t[2], 4243563512, 9)),
        (i = lt(i, a, r, n, t[7], 1735328473, 14)),
        (n = lt(n, i, a, r, t[12], 2368359562, 20)),
        (r = pt(r, n, i, a, t[5], 4294588738, 4)),
        (a = pt(a, r, n, i, t[8], 2272392833, 11)),
        (i = pt(i, a, r, n, t[11], 1839030562, 16)),
        (n = pt(n, i, a, r, t[14], 4259657740, 23)),
        (r = pt(r, n, i, a, t[1], 2763975236, 4)),
        (a = pt(a, r, n, i, t[4], 1272893353, 11)),
        (i = pt(i, a, r, n, t[7], 4139469664, 16)),
        (n = pt(n, i, a, r, t[10], 3200236656, 23)),
        (r = pt(r, n, i, a, t[13], 681279174, 4)),
        (a = pt(a, r, n, i, t[0], 3936430074, 11)),
        (i = pt(i, a, r, n, t[3], 3572445317, 16)),
        (n = pt(n, i, a, r, t[6], 76029189, 23)),
        (r = pt(r, n, i, a, t[9], 3654602809, 4)),
        (a = pt(a, r, n, i, t[12], 3873151461, 11)),
        (i = pt(i, a, r, n, t[15], 530742520, 16)),
        (n = pt(n, i, a, r, t[2], 3299628645, 23)),
        (r = vt(r, n, i, a, t[0], 4096336452, 6)),
        (a = vt(a, r, n, i, t[7], 1126891415, 10)),
        (i = vt(i, a, r, n, t[14], 2878612391, 15)),
        (n = vt(n, i, a, r, t[5], 4237533241, 21)),
        (r = vt(r, n, i, a, t[12], 1700485571, 6)),
        (a = vt(a, r, n, i, t[3], 2399980690, 10)),
        (i = vt(i, a, r, n, t[10], 4293915773, 15)),
        (n = vt(n, i, a, r, t[1], 2240044497, 21)),
        (r = vt(r, n, i, a, t[8], 1873313359, 6)),
        (a = vt(a, r, n, i, t[15], 4264355552, 10)),
        (i = vt(i, a, r, n, t[6], 2734768916, 15)),
        (n = vt(n, i, a, r, t[13], 1309151649, 21)),
        (r = vt(r, n, i, a, t[4], 4149444226, 6)),
        (a = vt(a, r, n, i, t[11], 3174756917, 10)),
        (i = vt(i, a, r, n, t[2], 718787259, 15)),
        (n = vt(n, i, a, r, t[9], 3951481745, 21)),
        (this._a = (this._a + r) | 0),
        (this._b = (this._b + n) | 0),
        (this._c = (this._c + i) | 0),
        (this._d = (this._d + a) | 0);
    };
    To.prototype._digest = function () {
      (this._block[this._blockOffset++] = 128),
        this._blockOffset > 56 &&
          (this._block.fill(0, this._blockOffset, 64),
          this._update(),
          (this._blockOffset = 0)),
        this._block.fill(0, this._blockOffset, 56),
        this._block.writeUInt32LE(this._length[0], 56),
        this._block.writeUInt32LE(this._length[1], 60),
        this._update();
      var t = $5.allocUnsafe(16);
      return (
        t.writeInt32LE(this._a, 0),
        t.writeInt32LE(this._b, 4),
        t.writeInt32LE(this._c, 8),
        t.writeInt32LE(this._d, 12),
        t
      );
    };
    function Po(t, e) {
      return (t << e) | (t >>> (32 - e));
    }
    function dt(t, e, r, n, i, a, h) {
      return (Po((t + ((e & r) | (~e & n)) + i + a) | 0, h) + e) | 0;
    }
    function lt(t, e, r, n, i, a, h) {
      return (Po((t + ((e & n) | (r & ~n)) + i + a) | 0, h) + e) | 0;
    }
    function pt(t, e, r, n, i, a, h) {
      return (Po((t + (e ^ r ^ n) + i + a) | 0, h) + e) | 0;
    }
    function vt(t, e, r, n, i, a, h) {
      return (Po((t + (r ^ (e | ~n)) + i + a) | 0, h) + e) | 0;
    }
    V1.exports = To;
  });
  var Co = R((zR, Y1) => {
    'use strict';
    S();
    var Sh = Et().Buffer,
      J5 = qe(),
      X1 = Mh(),
      X5 = new Array(16),
      Vf = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 7, 4, 13, 1, 10,
        6, 15, 3, 12, 0, 9, 5, 2, 14, 11, 8, 3, 10, 14, 4, 9, 15, 8, 1, 2, 7, 0,
        6, 13, 11, 5, 12, 1, 9, 11, 10, 0, 8, 12, 4, 13, 3, 7, 15, 14, 5, 6, 2,
        4, 0, 5, 9, 7, 12, 2, 10, 14, 1, 3, 8, 11, 6, 15, 13,
      ],
      Gf = [
        5, 14, 7, 0, 9, 2, 11, 4, 13, 6, 15, 8, 1, 10, 3, 12, 6, 11, 3, 7, 0,
        13, 5, 10, 14, 15, 8, 12, 4, 9, 1, 2, 15, 5, 1, 3, 7, 14, 6, 9, 11, 8,
        12, 2, 10, 0, 4, 13, 8, 6, 4, 1, 3, 11, 15, 0, 5, 12, 2, 13, 9, 7, 10,
        14, 12, 15, 10, 4, 1, 5, 8, 7, 6, 2, 13, 14, 0, 3, 9, 11,
      ],
      Wf = [
        11, 14, 15, 12, 5, 8, 7, 9, 11, 13, 14, 15, 6, 7, 9, 8, 7, 6, 8, 13, 11,
        9, 7, 15, 7, 12, 15, 9, 11, 7, 13, 12, 11, 13, 6, 7, 14, 9, 13, 15, 14,
        8, 13, 6, 5, 12, 7, 5, 11, 12, 14, 15, 14, 15, 9, 8, 9, 14, 5, 6, 8, 6,
        5, 12, 9, 15, 5, 11, 6, 8, 13, 12, 5, 12, 13, 14, 11, 8, 5, 6,
      ],
      $f = [
        8, 9, 9, 11, 13, 15, 15, 5, 7, 7, 8, 11, 14, 14, 12, 6, 9, 13, 15, 7,
        12, 8, 9, 11, 7, 7, 12, 7, 6, 15, 13, 11, 9, 7, 15, 11, 8, 6, 6, 14, 12,
        13, 5, 14, 13, 13, 7, 5, 15, 5, 8, 11, 14, 14, 6, 14, 6, 9, 12, 9, 12,
        5, 15, 8, 8, 5, 12, 9, 12, 5, 14, 6, 8, 13, 6, 5, 15, 13, 11, 11,
      ],
      Zf = [0, 1518500249, 1859775393, 2400959708, 2840853838],
      Jf = [1352829926, 1548603684, 1836072691, 2053994217, 0];
    function Oo() {
      X1.call(this, 64),
        (this._a = 1732584193),
        (this._b = 4023233417),
        (this._c = 2562383102),
        (this._d = 271733878),
        (this._e = 3285377520);
    }
    J5(Oo, X1);
    Oo.prototype._update = function () {
      for (var t = X5, e = 0; e < 16; ++e)
        t[e] = this._block.readInt32LE(e * 4);
      for (
        var r = this._a | 0,
          n = this._b | 0,
          i = this._c | 0,
          a = this._d | 0,
          h = this._e | 0,
          v = this._a | 0,
          g = this._b | 0,
          M = this._c | 0,
          x = this._d | 0,
          E = this._e | 0,
          I = 0;
        I < 80;
        I += 1
      ) {
        var q, k;
        I < 16
          ? ((q = G1(r, n, i, a, h, t[Vf[I]], Zf[0], Wf[I])),
            (k = J1(v, g, M, x, E, t[Gf[I]], Jf[0], $f[I])))
          : I < 32
            ? ((q = W1(r, n, i, a, h, t[Vf[I]], Zf[1], Wf[I])),
              (k = Z1(v, g, M, x, E, t[Gf[I]], Jf[1], $f[I])))
            : I < 48
              ? ((q = $1(r, n, i, a, h, t[Vf[I]], Zf[2], Wf[I])),
                (k = $1(v, g, M, x, E, t[Gf[I]], Jf[2], $f[I])))
              : I < 64
                ? ((q = Z1(r, n, i, a, h, t[Vf[I]], Zf[3], Wf[I])),
                  (k = W1(v, g, M, x, E, t[Gf[I]], Jf[3], $f[I])))
                : ((q = J1(r, n, i, a, h, t[Vf[I]], Zf[4], Wf[I])),
                  (k = G1(v, g, M, x, E, t[Gf[I]], Jf[4], $f[I]))),
          (r = h),
          (h = a),
          (a = Yi(i, 10)),
          (i = n),
          (n = q),
          (v = E),
          (E = x),
          (x = Yi(M, 10)),
          (M = g),
          (g = k);
      }
      var L = (this._b + i + x) | 0;
      (this._b = (this._c + a + E) | 0),
        (this._c = (this._d + h + v) | 0),
        (this._d = (this._e + r + g) | 0),
        (this._e = (this._a + n + M) | 0),
        (this._a = L);
    };
    Oo.prototype._digest = function () {
      (this._block[this._blockOffset++] = 128),
        this._blockOffset > 56 &&
          (this._block.fill(0, this._blockOffset, 64),
          this._update(),
          (this._blockOffset = 0)),
        this._block.fill(0, this._blockOffset, 56),
        this._block.writeUInt32LE(this._length[0], 56),
        this._block.writeUInt32LE(this._length[1], 60),
        this._update();
      var t = Sh.alloc ? Sh.alloc(20) : new Sh(20);
      return (
        t.writeInt32LE(this._a, 0),
        t.writeInt32LE(this._b, 4),
        t.writeInt32LE(this._c, 8),
        t.writeInt32LE(this._d, 12),
        t.writeInt32LE(this._e, 16),
        t
      );
    };
    function Yi(t, e) {
      return (t << e) | (t >>> (32 - e));
    }
    function G1(t, e, r, n, i, a, h, v) {
      return (Yi((t + (e ^ r ^ n) + a + h) | 0, v) + i) | 0;
    }
    function W1(t, e, r, n, i, a, h, v) {
      return (Yi((t + ((e & r) | (~e & n)) + a + h) | 0, v) + i) | 0;
    }
    function $1(t, e, r, n, i, a, h, v) {
      return (Yi((t + ((e | ~r) ^ n) + a + h) | 0, v) + i) | 0;
    }
    function Z1(t, e, r, n, i, a, h, v) {
      return (Yi((t + ((e & n) | (r & ~n)) + a + h) | 0, v) + i) | 0;
    }
    function J1(t, e, r, n, i, a, h, v) {
      return (Yi((t + (e ^ (r | ~n)) + a + h) | 0, v) + i) | 0;
    }
    Y1.exports = Oo;
  });
  var Qi = R((KR, ep) => {
    S();
    var Q1 = Ie().Buffer;
    function No(t, e) {
      (this._block = Q1.alloc(t)),
        (this._finalSize = e),
        (this._blockSize = t),
        (this._len = 0);
    }
    No.prototype.update = function (t, e) {
      typeof t == 'string' && ((e = e || 'utf8'), (t = Q1.from(t, e)));
      for (
        var r = this._block,
          n = this._blockSize,
          i = t.length,
          a = this._len,
          h = 0;
        h < i;

      ) {
        for (var v = a % n, g = Math.min(i - h, n - v), M = 0; M < g; M++)
          r[v + M] = t[h + M];
        (a += g), (h += g), a % n === 0 && this._update(r);
      }
      return (this._len += i), this;
    };
    No.prototype.digest = function (t) {
      var e = this._len % this._blockSize;
      (this._block[e] = 128),
        this._block.fill(0, e + 1),
        e >= this._finalSize &&
          (this._update(this._block), this._block.fill(0));
      var r = this._len * 8;
      if (r <= 4294967295) this._block.writeUInt32BE(r, this._blockSize - 4);
      else {
        var n = (r & 4294967295) >>> 0,
          i = (r - n) / 4294967296;
        this._block.writeUInt32BE(i, this._blockSize - 8),
          this._block.writeUInt32BE(n, this._blockSize - 4);
      }
      this._update(this._block);
      var a = this._hash();
      return t ? a.toString(t) : a;
    };
    No.prototype._update = function () {
      throw new Error('_update must be implemented by subclass');
    };
    ep.exports = No;
  });
  var ip = R((GR, rp) => {
    S();
    var Y5 = qe(),
      tp = Qi(),
      Q5 = Ie().Buffer,
      e7 = [1518500249, 1859775393, -1894007588, -899497514],
      t7 = new Array(80);
    function Xf() {
      this.init(), (this._w = t7), tp.call(this, 64, 56);
    }
    Y5(Xf, tp);
    Xf.prototype.init = function () {
      return (
        (this._a = 1732584193),
        (this._b = 4023233417),
        (this._c = 2562383102),
        (this._d = 271733878),
        (this._e = 3285377520),
        this
      );
    };
    function r7(t) {
      return (t << 5) | (t >>> 27);
    }
    function i7(t) {
      return (t << 30) | (t >>> 2);
    }
    function n7(t, e, r, n) {
      return t === 0
        ? (e & r) | (~e & n)
        : t === 2
          ? (e & r) | (e & n) | (r & n)
          : e ^ r ^ n;
    }
    Xf.prototype._update = function (t) {
      for (
        var e = this._w,
          r = this._a | 0,
          n = this._b | 0,
          i = this._c | 0,
          a = this._d | 0,
          h = this._e | 0,
          v = 0;
        v < 16;
        ++v
      )
        e[v] = t.readInt32BE(v * 4);
      for (; v < 80; ++v) e[v] = e[v - 3] ^ e[v - 8] ^ e[v - 14] ^ e[v - 16];
      for (var g = 0; g < 80; ++g) {
        var M = ~~(g / 20),
          x = (r7(r) + n7(M, n, i, a) + h + e[g] + e7[M]) | 0;
        (h = a), (a = i), (i = i7(n)), (n = r), (r = x);
      }
      (this._a = (r + this._a) | 0),
        (this._b = (n + this._b) | 0),
        (this._c = (i + this._c) | 0),
        (this._d = (a + this._d) | 0),
        (this._e = (h + this._e) | 0);
    };
    Xf.prototype._hash = function () {
      var t = Q5.allocUnsafe(20);
      return (
        t.writeInt32BE(this._a | 0, 0),
        t.writeInt32BE(this._b | 0, 4),
        t.writeInt32BE(this._c | 0, 8),
        t.writeInt32BE(this._d | 0, 12),
        t.writeInt32BE(this._e | 0, 16),
        t
      );
    };
    rp.exports = Xf;
  });
  var ap = R(($R, fp) => {
    S();
    var f7 = qe(),
      np = Qi(),
      a7 = Ie().Buffer,
      o7 = [1518500249, 1859775393, -1894007588, -899497514],
      s7 = new Array(80);
    function Yf() {
      this.init(), (this._w = s7), np.call(this, 64, 56);
    }
    f7(Yf, np);
    Yf.prototype.init = function () {
      return (
        (this._a = 1732584193),
        (this._b = 4023233417),
        (this._c = 2562383102),
        (this._d = 271733878),
        (this._e = 3285377520),
        this
      );
    };
    function h7(t) {
      return (t << 1) | (t >>> 31);
    }
    function u7(t) {
      return (t << 5) | (t >>> 27);
    }
    function c7(t) {
      return (t << 30) | (t >>> 2);
    }
    function d7(t, e, r, n) {
      return t === 0
        ? (e & r) | (~e & n)
        : t === 2
          ? (e & r) | (e & n) | (r & n)
          : e ^ r ^ n;
    }
    Yf.prototype._update = function (t) {
      for (
        var e = this._w,
          r = this._a | 0,
          n = this._b | 0,
          i = this._c | 0,
          a = this._d | 0,
          h = this._e | 0,
          v = 0;
        v < 16;
        ++v
      )
        e[v] = t.readInt32BE(v * 4);
      for (; v < 80; ++v)
        e[v] = h7(e[v - 3] ^ e[v - 8] ^ e[v - 14] ^ e[v - 16]);
      for (var g = 0; g < 80; ++g) {
        var M = ~~(g / 20),
          x = (u7(r) + d7(M, n, i, a) + h + e[g] + o7[M]) | 0;
        (h = a), (a = i), (i = c7(n)), (n = r), (r = x);
      }
      (this._a = (r + this._a) | 0),
        (this._b = (n + this._b) | 0),
        (this._c = (i + this._c) | 0),
        (this._d = (a + this._d) | 0),
        (this._e = (h + this._e) | 0);
    };
    Yf.prototype._hash = function () {
      var t = a7.allocUnsafe(20);
      return (
        t.writeInt32BE(this._a | 0, 0),
        t.writeInt32BE(this._b | 0, 4),
        t.writeInt32BE(this._c | 0, 8),
        t.writeInt32BE(this._d | 0, 12),
        t.writeInt32BE(this._e | 0, 16),
        t
      );
    };
    fp.exports = Yf;
  });
  var Eh = R((JR, sp) => {
    S();
    var l7 = qe(),
      op = Qi(),
      p7 = Ie().Buffer,
      v7 = [
        1116352408, 1899447441, 3049323471, 3921009573, 961987163, 1508970993,
        2453635748, 2870763221, 3624381080, 310598401, 607225278, 1426881987,
        1925078388, 2162078206, 2614888103, 3248222580, 3835390401, 4022224774,
        264347078, 604807628, 770255983, 1249150122, 1555081692, 1996064986,
        2554220882, 2821834349, 2952996808, 3210313671, 3336571891, 3584528711,
        113926993, 338241895, 666307205, 773529912, 1294757372, 1396182291,
        1695183700, 1986661051, 2177026350, 2456956037, 2730485921, 2820302411,
        3259730800, 3345764771, 3516065817, 3600352804, 4094571909, 275423344,
        430227734, 506948616, 659060556, 883997877, 958139571, 1322822218,
        1537002063, 1747873779, 1955562222, 2024104815, 2227730452, 2361852424,
        2428436474, 2756734187, 3204031479, 3329325298,
      ],
      b7 = new Array(64);
    function Qf() {
      this.init(), (this._w = b7), op.call(this, 64, 56);
    }
    l7(Qf, op);
    Qf.prototype.init = function () {
      return (
        (this._a = 1779033703),
        (this._b = 3144134277),
        (this._c = 1013904242),
        (this._d = 2773480762),
        (this._e = 1359893119),
        (this._f = 2600822924),
        (this._g = 528734635),
        (this._h = 1541459225),
        this
      );
    };
    function y7(t, e, r) {
      return r ^ (t & (e ^ r));
    }
    function m7(t, e, r) {
      return (t & e) | (r & (t | e));
    }
    function g7(t) {
      return (
        ((t >>> 2) | (t << 30)) ^
        ((t >>> 13) | (t << 19)) ^
        ((t >>> 22) | (t << 10))
      );
    }
    function w7(t) {
      return (
        ((t >>> 6) | (t << 26)) ^
        ((t >>> 11) | (t << 21)) ^
        ((t >>> 25) | (t << 7))
      );
    }
    function _7(t) {
      return ((t >>> 7) | (t << 25)) ^ ((t >>> 18) | (t << 14)) ^ (t >>> 3);
    }
    function x7(t) {
      return ((t >>> 17) | (t << 15)) ^ ((t >>> 19) | (t << 13)) ^ (t >>> 10);
    }
    Qf.prototype._update = function (t) {
      for (
        var e = this._w,
          r = this._a | 0,
          n = this._b | 0,
          i = this._c | 0,
          a = this._d | 0,
          h = this._e | 0,
          v = this._f | 0,
          g = this._g | 0,
          M = this._h | 0,
          x = 0;
        x < 16;
        ++x
      )
        e[x] = t.readInt32BE(x * 4);
      for (; x < 64; ++x)
        e[x] = (x7(e[x - 2]) + e[x - 7] + _7(e[x - 15]) + e[x - 16]) | 0;
      for (var E = 0; E < 64; ++E) {
        var I = (M + w7(h) + y7(h, v, g) + v7[E] + e[E]) | 0,
          q = (g7(r) + m7(r, n, i)) | 0;
        (M = g),
          (g = v),
          (v = h),
          (h = (a + I) | 0),
          (a = i),
          (i = n),
          (n = r),
          (r = (I + q) | 0);
      }
      (this._a = (r + this._a) | 0),
        (this._b = (n + this._b) | 0),
        (this._c = (i + this._c) | 0),
        (this._d = (a + this._d) | 0),
        (this._e = (h + this._e) | 0),
        (this._f = (v + this._f) | 0),
        (this._g = (g + this._g) | 0),
        (this._h = (M + this._h) | 0);
    };
    Qf.prototype._hash = function () {
      var t = p7.allocUnsafe(32);
      return (
        t.writeInt32BE(this._a, 0),
        t.writeInt32BE(this._b, 4),
        t.writeInt32BE(this._c, 8),
        t.writeInt32BE(this._d, 12),
        t.writeInt32BE(this._e, 16),
        t.writeInt32BE(this._f, 20),
        t.writeInt32BE(this._g, 24),
        t.writeInt32BE(this._h, 28),
        t
      );
    };
    sp.exports = Qf;
  });
  var up = R((YR, hp) => {
    S();
    var M7 = qe(),
      S7 = Eh(),
      E7 = Qi(),
      A7 = Ie().Buffer,
      B7 = new Array(64);
    function Do() {
      this.init(), (this._w = B7), E7.call(this, 64, 56);
    }
    M7(Do, S7);
    Do.prototype.init = function () {
      return (
        (this._a = 3238371032),
        (this._b = 914150663),
        (this._c = 812702999),
        (this._d = 4144912697),
        (this._e = 4290775857),
        (this._f = 1750603025),
        (this._g = 1694076839),
        (this._h = 3204075428),
        this
      );
    };
    Do.prototype._hash = function () {
      var t = A7.allocUnsafe(28);
      return (
        t.writeInt32BE(this._a, 0),
        t.writeInt32BE(this._b, 4),
        t.writeInt32BE(this._c, 8),
        t.writeInt32BE(this._d, 12),
        t.writeInt32BE(this._e, 16),
        t.writeInt32BE(this._f, 20),
        t.writeInt32BE(this._g, 24),
        t
      );
    };
    hp.exports = Do;
  });
  var Ah = R((eq, yp) => {
    S();
    var I7 = qe(),
      bp = Qi(),
      R7 = Ie().Buffer,
      cp = [
        1116352408, 3609767458, 1899447441, 602891725, 3049323471, 3964484399,
        3921009573, 2173295548, 961987163, 4081628472, 1508970993, 3053834265,
        2453635748, 2937671579, 2870763221, 3664609560, 3624381080, 2734883394,
        310598401, 1164996542, 607225278, 1323610764, 1426881987, 3590304994,
        1925078388, 4068182383, 2162078206, 991336113, 2614888103, 633803317,
        3248222580, 3479774868, 3835390401, 2666613458, 4022224774, 944711139,
        264347078, 2341262773, 604807628, 2007800933, 770255983, 1495990901,
        1249150122, 1856431235, 1555081692, 3175218132, 1996064986, 2198950837,
        2554220882, 3999719339, 2821834349, 766784016, 2952996808, 2566594879,
        3210313671, 3203337956, 3336571891, 1034457026, 3584528711, 2466948901,
        113926993, 3758326383, 338241895, 168717936, 666307205, 1188179964,
        773529912, 1546045734, 1294757372, 1522805485, 1396182291, 2643833823,
        1695183700, 2343527390, 1986661051, 1014477480, 2177026350, 1206759142,
        2456956037, 344077627, 2730485921, 1290863460, 2820302411, 3158454273,
        3259730800, 3505952657, 3345764771, 106217008, 3516065817, 3606008344,
        3600352804, 1432725776, 4094571909, 1467031594, 275423344, 851169720,
        430227734, 3100823752, 506948616, 1363258195, 659060556, 3750685593,
        883997877, 3785050280, 958139571, 3318307427, 1322822218, 3812723403,
        1537002063, 2003034995, 1747873779, 3602036899, 1955562222, 1575990012,
        2024104815, 1125592928, 2227730452, 2716904306, 2361852424, 442776044,
        2428436474, 593698344, 2756734187, 3733110249, 3204031479, 2999351573,
        3329325298, 3815920427, 3391569614, 3928383900, 3515267271, 566280711,
        3940187606, 3454069534, 4118630271, 4000239992, 116418474, 1914138554,
        174292421, 2731055270, 289380356, 3203993006, 460393269, 320620315,
        685471733, 587496836, 852142971, 1086792851, 1017036298, 365543100,
        1126000580, 2618297676, 1288033470, 3409855158, 1501505948, 4234509866,
        1607167915, 987167468, 1816402316, 1246189591,
      ],
      q7 = new Array(160);
    function ea() {
      this.init(), (this._w = q7), bp.call(this, 128, 112);
    }
    I7(ea, bp);
    ea.prototype.init = function () {
      return (
        (this._ah = 1779033703),
        (this._bh = 3144134277),
        (this._ch = 1013904242),
        (this._dh = 2773480762),
        (this._eh = 1359893119),
        (this._fh = 2600822924),
        (this._gh = 528734635),
        (this._hh = 1541459225),
        (this._al = 4089235720),
        (this._bl = 2227873595),
        (this._cl = 4271175723),
        (this._dl = 1595750129),
        (this._el = 2917565137),
        (this._fl = 725511199),
        (this._gl = 4215389547),
        (this._hl = 327033209),
        this
      );
    };
    function dp(t, e, r) {
      return r ^ (t & (e ^ r));
    }
    function lp(t, e, r) {
      return (t & e) | (r & (t | e));
    }
    function pp(t, e) {
      return (
        ((t >>> 28) | (e << 4)) ^
        ((e >>> 2) | (t << 30)) ^
        ((e >>> 7) | (t << 25))
      );
    }
    function vp(t, e) {
      return (
        ((t >>> 14) | (e << 18)) ^
        ((t >>> 18) | (e << 14)) ^
        ((e >>> 9) | (t << 23))
      );
    }
    function T7(t, e) {
      return ((t >>> 1) | (e << 31)) ^ ((t >>> 8) | (e << 24)) ^ (t >>> 7);
    }
    function P7(t, e) {
      return (
        ((t >>> 1) | (e << 31)) ^
        ((t >>> 8) | (e << 24)) ^
        ((t >>> 7) | (e << 25))
      );
    }
    function k7(t, e) {
      return ((t >>> 19) | (e << 13)) ^ ((e >>> 29) | (t << 3)) ^ (t >>> 6);
    }
    function O7(t, e) {
      return (
        ((t >>> 19) | (e << 13)) ^
        ((e >>> 29) | (t << 3)) ^
        ((t >>> 6) | (e << 26))
      );
    }
    function ft(t, e) {
      return t >>> 0 < e >>> 0 ? 1 : 0;
    }
    ea.prototype._update = function (t) {
      for (
        var e = this._w,
          r = this._ah | 0,
          n = this._bh | 0,
          i = this._ch | 0,
          a = this._dh | 0,
          h = this._eh | 0,
          v = this._fh | 0,
          g = this._gh | 0,
          M = this._hh | 0,
          x = this._al | 0,
          E = this._bl | 0,
          I = this._cl | 0,
          q = this._dl | 0,
          k = this._el | 0,
          L = this._fl | 0,
          xe = this._gl | 0,
          U = this._hl | 0,
          Me = 0;
        Me < 32;
        Me += 2
      )
        (e[Me] = t.readInt32BE(Me * 4)),
          (e[Me + 1] = t.readInt32BE(Me * 4 + 4));
      for (; Me < 160; Me += 2) {
        var Te = e[Me - 30],
          Ee = e[Me - 15 * 2 + 1],
          Fe = T7(Te, Ee),
          Se = P7(Ee, Te);
        (Te = e[Me - 2 * 2]), (Ee = e[Me - 2 * 2 + 1]);
        var $e = k7(Te, Ee),
          ke = O7(Ee, Te),
          Ze = e[Me - 7 * 2],
          B = e[Me - 7 * 2 + 1],
          b = e[Me - 16 * 2],
          _ = e[Me - 16 * 2 + 1],
          l = (Se + B) | 0,
          f = (Fe + Ze + ft(l, Se)) | 0;
        (l = (l + ke) | 0),
          (f = (f + $e + ft(l, ke)) | 0),
          (l = (l + _) | 0),
          (f = (f + b + ft(l, _)) | 0),
          (e[Me] = f),
          (e[Me + 1] = l);
      }
      for (var o = 0; o < 160; o += 2) {
        (f = e[o]), (l = e[o + 1]);
        var c = lp(r, n, i),
          p = lp(x, E, I),
          d = pp(r, x),
          u = pp(x, r),
          y = vp(h, k),
          m = vp(k, h),
          s = cp[o],
          w = cp[o + 1],
          T = dp(h, v, g),
          O = dp(k, L, xe),
          P = (U + m) | 0,
          N = (M + y + ft(P, U)) | 0;
        (P = (P + O) | 0),
          (N = (N + T + ft(P, O)) | 0),
          (P = (P + w) | 0),
          (N = (N + s + ft(P, w)) | 0),
          (P = (P + l) | 0),
          (N = (N + f + ft(P, l)) | 0);
        var F = (u + p) | 0,
          j = (d + c + ft(F, u)) | 0;
        (M = g),
          (U = xe),
          (g = v),
          (xe = L),
          (v = h),
          (L = k),
          (k = (q + P) | 0),
          (h = (a + N + ft(k, q)) | 0),
          (a = i),
          (q = I),
          (i = n),
          (I = E),
          (n = r),
          (E = x),
          (x = (P + F) | 0),
          (r = (N + j + ft(x, P)) | 0);
      }
      (this._al = (this._al + x) | 0),
        (this._bl = (this._bl + E) | 0),
        (this._cl = (this._cl + I) | 0),
        (this._dl = (this._dl + q) | 0),
        (this._el = (this._el + k) | 0),
        (this._fl = (this._fl + L) | 0),
        (this._gl = (this._gl + xe) | 0),
        (this._hl = (this._hl + U) | 0),
        (this._ah = (this._ah + r + ft(this._al, x)) | 0),
        (this._bh = (this._bh + n + ft(this._bl, E)) | 0),
        (this._ch = (this._ch + i + ft(this._cl, I)) | 0),
        (this._dh = (this._dh + a + ft(this._dl, q)) | 0),
        (this._eh = (this._eh + h + ft(this._el, k)) | 0),
        (this._fh = (this._fh + v + ft(this._fl, L)) | 0),
        (this._gh = (this._gh + g + ft(this._gl, xe)) | 0),
        (this._hh = (this._hh + M + ft(this._hl, U)) | 0);
    };
    ea.prototype._hash = function () {
      var t = R7.allocUnsafe(64);
      function e(r, n, i) {
        t.writeInt32BE(r, i), t.writeInt32BE(n, i + 4);
      }
      return (
        e(this._ah, this._al, 0),
        e(this._bh, this._bl, 8),
        e(this._ch, this._cl, 16),
        e(this._dh, this._dl, 24),
        e(this._eh, this._el, 32),
        e(this._fh, this._fl, 40),
        e(this._gh, this._gl, 48),
        e(this._hh, this._hl, 56),
        t
      );
    };
    yp.exports = ea;
  });
  var gp = R((rq, mp) => {
    S();
    var C7 = qe(),
      N7 = Ah(),
      D7 = Qi(),
      L7 = Ie().Buffer,
      F7 = new Array(160);
    function Lo() {
      this.init(), (this._w = F7), D7.call(this, 128, 112);
    }
    C7(Lo, N7);
    Lo.prototype.init = function () {
      return (
        (this._ah = 3418070365),
        (this._bh = 1654270250),
        (this._ch = 2438529370),
        (this._dh = 355462360),
        (this._eh = 1731405415),
        (this._fh = 2394180231),
        (this._gh = 3675008525),
        (this._hh = 1203062813),
        (this._al = 3238371032),
        (this._bl = 914150663),
        (this._cl = 812702999),
        (this._dl = 4144912697),
        (this._el = 4290775857),
        (this._fl = 1750603025),
        (this._gl = 1694076839),
        (this._hl = 3204075428),
        this
      );
    };
    Lo.prototype._hash = function () {
      var t = L7.allocUnsafe(48);
      function e(r, n, i) {
        t.writeInt32BE(r, i), t.writeInt32BE(n, i + 4);
      }
      return (
        e(this._ah, this._al, 0),
        e(this._bh, this._bl, 8),
        e(this._ch, this._cl, 16),
        e(this._dh, this._dl, 24),
        e(this._eh, this._el, 32),
        e(this._fh, this._fl, 40),
        t
      );
    };
    mp.exports = Lo;
  });
  var Fo = R((Ur, wp) => {
    S();
    var Ur = (wp.exports = function (e) {
      e = e.toLowerCase();
      var r = Ur[e];
      if (!r)
        throw new Error(e + ' is not supported (we accept pull requests)');
      return new r();
    });
    Ur.sha = ip();
    Ur.sha1 = ap();
    Ur.sha224 = up();
    Ur.sha256 = Eh();
    Ur.sha384 = gp();
    Ur.sha512 = Ah();
  });
  var zr = R((fq, Mp) => {
    S();
    var _p = Ie().Buffer,
      xp = Hf().Transform,
      j7 = yo().StringDecoder,
      U7 = qe();
    function cr(t) {
      xp.call(this),
        (this.hashMode = typeof t == 'string'),
        this.hashMode
          ? (this[t] = this._finalOrDigest)
          : (this.final = this._finalOrDigest),
        this._final && ((this.__final = this._final), (this._final = null)),
        (this._decoder = null),
        (this._encoding = null);
    }
    U7(cr, xp);
    cr.prototype.update = function (t, e, r) {
      typeof t == 'string' && (t = _p.from(t, e));
      var n = this._update(t);
      return this.hashMode ? this : (r && (n = this._toString(n, r)), n);
    };
    cr.prototype.setAutoPadding = function () {};
    cr.prototype.getAuthTag = function () {
      throw new Error('trying to get auth tag in unsupported state');
    };
    cr.prototype.setAuthTag = function () {
      throw new Error('trying to set auth tag in unsupported state');
    };
    cr.prototype.setAAD = function () {
      throw new Error('trying to set aad in unsupported state');
    };
    cr.prototype._transform = function (t, e, r) {
      var n;
      try {
        this.hashMode ? this._update(t) : this.push(this._update(t));
      } catch (i) {
        n = i;
      } finally {
        r(n);
      }
    };
    cr.prototype._flush = function (t) {
      var e;
      try {
        this.push(this.__final());
      } catch (r) {
        e = r;
      }
      t(e);
    };
    cr.prototype._finalOrDigest = function (t) {
      var e = this.__final() || _p.alloc(0);
      return t && (e = this._toString(e, t, !0)), e;
    };
    cr.prototype._toString = function (t, e, r) {
      if (
        (this._decoder || ((this._decoder = new j7(e)), (this._encoding = e)),
        this._encoding !== e)
      )
        throw new Error("can't switch encodings");
      var n = this._decoder.write(t);
      return r && (n += this._decoder.end()), n;
    };
    Mp.exports = cr;
  });
  var zn = R((oq, Ep) => {
    'use strict';
    S();
    var z7 = qe(),
      H7 = ko(),
      K7 = Co(),
      V7 = Fo(),
      Sp = zr();
    function jo(t) {
      Sp.call(this, 'digest'), (this._hash = t);
    }
    z7(jo, Sp);
    jo.prototype._update = function (t) {
      this._hash.update(t);
    };
    jo.prototype._final = function () {
      return this._hash.digest();
    };
    Ep.exports = function (e) {
      return (
        (e = e.toLowerCase()),
        e === 'md5'
          ? new H7()
          : e === 'rmd160' || e === 'ripemd160'
            ? new K7()
            : new jo(V7(e))
      );
    };
  });
  var Ip = R((hq, Bp) => {
    'use strict';
    S();
    var G7 = qe(),
      en = Ie().Buffer,
      Ap = zr(),
      W7 = en.alloc(128),
      Hn = 64;
    function Uo(t, e) {
      Ap.call(this, 'digest'),
        typeof e == 'string' && (e = en.from(e)),
        (this._alg = t),
        (this._key = e),
        e.length > Hn
          ? (e = t(e))
          : e.length < Hn && (e = en.concat([e, W7], Hn));
      for (
        var r = (this._ipad = en.allocUnsafe(Hn)),
          n = (this._opad = en.allocUnsafe(Hn)),
          i = 0;
        i < Hn;
        i++
      )
        (r[i] = e[i] ^ 54), (n[i] = e[i] ^ 92);
      this._hash = [r];
    }
    G7(Uo, Ap);
    Uo.prototype._update = function (t) {
      this._hash.push(t);
    };
    Uo.prototype._final = function () {
      var t = this._alg(en.concat(this._hash));
      return this._alg(en.concat([this._opad, t]));
    };
    Bp.exports = Uo;
  });
  var Bh = R((cq, Rp) => {
    S();
    var $7 = ko();
    Rp.exports = function (t) {
      return new $7().update(t).digest();
    };
  });
  var qh = R((lq, Tp) => {
    'use strict';
    S();
    var Z7 = qe(),
      J7 = Ip(),
      qp = zr(),
      ta = Ie().Buffer,
      X7 = Bh(),
      Ih = Co(),
      Rh = Fo(),
      Y7 = ta.alloc(128);
    function ra(t, e) {
      qp.call(this, 'digest'), typeof e == 'string' && (e = ta.from(e));
      var r = t === 'sha512' || t === 'sha384' ? 128 : 64;
      if (((this._alg = t), (this._key = e), e.length > r)) {
        var n = t === 'rmd160' ? new Ih() : Rh(t);
        e = n.update(e).digest();
      } else e.length < r && (e = ta.concat([e, Y7], r));
      for (
        var i = (this._ipad = ta.allocUnsafe(r)),
          a = (this._opad = ta.allocUnsafe(r)),
          h = 0;
        h < r;
        h++
      )
        (i[h] = e[h] ^ 54), (a[h] = e[h] ^ 92);
      (this._hash = t === 'rmd160' ? new Ih() : Rh(t)), this._hash.update(i);
    }
    Z7(ra, qp);
    ra.prototype._update = function (t) {
      this._hash.update(t);
    };
    ra.prototype._final = function () {
      var t = this._hash.digest(),
        e = this._alg === 'rmd160' ? new Ih() : Rh(this._alg);
      return e.update(this._opad).update(t).digest();
    };
    Tp.exports = function (e, r) {
      return (
        (e = e.toLowerCase()),
        e === 'rmd160' || e === 'ripemd160'
          ? new ra('rmd160', r)
          : e === 'md5'
            ? new J7(X7, r)
            : new ra(e, r)
      );
    };
  });
  var Th = R((vq, Q7) => {
    Q7.exports = {
      sha224WithRSAEncryption: {
        sign: 'rsa',
        hash: 'sha224',
        id: '302d300d06096086480165030402040500041c',
      },
      'RSA-SHA224': {
        sign: 'ecdsa/rsa',
        hash: 'sha224',
        id: '302d300d06096086480165030402040500041c',
      },
      sha256WithRSAEncryption: {
        sign: 'rsa',
        hash: 'sha256',
        id: '3031300d060960864801650304020105000420',
      },
      'RSA-SHA256': {
        sign: 'ecdsa/rsa',
        hash: 'sha256',
        id: '3031300d060960864801650304020105000420',
      },
      sha384WithRSAEncryption: {
        sign: 'rsa',
        hash: 'sha384',
        id: '3041300d060960864801650304020205000430',
      },
      'RSA-SHA384': {
        sign: 'ecdsa/rsa',
        hash: 'sha384',
        id: '3041300d060960864801650304020205000430',
      },
      sha512WithRSAEncryption: {
        sign: 'rsa',
        hash: 'sha512',
        id: '3051300d060960864801650304020305000440',
      },
      'RSA-SHA512': {
        sign: 'ecdsa/rsa',
        hash: 'sha512',
        id: '3051300d060960864801650304020305000440',
      },
      'RSA-SHA1': {
        sign: 'rsa',
        hash: 'sha1',
        id: '3021300906052b0e03021a05000414',
      },
      'ecdsa-with-SHA1': { sign: 'ecdsa', hash: 'sha1', id: '' },
      sha256: { sign: 'ecdsa', hash: 'sha256', id: '' },
      sha224: { sign: 'ecdsa', hash: 'sha224', id: '' },
      sha384: { sign: 'ecdsa', hash: 'sha384', id: '' },
      sha512: { sign: 'ecdsa', hash: 'sha512', id: '' },
      'DSA-SHA': { sign: 'dsa', hash: 'sha1', id: '' },
      'DSA-SHA1': { sign: 'dsa', hash: 'sha1', id: '' },
      DSA: { sign: 'dsa', hash: 'sha1', id: '' },
      'DSA-WITH-SHA224': { sign: 'dsa', hash: 'sha224', id: '' },
      'DSA-SHA224': { sign: 'dsa', hash: 'sha224', id: '' },
      'DSA-WITH-SHA256': { sign: 'dsa', hash: 'sha256', id: '' },
      'DSA-SHA256': { sign: 'dsa', hash: 'sha256', id: '' },
      'DSA-WITH-SHA384': { sign: 'dsa', hash: 'sha384', id: '' },
      'DSA-SHA384': { sign: 'dsa', hash: 'sha384', id: '' },
      'DSA-WITH-SHA512': { sign: 'dsa', hash: 'sha512', id: '' },
      'DSA-SHA512': { sign: 'dsa', hash: 'sha512', id: '' },
      'DSA-RIPEMD160': { sign: 'dsa', hash: 'rmd160', id: '' },
      ripemd160WithRSA: {
        sign: 'rsa',
        hash: 'rmd160',
        id: '3021300906052b2403020105000414',
      },
      'RSA-RIPEMD160': {
        sign: 'rsa',
        hash: 'rmd160',
        id: '3021300906052b2403020105000414',
      },
      md5WithRSAEncryption: {
        sign: 'rsa',
        hash: 'md5',
        id: '3020300c06082a864886f70d020505000410',
      },
      'RSA-MD5': {
        sign: 'rsa',
        hash: 'md5',
        id: '3020300c06082a864886f70d020505000410',
      },
    };
  });
  var kp = R((bq, Pp) => {
    S();
    Pp.exports = Th();
  });
  var Ph = R((mq, Op) => {
    S();
    var e9 = Math.pow(2, 30) - 1;
    Op.exports = function (t, e) {
      if (typeof t != 'number') throw new TypeError('Iterations not a number');
      if (t < 0) throw new TypeError('Bad iterations');
      if (typeof e != 'number') throw new TypeError('Key length not a number');
      if (e < 0 || e > e9 || e !== e) throw new TypeError('Bad key length');
    };
  });
  var kh = R((wq, Np) => {
    S();
    var zo;
    globalThis.process && globalThis.process.browser
      ? (zo = 'utf-8')
      : globalThis.process && globalThis.process.version
        ? ((Cp = parseInt(process.version.split('.')[0].slice(1), 10)),
          (zo = Cp >= 6 ? 'utf-8' : 'binary'))
        : (zo = 'utf-8');
    var Cp;
    Np.exports = zo;
  });
  var Ch = R((xq, Dp) => {
    S();
    var Oh = Ie().Buffer;
    Dp.exports = function (t, e, r) {
      if (Oh.isBuffer(t)) return t;
      if (typeof t == 'string') return Oh.from(t, e);
      if (ArrayBuffer.isView(t)) return Oh.from(t.buffer);
      throw new TypeError(
        r + ' must be a string, a Buffer, a typed array or a DataView'
      );
    };
  });
  var Nh = R((Sq, Up) => {
    S();
    var t9 = Bh(),
      r9 = Co(),
      i9 = Fo(),
      tn = Ie().Buffer,
      n9 = Ph(),
      Lp = kh(),
      Fp = Ch(),
      f9 = tn.alloc(128),
      Ho = {
        md5: 16,
        sha1: 20,
        sha224: 28,
        sha256: 32,
        sha384: 48,
        sha512: 64,
        rmd160: 20,
        ripemd160: 20,
      };
    function jp(t, e, r) {
      var n = a9(t),
        i = t === 'sha512' || t === 'sha384' ? 128 : 64;
      e.length > i ? (e = n(e)) : e.length < i && (e = tn.concat([e, f9], i));
      for (
        var a = tn.allocUnsafe(i + Ho[t]), h = tn.allocUnsafe(i + Ho[t]), v = 0;
        v < i;
        v++
      )
        (a[v] = e[v] ^ 54), (h[v] = e[v] ^ 92);
      var g = tn.allocUnsafe(i + r + 4);
      a.copy(g, 0, 0, i),
        (this.ipad1 = g),
        (this.ipad2 = a),
        (this.opad = h),
        (this.alg = t),
        (this.blocksize = i),
        (this.hash = n),
        (this.size = Ho[t]);
    }
    jp.prototype.run = function (t, e) {
      t.copy(e, this.blocksize);
      var r = this.hash(e);
      return r.copy(this.opad, this.blocksize), this.hash(this.opad);
    };
    function a9(t) {
      function e(n) {
        return i9(t).update(n).digest();
      }
      function r(n) {
        return new r9().update(n).digest();
      }
      return t === 'rmd160' || t === 'ripemd160' ? r : t === 'md5' ? t9 : e;
    }
    function o9(t, e, r, n, i) {
      n9(r, n),
        (t = Fp(t, Lp, 'Password')),
        (e = Fp(e, Lp, 'Salt')),
        (i = i || 'sha1');
      var a = new jp(i, t, e.length),
        h = tn.allocUnsafe(n),
        v = tn.allocUnsafe(e.length + 4);
      e.copy(v, 0, 0, e.length);
      for (var g = 0, M = Ho[i], x = Math.ceil(n / M), E = 1; E <= x; E++) {
        v.writeUInt32BE(E, e.length);
        for (var I = a.run(v, a.ipad1), q = I, k = 1; k < r; k++) {
          q = a.run(q, a.ipad2);
          for (var L = 0; L < M; L++) I[L] ^= q[L];
        }
        I.copy(h, g), (g += M);
      }
      return h;
    }
    Up.exports = o9;
  });
  var $p = R((Aq, Wp) => {
    S();
    var Vp = Ie().Buffer,
      s9 = Ph(),
      zp = kh(),
      Hp = Nh(),
      Kp = Ch(),
      Ko,
      ia = globalThis.crypto && globalThis.crypto.subtle,
      h9 = {
        sha: 'SHA-1',
        'sha-1': 'SHA-1',
        sha1: 'SHA-1',
        sha256: 'SHA-256',
        'sha-256': 'SHA-256',
        sha384: 'SHA-384',
        'sha-384': 'SHA-384',
        'sha-512': 'SHA-512',
        sha512: 'SHA-512',
      },
      Dh = [];
    function u9(t) {
      if (
        (globalThis.process && !globalThis.process.browser) ||
        !ia ||
        !ia.importKey ||
        !ia.deriveBits
      )
        return Promise.resolve(!1);
      if (Dh[t] !== void 0) return Dh[t];
      Ko = Ko || Vp.alloc(8);
      var e = Gp(Ko, Ko, 10, 128, t)
        .then(function () {
          return !0;
        })
        .catch(function () {
          return !1;
        });
      return (Dh[t] = e), e;
    }
    var rn;
    function Lh() {
      return (
        rn ||
        (globalThis.process && globalThis.process.nextTick
          ? (rn = globalThis.process.nextTick)
          : globalThis.queueMicrotask
            ? (rn = globalThis.queueMicrotask)
            : globalThis.setImmediate
              ? (rn = globalThis.setImmediate)
              : (rn = globalThis.setTimeout),
        rn)
      );
    }
    function Gp(t, e, r, n, i) {
      return ia
        .importKey('raw', t, { name: 'PBKDF2' }, !1, ['deriveBits'])
        .then(function (a) {
          return ia.deriveBits(
            { name: 'PBKDF2', salt: e, iterations: r, hash: { name: i } },
            a,
            n << 3
          );
        })
        .then(function (a) {
          return Vp.from(a);
        });
    }
    function c9(t, e) {
      t.then(
        function (r) {
          Lh()(function () {
            e(null, r);
          });
        },
        function (r) {
          Lh()(function () {
            e(r);
          });
        }
      );
    }
    Wp.exports = function (t, e, r, n, i, a) {
      typeof i == 'function' && ((a = i), (i = void 0)), (i = i || 'sha1');
      var h = h9[i.toLowerCase()];
      if (!h || typeof globalThis.Promise != 'function') {
        Lh()(function () {
          var v;
          try {
            v = Hp(t, e, r, n, i);
          } catch (g) {
            return a(g);
          }
          a(null, v);
        });
        return;
      }
      if (
        (s9(r, n),
        (t = Kp(t, zp, 'Password')),
        (e = Kp(e, zp, 'Salt')),
        typeof a != 'function')
      )
        throw new Error('No callback provided to pbkdf2');
      c9(
        u9(h).then(function (v) {
          return v ? Gp(t, e, r, n, h) : Hp(t, e, r, n, i);
        }),
        a
      );
    };
  });
  var jh = R((Fh) => {
    S();
    Fh.pbkdf2 = $p();
    Fh.pbkdf2Sync = Nh();
  });
  var Uh = R((Ct) => {
    'use strict';
    S();
    Ct.readUInt32BE = function (e, r) {
      var n = (e[0 + r] << 24) | (e[1 + r] << 16) | (e[2 + r] << 8) | e[3 + r];
      return n >>> 0;
    };
    Ct.writeUInt32BE = function (e, r, n) {
      (e[0 + n] = r >>> 24),
        (e[1 + n] = (r >>> 16) & 255),
        (e[2 + n] = (r >>> 8) & 255),
        (e[3 + n] = r & 255);
    };
    Ct.ip = function (e, r, n, i) {
      for (var a = 0, h = 0, v = 6; v >= 0; v -= 2) {
        for (var g = 0; g <= 24; g += 8) (a <<= 1), (a |= (r >>> (g + v)) & 1);
        for (var g = 0; g <= 24; g += 8) (a <<= 1), (a |= (e >>> (g + v)) & 1);
      }
      for (var v = 6; v >= 0; v -= 2) {
        for (var g = 1; g <= 25; g += 8) (h <<= 1), (h |= (r >>> (g + v)) & 1);
        for (var g = 1; g <= 25; g += 8) (h <<= 1), (h |= (e >>> (g + v)) & 1);
      }
      (n[i + 0] = a >>> 0), (n[i + 1] = h >>> 0);
    };
    Ct.rip = function (e, r, n, i) {
      for (var a = 0, h = 0, v = 0; v < 4; v++)
        for (var g = 24; g >= 0; g -= 8)
          (a <<= 1),
            (a |= (r >>> (g + v)) & 1),
            (a <<= 1),
            (a |= (e >>> (g + v)) & 1);
      for (var v = 4; v < 8; v++)
        for (var g = 24; g >= 0; g -= 8)
          (h <<= 1),
            (h |= (r >>> (g + v)) & 1),
            (h <<= 1),
            (h |= (e >>> (g + v)) & 1);
      (n[i + 0] = a >>> 0), (n[i + 1] = h >>> 0);
    };
    Ct.pc1 = function (e, r, n, i) {
      for (var a = 0, h = 0, v = 7; v >= 5; v--) {
        for (var g = 0; g <= 24; g += 8) (a <<= 1), (a |= (r >> (g + v)) & 1);
        for (var g = 0; g <= 24; g += 8) (a <<= 1), (a |= (e >> (g + v)) & 1);
      }
      for (var g = 0; g <= 24; g += 8) (a <<= 1), (a |= (r >> (g + v)) & 1);
      for (var v = 1; v <= 3; v++) {
        for (var g = 0; g <= 24; g += 8) (h <<= 1), (h |= (r >> (g + v)) & 1);
        for (var g = 0; g <= 24; g += 8) (h <<= 1), (h |= (e >> (g + v)) & 1);
      }
      for (var g = 0; g <= 24; g += 8) (h <<= 1), (h |= (e >> (g + v)) & 1);
      (n[i + 0] = a >>> 0), (n[i + 1] = h >>> 0);
    };
    Ct.r28shl = function (e, r) {
      return ((e << r) & 268435455) | (e >>> (28 - r));
    };
    var Vo = [
      14, 11, 17, 4, 27, 23, 25, 0, 13, 22, 7, 18, 5, 9, 16, 24, 2, 20, 12, 21,
      1, 8, 15, 26, 15, 4, 25, 19, 9, 1, 26, 16, 5, 11, 23, 8, 12, 7, 17, 0, 22,
      3, 10, 14, 6, 20, 27, 24,
    ];
    Ct.pc2 = function (e, r, n, i) {
      for (var a = 0, h = 0, v = Vo.length >>> 1, g = 0; g < v; g++)
        (a <<= 1), (a |= (e >>> Vo[g]) & 1);
      for (var g = v; g < Vo.length; g++) (h <<= 1), (h |= (r >>> Vo[g]) & 1);
      (n[i + 0] = a >>> 0), (n[i + 1] = h >>> 0);
    };
    Ct.expand = function (e, r, n) {
      var i = 0,
        a = 0;
      i = ((e & 1) << 5) | (e >>> 27);
      for (var h = 23; h >= 15; h -= 4) (i <<= 6), (i |= (e >>> h) & 63);
      for (var h = 11; h >= 3; h -= 4) (a |= (e >>> h) & 63), (a <<= 6);
      (a |= ((e & 31) << 1) | (e >>> 31)),
        (r[n + 0] = i >>> 0),
        (r[n + 1] = a >>> 0);
    };
    var Zp = [
      14, 0, 4, 15, 13, 7, 1, 4, 2, 14, 15, 2, 11, 13, 8, 1, 3, 10, 10, 6, 6,
      12, 12, 11, 5, 9, 9, 5, 0, 3, 7, 8, 4, 15, 1, 12, 14, 8, 8, 2, 13, 4, 6,
      9, 2, 1, 11, 7, 15, 5, 12, 11, 9, 3, 7, 14, 3, 10, 10, 0, 5, 6, 0, 13, 15,
      3, 1, 13, 8, 4, 14, 7, 6, 15, 11, 2, 3, 8, 4, 14, 9, 12, 7, 0, 2, 1, 13,
      10, 12, 6, 0, 9, 5, 11, 10, 5, 0, 13, 14, 8, 7, 10, 11, 1, 10, 3, 4, 15,
      13, 4, 1, 2, 5, 11, 8, 6, 12, 7, 6, 12, 9, 0, 3, 5, 2, 14, 15, 9, 10, 13,
      0, 7, 9, 0, 14, 9, 6, 3, 3, 4, 15, 6, 5, 10, 1, 2, 13, 8, 12, 5, 7, 14,
      11, 12, 4, 11, 2, 15, 8, 1, 13, 1, 6, 10, 4, 13, 9, 0, 8, 6, 15, 9, 3, 8,
      0, 7, 11, 4, 1, 15, 2, 14, 12, 3, 5, 11, 10, 5, 14, 2, 7, 12, 7, 13, 13,
      8, 14, 11, 3, 5, 0, 6, 6, 15, 9, 0, 10, 3, 1, 4, 2, 7, 8, 2, 5, 12, 11, 1,
      12, 10, 4, 14, 15, 9, 10, 3, 6, 15, 9, 0, 0, 6, 12, 10, 11, 1, 7, 13, 13,
      8, 15, 9, 1, 4, 3, 5, 14, 11, 5, 12, 2, 7, 8, 2, 4, 14, 2, 14, 12, 11, 4,
      2, 1, 12, 7, 4, 10, 7, 11, 13, 6, 1, 8, 5, 5, 0, 3, 15, 15, 10, 13, 3, 0,
      9, 14, 8, 9, 6, 4, 11, 2, 8, 1, 12, 11, 7, 10, 1, 13, 14, 7, 2, 8, 13, 15,
      6, 9, 15, 12, 0, 5, 9, 6, 10, 3, 4, 0, 5, 14, 3, 12, 10, 1, 15, 10, 4, 15,
      2, 9, 7, 2, 12, 6, 9, 8, 5, 0, 6, 13, 1, 3, 13, 4, 14, 14, 0, 7, 11, 5, 3,
      11, 8, 9, 4, 14, 3, 15, 2, 5, 12, 2, 9, 8, 5, 12, 15, 3, 10, 7, 11, 0, 14,
      4, 1, 10, 7, 1, 6, 13, 0, 11, 8, 6, 13, 4, 13, 11, 0, 2, 11, 14, 7, 15, 4,
      0, 9, 8, 1, 13, 10, 3, 14, 12, 3, 9, 5, 7, 12, 5, 2, 10, 15, 6, 8, 1, 6,
      1, 6, 4, 11, 11, 13, 13, 8, 12, 1, 3, 4, 7, 10, 14, 7, 10, 9, 15, 5, 6, 0,
      8, 15, 0, 14, 5, 2, 9, 3, 2, 12, 13, 1, 2, 15, 8, 13, 4, 8, 6, 10, 15, 3,
      11, 7, 1, 4, 10, 12, 9, 5, 3, 6, 14, 11, 5, 0, 0, 14, 12, 9, 7, 2, 7, 2,
      11, 1, 4, 14, 1, 7, 9, 4, 12, 10, 14, 8, 2, 13, 0, 15, 6, 12, 10, 9, 13,
      0, 15, 3, 3, 5, 5, 6, 8, 11,
    ];
    Ct.substitute = function (e, r) {
      for (var n = 0, i = 0; i < 4; i++) {
        var a = (e >>> (18 - i * 6)) & 63,
          h = Zp[i * 64 + a];
        (n <<= 4), (n |= h);
      }
      for (var i = 0; i < 4; i++) {
        var a = (r >>> (18 - i * 6)) & 63,
          h = Zp[4 * 64 + i * 64 + a];
        (n <<= 4), (n |= h);
      }
      return n >>> 0;
    };
    var Jp = [
      16, 25, 12, 11, 3, 20, 4, 15, 31, 17, 9, 6, 27, 14, 1, 22, 30, 24, 8, 18,
      0, 5, 29, 23, 13, 19, 2, 26, 10, 21, 28, 7,
    ];
    Ct.permute = function (e) {
      for (var r = 0, n = 0; n < Jp.length; n++)
        (r <<= 1), (r |= (e >>> Jp[n]) & 1);
      return r >>> 0;
    };
    Ct.padSplit = function (e, r, n) {
      for (var i = e.toString(2); i.length < r; ) i = '0' + i;
      for (var a = [], h = 0; h < r; h += n) a.push(i.slice(h, h + n));
      return a.join(' ');
    };
  });
  var At = R((Pq, Yp) => {
    S();
    Yp.exports = Xp;
    function Xp(t, e) {
      if (!t) throw new Error(e || 'Assertion failed');
    }
    Xp.equal = function (e, r, n) {
      if (e != r) throw new Error(n || 'Assertion failed: ' + e + ' != ' + r);
    };
  });
  var Go = R((Oq, Qp) => {
    'use strict';
    S();
    var d9 = At();
    function Nt(t) {
      (this.options = t),
        (this.type = this.options.type),
        (this.blockSize = 8),
        this._init(),
        (this.buffer = new Array(this.blockSize)),
        (this.bufferOff = 0);
    }
    Qp.exports = Nt;
    Nt.prototype._init = function () {};
    Nt.prototype.update = function (e) {
      return e.length === 0
        ? []
        : this.type === 'decrypt'
          ? this._updateDecrypt(e)
          : this._updateEncrypt(e);
    };
    Nt.prototype._buffer = function (e, r) {
      for (
        var n = Math.min(this.buffer.length - this.bufferOff, e.length - r),
          i = 0;
        i < n;
        i++
      )
        this.buffer[this.bufferOff + i] = e[r + i];
      return (this.bufferOff += n), n;
    };
    Nt.prototype._flushBuffer = function (e, r) {
      return (
        this._update(this.buffer, 0, e, r), (this.bufferOff = 0), this.blockSize
      );
    };
    Nt.prototype._updateEncrypt = function (e) {
      var r = 0,
        n = 0,
        i = ((this.bufferOff + e.length) / this.blockSize) | 0,
        a = new Array(i * this.blockSize);
      this.bufferOff !== 0 &&
        ((r += this._buffer(e, r)),
        this.bufferOff === this.buffer.length &&
          (n += this._flushBuffer(a, n)));
      for (
        var h = e.length - ((e.length - r) % this.blockSize);
        r < h;
        r += this.blockSize
      )
        this._update(e, r, a, n), (n += this.blockSize);
      for (; r < e.length; r++, this.bufferOff++)
        this.buffer[this.bufferOff] = e[r];
      return a;
    };
    Nt.prototype._updateDecrypt = function (e) {
      for (
        var r = 0,
          n = 0,
          i = Math.ceil((this.bufferOff + e.length) / this.blockSize) - 1,
          a = new Array(i * this.blockSize);
        i > 0;
        i--
      )
        (r += this._buffer(e, r)), (n += this._flushBuffer(a, n));
      return (r += this._buffer(e, r)), a;
    };
    Nt.prototype.final = function (e) {
      var r;
      e && (r = this.update(e));
      var n;
      return (
        this.type === 'encrypt'
          ? (n = this._finalEncrypt())
          : (n = this._finalDecrypt()),
        r ? r.concat(n) : n
      );
    };
    Nt.prototype._pad = function (e, r) {
      if (r === 0) return !1;
      for (; r < e.length; ) e[r++] = 0;
      return !0;
    };
    Nt.prototype._finalEncrypt = function () {
      if (!this._pad(this.buffer, this.bufferOff)) return [];
      var e = new Array(this.blockSize);
      return this._update(this.buffer, 0, e, 0), e;
    };
    Nt.prototype._unpad = function (e) {
      return e;
    };
    Nt.prototype._finalDecrypt = function () {
      d9.equal(this.bufferOff, this.blockSize, 'Not enough data to decrypt');
      var e = new Array(this.blockSize);
      return this._flushBuffer(e, 0), this._unpad(e);
    };
  });
  var zh = R((Nq, rv) => {
    'use strict';
    S();
    var ev = At(),
      l9 = qe(),
      rt = Uh(),
      tv = Go();
    function p9() {
      (this.tmp = new Array(2)), (this.keys = null);
    }
    function Ir(t) {
      tv.call(this, t);
      var e = new p9();
      (this._desState = e), this.deriveKeys(e, t.key);
    }
    l9(Ir, tv);
    rv.exports = Ir;
    Ir.create = function (e) {
      return new Ir(e);
    };
    var v9 = [1, 1, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 1];
    Ir.prototype.deriveKeys = function (e, r) {
      (e.keys = new Array(16 * 2)),
        ev.equal(r.length, this.blockSize, 'Invalid key length');
      var n = rt.readUInt32BE(r, 0),
        i = rt.readUInt32BE(r, 4);
      rt.pc1(n, i, e.tmp, 0), (n = e.tmp[0]), (i = e.tmp[1]);
      for (var a = 0; a < e.keys.length; a += 2) {
        var h = v9[a >>> 1];
        (n = rt.r28shl(n, h)), (i = rt.r28shl(i, h)), rt.pc2(n, i, e.keys, a);
      }
    };
    Ir.prototype._update = function (e, r, n, i) {
      var a = this._desState,
        h = rt.readUInt32BE(e, r),
        v = rt.readUInt32BE(e, r + 4);
      rt.ip(h, v, a.tmp, 0),
        (h = a.tmp[0]),
        (v = a.tmp[1]),
        this.type === 'encrypt'
          ? this._encrypt(a, h, v, a.tmp, 0)
          : this._decrypt(a, h, v, a.tmp, 0),
        (h = a.tmp[0]),
        (v = a.tmp[1]),
        rt.writeUInt32BE(n, h, i),
        rt.writeUInt32BE(n, v, i + 4);
    };
    Ir.prototype._pad = function (e, r) {
      for (var n = e.length - r, i = r; i < e.length; i++) e[i] = n;
      return !0;
    };
    Ir.prototype._unpad = function (e) {
      for (var r = e[e.length - 1], n = e.length - r; n < e.length; n++)
        ev.equal(e[n], r);
      return e.slice(0, e.length - r);
    };
    Ir.prototype._encrypt = function (e, r, n, i, a) {
      for (var h = r, v = n, g = 0; g < e.keys.length; g += 2) {
        var M = e.keys[g],
          x = e.keys[g + 1];
        rt.expand(v, e.tmp, 0), (M ^= e.tmp[0]), (x ^= e.tmp[1]);
        var E = rt.substitute(M, x),
          I = rt.permute(E),
          q = v;
        (v = (h ^ I) >>> 0), (h = q);
      }
      rt.rip(v, h, i, a);
    };
    Ir.prototype._decrypt = function (e, r, n, i, a) {
      for (var h = n, v = r, g = e.keys.length - 2; g >= 0; g -= 2) {
        var M = e.keys[g],
          x = e.keys[g + 1];
        rt.expand(h, e.tmp, 0), (M ^= e.tmp[0]), (x ^= e.tmp[1]);
        var E = rt.substitute(M, x),
          I = rt.permute(E),
          q = h;
        (h = (v ^ I) >>> 0), (v = q);
      }
      rt.rip(h, v, i, a);
    };
  });
  var nv = R((iv) => {
    'use strict';
    S();
    var b9 = At(),
      y9 = qe(),
      Wo = {};
    function m9(t) {
      b9.equal(t.length, 8, 'Invalid IV length'), (this.iv = new Array(8));
      for (var e = 0; e < this.iv.length; e++) this.iv[e] = t[e];
    }
    function g9(t) {
      function e(a) {
        t.call(this, a), this._cbcInit();
      }
      y9(e, t);
      for (var r = Object.keys(Wo), n = 0; n < r.length; n++) {
        var i = r[n];
        e.prototype[i] = Wo[i];
      }
      return (
        (e.create = function (h) {
          return new e(h);
        }),
        e
      );
    }
    iv.instantiate = g9;
    Wo._cbcInit = function () {
      var e = new m9(this.options.iv);
      this._cbcState = e;
    };
    Wo._update = function (e, r, n, i) {
      var a = this._cbcState,
        h = this.constructor.super_.prototype,
        v = a.iv;
      if (this.type === 'encrypt') {
        for (var g = 0; g < this.blockSize; g++) v[g] ^= e[r + g];
        h._update.call(this, v, 0, n, i);
        for (var g = 0; g < this.blockSize; g++) v[g] = n[i + g];
      } else {
        h._update.call(this, e, r, n, i);
        for (var g = 0; g < this.blockSize; g++) n[i + g] ^= v[g];
        for (var g = 0; g < this.blockSize; g++) v[g] = e[r + g];
      }
    };
  });
  var ov = R((jq, av) => {
    'use strict';
    S();
    var w9 = At(),
      _9 = qe(),
      fv = Go(),
      Ei = zh();
    function x9(t, e) {
      w9.equal(e.length, 24, 'Invalid key length');
      var r = e.slice(0, 8),
        n = e.slice(8, 16),
        i = e.slice(16, 24);
      t === 'encrypt'
        ? (this.ciphers = [
            Ei.create({ type: 'encrypt', key: r }),
            Ei.create({ type: 'decrypt', key: n }),
            Ei.create({ type: 'encrypt', key: i }),
          ])
        : (this.ciphers = [
            Ei.create({ type: 'decrypt', key: i }),
            Ei.create({ type: 'encrypt', key: n }),
            Ei.create({ type: 'decrypt', key: r }),
          ]);
    }
    function nn(t) {
      fv.call(this, t);
      var e = new x9(this.type, this.options.key);
      this._edeState = e;
    }
    _9(nn, fv);
    av.exports = nn;
    nn.create = function (e) {
      return new nn(e);
    };
    nn.prototype._update = function (e, r, n, i) {
      var a = this._edeState;
      a.ciphers[0]._update(e, r, n, i),
        a.ciphers[1]._update(n, i, n, i),
        a.ciphers[2]._update(n, i, n, i);
    };
    nn.prototype._pad = Ei.prototype._pad;
    nn.prototype._unpad = Ei.prototype._unpad;
  });
  var sv = R((Kn) => {
    'use strict';
    S();
    Kn.utils = Uh();
    Kn.Cipher = Go();
    Kn.DES = zh();
    Kn.CBC = nv();
    Kn.EDE = ov();
  });
  var cv = R((Kq, uv) => {
    S();
    var hv = zr(),
      Hr = sv(),
      M9 = qe(),
      fn = Ie().Buffer,
      na = {
        'des-ede3-cbc': Hr.CBC.instantiate(Hr.EDE),
        'des-ede3': Hr.EDE,
        'des-ede-cbc': Hr.CBC.instantiate(Hr.EDE),
        'des-ede': Hr.EDE,
        'des-cbc': Hr.CBC.instantiate(Hr.DES),
        'des-ecb': Hr.DES,
      };
    na.des = na['des-cbc'];
    na.des3 = na['des-ede3-cbc'];
    uv.exports = $o;
    M9($o, hv);
    function $o(t) {
      hv.call(this);
      var e = t.mode.toLowerCase(),
        r = na[e],
        n;
      t.decrypt ? (n = 'decrypt') : (n = 'encrypt');
      var i = t.key;
      fn.isBuffer(i) || (i = fn.from(i)),
        (e === 'des-ede' || e === 'des-ede-cbc') &&
          (i = fn.concat([i, i.slice(0, 8)]));
      var a = t.iv;
      fn.isBuffer(a) || (a = fn.from(a)),
        (this._des = r.create({ key: i, iv: a, type: n }));
    }
    $o.prototype._update = function (t) {
      return fn.from(this._des.update(t));
    };
    $o.prototype._final = function () {
      return fn.from(this._des.final());
    };
  });
  var dv = R((Hh) => {
    S();
    Hh.encrypt = function (t, e) {
      return t._cipher.encryptBlock(e);
    };
    Hh.decrypt = function (t, e) {
      return t._cipher.decryptBlock(e);
    };
  });
  var Vn = R(($q, lv) => {
    S();
    lv.exports = function (e, r) {
      for (
        var n = Math.min(e.length, r.length), i = new Buffer(n), a = 0;
        a < n;
        ++a
      )
        i[a] = e[a] ^ r[a];
      return i;
    };
  });
  var vv = R((Kh) => {
    S();
    var pv = Vn();
    Kh.encrypt = function (t, e) {
      var r = pv(e, t._prev);
      return (t._prev = t._cipher.encryptBlock(r)), t._prev;
    };
    Kh.decrypt = function (t, e) {
      var r = t._prev;
      t._prev = e;
      var n = t._cipher.decryptBlock(e);
      return pv(n, r);
    };
  });
  var mv = R((yv) => {
    S();
    var fa = Ie().Buffer,
      S9 = Vn();
    function bv(t, e, r) {
      var n = e.length,
        i = S9(e, t._cache);
      return (
        (t._cache = t._cache.slice(n)),
        (t._prev = fa.concat([t._prev, r ? e : i])),
        i
      );
    }
    yv.encrypt = function (t, e, r) {
      for (var n = fa.allocUnsafe(0), i; e.length; )
        if (
          (t._cache.length === 0 &&
            ((t._cache = t._cipher.encryptBlock(t._prev)),
            (t._prev = fa.allocUnsafe(0))),
          t._cache.length <= e.length)
        )
          (i = t._cache.length),
            (n = fa.concat([n, bv(t, e.slice(0, i), r)])),
            (e = e.slice(i));
        else {
          n = fa.concat([n, bv(t, e, r)]);
          break;
        }
      return n;
    };
  });
  var wv = R((gv) => {
    S();
    var Vh = Ie().Buffer;
    function E9(t, e, r) {
      var n = t._cipher.encryptBlock(t._prev),
        i = n[0] ^ e;
      return (t._prev = Vh.concat([t._prev.slice(1), Vh.from([r ? e : i])])), i;
    }
    gv.encrypt = function (t, e, r) {
      for (var n = e.length, i = Vh.allocUnsafe(n), a = -1; ++a < n; )
        i[a] = E9(t, e[a], r);
      return i;
    };
  });
  var xv = R((_v) => {
    S();
    var Zo = Ie().Buffer;
    function A9(t, e, r) {
      for (var n, i = -1, a = 8, h = 0, v, g; ++i < a; )
        (n = t._cipher.encryptBlock(t._prev)),
          (v = e & (1 << (7 - i)) ? 128 : 0),
          (g = n[0] ^ v),
          (h += (g & 128) >> i % 8),
          (t._prev = B9(t._prev, r ? v : g));
      return h;
    }
    function B9(t, e) {
      var r = t.length,
        n = -1,
        i = Zo.allocUnsafe(t.length);
      for (t = Zo.concat([t, Zo.from([e])]); ++n < r; )
        i[n] = (t[n] << 1) | (t[n + 1] >> 7);
      return i;
    }
    _v.encrypt = function (t, e, r) {
      for (var n = e.length, i = Zo.allocUnsafe(n), a = -1; ++a < n; )
        i[a] = A9(t, e[a], r);
      return i;
    };
  });
  var Sv = R((Mv) => {
    S();
    var I9 = Vn();
    function R9(t) {
      return (t._prev = t._cipher.encryptBlock(t._prev)), t._prev;
    }
    Mv.encrypt = function (t, e) {
      for (; t._cache.length < e.length; )
        t._cache = Buffer.concat([t._cache, R9(t)]);
      var r = t._cache.slice(0, e.length);
      return (t._cache = t._cache.slice(e.length)), I9(e, r);
    };
  });
  var Gh = R((aT, Ev) => {
    S();
    function q9(t) {
      for (var e = t.length, r; e--; )
        if (((r = t.readUInt8(e)), r === 255)) t.writeUInt8(0, e);
        else {
          r++, t.writeUInt8(r, e);
          break;
        }
    }
    Ev.exports = q9;
  });
  var $h = R((Bv) => {
    S();
    var T9 = Vn(),
      Av = Ie().Buffer,
      P9 = Gh();
    function k9(t) {
      var e = t._cipher.encryptBlockRaw(t._prev);
      return P9(t._prev), e;
    }
    var Wh = 16;
    Bv.encrypt = function (t, e) {
      var r = Math.ceil(e.length / Wh),
        n = t._cache.length;
      t._cache = Av.concat([t._cache, Av.allocUnsafe(r * Wh)]);
      for (var i = 0; i < r; i++) {
        var a = k9(t),
          h = n + i * Wh;
        t._cache.writeUInt32BE(a[0], h + 0),
          t._cache.writeUInt32BE(a[1], h + 4),
          t._cache.writeUInt32BE(a[2], h + 8),
          t._cache.writeUInt32BE(a[3], h + 12);
      }
      var v = t._cache.slice(0, e.length);
      return (t._cache = t._cache.slice(e.length)), T9(e, v);
    };
  });
  var Zh = R((uT, O9) => {
    O9.exports = {
      'aes-128-ecb': {
        cipher: 'AES',
        key: 128,
        iv: 0,
        mode: 'ECB',
        type: 'block',
      },
      'aes-192-ecb': {
        cipher: 'AES',
        key: 192,
        iv: 0,
        mode: 'ECB',
        type: 'block',
      },
      'aes-256-ecb': {
        cipher: 'AES',
        key: 256,
        iv: 0,
        mode: 'ECB',
        type: 'block',
      },
      'aes-128-cbc': {
        cipher: 'AES',
        key: 128,
        iv: 16,
        mode: 'CBC',
        type: 'block',
      },
      'aes-192-cbc': {
        cipher: 'AES',
        key: 192,
        iv: 16,
        mode: 'CBC',
        type: 'block',
      },
      'aes-256-cbc': {
        cipher: 'AES',
        key: 256,
        iv: 16,
        mode: 'CBC',
        type: 'block',
      },
      aes128: { cipher: 'AES', key: 128, iv: 16, mode: 'CBC', type: 'block' },
      aes192: { cipher: 'AES', key: 192, iv: 16, mode: 'CBC', type: 'block' },
      aes256: { cipher: 'AES', key: 256, iv: 16, mode: 'CBC', type: 'block' },
      'aes-128-cfb': {
        cipher: 'AES',
        key: 128,
        iv: 16,
        mode: 'CFB',
        type: 'stream',
      },
      'aes-192-cfb': {
        cipher: 'AES',
        key: 192,
        iv: 16,
        mode: 'CFB',
        type: 'stream',
      },
      'aes-256-cfb': {
        cipher: 'AES',
        key: 256,
        iv: 16,
        mode: 'CFB',
        type: 'stream',
      },
      'aes-128-cfb8': {
        cipher: 'AES',
        key: 128,
        iv: 16,
        mode: 'CFB8',
        type: 'stream',
      },
      'aes-192-cfb8': {
        cipher: 'AES',
        key: 192,
        iv: 16,
        mode: 'CFB8',
        type: 'stream',
      },
      'aes-256-cfb8': {
        cipher: 'AES',
        key: 256,
        iv: 16,
        mode: 'CFB8',
        type: 'stream',
      },
      'aes-128-cfb1': {
        cipher: 'AES',
        key: 128,
        iv: 16,
        mode: 'CFB1',
        type: 'stream',
      },
      'aes-192-cfb1': {
        cipher: 'AES',
        key: 192,
        iv: 16,
        mode: 'CFB1',
        type: 'stream',
      },
      'aes-256-cfb1': {
        cipher: 'AES',
        key: 256,
        iv: 16,
        mode: 'CFB1',
        type: 'stream',
      },
      'aes-128-ofb': {
        cipher: 'AES',
        key: 128,
        iv: 16,
        mode: 'OFB',
        type: 'stream',
      },
      'aes-192-ofb': {
        cipher: 'AES',
        key: 192,
        iv: 16,
        mode: 'OFB',
        type: 'stream',
      },
      'aes-256-ofb': {
        cipher: 'AES',
        key: 256,
        iv: 16,
        mode: 'OFB',
        type: 'stream',
      },
      'aes-128-ctr': {
        cipher: 'AES',
        key: 128,
        iv: 16,
        mode: 'CTR',
        type: 'stream',
      },
      'aes-192-ctr': {
        cipher: 'AES',
        key: 192,
        iv: 16,
        mode: 'CTR',
        type: 'stream',
      },
      'aes-256-ctr': {
        cipher: 'AES',
        key: 256,
        iv: 16,
        mode: 'CTR',
        type: 'stream',
      },
      'aes-128-gcm': {
        cipher: 'AES',
        key: 128,
        iv: 12,
        mode: 'GCM',
        type: 'auth',
      },
      'aes-192-gcm': {
        cipher: 'AES',
        key: 192,
        iv: 12,
        mode: 'GCM',
        type: 'auth',
      },
      'aes-256-gcm': {
        cipher: 'AES',
        key: 256,
        iv: 12,
        mode: 'GCM',
        type: 'auth',
      },
    };
  });
  var Xo = R((cT, Iv) => {
    S();
    var C9 = {
        ECB: dv(),
        CBC: vv(),
        CFB: mv(),
        CFB8: wv(),
        CFB1: xv(),
        OFB: Sv(),
        CTR: $h(),
        GCM: $h(),
      },
      Jo = Zh();
    for (Jh in Jo) Jo[Jh].module = C9[Jo[Jh].mode];
    var Jh;
    Iv.exports = Jo;
  });
  var aa = R((lT, qv) => {
    S();
    var Yo = Ie().Buffer;
    function Yh(t) {
      Yo.isBuffer(t) || (t = Yo.from(t));
      for (var e = (t.length / 4) | 0, r = new Array(e), n = 0; n < e; n++)
        r[n] = t.readUInt32BE(n * 4);
      return r;
    }
    function Xh(t) {
      for (var e = 0; e < t.length; t++) t[e] = 0;
    }
    function Rv(t, e, r, n, i) {
      for (
        var a = r[0],
          h = r[1],
          v = r[2],
          g = r[3],
          M = t[0] ^ e[0],
          x = t[1] ^ e[1],
          E = t[2] ^ e[2],
          I = t[3] ^ e[3],
          q,
          k,
          L,
          xe,
          U = 4,
          Me = 1;
        Me < i;
        Me++
      )
        (q =
          a[M >>> 24] ^
          h[(x >>> 16) & 255] ^
          v[(E >>> 8) & 255] ^
          g[I & 255] ^
          e[U++]),
          (k =
            a[x >>> 24] ^
            h[(E >>> 16) & 255] ^
            v[(I >>> 8) & 255] ^
            g[M & 255] ^
            e[U++]),
          (L =
            a[E >>> 24] ^
            h[(I >>> 16) & 255] ^
            v[(M >>> 8) & 255] ^
            g[x & 255] ^
            e[U++]),
          (xe =
            a[I >>> 24] ^
            h[(M >>> 16) & 255] ^
            v[(x >>> 8) & 255] ^
            g[E & 255] ^
            e[U++]),
          (M = q),
          (x = k),
          (E = L),
          (I = xe);
      return (
        (q =
          ((n[M >>> 24] << 24) |
            (n[(x >>> 16) & 255] << 16) |
            (n[(E >>> 8) & 255] << 8) |
            n[I & 255]) ^
          e[U++]),
        (k =
          ((n[x >>> 24] << 24) |
            (n[(E >>> 16) & 255] << 16) |
            (n[(I >>> 8) & 255] << 8) |
            n[M & 255]) ^
          e[U++]),
        (L =
          ((n[E >>> 24] << 24) |
            (n[(I >>> 16) & 255] << 16) |
            (n[(M >>> 8) & 255] << 8) |
            n[x & 255]) ^
          e[U++]),
        (xe =
          ((n[I >>> 24] << 24) |
            (n[(M >>> 16) & 255] << 16) |
            (n[(x >>> 8) & 255] << 8) |
            n[E & 255]) ^
          e[U++]),
        (q = q >>> 0),
        (k = k >>> 0),
        (L = L >>> 0),
        (xe = xe >>> 0),
        [q, k, L, xe]
      );
    }
    var N9 = [0, 1, 2, 4, 8, 16, 32, 64, 128, 27, 54],
      tt = (function () {
        for (var t = new Array(256), e = 0; e < 256; e++)
          e < 128 ? (t[e] = e << 1) : (t[e] = (e << 1) ^ 283);
        for (
          var r = [],
            n = [],
            i = [[], [], [], []],
            a = [[], [], [], []],
            h = 0,
            v = 0,
            g = 0;
          g < 256;
          ++g
        ) {
          var M = v ^ (v << 1) ^ (v << 2) ^ (v << 3) ^ (v << 4);
          (M = (M >>> 8) ^ (M & 255) ^ 99), (r[h] = M), (n[M] = h);
          var x = t[h],
            E = t[x],
            I = t[E],
            q = (t[M] * 257) ^ (M * 16843008);
          (i[0][h] = (q << 24) | (q >>> 8)),
            (i[1][h] = (q << 16) | (q >>> 16)),
            (i[2][h] = (q << 8) | (q >>> 24)),
            (i[3][h] = q),
            (q = (I * 16843009) ^ (E * 65537) ^ (x * 257) ^ (h * 16843008)),
            (a[0][M] = (q << 24) | (q >>> 8)),
            (a[1][M] = (q << 16) | (q >>> 16)),
            (a[2][M] = (q << 8) | (q >>> 24)),
            (a[3][M] = q),
            h === 0 ? (h = v = 1) : ((h = x ^ t[t[t[I ^ x]]]), (v ^= t[t[v]]));
        }
        return { SBOX: r, INV_SBOX: n, SUB_MIX: i, INV_SUB_MIX: a };
      })();
    function Dt(t) {
      (this._key = Yh(t)), this._reset();
    }
    Dt.blockSize = 4 * 4;
    Dt.keySize = 256 / 8;
    Dt.prototype.blockSize = Dt.blockSize;
    Dt.prototype.keySize = Dt.keySize;
    Dt.prototype._reset = function () {
      for (
        var t = this._key,
          e = t.length,
          r = e + 6,
          n = (r + 1) * 4,
          i = [],
          a = 0;
        a < e;
        a++
      )
        i[a] = t[a];
      for (a = e; a < n; a++) {
        var h = i[a - 1];
        a % e === 0
          ? ((h = (h << 8) | (h >>> 24)),
            (h =
              (tt.SBOX[h >>> 24] << 24) |
              (tt.SBOX[(h >>> 16) & 255] << 16) |
              (tt.SBOX[(h >>> 8) & 255] << 8) |
              tt.SBOX[h & 255]),
            (h ^= N9[(a / e) | 0] << 24))
          : e > 6 &&
            a % e === 4 &&
            (h =
              (tt.SBOX[h >>> 24] << 24) |
              (tt.SBOX[(h >>> 16) & 255] << 16) |
              (tt.SBOX[(h >>> 8) & 255] << 8) |
              tt.SBOX[h & 255]),
          (i[a] = i[a - e] ^ h);
      }
      for (var v = [], g = 0; g < n; g++) {
        var M = n - g,
          x = i[M - (g % 4 ? 0 : 4)];
        g < 4 || M <= 4
          ? (v[g] = x)
          : (v[g] =
              tt.INV_SUB_MIX[0][tt.SBOX[x >>> 24]] ^
              tt.INV_SUB_MIX[1][tt.SBOX[(x >>> 16) & 255]] ^
              tt.INV_SUB_MIX[2][tt.SBOX[(x >>> 8) & 255]] ^
              tt.INV_SUB_MIX[3][tt.SBOX[x & 255]]);
      }
      (this._nRounds = r), (this._keySchedule = i), (this._invKeySchedule = v);
    };
    Dt.prototype.encryptBlockRaw = function (t) {
      return (
        (t = Yh(t)),
        Rv(t, this._keySchedule, tt.SUB_MIX, tt.SBOX, this._nRounds)
      );
    };
    Dt.prototype.encryptBlock = function (t) {
      var e = this.encryptBlockRaw(t),
        r = Yo.allocUnsafe(16);
      return (
        r.writeUInt32BE(e[0], 0),
        r.writeUInt32BE(e[1], 4),
        r.writeUInt32BE(e[2], 8),
        r.writeUInt32BE(e[3], 12),
        r
      );
    };
    Dt.prototype.decryptBlock = function (t) {
      t = Yh(t);
      var e = t[1];
      (t[1] = t[3]), (t[3] = e);
      var r = Rv(
          t,
          this._invKeySchedule,
          tt.INV_SUB_MIX,
          tt.INV_SBOX,
          this._nRounds
        ),
        n = Yo.allocUnsafe(16);
      return (
        n.writeUInt32BE(r[0], 0),
        n.writeUInt32BE(r[3], 4),
        n.writeUInt32BE(r[2], 8),
        n.writeUInt32BE(r[1], 12),
        n
      );
    };
    Dt.prototype.scrub = function () {
      Xh(this._keySchedule), Xh(this._invKeySchedule), Xh(this._key);
    };
    qv.exports.AES = Dt;
  });
  var kv = R((vT, Pv) => {
    S();
    var Gn = Ie().Buffer,
      D9 = Gn.alloc(16, 0);
    function L9(t) {
      return [
        t.readUInt32BE(0),
        t.readUInt32BE(4),
        t.readUInt32BE(8),
        t.readUInt32BE(12),
      ];
    }
    function Tv(t) {
      var e = Gn.allocUnsafe(16);
      return (
        e.writeUInt32BE(t[0] >>> 0, 0),
        e.writeUInt32BE(t[1] >>> 0, 4),
        e.writeUInt32BE(t[2] >>> 0, 8),
        e.writeUInt32BE(t[3] >>> 0, 12),
        e
      );
    }
    function oa(t) {
      (this.h = t),
        (this.state = Gn.alloc(16, 0)),
        (this.cache = Gn.allocUnsafe(0));
    }
    oa.prototype.ghash = function (t) {
      for (var e = -1; ++e < t.length; ) this.state[e] ^= t[e];
      this._multiply();
    };
    oa.prototype._multiply = function () {
      for (var t = L9(this.h), e = [0, 0, 0, 0], r, n, i, a = -1; ++a < 128; ) {
        for (
          n = (this.state[~~(a / 8)] & (1 << (7 - (a % 8)))) !== 0,
            n &&
              ((e[0] ^= t[0]), (e[1] ^= t[1]), (e[2] ^= t[2]), (e[3] ^= t[3])),
            i = (t[3] & 1) !== 0,
            r = 3;
          r > 0;
          r--
        )
          t[r] = (t[r] >>> 1) | ((t[r - 1] & 1) << 31);
        (t[0] = t[0] >>> 1), i && (t[0] = t[0] ^ (225 << 24));
      }
      this.state = Tv(e);
    };
    oa.prototype.update = function (t) {
      this.cache = Gn.concat([this.cache, t]);
      for (var e; this.cache.length >= 16; )
        (e = this.cache.slice(0, 16)),
          (this.cache = this.cache.slice(16)),
          this.ghash(e);
    };
    oa.prototype.final = function (t, e) {
      return (
        this.cache.length && this.ghash(Gn.concat([this.cache, D9], 16)),
        this.ghash(Tv([0, t, 0, e])),
        this.state
      );
    };
    Pv.exports = oa;
  });
  var Qh = R((yT, Nv) => {
    S();
    var F9 = aa(),
      _t = Ie().Buffer,
      Ov = zr(),
      j9 = qe(),
      Cv = kv(),
      U9 = Vn(),
      z9 = Gh();
    function H9(t, e) {
      var r = 0;
      t.length !== e.length && r++;
      for (var n = Math.min(t.length, e.length), i = 0; i < n; ++i)
        r += t[i] ^ e[i];
      return r;
    }
    function K9(t, e, r) {
      if (e.length === 12)
        return (
          (t._finID = _t.concat([e, _t.from([0, 0, 0, 1])])),
          _t.concat([e, _t.from([0, 0, 0, 2])])
        );
      var n = new Cv(r),
        i = e.length,
        a = i % 16;
      n.update(e),
        a && ((a = 16 - a), n.update(_t.alloc(a, 0))),
        n.update(_t.alloc(8, 0));
      var h = i * 8,
        v = _t.alloc(8);
      v.writeUIntBE(h, 0, 8), n.update(v), (t._finID = n.state);
      var g = _t.from(t._finID);
      return z9(g), g;
    }
    function an(t, e, r, n) {
      Ov.call(this);
      var i = _t.alloc(4, 0);
      this._cipher = new F9.AES(e);
      var a = this._cipher.encryptBlock(i);
      (this._ghash = new Cv(a)),
        (r = K9(this, r, a)),
        (this._prev = _t.from(r)),
        (this._cache = _t.allocUnsafe(0)),
        (this._secCache = _t.allocUnsafe(0)),
        (this._decrypt = n),
        (this._alen = 0),
        (this._len = 0),
        (this._mode = t),
        (this._authTag = null),
        (this._called = !1);
    }
    j9(an, Ov);
    an.prototype._update = function (t) {
      if (!this._called && this._alen) {
        var e = 16 - (this._alen % 16);
        e < 16 && ((e = _t.alloc(e, 0)), this._ghash.update(e));
      }
      this._called = !0;
      var r = this._mode.encrypt(this, t);
      return (
        this._decrypt ? this._ghash.update(t) : this._ghash.update(r),
        (this._len += t.length),
        r
      );
    };
    an.prototype._final = function () {
      if (this._decrypt && !this._authTag)
        throw new Error('Unsupported state or unable to authenticate data');
      var t = U9(
        this._ghash.final(this._alen * 8, this._len * 8),
        this._cipher.encryptBlock(this._finID)
      );
      if (this._decrypt && H9(t, this._authTag))
        throw new Error('Unsupported state or unable to authenticate data');
      (this._authTag = t), this._cipher.scrub();
    };
    an.prototype.getAuthTag = function () {
      if (this._decrypt || !_t.isBuffer(this._authTag))
        throw new Error('Attempting to get auth tag in unsupported state');
      return this._authTag;
    };
    an.prototype.setAuthTag = function (e) {
      if (!this._decrypt)
        throw new Error('Attempting to set auth tag in unsupported state');
      this._authTag = e;
    };
    an.prototype.setAAD = function (e) {
      if (this._called)
        throw new Error('Attempting to set AAD in unsupported state');
      this._ghash.update(e), (this._alen += e.length);
    };
    Nv.exports = an;
  });
  var tu = R((gT, Lv) => {
    S();
    var V9 = aa(),
      eu = Ie().Buffer,
      Dv = zr(),
      G9 = qe();
    function Qo(t, e, r, n) {
      Dv.call(this),
        (this._cipher = new V9.AES(e)),
        (this._prev = eu.from(r)),
        (this._cache = eu.allocUnsafe(0)),
        (this._secCache = eu.allocUnsafe(0)),
        (this._decrypt = n),
        (this._mode = t);
    }
    G9(Qo, Dv);
    Qo.prototype._update = function (t) {
      return this._mode.encrypt(this, t, this._decrypt);
    };
    Qo.prototype._final = function () {
      this._cipher.scrub();
    };
    Lv.exports = Qo;
  });
  var sa = R((_T, Fv) => {
    S();
    var on = Ie().Buffer,
      W9 = ko();
    function $9(t, e, r, n) {
      if (
        (on.isBuffer(t) || (t = on.from(t, 'binary')),
        e && (on.isBuffer(e) || (e = on.from(e, 'binary')), e.length !== 8))
      )
        throw new RangeError('salt should be Buffer with 8 byte length');
      for (
        var i = r / 8, a = on.alloc(i), h = on.alloc(n || 0), v = on.alloc(0);
        i > 0 || n > 0;

      ) {
        var g = new W9();
        g.update(v), g.update(t), e && g.update(e), (v = g.digest());
        var M = 0;
        if (i > 0) {
          var x = a.length - i;
          (M = Math.min(i, v.length)), v.copy(a, x, 0, M), (i -= M);
        }
        if (M < v.length && n > 0) {
          var E = h.length - n,
            I = Math.min(n, v.length - M);
          v.copy(h, E, M, M + I), (n -= I);
        }
      }
      return v.fill(0), { key: a, iv: h };
    }
    Fv.exports = $9;
  });
  var Hv = R((ru) => {
    S();
    var jv = Xo(),
      Z9 = Qh(),
      Kr = Ie().Buffer,
      J9 = tu(),
      Uv = zr(),
      X9 = aa(),
      Y9 = sa(),
      Q9 = qe();
    function ha(t, e, r) {
      Uv.call(this),
        (this._cache = new es()),
        (this._cipher = new X9.AES(e)),
        (this._prev = Kr.from(r)),
        (this._mode = t),
        (this._autopadding = !0);
    }
    Q9(ha, Uv);
    ha.prototype._update = function (t) {
      this._cache.add(t);
      for (var e, r, n = []; (e = this._cache.get()); )
        (r = this._mode.encrypt(this, e)), n.push(r);
      return Kr.concat(n);
    };
    var ew = Kr.alloc(16, 16);
    ha.prototype._final = function () {
      var t = this._cache.flush();
      if (this._autopadding)
        return (t = this._mode.encrypt(this, t)), this._cipher.scrub(), t;
      if (!t.equals(ew))
        throw (
          (this._cipher.scrub(), new Error('data not multiple of block length'))
        );
    };
    ha.prototype.setAutoPadding = function (t) {
      return (this._autopadding = !!t), this;
    };
    function es() {
      this.cache = Kr.allocUnsafe(0);
    }
    es.prototype.add = function (t) {
      this.cache = Kr.concat([this.cache, t]);
    };
    es.prototype.get = function () {
      if (this.cache.length > 15) {
        var t = this.cache.slice(0, 16);
        return (this.cache = this.cache.slice(16)), t;
      }
      return null;
    };
    es.prototype.flush = function () {
      for (
        var t = 16 - this.cache.length, e = Kr.allocUnsafe(t), r = -1;
        ++r < t;

      )
        e.writeUInt8(t, r);
      return Kr.concat([this.cache, e]);
    };
    function zv(t, e, r) {
      var n = jv[t.toLowerCase()];
      if (!n) throw new TypeError('invalid suite type');
      if ((typeof e == 'string' && (e = Kr.from(e)), e.length !== n.key / 8))
        throw new TypeError('invalid key length ' + e.length);
      if (
        (typeof r == 'string' && (r = Kr.from(r)),
        n.mode !== 'GCM' && r.length !== n.iv)
      )
        throw new TypeError('invalid iv length ' + r.length);
      return n.type === 'stream'
        ? new J9(n.module, e, r)
        : n.type === 'auth'
          ? new Z9(n.module, e, r)
          : new ha(n.module, e, r);
    }
    function tw(t, e) {
      var r = jv[t.toLowerCase()];
      if (!r) throw new TypeError('invalid suite type');
      var n = Y9(e, !1, r.key, r.iv);
      return zv(t, n.key, n.iv);
    }
    ru.createCipheriv = zv;
    ru.createCipher = tw;
  });
  var Wv = R((iu) => {
    S();
    var rw = Qh(),
      Wn = Ie().Buffer,
      Kv = Xo(),
      iw = tu(),
      Vv = zr(),
      nw = aa(),
      fw = sa(),
      aw = qe();
    function ua(t, e, r) {
      Vv.call(this),
        (this._cache = new ts()),
        (this._last = void 0),
        (this._cipher = new nw.AES(e)),
        (this._prev = Wn.from(r)),
        (this._mode = t),
        (this._autopadding = !0);
    }
    aw(ua, Vv);
    ua.prototype._update = function (t) {
      this._cache.add(t);
      for (var e, r, n = []; (e = this._cache.get(this._autopadding)); )
        (r = this._mode.decrypt(this, e)), n.push(r);
      return Wn.concat(n);
    };
    ua.prototype._final = function () {
      var t = this._cache.flush();
      if (this._autopadding) return ow(this._mode.decrypt(this, t));
      if (t) throw new Error('data not multiple of block length');
    };
    ua.prototype.setAutoPadding = function (t) {
      return (this._autopadding = !!t), this;
    };
    function ts() {
      this.cache = Wn.allocUnsafe(0);
    }
    ts.prototype.add = function (t) {
      this.cache = Wn.concat([this.cache, t]);
    };
    ts.prototype.get = function (t) {
      var e;
      if (t) {
        if (this.cache.length > 16)
          return (
            (e = this.cache.slice(0, 16)),
            (this.cache = this.cache.slice(16)),
            e
          );
      } else if (this.cache.length >= 16)
        return (
          (e = this.cache.slice(0, 16)), (this.cache = this.cache.slice(16)), e
        );
      return null;
    };
    ts.prototype.flush = function () {
      if (this.cache.length) return this.cache;
    };
    function ow(t) {
      var e = t[15];
      if (e < 1 || e > 16) throw new Error('unable to decrypt data');
      for (var r = -1; ++r < e; )
        if (t[r + (16 - e)] !== e) throw new Error('unable to decrypt data');
      if (e !== 16) return t.slice(0, 16 - e);
    }
    function Gv(t, e, r) {
      var n = Kv[t.toLowerCase()];
      if (!n) throw new TypeError('invalid suite type');
      if (
        (typeof r == 'string' && (r = Wn.from(r)),
        n.mode !== 'GCM' && r.length !== n.iv)
      )
        throw new TypeError('invalid iv length ' + r.length);
      if ((typeof e == 'string' && (e = Wn.from(e)), e.length !== n.key / 8))
        throw new TypeError('invalid key length ' + e.length);
      return n.type === 'stream'
        ? new iw(n.module, e, r, !0)
        : n.type === 'auth'
          ? new rw(n.module, e, r, !0)
          : new ua(n.module, e, r);
    }
    function sw(t, e) {
      var r = Kv[t.toLowerCase()];
      if (!r) throw new TypeError('invalid suite type');
      var n = fw(e, !1, r.key, r.iv);
      return Gv(t, n.key, n.iv);
    }
    iu.createDecipher = sw;
    iu.createDecipheriv = Gv;
  });
  var rs = R((dr) => {
    S();
    var $v = Hv(),
      Zv = Wv(),
      hw = Zh();
    function uw() {
      return Object.keys(hw);
    }
    dr.createCipher = dr.Cipher = $v.createCipher;
    dr.createCipheriv = dr.Cipheriv = $v.createCipheriv;
    dr.createDecipher = dr.Decipher = Zv.createDecipher;
    dr.createDecipheriv = dr.Decipheriv = Zv.createDecipheriv;
    dr.listCiphers = dr.getCiphers = uw;
  });
  var Jv = R((Vr) => {
    S();
    Vr['des-ecb'] = { key: 8, iv: 0 };
    Vr['des-cbc'] = Vr.des = { key: 8, iv: 8 };
    Vr['des-ede3-cbc'] = Vr.des3 = { key: 24, iv: 8 };
    Vr['des-ede3'] = { key: 24, iv: 0 };
    Vr['des-ede-cbc'] = { key: 16, iv: 8 };
    Vr['des-ede'] = { key: 16, iv: 0 };
  });
  var tb = R((lr) => {
    S();
    var Xv = cv(),
      nu = rs(),
      Ai = Xo(),
      Gr = Jv(),
      Yv = sa();
    function cw(t, e) {
      t = t.toLowerCase();
      var r, n;
      if (Ai[t]) (r = Ai[t].key), (n = Ai[t].iv);
      else if (Gr[t]) (r = Gr[t].key * 8), (n = Gr[t].iv);
      else throw new TypeError('invalid suite type');
      var i = Yv(e, !1, r, n);
      return Qv(t, i.key, i.iv);
    }
    function dw(t, e) {
      t = t.toLowerCase();
      var r, n;
      if (Ai[t]) (r = Ai[t].key), (n = Ai[t].iv);
      else if (Gr[t]) (r = Gr[t].key * 8), (n = Gr[t].iv);
      else throw new TypeError('invalid suite type');
      var i = Yv(e, !1, r, n);
      return eb(t, i.key, i.iv);
    }
    function Qv(t, e, r) {
      if (((t = t.toLowerCase()), Ai[t])) return nu.createCipheriv(t, e, r);
      if (Gr[t]) return new Xv({ key: e, iv: r, mode: t });
      throw new TypeError('invalid suite type');
    }
    function eb(t, e, r) {
      if (((t = t.toLowerCase()), Ai[t])) return nu.createDecipheriv(t, e, r);
      if (Gr[t]) return new Xv({ key: e, iv: r, mode: t, decrypt: !0 });
      throw new TypeError('invalid suite type');
    }
    function lw() {
      return Object.keys(Gr).concat(nu.getCiphers());
    }
    lr.createCipher = lr.Cipher = cw;
    lr.createCipheriv = lr.Cipheriv = Qv;
    lr.createDecipher = lr.Decipher = dw;
    lr.createDecipheriv = lr.Decipheriv = eb;
    lr.listCiphers = lr.getCiphers = lw;
  });
  var it = R((rb, fu) => {
    S();
    (function (t, e) {
      'use strict';
      function r(B, b) {
        if (!B) throw new Error(b || 'Assertion failed');
      }
      function n(B, b) {
        B.super_ = b;
        var _ = function () {};
        (_.prototype = b.prototype),
          (B.prototype = new _()),
          (B.prototype.constructor = B);
      }
      function i(B, b, _) {
        if (i.isBN(B)) return B;
        (this.negative = 0),
          (this.words = null),
          (this.length = 0),
          (this.red = null),
          B !== null &&
            ((b === 'le' || b === 'be') && ((_ = b), (b = 10)),
            this._init(B || 0, b || 10, _ || 'be'));
      }
      typeof t == 'object' ? (t.exports = i) : (e.BN = i),
        (i.BN = i),
        (i.wordSize = 26);
      var a;
      try {
        typeof window < 'u' && typeof window.Buffer < 'u'
          ? (a = window.Buffer)
          : (a = Et().Buffer);
      } catch {}
      (i.isBN = function (b) {
        return b instanceof i
          ? !0
          : b !== null &&
              typeof b == 'object' &&
              b.constructor.wordSize === i.wordSize &&
              Array.isArray(b.words);
      }),
        (i.max = function (b, _) {
          return b.cmp(_) > 0 ? b : _;
        }),
        (i.min = function (b, _) {
          return b.cmp(_) < 0 ? b : _;
        }),
        (i.prototype._init = function (b, _, l) {
          if (typeof b == 'number') return this._initNumber(b, _, l);
          if (typeof b == 'object') return this._initArray(b, _, l);
          _ === 'hex' && (_ = 16),
            r(_ === (_ | 0) && _ >= 2 && _ <= 36),
            (b = b.toString().replace(/\s+/g, ''));
          var f = 0;
          b[0] === '-' && (f++, (this.negative = 1)),
            f < b.length &&
              (_ === 16
                ? this._parseHex(b, f, l)
                : (this._parseBase(b, _, f),
                  l === 'le' && this._initArray(this.toArray(), _, l)));
        }),
        (i.prototype._initNumber = function (b, _, l) {
          b < 0 && ((this.negative = 1), (b = -b)),
            b < 67108864
              ? ((this.words = [b & 67108863]), (this.length = 1))
              : b < 4503599627370496
                ? ((this.words = [b & 67108863, (b / 67108864) & 67108863]),
                  (this.length = 2))
                : (r(b < 9007199254740992),
                  (this.words = [b & 67108863, (b / 67108864) & 67108863, 1]),
                  (this.length = 3)),
            l === 'le' && this._initArray(this.toArray(), _, l);
        }),
        (i.prototype._initArray = function (b, _, l) {
          if ((r(typeof b.length == 'number'), b.length <= 0))
            return (this.words = [0]), (this.length = 1), this;
          (this.length = Math.ceil(b.length / 3)),
            (this.words = new Array(this.length));
          for (var f = 0; f < this.length; f++) this.words[f] = 0;
          var o,
            c,
            p = 0;
          if (l === 'be')
            for (f = b.length - 1, o = 0; f >= 0; f -= 3)
              (c = b[f] | (b[f - 1] << 8) | (b[f - 2] << 16)),
                (this.words[o] |= (c << p) & 67108863),
                (this.words[o + 1] = (c >>> (26 - p)) & 67108863),
                (p += 24),
                p >= 26 && ((p -= 26), o++);
          else if (l === 'le')
            for (f = 0, o = 0; f < b.length; f += 3)
              (c = b[f] | (b[f + 1] << 8) | (b[f + 2] << 16)),
                (this.words[o] |= (c << p) & 67108863),
                (this.words[o + 1] = (c >>> (26 - p)) & 67108863),
                (p += 24),
                p >= 26 && ((p -= 26), o++);
          return this.strip();
        });
      function h(B, b) {
        var _ = B.charCodeAt(b);
        return _ >= 65 && _ <= 70
          ? _ - 55
          : _ >= 97 && _ <= 102
            ? _ - 87
            : (_ - 48) & 15;
      }
      function v(B, b, _) {
        var l = h(B, _);
        return _ - 1 >= b && (l |= h(B, _ - 1) << 4), l;
      }
      i.prototype._parseHex = function (b, _, l) {
        (this.length = Math.ceil((b.length - _) / 6)),
          (this.words = new Array(this.length));
        for (var f = 0; f < this.length; f++) this.words[f] = 0;
        var o = 0,
          c = 0,
          p;
        if (l === 'be')
          for (f = b.length - 1; f >= _; f -= 2)
            (p = v(b, _, f) << o),
              (this.words[c] |= p & 67108863),
              o >= 18
                ? ((o -= 18), (c += 1), (this.words[c] |= p >>> 26))
                : (o += 8);
        else {
          var d = b.length - _;
          for (f = d % 2 === 0 ? _ + 1 : _; f < b.length; f += 2)
            (p = v(b, _, f) << o),
              (this.words[c] |= p & 67108863),
              o >= 18
                ? ((o -= 18), (c += 1), (this.words[c] |= p >>> 26))
                : (o += 8);
        }
        this.strip();
      };
      function g(B, b, _, l) {
        for (var f = 0, o = Math.min(B.length, _), c = b; c < o; c++) {
          var p = B.charCodeAt(c) - 48;
          (f *= l),
            p >= 49
              ? (f += p - 49 + 10)
              : p >= 17
                ? (f += p - 17 + 10)
                : (f += p);
        }
        return f;
      }
      (i.prototype._parseBase = function (b, _, l) {
        (this.words = [0]), (this.length = 1);
        for (var f = 0, o = 1; o <= 67108863; o *= _) f++;
        f--, (o = (o / _) | 0);
        for (
          var c = b.length - l,
            p = c % f,
            d = Math.min(c, c - p) + l,
            u = 0,
            y = l;
          y < d;
          y += f
        )
          (u = g(b, y, y + f, _)),
            this.imuln(o),
            this.words[0] + u < 67108864
              ? (this.words[0] += u)
              : this._iaddn(u);
        if (p !== 0) {
          var m = 1;
          for (u = g(b, y, b.length, _), y = 0; y < p; y++) m *= _;
          this.imuln(m),
            this.words[0] + u < 67108864
              ? (this.words[0] += u)
              : this._iaddn(u);
        }
        this.strip();
      }),
        (i.prototype.copy = function (b) {
          b.words = new Array(this.length);
          for (var _ = 0; _ < this.length; _++) b.words[_] = this.words[_];
          (b.length = this.length),
            (b.negative = this.negative),
            (b.red = this.red);
        }),
        (i.prototype.clone = function () {
          var b = new i(null);
          return this.copy(b), b;
        }),
        (i.prototype._expand = function (b) {
          for (; this.length < b; ) this.words[this.length++] = 0;
          return this;
        }),
        (i.prototype.strip = function () {
          for (; this.length > 1 && this.words[this.length - 1] === 0; )
            this.length--;
          return this._normSign();
        }),
        (i.prototype._normSign = function () {
          return (
            this.length === 1 && this.words[0] === 0 && (this.negative = 0),
            this
          );
        }),
        (i.prototype.inspect = function () {
          return (this.red ? '<BN-R: ' : '<BN: ') + this.toString(16) + '>';
        });
      var M = [
          '',
          '0',
          '00',
          '000',
          '0000',
          '00000',
          '000000',
          '0000000',
          '00000000',
          '000000000',
          '0000000000',
          '00000000000',
          '000000000000',
          '0000000000000',
          '00000000000000',
          '000000000000000',
          '0000000000000000',
          '00000000000000000',
          '000000000000000000',
          '0000000000000000000',
          '00000000000000000000',
          '000000000000000000000',
          '0000000000000000000000',
          '00000000000000000000000',
          '000000000000000000000000',
          '0000000000000000000000000',
        ],
        x = [
          0, 0, 25, 16, 12, 11, 10, 9, 8, 8, 7, 7, 7, 7, 6, 6, 6, 6, 6, 6, 6, 5,
          5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
        ],
        E = [
          0, 0, 33554432, 43046721, 16777216, 48828125, 60466176, 40353607,
          16777216, 43046721, 1e7, 19487171, 35831808, 62748517, 7529536,
          11390625, 16777216, 24137569, 34012224, 47045881, 64e6, 4084101,
          5153632, 6436343, 7962624, 9765625, 11881376, 14348907, 17210368,
          20511149, 243e5, 28629151, 33554432, 39135393, 45435424, 52521875,
          60466176,
        ];
      (i.prototype.toString = function (b, _) {
        (b = b || 10), (_ = _ | 0 || 1);
        var l;
        if (b === 16 || b === 'hex') {
          l = '';
          for (var f = 0, o = 0, c = 0; c < this.length; c++) {
            var p = this.words[c],
              d = (((p << f) | o) & 16777215).toString(16);
            (o = (p >>> (24 - f)) & 16777215),
              o !== 0 || c !== this.length - 1
                ? (l = M[6 - d.length] + d + l)
                : (l = d + l),
              (f += 2),
              f >= 26 && ((f -= 26), c--);
          }
          for (o !== 0 && (l = o.toString(16) + l); l.length % _ !== 0; )
            l = '0' + l;
          return this.negative !== 0 && (l = '-' + l), l;
        }
        if (b === (b | 0) && b >= 2 && b <= 36) {
          var u = x[b],
            y = E[b];
          l = '';
          var m = this.clone();
          for (m.negative = 0; !m.isZero(); ) {
            var s = m.modn(y).toString(b);
            (m = m.idivn(y)),
              m.isZero() ? (l = s + l) : (l = M[u - s.length] + s + l);
          }
          for (this.isZero() && (l = '0' + l); l.length % _ !== 0; )
            l = '0' + l;
          return this.negative !== 0 && (l = '-' + l), l;
        }
        r(!1, 'Base should be between 2 and 36');
      }),
        (i.prototype.toNumber = function () {
          var b = this.words[0];
          return (
            this.length === 2
              ? (b += this.words[1] * 67108864)
              : this.length === 3 && this.words[2] === 1
                ? (b += 4503599627370496 + this.words[1] * 67108864)
                : this.length > 2 &&
                  r(!1, 'Number can only safely store up to 53 bits'),
            this.negative !== 0 ? -b : b
          );
        }),
        (i.prototype.toJSON = function () {
          return this.toString(16);
        }),
        (i.prototype.toBuffer = function (b, _) {
          return r(typeof a < 'u'), this.toArrayLike(a, b, _);
        }),
        (i.prototype.toArray = function (b, _) {
          return this.toArrayLike(Array, b, _);
        }),
        (i.prototype.toArrayLike = function (b, _, l) {
          var f = this.byteLength(),
            o = l || Math.max(1, f);
          r(f <= o, 'byte array longer than desired length'),
            r(o > 0, 'Requested array length <= 0'),
            this.strip();
          var c = _ === 'le',
            p = new b(o),
            d,
            u,
            y = this.clone();
          if (c) {
            for (u = 0; !y.isZero(); u++)
              (d = y.andln(255)), y.iushrn(8), (p[u] = d);
            for (; u < o; u++) p[u] = 0;
          } else {
            for (u = 0; u < o - f; u++) p[u] = 0;
            for (u = 0; !y.isZero(); u++)
              (d = y.andln(255)), y.iushrn(8), (p[o - u - 1] = d);
          }
          return p;
        }),
        Math.clz32
          ? (i.prototype._countBits = function (b) {
              return 32 - Math.clz32(b);
            })
          : (i.prototype._countBits = function (b) {
              var _ = b,
                l = 0;
              return (
                _ >= 4096 && ((l += 13), (_ >>>= 13)),
                _ >= 64 && ((l += 7), (_ >>>= 7)),
                _ >= 8 && ((l += 4), (_ >>>= 4)),
                _ >= 2 && ((l += 2), (_ >>>= 2)),
                l + _
              );
            }),
        (i.prototype._zeroBits = function (b) {
          if (b === 0) return 26;
          var _ = b,
            l = 0;
          return (
            (_ & 8191) === 0 && ((l += 13), (_ >>>= 13)),
            (_ & 127) === 0 && ((l += 7), (_ >>>= 7)),
            (_ & 15) === 0 && ((l += 4), (_ >>>= 4)),
            (_ & 3) === 0 && ((l += 2), (_ >>>= 2)),
            (_ & 1) === 0 && l++,
            l
          );
        }),
        (i.prototype.bitLength = function () {
          var b = this.words[this.length - 1],
            _ = this._countBits(b);
          return (this.length - 1) * 26 + _;
        });
      function I(B) {
        for (var b = new Array(B.bitLength()), _ = 0; _ < b.length; _++) {
          var l = (_ / 26) | 0,
            f = _ % 26;
          b[_] = (B.words[l] & (1 << f)) >>> f;
        }
        return b;
      }
      (i.prototype.zeroBits = function () {
        if (this.isZero()) return 0;
        for (var b = 0, _ = 0; _ < this.length; _++) {
          var l = this._zeroBits(this.words[_]);
          if (((b += l), l !== 26)) break;
        }
        return b;
      }),
        (i.prototype.byteLength = function () {
          return Math.ceil(this.bitLength() / 8);
        }),
        (i.prototype.toTwos = function (b) {
          return this.negative !== 0
            ? this.abs().inotn(b).iaddn(1)
            : this.clone();
        }),
        (i.prototype.fromTwos = function (b) {
          return this.testn(b - 1)
            ? this.notn(b).iaddn(1).ineg()
            : this.clone();
        }),
        (i.prototype.isNeg = function () {
          return this.negative !== 0;
        }),
        (i.prototype.neg = function () {
          return this.clone().ineg();
        }),
        (i.prototype.ineg = function () {
          return this.isZero() || (this.negative ^= 1), this;
        }),
        (i.prototype.iuor = function (b) {
          for (; this.length < b.length; ) this.words[this.length++] = 0;
          for (var _ = 0; _ < b.length; _++)
            this.words[_] = this.words[_] | b.words[_];
          return this.strip();
        }),
        (i.prototype.ior = function (b) {
          return r((this.negative | b.negative) === 0), this.iuor(b);
        }),
        (i.prototype.or = function (b) {
          return this.length > b.length
            ? this.clone().ior(b)
            : b.clone().ior(this);
        }),
        (i.prototype.uor = function (b) {
          return this.length > b.length
            ? this.clone().iuor(b)
            : b.clone().iuor(this);
        }),
        (i.prototype.iuand = function (b) {
          var _;
          this.length > b.length ? (_ = b) : (_ = this);
          for (var l = 0; l < _.length; l++)
            this.words[l] = this.words[l] & b.words[l];
          return (this.length = _.length), this.strip();
        }),
        (i.prototype.iand = function (b) {
          return r((this.negative | b.negative) === 0), this.iuand(b);
        }),
        (i.prototype.and = function (b) {
          return this.length > b.length
            ? this.clone().iand(b)
            : b.clone().iand(this);
        }),
        (i.prototype.uand = function (b) {
          return this.length > b.length
            ? this.clone().iuand(b)
            : b.clone().iuand(this);
        }),
        (i.prototype.iuxor = function (b) {
          var _, l;
          this.length > b.length
            ? ((_ = this), (l = b))
            : ((_ = b), (l = this));
          for (var f = 0; f < l.length; f++)
            this.words[f] = _.words[f] ^ l.words[f];
          if (this !== _) for (; f < _.length; f++) this.words[f] = _.words[f];
          return (this.length = _.length), this.strip();
        }),
        (i.prototype.ixor = function (b) {
          return r((this.negative | b.negative) === 0), this.iuxor(b);
        }),
        (i.prototype.xor = function (b) {
          return this.length > b.length
            ? this.clone().ixor(b)
            : b.clone().ixor(this);
        }),
        (i.prototype.uxor = function (b) {
          return this.length > b.length
            ? this.clone().iuxor(b)
            : b.clone().iuxor(this);
        }),
        (i.prototype.inotn = function (b) {
          r(typeof b == 'number' && b >= 0);
          var _ = Math.ceil(b / 26) | 0,
            l = b % 26;
          this._expand(_), l > 0 && _--;
          for (var f = 0; f < _; f++) this.words[f] = ~this.words[f] & 67108863;
          return (
            l > 0 && (this.words[f] = ~this.words[f] & (67108863 >> (26 - l))),
            this.strip()
          );
        }),
        (i.prototype.notn = function (b) {
          return this.clone().inotn(b);
        }),
        (i.prototype.setn = function (b, _) {
          r(typeof b == 'number' && b >= 0);
          var l = (b / 26) | 0,
            f = b % 26;
          return (
            this._expand(l + 1),
            _
              ? (this.words[l] = this.words[l] | (1 << f))
              : (this.words[l] = this.words[l] & ~(1 << f)),
            this.strip()
          );
        }),
        (i.prototype.iadd = function (b) {
          var _;
          if (this.negative !== 0 && b.negative === 0)
            return (
              (this.negative = 0),
              (_ = this.isub(b)),
              (this.negative ^= 1),
              this._normSign()
            );
          if (this.negative === 0 && b.negative !== 0)
            return (
              (b.negative = 0),
              (_ = this.isub(b)),
              (b.negative = 1),
              _._normSign()
            );
          var l, f;
          this.length > b.length
            ? ((l = this), (f = b))
            : ((l = b), (f = this));
          for (var o = 0, c = 0; c < f.length; c++)
            (_ = (l.words[c] | 0) + (f.words[c] | 0) + o),
              (this.words[c] = _ & 67108863),
              (o = _ >>> 26);
          for (; o !== 0 && c < l.length; c++)
            (_ = (l.words[c] | 0) + o),
              (this.words[c] = _ & 67108863),
              (o = _ >>> 26);
          if (((this.length = l.length), o !== 0))
            (this.words[this.length] = o), this.length++;
          else if (l !== this)
            for (; c < l.length; c++) this.words[c] = l.words[c];
          return this;
        }),
        (i.prototype.add = function (b) {
          var _;
          return b.negative !== 0 && this.negative === 0
            ? ((b.negative = 0), (_ = this.sub(b)), (b.negative ^= 1), _)
            : b.negative === 0 && this.negative !== 0
              ? ((this.negative = 0), (_ = b.sub(this)), (this.negative = 1), _)
              : this.length > b.length
                ? this.clone().iadd(b)
                : b.clone().iadd(this);
        }),
        (i.prototype.isub = function (b) {
          if (b.negative !== 0) {
            b.negative = 0;
            var _ = this.iadd(b);
            return (b.negative = 1), _._normSign();
          } else if (this.negative !== 0)
            return (
              (this.negative = 0),
              this.iadd(b),
              (this.negative = 1),
              this._normSign()
            );
          var l = this.cmp(b);
          if (l === 0)
            return (
              (this.negative = 0), (this.length = 1), (this.words[0] = 0), this
            );
          var f, o;
          l > 0 ? ((f = this), (o = b)) : ((f = b), (o = this));
          for (var c = 0, p = 0; p < o.length; p++)
            (_ = (f.words[p] | 0) - (o.words[p] | 0) + c),
              (c = _ >> 26),
              (this.words[p] = _ & 67108863);
          for (; c !== 0 && p < f.length; p++)
            (_ = (f.words[p] | 0) + c),
              (c = _ >> 26),
              (this.words[p] = _ & 67108863);
          if (c === 0 && p < f.length && f !== this)
            for (; p < f.length; p++) this.words[p] = f.words[p];
          return (
            (this.length = Math.max(this.length, p)),
            f !== this && (this.negative = 1),
            this.strip()
          );
        }),
        (i.prototype.sub = function (b) {
          return this.clone().isub(b);
        });
      function q(B, b, _) {
        _.negative = b.negative ^ B.negative;
        var l = (B.length + b.length) | 0;
        (_.length = l), (l = (l - 1) | 0);
        var f = B.words[0] | 0,
          o = b.words[0] | 0,
          c = f * o,
          p = c & 67108863,
          d = (c / 67108864) | 0;
        _.words[0] = p;
        for (var u = 1; u < l; u++) {
          for (
            var y = d >>> 26,
              m = d & 67108863,
              s = Math.min(u, b.length - 1),
              w = Math.max(0, u - B.length + 1);
            w <= s;
            w++
          ) {
            var T = (u - w) | 0;
            (f = B.words[T] | 0),
              (o = b.words[w] | 0),
              (c = f * o + m),
              (y += (c / 67108864) | 0),
              (m = c & 67108863);
          }
          (_.words[u] = m | 0), (d = y | 0);
        }
        return d !== 0 ? (_.words[u] = d | 0) : _.length--, _.strip();
      }
      var k = function (b, _, l) {
        var f = b.words,
          o = _.words,
          c = l.words,
          p = 0,
          d,
          u,
          y,
          m = f[0] | 0,
          s = m & 8191,
          w = m >>> 13,
          T = f[1] | 0,
          O = T & 8191,
          P = T >>> 13,
          N = f[2] | 0,
          F = N & 8191,
          j = N >>> 13,
          et = f[3] | 0,
          z = et & 8191,
          H = et >>> 13,
          ei = f[4] | 0,
          K = ei & 8191,
          V = ei >>> 13,
          ti = f[5] | 0,
          G = ti & 8191,
          W = ti >>> 13,
          ri = f[6] | 0,
          $ = ri & 8191,
          Z = ri >>> 13,
          ii = f[7] | 0,
          J = ii & 8191,
          X = ii >>> 13,
          ni = f[8] | 0,
          Y = ni & 8191,
          Q = ni >>> 13,
          fi = f[9] | 0,
          ee = fi & 8191,
          te = fi >>> 13,
          ai = o[0] | 0,
          re = ai & 8191,
          ie = ai >>> 13,
          oi = o[1] | 0,
          ne = oi & 8191,
          fe = oi >>> 13,
          si = o[2] | 0,
          ae = si & 8191,
          oe = si >>> 13,
          hi = o[3] | 0,
          se = hi & 8191,
          he = hi >>> 13,
          ui = o[4] | 0,
          ue = ui & 8191,
          ce = ui >>> 13,
          ci = o[5] | 0,
          de = ci & 8191,
          le = ci >>> 13,
          di = o[6] | 0,
          pe = di & 8191,
          ve = di >>> 13,
          li = o[7] | 0,
          be = li & 8191,
          ye = li >>> 13,
          pi = o[8] | 0,
          me = pi & 8191,
          ge = pi >>> 13,
          vi = o[9] | 0,
          we = vi & 8191,
          _e = vi >>> 13;
        (l.negative = b.negative ^ _.negative),
          (l.length = 19),
          (d = Math.imul(s, re)),
          (u = Math.imul(s, ie)),
          (u = (u + Math.imul(w, re)) | 0),
          (y = Math.imul(w, ie));
        var Lr = (((p + d) | 0) + ((u & 8191) << 13)) | 0;
        (p = (((y + (u >>> 13)) | 0) + (Lr >>> 26)) | 0),
          (Lr &= 67108863),
          (d = Math.imul(O, re)),
          (u = Math.imul(O, ie)),
          (u = (u + Math.imul(P, re)) | 0),
          (y = Math.imul(P, ie)),
          (d = (d + Math.imul(s, ne)) | 0),
          (u = (u + Math.imul(s, fe)) | 0),
          (u = (u + Math.imul(w, ne)) | 0),
          (y = (y + Math.imul(w, fe)) | 0);
        var Ae = (((p + d) | 0) + ((u & 8191) << 13)) | 0;
        (p = (((y + (u >>> 13)) | 0) + (Ae >>> 26)) | 0),
          (Ae &= 67108863),
          (d = Math.imul(F, re)),
          (u = Math.imul(F, ie)),
          (u = (u + Math.imul(j, re)) | 0),
          (y = Math.imul(j, ie)),
          (d = (d + Math.imul(O, ne)) | 0),
          (u = (u + Math.imul(O, fe)) | 0),
          (u = (u + Math.imul(P, ne)) | 0),
          (y = (y + Math.imul(P, fe)) | 0),
          (d = (d + Math.imul(s, ae)) | 0),
          (u = (u + Math.imul(s, oe)) | 0),
          (u = (u + Math.imul(w, ae)) | 0),
          (y = (y + Math.imul(w, oe)) | 0);
        var Be = (((p + d) | 0) + ((u & 8191) << 13)) | 0;
        (p = (((y + (u >>> 13)) | 0) + (Be >>> 26)) | 0),
          (Be &= 67108863),
          (d = Math.imul(z, re)),
          (u = Math.imul(z, ie)),
          (u = (u + Math.imul(H, re)) | 0),
          (y = Math.imul(H, ie)),
          (d = (d + Math.imul(F, ne)) | 0),
          (u = (u + Math.imul(F, fe)) | 0),
          (u = (u + Math.imul(j, ne)) | 0),
          (y = (y + Math.imul(j, fe)) | 0),
          (d = (d + Math.imul(O, ae)) | 0),
          (u = (u + Math.imul(O, oe)) | 0),
          (u = (u + Math.imul(P, ae)) | 0),
          (y = (y + Math.imul(P, oe)) | 0),
          (d = (d + Math.imul(s, se)) | 0),
          (u = (u + Math.imul(s, he)) | 0),
          (u = (u + Math.imul(w, se)) | 0),
          (y = (y + Math.imul(w, he)) | 0);
        var $t = (((p + d) | 0) + ((u & 8191) << 13)) | 0;
        (p = (((y + (u >>> 13)) | 0) + ($t >>> 26)) | 0),
          ($t &= 67108863),
          (d = Math.imul(K, re)),
          (u = Math.imul(K, ie)),
          (u = (u + Math.imul(V, re)) | 0),
          (y = Math.imul(V, ie)),
          (d = (d + Math.imul(z, ne)) | 0),
          (u = (u + Math.imul(z, fe)) | 0),
          (u = (u + Math.imul(H, ne)) | 0),
          (y = (y + Math.imul(H, fe)) | 0),
          (d = (d + Math.imul(F, ae)) | 0),
          (u = (u + Math.imul(F, oe)) | 0),
          (u = (u + Math.imul(j, ae)) | 0),
          (y = (y + Math.imul(j, oe)) | 0),
          (d = (d + Math.imul(O, se)) | 0),
          (u = (u + Math.imul(O, he)) | 0),
          (u = (u + Math.imul(P, se)) | 0),
          (y = (y + Math.imul(P, he)) | 0),
          (d = (d + Math.imul(s, ue)) | 0),
          (u = (u + Math.imul(s, ce)) | 0),
          (u = (u + Math.imul(w, ue)) | 0),
          (y = (y + Math.imul(w, ce)) | 0);
        var Zt = (((p + d) | 0) + ((u & 8191) << 13)) | 0;
        (p = (((y + (u >>> 13)) | 0) + (Zt >>> 26)) | 0),
          (Zt &= 67108863),
          (d = Math.imul(G, re)),
          (u = Math.imul(G, ie)),
          (u = (u + Math.imul(W, re)) | 0),
          (y = Math.imul(W, ie)),
          (d = (d + Math.imul(K, ne)) | 0),
          (u = (u + Math.imul(K, fe)) | 0),
          (u = (u + Math.imul(V, ne)) | 0),
          (y = (y + Math.imul(V, fe)) | 0),
          (d = (d + Math.imul(z, ae)) | 0),
          (u = (u + Math.imul(z, oe)) | 0),
          (u = (u + Math.imul(H, ae)) | 0),
          (y = (y + Math.imul(H, oe)) | 0),
          (d = (d + Math.imul(F, se)) | 0),
          (u = (u + Math.imul(F, he)) | 0),
          (u = (u + Math.imul(j, se)) | 0),
          (y = (y + Math.imul(j, he)) | 0),
          (d = (d + Math.imul(O, ue)) | 0),
          (u = (u + Math.imul(O, ce)) | 0),
          (u = (u + Math.imul(P, ue)) | 0),
          (y = (y + Math.imul(P, ce)) | 0),
          (d = (d + Math.imul(s, de)) | 0),
          (u = (u + Math.imul(s, le)) | 0),
          (u = (u + Math.imul(w, de)) | 0),
          (y = (y + Math.imul(w, le)) | 0);
        var Jt = (((p + d) | 0) + ((u & 8191) << 13)) | 0;
        (p = (((y + (u >>> 13)) | 0) + (Jt >>> 26)) | 0),
          (Jt &= 67108863),
          (d = Math.imul($, re)),
          (u = Math.imul($, ie)),
          (u = (u + Math.imul(Z, re)) | 0),
          (y = Math.imul(Z, ie)),
          (d = (d + Math.imul(G, ne)) | 0),
          (u = (u + Math.imul(G, fe)) | 0),
          (u = (u + Math.imul(W, ne)) | 0),
          (y = (y + Math.imul(W, fe)) | 0),
          (d = (d + Math.imul(K, ae)) | 0),
          (u = (u + Math.imul(K, oe)) | 0),
          (u = (u + Math.imul(V, ae)) | 0),
          (y = (y + Math.imul(V, oe)) | 0),
          (d = (d + Math.imul(z, se)) | 0),
          (u = (u + Math.imul(z, he)) | 0),
          (u = (u + Math.imul(H, se)) | 0),
          (y = (y + Math.imul(H, he)) | 0),
          (d = (d + Math.imul(F, ue)) | 0),
          (u = (u + Math.imul(F, ce)) | 0),
          (u = (u + Math.imul(j, ue)) | 0),
          (y = (y + Math.imul(j, ce)) | 0),
          (d = (d + Math.imul(O, de)) | 0),
          (u = (u + Math.imul(O, le)) | 0),
          (u = (u + Math.imul(P, de)) | 0),
          (y = (y + Math.imul(P, le)) | 0),
          (d = (d + Math.imul(s, pe)) | 0),
          (u = (u + Math.imul(s, ve)) | 0),
          (u = (u + Math.imul(w, pe)) | 0),
          (y = (y + Math.imul(w, ve)) | 0);
        var Xt = (((p + d) | 0) + ((u & 8191) << 13)) | 0;
        (p = (((y + (u >>> 13)) | 0) + (Xt >>> 26)) | 0),
          (Xt &= 67108863),
          (d = Math.imul(J, re)),
          (u = Math.imul(J, ie)),
          (u = (u + Math.imul(X, re)) | 0),
          (y = Math.imul(X, ie)),
          (d = (d + Math.imul($, ne)) | 0),
          (u = (u + Math.imul($, fe)) | 0),
          (u = (u + Math.imul(Z, ne)) | 0),
          (y = (y + Math.imul(Z, fe)) | 0),
          (d = (d + Math.imul(G, ae)) | 0),
          (u = (u + Math.imul(G, oe)) | 0),
          (u = (u + Math.imul(W, ae)) | 0),
          (y = (y + Math.imul(W, oe)) | 0),
          (d = (d + Math.imul(K, se)) | 0),
          (u = (u + Math.imul(K, he)) | 0),
          (u = (u + Math.imul(V, se)) | 0),
          (y = (y + Math.imul(V, he)) | 0),
          (d = (d + Math.imul(z, ue)) | 0),
          (u = (u + Math.imul(z, ce)) | 0),
          (u = (u + Math.imul(H, ue)) | 0),
          (y = (y + Math.imul(H, ce)) | 0),
          (d = (d + Math.imul(F, de)) | 0),
          (u = (u + Math.imul(F, le)) | 0),
          (u = (u + Math.imul(j, de)) | 0),
          (y = (y + Math.imul(j, le)) | 0),
          (d = (d + Math.imul(O, pe)) | 0),
          (u = (u + Math.imul(O, ve)) | 0),
          (u = (u + Math.imul(P, pe)) | 0),
          (y = (y + Math.imul(P, ve)) | 0),
          (d = (d + Math.imul(s, be)) | 0),
          (u = (u + Math.imul(s, ye)) | 0),
          (u = (u + Math.imul(w, be)) | 0),
          (y = (y + Math.imul(w, ye)) | 0);
        var Yt = (((p + d) | 0) + ((u & 8191) << 13)) | 0;
        (p = (((y + (u >>> 13)) | 0) + (Yt >>> 26)) | 0),
          (Yt &= 67108863),
          (d = Math.imul(Y, re)),
          (u = Math.imul(Y, ie)),
          (u = (u + Math.imul(Q, re)) | 0),
          (y = Math.imul(Q, ie)),
          (d = (d + Math.imul(J, ne)) | 0),
          (u = (u + Math.imul(J, fe)) | 0),
          (u = (u + Math.imul(X, ne)) | 0),
          (y = (y + Math.imul(X, fe)) | 0),
          (d = (d + Math.imul($, ae)) | 0),
          (u = (u + Math.imul($, oe)) | 0),
          (u = (u + Math.imul(Z, ae)) | 0),
          (y = (y + Math.imul(Z, oe)) | 0),
          (d = (d + Math.imul(G, se)) | 0),
          (u = (u + Math.imul(G, he)) | 0),
          (u = (u + Math.imul(W, se)) | 0),
          (y = (y + Math.imul(W, he)) | 0),
          (d = (d + Math.imul(K, ue)) | 0),
          (u = (u + Math.imul(K, ce)) | 0),
          (u = (u + Math.imul(V, ue)) | 0),
          (y = (y + Math.imul(V, ce)) | 0),
          (d = (d + Math.imul(z, de)) | 0),
          (u = (u + Math.imul(z, le)) | 0),
          (u = (u + Math.imul(H, de)) | 0),
          (y = (y + Math.imul(H, le)) | 0),
          (d = (d + Math.imul(F, pe)) | 0),
          (u = (u + Math.imul(F, ve)) | 0),
          (u = (u + Math.imul(j, pe)) | 0),
          (y = (y + Math.imul(j, ve)) | 0),
          (d = (d + Math.imul(O, be)) | 0),
          (u = (u + Math.imul(O, ye)) | 0),
          (u = (u + Math.imul(P, be)) | 0),
          (y = (y + Math.imul(P, ye)) | 0),
          (d = (d + Math.imul(s, me)) | 0),
          (u = (u + Math.imul(s, ge)) | 0),
          (u = (u + Math.imul(w, me)) | 0),
          (y = (y + Math.imul(w, ge)) | 0);
        var Qt = (((p + d) | 0) + ((u & 8191) << 13)) | 0;
        (p = (((y + (u >>> 13)) | 0) + (Qt >>> 26)) | 0),
          (Qt &= 67108863),
          (d = Math.imul(ee, re)),
          (u = Math.imul(ee, ie)),
          (u = (u + Math.imul(te, re)) | 0),
          (y = Math.imul(te, ie)),
          (d = (d + Math.imul(Y, ne)) | 0),
          (u = (u + Math.imul(Y, fe)) | 0),
          (u = (u + Math.imul(Q, ne)) | 0),
          (y = (y + Math.imul(Q, fe)) | 0),
          (d = (d + Math.imul(J, ae)) | 0),
          (u = (u + Math.imul(J, oe)) | 0),
          (u = (u + Math.imul(X, ae)) | 0),
          (y = (y + Math.imul(X, oe)) | 0),
          (d = (d + Math.imul($, se)) | 0),
          (u = (u + Math.imul($, he)) | 0),
          (u = (u + Math.imul(Z, se)) | 0),
          (y = (y + Math.imul(Z, he)) | 0),
          (d = (d + Math.imul(G, ue)) | 0),
          (u = (u + Math.imul(G, ce)) | 0),
          (u = (u + Math.imul(W, ue)) | 0),
          (y = (y + Math.imul(W, ce)) | 0),
          (d = (d + Math.imul(K, de)) | 0),
          (u = (u + Math.imul(K, le)) | 0),
          (u = (u + Math.imul(V, de)) | 0),
          (y = (y + Math.imul(V, le)) | 0),
          (d = (d + Math.imul(z, pe)) | 0),
          (u = (u + Math.imul(z, ve)) | 0),
          (u = (u + Math.imul(H, pe)) | 0),
          (y = (y + Math.imul(H, ve)) | 0),
          (d = (d + Math.imul(F, be)) | 0),
          (u = (u + Math.imul(F, ye)) | 0),
          (u = (u + Math.imul(j, be)) | 0),
          (y = (y + Math.imul(j, ye)) | 0),
          (d = (d + Math.imul(O, me)) | 0),
          (u = (u + Math.imul(O, ge)) | 0),
          (u = (u + Math.imul(P, me)) | 0),
          (y = (y + Math.imul(P, ge)) | 0),
          (d = (d + Math.imul(s, we)) | 0),
          (u = (u + Math.imul(s, _e)) | 0),
          (u = (u + Math.imul(w, we)) | 0),
          (y = (y + Math.imul(w, _e)) | 0);
        var er = (((p + d) | 0) + ((u & 8191) << 13)) | 0;
        (p = (((y + (u >>> 13)) | 0) + (er >>> 26)) | 0),
          (er &= 67108863),
          (d = Math.imul(ee, ne)),
          (u = Math.imul(ee, fe)),
          (u = (u + Math.imul(te, ne)) | 0),
          (y = Math.imul(te, fe)),
          (d = (d + Math.imul(Y, ae)) | 0),
          (u = (u + Math.imul(Y, oe)) | 0),
          (u = (u + Math.imul(Q, ae)) | 0),
          (y = (y + Math.imul(Q, oe)) | 0),
          (d = (d + Math.imul(J, se)) | 0),
          (u = (u + Math.imul(J, he)) | 0),
          (u = (u + Math.imul(X, se)) | 0),
          (y = (y + Math.imul(X, he)) | 0),
          (d = (d + Math.imul($, ue)) | 0),
          (u = (u + Math.imul($, ce)) | 0),
          (u = (u + Math.imul(Z, ue)) | 0),
          (y = (y + Math.imul(Z, ce)) | 0),
          (d = (d + Math.imul(G, de)) | 0),
          (u = (u + Math.imul(G, le)) | 0),
          (u = (u + Math.imul(W, de)) | 0),
          (y = (y + Math.imul(W, le)) | 0),
          (d = (d + Math.imul(K, pe)) | 0),
          (u = (u + Math.imul(K, ve)) | 0),
          (u = (u + Math.imul(V, pe)) | 0),
          (y = (y + Math.imul(V, ve)) | 0),
          (d = (d + Math.imul(z, be)) | 0),
          (u = (u + Math.imul(z, ye)) | 0),
          (u = (u + Math.imul(H, be)) | 0),
          (y = (y + Math.imul(H, ye)) | 0),
          (d = (d + Math.imul(F, me)) | 0),
          (u = (u + Math.imul(F, ge)) | 0),
          (u = (u + Math.imul(j, me)) | 0),
          (y = (y + Math.imul(j, ge)) | 0),
          (d = (d + Math.imul(O, we)) | 0),
          (u = (u + Math.imul(O, _e)) | 0),
          (u = (u + Math.imul(P, we)) | 0),
          (y = (y + Math.imul(P, _e)) | 0);
        var tr = (((p + d) | 0) + ((u & 8191) << 13)) | 0;
        (p = (((y + (u >>> 13)) | 0) + (tr >>> 26)) | 0),
          (tr &= 67108863),
          (d = Math.imul(ee, ae)),
          (u = Math.imul(ee, oe)),
          (u = (u + Math.imul(te, ae)) | 0),
          (y = Math.imul(te, oe)),
          (d = (d + Math.imul(Y, se)) | 0),
          (u = (u + Math.imul(Y, he)) | 0),
          (u = (u + Math.imul(Q, se)) | 0),
          (y = (y + Math.imul(Q, he)) | 0),
          (d = (d + Math.imul(J, ue)) | 0),
          (u = (u + Math.imul(J, ce)) | 0),
          (u = (u + Math.imul(X, ue)) | 0),
          (y = (y + Math.imul(X, ce)) | 0),
          (d = (d + Math.imul($, de)) | 0),
          (u = (u + Math.imul($, le)) | 0),
          (u = (u + Math.imul(Z, de)) | 0),
          (y = (y + Math.imul(Z, le)) | 0),
          (d = (d + Math.imul(G, pe)) | 0),
          (u = (u + Math.imul(G, ve)) | 0),
          (u = (u + Math.imul(W, pe)) | 0),
          (y = (y + Math.imul(W, ve)) | 0),
          (d = (d + Math.imul(K, be)) | 0),
          (u = (u + Math.imul(K, ye)) | 0),
          (u = (u + Math.imul(V, be)) | 0),
          (y = (y + Math.imul(V, ye)) | 0),
          (d = (d + Math.imul(z, me)) | 0),
          (u = (u + Math.imul(z, ge)) | 0),
          (u = (u + Math.imul(H, me)) | 0),
          (y = (y + Math.imul(H, ge)) | 0),
          (d = (d + Math.imul(F, we)) | 0),
          (u = (u + Math.imul(F, _e)) | 0),
          (u = (u + Math.imul(j, we)) | 0),
          (y = (y + Math.imul(j, _e)) | 0);
        var rr = (((p + d) | 0) + ((u & 8191) << 13)) | 0;
        (p = (((y + (u >>> 13)) | 0) + (rr >>> 26)) | 0),
          (rr &= 67108863),
          (d = Math.imul(ee, se)),
          (u = Math.imul(ee, he)),
          (u = (u + Math.imul(te, se)) | 0),
          (y = Math.imul(te, he)),
          (d = (d + Math.imul(Y, ue)) | 0),
          (u = (u + Math.imul(Y, ce)) | 0),
          (u = (u + Math.imul(Q, ue)) | 0),
          (y = (y + Math.imul(Q, ce)) | 0),
          (d = (d + Math.imul(J, de)) | 0),
          (u = (u + Math.imul(J, le)) | 0),
          (u = (u + Math.imul(X, de)) | 0),
          (y = (y + Math.imul(X, le)) | 0),
          (d = (d + Math.imul($, pe)) | 0),
          (u = (u + Math.imul($, ve)) | 0),
          (u = (u + Math.imul(Z, pe)) | 0),
          (y = (y + Math.imul(Z, ve)) | 0),
          (d = (d + Math.imul(G, be)) | 0),
          (u = (u + Math.imul(G, ye)) | 0),
          (u = (u + Math.imul(W, be)) | 0),
          (y = (y + Math.imul(W, ye)) | 0),
          (d = (d + Math.imul(K, me)) | 0),
          (u = (u + Math.imul(K, ge)) | 0),
          (u = (u + Math.imul(V, me)) | 0),
          (y = (y + Math.imul(V, ge)) | 0),
          (d = (d + Math.imul(z, we)) | 0),
          (u = (u + Math.imul(z, _e)) | 0),
          (u = (u + Math.imul(H, we)) | 0),
          (y = (y + Math.imul(H, _e)) | 0);
        var ir = (((p + d) | 0) + ((u & 8191) << 13)) | 0;
        (p = (((y + (u >>> 13)) | 0) + (ir >>> 26)) | 0),
          (ir &= 67108863),
          (d = Math.imul(ee, ue)),
          (u = Math.imul(ee, ce)),
          (u = (u + Math.imul(te, ue)) | 0),
          (y = Math.imul(te, ce)),
          (d = (d + Math.imul(Y, de)) | 0),
          (u = (u + Math.imul(Y, le)) | 0),
          (u = (u + Math.imul(Q, de)) | 0),
          (y = (y + Math.imul(Q, le)) | 0),
          (d = (d + Math.imul(J, pe)) | 0),
          (u = (u + Math.imul(J, ve)) | 0),
          (u = (u + Math.imul(X, pe)) | 0),
          (y = (y + Math.imul(X, ve)) | 0),
          (d = (d + Math.imul($, be)) | 0),
          (u = (u + Math.imul($, ye)) | 0),
          (u = (u + Math.imul(Z, be)) | 0),
          (y = (y + Math.imul(Z, ye)) | 0),
          (d = (d + Math.imul(G, me)) | 0),
          (u = (u + Math.imul(G, ge)) | 0),
          (u = (u + Math.imul(W, me)) | 0),
          (y = (y + Math.imul(W, ge)) | 0),
          (d = (d + Math.imul(K, we)) | 0),
          (u = (u + Math.imul(K, _e)) | 0),
          (u = (u + Math.imul(V, we)) | 0),
          (y = (y + Math.imul(V, _e)) | 0);
        var nr = (((p + d) | 0) + ((u & 8191) << 13)) | 0;
        (p = (((y + (u >>> 13)) | 0) + (nr >>> 26)) | 0),
          (nr &= 67108863),
          (d = Math.imul(ee, de)),
          (u = Math.imul(ee, le)),
          (u = (u + Math.imul(te, de)) | 0),
          (y = Math.imul(te, le)),
          (d = (d + Math.imul(Y, pe)) | 0),
          (u = (u + Math.imul(Y, ve)) | 0),
          (u = (u + Math.imul(Q, pe)) | 0),
          (y = (y + Math.imul(Q, ve)) | 0),
          (d = (d + Math.imul(J, be)) | 0),
          (u = (u + Math.imul(J, ye)) | 0),
          (u = (u + Math.imul(X, be)) | 0),
          (y = (y + Math.imul(X, ye)) | 0),
          (d = (d + Math.imul($, me)) | 0),
          (u = (u + Math.imul($, ge)) | 0),
          (u = (u + Math.imul(Z, me)) | 0),
          (y = (y + Math.imul(Z, ge)) | 0),
          (d = (d + Math.imul(G, we)) | 0),
          (u = (u + Math.imul(G, _e)) | 0),
          (u = (u + Math.imul(W, we)) | 0),
          (y = (y + Math.imul(W, _e)) | 0);
        var fr = (((p + d) | 0) + ((u & 8191) << 13)) | 0;
        (p = (((y + (u >>> 13)) | 0) + (fr >>> 26)) | 0),
          (fr &= 67108863),
          (d = Math.imul(ee, pe)),
          (u = Math.imul(ee, ve)),
          (u = (u + Math.imul(te, pe)) | 0),
          (y = Math.imul(te, ve)),
          (d = (d + Math.imul(Y, be)) | 0),
          (u = (u + Math.imul(Y, ye)) | 0),
          (u = (u + Math.imul(Q, be)) | 0),
          (y = (y + Math.imul(Q, ye)) | 0),
          (d = (d + Math.imul(J, me)) | 0),
          (u = (u + Math.imul(J, ge)) | 0),
          (u = (u + Math.imul(X, me)) | 0),
          (y = (y + Math.imul(X, ge)) | 0),
          (d = (d + Math.imul($, we)) | 0),
          (u = (u + Math.imul($, _e)) | 0),
          (u = (u + Math.imul(Z, we)) | 0),
          (y = (y + Math.imul(Z, _e)) | 0);
        var ar = (((p + d) | 0) + ((u & 8191) << 13)) | 0;
        (p = (((y + (u >>> 13)) | 0) + (ar >>> 26)) | 0),
          (ar &= 67108863),
          (d = Math.imul(ee, be)),
          (u = Math.imul(ee, ye)),
          (u = (u + Math.imul(te, be)) | 0),
          (y = Math.imul(te, ye)),
          (d = (d + Math.imul(Y, me)) | 0),
          (u = (u + Math.imul(Y, ge)) | 0),
          (u = (u + Math.imul(Q, me)) | 0),
          (y = (y + Math.imul(Q, ge)) | 0),
          (d = (d + Math.imul(J, we)) | 0),
          (u = (u + Math.imul(J, _e)) | 0),
          (u = (u + Math.imul(X, we)) | 0),
          (y = (y + Math.imul(X, _e)) | 0);
        var or = (((p + d) | 0) + ((u & 8191) << 13)) | 0;
        (p = (((y + (u >>> 13)) | 0) + (or >>> 26)) | 0),
          (or &= 67108863),
          (d = Math.imul(ee, me)),
          (u = Math.imul(ee, ge)),
          (u = (u + Math.imul(te, me)) | 0),
          (y = Math.imul(te, ge)),
          (d = (d + Math.imul(Y, we)) | 0),
          (u = (u + Math.imul(Y, _e)) | 0),
          (u = (u + Math.imul(Q, we)) | 0),
          (y = (y + Math.imul(Q, _e)) | 0);
        var sr = (((p + d) | 0) + ((u & 8191) << 13)) | 0;
        (p = (((y + (u >>> 13)) | 0) + (sr >>> 26)) | 0),
          (sr &= 67108863),
          (d = Math.imul(ee, we)),
          (u = Math.imul(ee, _e)),
          (u = (u + Math.imul(te, we)) | 0),
          (y = Math.imul(te, _e));
        var hr = (((p + d) | 0) + ((u & 8191) << 13)) | 0;
        return (
          (p = (((y + (u >>> 13)) | 0) + (hr >>> 26)) | 0),
          (hr &= 67108863),
          (c[0] = Lr),
          (c[1] = Ae),
          (c[2] = Be),
          (c[3] = $t),
          (c[4] = Zt),
          (c[5] = Jt),
          (c[6] = Xt),
          (c[7] = Yt),
          (c[8] = Qt),
          (c[9] = er),
          (c[10] = tr),
          (c[11] = rr),
          (c[12] = ir),
          (c[13] = nr),
          (c[14] = fr),
          (c[15] = ar),
          (c[16] = or),
          (c[17] = sr),
          (c[18] = hr),
          p !== 0 && ((c[19] = p), l.length++),
          l
        );
      };
      Math.imul || (k = q);
      function L(B, b, _) {
        (_.negative = b.negative ^ B.negative),
          (_.length = B.length + b.length);
        for (var l = 0, f = 0, o = 0; o < _.length - 1; o++) {
          var c = f;
          f = 0;
          for (
            var p = l & 67108863,
              d = Math.min(o, b.length - 1),
              u = Math.max(0, o - B.length + 1);
            u <= d;
            u++
          ) {
            var y = o - u,
              m = B.words[y] | 0,
              s = b.words[u] | 0,
              w = m * s,
              T = w & 67108863;
            (c = (c + ((w / 67108864) | 0)) | 0),
              (T = (T + p) | 0),
              (p = T & 67108863),
              (c = (c + (T >>> 26)) | 0),
              (f += c >>> 26),
              (c &= 67108863);
          }
          (_.words[o] = p), (l = c), (c = f);
        }
        return l !== 0 ? (_.words[o] = l) : _.length--, _.strip();
      }
      function xe(B, b, _) {
        var l = new U();
        return l.mulp(B, b, _);
      }
      i.prototype.mulTo = function (b, _) {
        var l,
          f = this.length + b.length;
        return (
          this.length === 10 && b.length === 10
            ? (l = k(this, b, _))
            : f < 63
              ? (l = q(this, b, _))
              : f < 1024
                ? (l = L(this, b, _))
                : (l = xe(this, b, _)),
          l
        );
      };
      function U(B, b) {
        (this.x = B), (this.y = b);
      }
      (U.prototype.makeRBT = function (b) {
        for (
          var _ = new Array(b), l = i.prototype._countBits(b) - 1, f = 0;
          f < b;
          f++
        )
          _[f] = this.revBin(f, l, b);
        return _;
      }),
        (U.prototype.revBin = function (b, _, l) {
          if (b === 0 || b === l - 1) return b;
          for (var f = 0, o = 0; o < _; o++)
            (f |= (b & 1) << (_ - o - 1)), (b >>= 1);
          return f;
        }),
        (U.prototype.permute = function (b, _, l, f, o, c) {
          for (var p = 0; p < c; p++) (f[p] = _[b[p]]), (o[p] = l[b[p]]);
        }),
        (U.prototype.transform = function (b, _, l, f, o, c) {
          this.permute(c, b, _, l, f, o);
          for (var p = 1; p < o; p <<= 1)
            for (
              var d = p << 1,
                u = Math.cos((2 * Math.PI) / d),
                y = Math.sin((2 * Math.PI) / d),
                m = 0;
              m < o;
              m += d
            )
              for (var s = u, w = y, T = 0; T < p; T++) {
                var O = l[m + T],
                  P = f[m + T],
                  N = l[m + T + p],
                  F = f[m + T + p],
                  j = s * N - w * F;
                (F = s * F + w * N),
                  (N = j),
                  (l[m + T] = O + N),
                  (f[m + T] = P + F),
                  (l[m + T + p] = O - N),
                  (f[m + T + p] = P - F),
                  T !== d &&
                    ((j = u * s - y * w), (w = u * w + y * s), (s = j));
              }
        }),
        (U.prototype.guessLen13b = function (b, _) {
          var l = Math.max(_, b) | 1,
            f = l & 1,
            o = 0;
          for (l = (l / 2) | 0; l; l = l >>> 1) o++;
          return 1 << (o + 1 + f);
        }),
        (U.prototype.conjugate = function (b, _, l) {
          if (!(l <= 1))
            for (var f = 0; f < l / 2; f++) {
              var o = b[f];
              (b[f] = b[l - f - 1]),
                (b[l - f - 1] = o),
                (o = _[f]),
                (_[f] = -_[l - f - 1]),
                (_[l - f - 1] = -o);
            }
        }),
        (U.prototype.normalize13b = function (b, _) {
          for (var l = 0, f = 0; f < _ / 2; f++) {
            var o =
              Math.round(b[2 * f + 1] / _) * 8192 +
              Math.round(b[2 * f] / _) +
              l;
            (b[f] = o & 67108863),
              o < 67108864 ? (l = 0) : (l = (o / 67108864) | 0);
          }
          return b;
        }),
        (U.prototype.convert13b = function (b, _, l, f) {
          for (var o = 0, c = 0; c < _; c++)
            (o = o + (b[c] | 0)),
              (l[2 * c] = o & 8191),
              (o = o >>> 13),
              (l[2 * c + 1] = o & 8191),
              (o = o >>> 13);
          for (c = 2 * _; c < f; ++c) l[c] = 0;
          r(o === 0), r((o & -8192) === 0);
        }),
        (U.prototype.stub = function (b) {
          for (var _ = new Array(b), l = 0; l < b; l++) _[l] = 0;
          return _;
        }),
        (U.prototype.mulp = function (b, _, l) {
          var f = 2 * this.guessLen13b(b.length, _.length),
            o = this.makeRBT(f),
            c = this.stub(f),
            p = new Array(f),
            d = new Array(f),
            u = new Array(f),
            y = new Array(f),
            m = new Array(f),
            s = new Array(f),
            w = l.words;
          (w.length = f),
            this.convert13b(b.words, b.length, p, f),
            this.convert13b(_.words, _.length, y, f),
            this.transform(p, c, d, u, f, o),
            this.transform(y, c, m, s, f, o);
          for (var T = 0; T < f; T++) {
            var O = d[T] * m[T] - u[T] * s[T];
            (u[T] = d[T] * s[T] + u[T] * m[T]), (d[T] = O);
          }
          return (
            this.conjugate(d, u, f),
            this.transform(d, u, w, c, f, o),
            this.conjugate(w, c, f),
            this.normalize13b(w, f),
            (l.negative = b.negative ^ _.negative),
            (l.length = b.length + _.length),
            l.strip()
          );
        }),
        (i.prototype.mul = function (b) {
          var _ = new i(null);
          return (
            (_.words = new Array(this.length + b.length)), this.mulTo(b, _)
          );
        }),
        (i.prototype.mulf = function (b) {
          var _ = new i(null);
          return (_.words = new Array(this.length + b.length)), xe(this, b, _);
        }),
        (i.prototype.imul = function (b) {
          return this.clone().mulTo(b, this);
        }),
        (i.prototype.imuln = function (b) {
          r(typeof b == 'number'), r(b < 67108864);
          for (var _ = 0, l = 0; l < this.length; l++) {
            var f = (this.words[l] | 0) * b,
              o = (f & 67108863) + (_ & 67108863);
            (_ >>= 26),
              (_ += (f / 67108864) | 0),
              (_ += o >>> 26),
              (this.words[l] = o & 67108863);
          }
          return _ !== 0 && ((this.words[l] = _), this.length++), this;
        }),
        (i.prototype.muln = function (b) {
          return this.clone().imuln(b);
        }),
        (i.prototype.sqr = function () {
          return this.mul(this);
        }),
        (i.prototype.isqr = function () {
          return this.imul(this.clone());
        }),
        (i.prototype.pow = function (b) {
          var _ = I(b);
          if (_.length === 0) return new i(1);
          for (
            var l = this, f = 0;
            f < _.length && _[f] === 0;
            f++, l = l.sqr()
          );
          if (++f < _.length)
            for (var o = l.sqr(); f < _.length; f++, o = o.sqr())
              _[f] !== 0 && (l = l.mul(o));
          return l;
        }),
        (i.prototype.iushln = function (b) {
          r(typeof b == 'number' && b >= 0);
          var _ = b % 26,
            l = (b - _) / 26,
            f = (67108863 >>> (26 - _)) << (26 - _),
            o;
          if (_ !== 0) {
            var c = 0;
            for (o = 0; o < this.length; o++) {
              var p = this.words[o] & f,
                d = ((this.words[o] | 0) - p) << _;
              (this.words[o] = d | c), (c = p >>> (26 - _));
            }
            c && ((this.words[o] = c), this.length++);
          }
          if (l !== 0) {
            for (o = this.length - 1; o >= 0; o--)
              this.words[o + l] = this.words[o];
            for (o = 0; o < l; o++) this.words[o] = 0;
            this.length += l;
          }
          return this.strip();
        }),
        (i.prototype.ishln = function (b) {
          return r(this.negative === 0), this.iushln(b);
        }),
        (i.prototype.iushrn = function (b, _, l) {
          r(typeof b == 'number' && b >= 0);
          var f;
          _ ? (f = (_ - (_ % 26)) / 26) : (f = 0);
          var o = b % 26,
            c = Math.min((b - o) / 26, this.length),
            p = 67108863 ^ ((67108863 >>> o) << o),
            d = l;
          if (((f -= c), (f = Math.max(0, f)), d)) {
            for (var u = 0; u < c; u++) d.words[u] = this.words[u];
            d.length = c;
          }
          if (c !== 0)
            if (this.length > c)
              for (this.length -= c, u = 0; u < this.length; u++)
                this.words[u] = this.words[u + c];
            else (this.words[0] = 0), (this.length = 1);
          var y = 0;
          for (u = this.length - 1; u >= 0 && (y !== 0 || u >= f); u--) {
            var m = this.words[u] | 0;
            (this.words[u] = (y << (26 - o)) | (m >>> o)), (y = m & p);
          }
          return (
            d && y !== 0 && (d.words[d.length++] = y),
            this.length === 0 && ((this.words[0] = 0), (this.length = 1)),
            this.strip()
          );
        }),
        (i.prototype.ishrn = function (b, _, l) {
          return r(this.negative === 0), this.iushrn(b, _, l);
        }),
        (i.prototype.shln = function (b) {
          return this.clone().ishln(b);
        }),
        (i.prototype.ushln = function (b) {
          return this.clone().iushln(b);
        }),
        (i.prototype.shrn = function (b) {
          return this.clone().ishrn(b);
        }),
        (i.prototype.ushrn = function (b) {
          return this.clone().iushrn(b);
        }),
        (i.prototype.testn = function (b) {
          r(typeof b == 'number' && b >= 0);
          var _ = b % 26,
            l = (b - _) / 26,
            f = 1 << _;
          if (this.length <= l) return !1;
          var o = this.words[l];
          return !!(o & f);
        }),
        (i.prototype.imaskn = function (b) {
          r(typeof b == 'number' && b >= 0);
          var _ = b % 26,
            l = (b - _) / 26;
          if (
            (r(this.negative === 0, 'imaskn works only with positive numbers'),
            this.length <= l)
          )
            return this;
          if (
            (_ !== 0 && l++, (this.length = Math.min(l, this.length)), _ !== 0)
          ) {
            var f = 67108863 ^ ((67108863 >>> _) << _);
            this.words[this.length - 1] &= f;
          }
          return this.strip();
        }),
        (i.prototype.maskn = function (b) {
          return this.clone().imaskn(b);
        }),
        (i.prototype.iaddn = function (b) {
          return (
            r(typeof b == 'number'),
            r(b < 67108864),
            b < 0
              ? this.isubn(-b)
              : this.negative !== 0
                ? this.length === 1 && (this.words[0] | 0) < b
                  ? ((this.words[0] = b - (this.words[0] | 0)),
                    (this.negative = 0),
                    this)
                  : ((this.negative = 0),
                    this.isubn(b),
                    (this.negative = 1),
                    this)
                : this._iaddn(b)
          );
        }),
        (i.prototype._iaddn = function (b) {
          this.words[0] += b;
          for (var _ = 0; _ < this.length && this.words[_] >= 67108864; _++)
            (this.words[_] -= 67108864),
              _ === this.length - 1
                ? (this.words[_ + 1] = 1)
                : this.words[_ + 1]++;
          return (this.length = Math.max(this.length, _ + 1)), this;
        }),
        (i.prototype.isubn = function (b) {
          if ((r(typeof b == 'number'), r(b < 67108864), b < 0))
            return this.iaddn(-b);
          if (this.negative !== 0)
            return (
              (this.negative = 0), this.iaddn(b), (this.negative = 1), this
            );
          if (((this.words[0] -= b), this.length === 1 && this.words[0] < 0))
            (this.words[0] = -this.words[0]), (this.negative = 1);
          else
            for (var _ = 0; _ < this.length && this.words[_] < 0; _++)
              (this.words[_] += 67108864), (this.words[_ + 1] -= 1);
          return this.strip();
        }),
        (i.prototype.addn = function (b) {
          return this.clone().iaddn(b);
        }),
        (i.prototype.subn = function (b) {
          return this.clone().isubn(b);
        }),
        (i.prototype.iabs = function () {
          return (this.negative = 0), this;
        }),
        (i.prototype.abs = function () {
          return this.clone().iabs();
        }),
        (i.prototype._ishlnsubmul = function (b, _, l) {
          var f = b.length + l,
            o;
          this._expand(f);
          var c,
            p = 0;
          for (o = 0; o < b.length; o++) {
            c = (this.words[o + l] | 0) + p;
            var d = (b.words[o] | 0) * _;
            (c -= d & 67108863),
              (p = (c >> 26) - ((d / 67108864) | 0)),
              (this.words[o + l] = c & 67108863);
          }
          for (; o < this.length - l; o++)
            (c = (this.words[o + l] | 0) + p),
              (p = c >> 26),
              (this.words[o + l] = c & 67108863);
          if (p === 0) return this.strip();
          for (r(p === -1), p = 0, o = 0; o < this.length; o++)
            (c = -(this.words[o] | 0) + p),
              (p = c >> 26),
              (this.words[o] = c & 67108863);
          return (this.negative = 1), this.strip();
        }),
        (i.prototype._wordDiv = function (b, _) {
          var l = this.length - b.length,
            f = this.clone(),
            o = b,
            c = o.words[o.length - 1] | 0,
            p = this._countBits(c);
          (l = 26 - p),
            l !== 0 &&
              ((o = o.ushln(l)), f.iushln(l), (c = o.words[o.length - 1] | 0));
          var d = f.length - o.length,
            u;
          if (_ !== 'mod') {
            (u = new i(null)),
              (u.length = d + 1),
              (u.words = new Array(u.length));
            for (var y = 0; y < u.length; y++) u.words[y] = 0;
          }
          var m = f.clone()._ishlnsubmul(o, 1, d);
          m.negative === 0 && ((f = m), u && (u.words[d] = 1));
          for (var s = d - 1; s >= 0; s--) {
            var w =
              (f.words[o.length + s] | 0) * 67108864 +
              (f.words[o.length + s - 1] | 0);
            for (
              w = Math.min((w / c) | 0, 67108863), f._ishlnsubmul(o, w, s);
              f.negative !== 0;

            )
              w--,
                (f.negative = 0),
                f._ishlnsubmul(o, 1, s),
                f.isZero() || (f.negative ^= 1);
            u && (u.words[s] = w);
          }
          return (
            u && u.strip(),
            f.strip(),
            _ !== 'div' && l !== 0 && f.iushrn(l),
            { div: u || null, mod: f }
          );
        }),
        (i.prototype.divmod = function (b, _, l) {
          if ((r(!b.isZero()), this.isZero()))
            return { div: new i(0), mod: new i(0) };
          var f, o, c;
          return this.negative !== 0 && b.negative === 0
            ? ((c = this.neg().divmod(b, _)),
              _ !== 'mod' && (f = c.div.neg()),
              _ !== 'div' &&
                ((o = c.mod.neg()), l && o.negative !== 0 && o.iadd(b)),
              { div: f, mod: o })
            : this.negative === 0 && b.negative !== 0
              ? ((c = this.divmod(b.neg(), _)),
                _ !== 'mod' && (f = c.div.neg()),
                { div: f, mod: c.mod })
              : (this.negative & b.negative) !== 0
                ? ((c = this.neg().divmod(b.neg(), _)),
                  _ !== 'div' &&
                    ((o = c.mod.neg()), l && o.negative !== 0 && o.isub(b)),
                  { div: c.div, mod: o })
                : b.length > this.length || this.cmp(b) < 0
                  ? { div: new i(0), mod: this }
                  : b.length === 1
                    ? _ === 'div'
                      ? { div: this.divn(b.words[0]), mod: null }
                      : _ === 'mod'
                        ? { div: null, mod: new i(this.modn(b.words[0])) }
                        : {
                            div: this.divn(b.words[0]),
                            mod: new i(this.modn(b.words[0])),
                          }
                    : this._wordDiv(b, _);
        }),
        (i.prototype.div = function (b) {
          return this.divmod(b, 'div', !1).div;
        }),
        (i.prototype.mod = function (b) {
          return this.divmod(b, 'mod', !1).mod;
        }),
        (i.prototype.umod = function (b) {
          return this.divmod(b, 'mod', !0).mod;
        }),
        (i.prototype.divRound = function (b) {
          var _ = this.divmod(b);
          if (_.mod.isZero()) return _.div;
          var l = _.div.negative !== 0 ? _.mod.isub(b) : _.mod,
            f = b.ushrn(1),
            o = b.andln(1),
            c = l.cmp(f);
          return c < 0 || (o === 1 && c === 0)
            ? _.div
            : _.div.negative !== 0
              ? _.div.isubn(1)
              : _.div.iaddn(1);
        }),
        (i.prototype.modn = function (b) {
          r(b <= 67108863);
          for (var _ = (1 << 26) % b, l = 0, f = this.length - 1; f >= 0; f--)
            l = (_ * l + (this.words[f] | 0)) % b;
          return l;
        }),
        (i.prototype.idivn = function (b) {
          r(b <= 67108863);
          for (var _ = 0, l = this.length - 1; l >= 0; l--) {
            var f = (this.words[l] | 0) + _ * 67108864;
            (this.words[l] = (f / b) | 0), (_ = f % b);
          }
          return this.strip();
        }),
        (i.prototype.divn = function (b) {
          return this.clone().idivn(b);
        }),
        (i.prototype.egcd = function (b) {
          r(b.negative === 0), r(!b.isZero());
          var _ = this,
            l = b.clone();
          _.negative !== 0 ? (_ = _.umod(b)) : (_ = _.clone());
          for (
            var f = new i(1), o = new i(0), c = new i(0), p = new i(1), d = 0;
            _.isEven() && l.isEven();

          )
            _.iushrn(1), l.iushrn(1), ++d;
          for (var u = l.clone(), y = _.clone(); !_.isZero(); ) {
            for (
              var m = 0, s = 1;
              (_.words[0] & s) === 0 && m < 26;
              ++m, s <<= 1
            );
            if (m > 0)
              for (_.iushrn(m); m-- > 0; )
                (f.isOdd() || o.isOdd()) && (f.iadd(u), o.isub(y)),
                  f.iushrn(1),
                  o.iushrn(1);
            for (
              var w = 0, T = 1;
              (l.words[0] & T) === 0 && w < 26;
              ++w, T <<= 1
            );
            if (w > 0)
              for (l.iushrn(w); w-- > 0; )
                (c.isOdd() || p.isOdd()) && (c.iadd(u), p.isub(y)),
                  c.iushrn(1),
                  p.iushrn(1);
            _.cmp(l) >= 0
              ? (_.isub(l), f.isub(c), o.isub(p))
              : (l.isub(_), c.isub(f), p.isub(o));
          }
          return { a: c, b: p, gcd: l.iushln(d) };
        }),
        (i.prototype._invmp = function (b) {
          r(b.negative === 0), r(!b.isZero());
          var _ = this,
            l = b.clone();
          _.negative !== 0 ? (_ = _.umod(b)) : (_ = _.clone());
          for (
            var f = new i(1), o = new i(0), c = l.clone();
            _.cmpn(1) > 0 && l.cmpn(1) > 0;

          ) {
            for (
              var p = 0, d = 1;
              (_.words[0] & d) === 0 && p < 26;
              ++p, d <<= 1
            );
            if (p > 0)
              for (_.iushrn(p); p-- > 0; ) f.isOdd() && f.iadd(c), f.iushrn(1);
            for (
              var u = 0, y = 1;
              (l.words[0] & y) === 0 && u < 26;
              ++u, y <<= 1
            );
            if (u > 0)
              for (l.iushrn(u); u-- > 0; ) o.isOdd() && o.iadd(c), o.iushrn(1);
            _.cmp(l) >= 0 ? (_.isub(l), f.isub(o)) : (l.isub(_), o.isub(f));
          }
          var m;
          return (
            _.cmpn(1) === 0 ? (m = f) : (m = o), m.cmpn(0) < 0 && m.iadd(b), m
          );
        }),
        (i.prototype.gcd = function (b) {
          if (this.isZero()) return b.abs();
          if (b.isZero()) return this.abs();
          var _ = this.clone(),
            l = b.clone();
          (_.negative = 0), (l.negative = 0);
          for (var f = 0; _.isEven() && l.isEven(); f++)
            _.iushrn(1), l.iushrn(1);
          do {
            for (; _.isEven(); ) _.iushrn(1);
            for (; l.isEven(); ) l.iushrn(1);
            var o = _.cmp(l);
            if (o < 0) {
              var c = _;
              (_ = l), (l = c);
            } else if (o === 0 || l.cmpn(1) === 0) break;
            _.isub(l);
          } while (!0);
          return l.iushln(f);
        }),
        (i.prototype.invm = function (b) {
          return this.egcd(b).a.umod(b);
        }),
        (i.prototype.isEven = function () {
          return (this.words[0] & 1) === 0;
        }),
        (i.prototype.isOdd = function () {
          return (this.words[0] & 1) === 1;
        }),
        (i.prototype.andln = function (b) {
          return this.words[0] & b;
        }),
        (i.prototype.bincn = function (b) {
          r(typeof b == 'number');
          var _ = b % 26,
            l = (b - _) / 26,
            f = 1 << _;
          if (this.length <= l)
            return this._expand(l + 1), (this.words[l] |= f), this;
          for (var o = f, c = l; o !== 0 && c < this.length; c++) {
            var p = this.words[c] | 0;
            (p += o), (o = p >>> 26), (p &= 67108863), (this.words[c] = p);
          }
          return o !== 0 && ((this.words[c] = o), this.length++), this;
        }),
        (i.prototype.isZero = function () {
          return this.length === 1 && this.words[0] === 0;
        }),
        (i.prototype.cmpn = function (b) {
          var _ = b < 0;
          if (this.negative !== 0 && !_) return -1;
          if (this.negative === 0 && _) return 1;
          this.strip();
          var l;
          if (this.length > 1) l = 1;
          else {
            _ && (b = -b), r(b <= 67108863, 'Number is too big');
            var f = this.words[0] | 0;
            l = f === b ? 0 : f < b ? -1 : 1;
          }
          return this.negative !== 0 ? -l | 0 : l;
        }),
        (i.prototype.cmp = function (b) {
          if (this.negative !== 0 && b.negative === 0) return -1;
          if (this.negative === 0 && b.negative !== 0) return 1;
          var _ = this.ucmp(b);
          return this.negative !== 0 ? -_ | 0 : _;
        }),
        (i.prototype.ucmp = function (b) {
          if (this.length > b.length) return 1;
          if (this.length < b.length) return -1;
          for (var _ = 0, l = this.length - 1; l >= 0; l--) {
            var f = this.words[l] | 0,
              o = b.words[l] | 0;
            if (f !== o) {
              f < o ? (_ = -1) : f > o && (_ = 1);
              break;
            }
          }
          return _;
        }),
        (i.prototype.gtn = function (b) {
          return this.cmpn(b) === 1;
        }),
        (i.prototype.gt = function (b) {
          return this.cmp(b) === 1;
        }),
        (i.prototype.gten = function (b) {
          return this.cmpn(b) >= 0;
        }),
        (i.prototype.gte = function (b) {
          return this.cmp(b) >= 0;
        }),
        (i.prototype.ltn = function (b) {
          return this.cmpn(b) === -1;
        }),
        (i.prototype.lt = function (b) {
          return this.cmp(b) === -1;
        }),
        (i.prototype.lten = function (b) {
          return this.cmpn(b) <= 0;
        }),
        (i.prototype.lte = function (b) {
          return this.cmp(b) <= 0;
        }),
        (i.prototype.eqn = function (b) {
          return this.cmpn(b) === 0;
        }),
        (i.prototype.eq = function (b) {
          return this.cmp(b) === 0;
        }),
        (i.red = function (b) {
          return new ke(b);
        }),
        (i.prototype.toRed = function (b) {
          return (
            r(!this.red, 'Already a number in reduction context'),
            r(this.negative === 0, 'red works only with positives'),
            b.convertTo(this)._forceRed(b)
          );
        }),
        (i.prototype.fromRed = function () {
          return (
            r(this.red, 'fromRed works only with numbers in reduction context'),
            this.red.convertFrom(this)
          );
        }),
        (i.prototype._forceRed = function (b) {
          return (this.red = b), this;
        }),
        (i.prototype.forceRed = function (b) {
          return (
            r(!this.red, 'Already a number in reduction context'),
            this._forceRed(b)
          );
        }),
        (i.prototype.redAdd = function (b) {
          return (
            r(this.red, 'redAdd works only with red numbers'),
            this.red.add(this, b)
          );
        }),
        (i.prototype.redIAdd = function (b) {
          return (
            r(this.red, 'redIAdd works only with red numbers'),
            this.red.iadd(this, b)
          );
        }),
        (i.prototype.redSub = function (b) {
          return (
            r(this.red, 'redSub works only with red numbers'),
            this.red.sub(this, b)
          );
        }),
        (i.prototype.redISub = function (b) {
          return (
            r(this.red, 'redISub works only with red numbers'),
            this.red.isub(this, b)
          );
        }),
        (i.prototype.redShl = function (b) {
          return (
            r(this.red, 'redShl works only with red numbers'),
            this.red.shl(this, b)
          );
        }),
        (i.prototype.redMul = function (b) {
          return (
            r(this.red, 'redMul works only with red numbers'),
            this.red._verify2(this, b),
            this.red.mul(this, b)
          );
        }),
        (i.prototype.redIMul = function (b) {
          return (
            r(this.red, 'redMul works only with red numbers'),
            this.red._verify2(this, b),
            this.red.imul(this, b)
          );
        }),
        (i.prototype.redSqr = function () {
          return (
            r(this.red, 'redSqr works only with red numbers'),
            this.red._verify1(this),
            this.red.sqr(this)
          );
        }),
        (i.prototype.redISqr = function () {
          return (
            r(this.red, 'redISqr works only with red numbers'),
            this.red._verify1(this),
            this.red.isqr(this)
          );
        }),
        (i.prototype.redSqrt = function () {
          return (
            r(this.red, 'redSqrt works only with red numbers'),
            this.red._verify1(this),
            this.red.sqrt(this)
          );
        }),
        (i.prototype.redInvm = function () {
          return (
            r(this.red, 'redInvm works only with red numbers'),
            this.red._verify1(this),
            this.red.invm(this)
          );
        }),
        (i.prototype.redNeg = function () {
          return (
            r(this.red, 'redNeg works only with red numbers'),
            this.red._verify1(this),
            this.red.neg(this)
          );
        }),
        (i.prototype.redPow = function (b) {
          return (
            r(this.red && !b.red, 'redPow(normalNum)'),
            this.red._verify1(this),
            this.red.pow(this, b)
          );
        });
      var Me = { k256: null, p224: null, p192: null, p25519: null };
      function Te(B, b) {
        (this.name = B),
          (this.p = new i(b, 16)),
          (this.n = this.p.bitLength()),
          (this.k = new i(1).iushln(this.n).isub(this.p)),
          (this.tmp = this._tmp());
      }
      (Te.prototype._tmp = function () {
        var b = new i(null);
        return (b.words = new Array(Math.ceil(this.n / 13))), b;
      }),
        (Te.prototype.ireduce = function (b) {
          var _ = b,
            l;
          do
            this.split(_, this.tmp),
              (_ = this.imulK(_)),
              (_ = _.iadd(this.tmp)),
              (l = _.bitLength());
          while (l > this.n);
          var f = l < this.n ? -1 : _.ucmp(this.p);
          return (
            f === 0
              ? ((_.words[0] = 0), (_.length = 1))
              : f > 0
                ? _.isub(this.p)
                : _.strip !== void 0
                  ? _.strip()
                  : _._strip(),
            _
          );
        }),
        (Te.prototype.split = function (b, _) {
          b.iushrn(this.n, 0, _);
        }),
        (Te.prototype.imulK = function (b) {
          return b.imul(this.k);
        });
      function Ee() {
        Te.call(
          this,
          'k256',
          'ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff fffffffe fffffc2f'
        );
      }
      n(Ee, Te),
        (Ee.prototype.split = function (b, _) {
          for (var l = 4194303, f = Math.min(b.length, 9), o = 0; o < f; o++)
            _.words[o] = b.words[o];
          if (((_.length = f), b.length <= 9)) {
            (b.words[0] = 0), (b.length = 1);
            return;
          }
          var c = b.words[9];
          for (_.words[_.length++] = c & l, o = 10; o < b.length; o++) {
            var p = b.words[o] | 0;
            (b.words[o - 10] = ((p & l) << 4) | (c >>> 22)), (c = p);
          }
          (c >>>= 22),
            (b.words[o - 10] = c),
            c === 0 && b.length > 10 ? (b.length -= 10) : (b.length -= 9);
        }),
        (Ee.prototype.imulK = function (b) {
          (b.words[b.length] = 0), (b.words[b.length + 1] = 0), (b.length += 2);
          for (var _ = 0, l = 0; l < b.length; l++) {
            var f = b.words[l] | 0;
            (_ += f * 977),
              (b.words[l] = _ & 67108863),
              (_ = f * 64 + ((_ / 67108864) | 0));
          }
          return (
            b.words[b.length - 1] === 0 &&
              (b.length--, b.words[b.length - 1] === 0 && b.length--),
            b
          );
        });
      function Fe() {
        Te.call(
          this,
          'p224',
          'ffffffff ffffffff ffffffff ffffffff 00000000 00000000 00000001'
        );
      }
      n(Fe, Te);
      function Se() {
        Te.call(
          this,
          'p192',
          'ffffffff ffffffff ffffffff fffffffe ffffffff ffffffff'
        );
      }
      n(Se, Te);
      function $e() {
        Te.call(
          this,
          '25519',
          '7fffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffed'
        );
      }
      n($e, Te),
        ($e.prototype.imulK = function (b) {
          for (var _ = 0, l = 0; l < b.length; l++) {
            var f = (b.words[l] | 0) * 19 + _,
              o = f & 67108863;
            (f >>>= 26), (b.words[l] = o), (_ = f);
          }
          return _ !== 0 && (b.words[b.length++] = _), b;
        }),
        (i._prime = function (b) {
          if (Me[b]) return Me[b];
          var _;
          if (b === 'k256') _ = new Ee();
          else if (b === 'p224') _ = new Fe();
          else if (b === 'p192') _ = new Se();
          else if (b === 'p25519') _ = new $e();
          else throw new Error('Unknown prime ' + b);
          return (Me[b] = _), _;
        });
      function ke(B) {
        if (typeof B == 'string') {
          var b = i._prime(B);
          (this.m = b.p), (this.prime = b);
        } else
          r(B.gtn(1), 'modulus must be greater than 1'),
            (this.m = B),
            (this.prime = null);
      }
      (ke.prototype._verify1 = function (b) {
        r(b.negative === 0, 'red works only with positives'),
          r(b.red, 'red works only with red numbers');
      }),
        (ke.prototype._verify2 = function (b, _) {
          r((b.negative | _.negative) === 0, 'red works only with positives'),
            r(b.red && b.red === _.red, 'red works only with red numbers');
        }),
        (ke.prototype.imod = function (b) {
          return this.prime
            ? this.prime.ireduce(b)._forceRed(this)
            : b.umod(this.m)._forceRed(this);
        }),
        (ke.prototype.neg = function (b) {
          return b.isZero() ? b.clone() : this.m.sub(b)._forceRed(this);
        }),
        (ke.prototype.add = function (b, _) {
          this._verify2(b, _);
          var l = b.add(_);
          return l.cmp(this.m) >= 0 && l.isub(this.m), l._forceRed(this);
        }),
        (ke.prototype.iadd = function (b, _) {
          this._verify2(b, _);
          var l = b.iadd(_);
          return l.cmp(this.m) >= 0 && l.isub(this.m), l;
        }),
        (ke.prototype.sub = function (b, _) {
          this._verify2(b, _);
          var l = b.sub(_);
          return l.cmpn(0) < 0 && l.iadd(this.m), l._forceRed(this);
        }),
        (ke.prototype.isub = function (b, _) {
          this._verify2(b, _);
          var l = b.isub(_);
          return l.cmpn(0) < 0 && l.iadd(this.m), l;
        }),
        (ke.prototype.shl = function (b, _) {
          return this._verify1(b), this.imod(b.ushln(_));
        }),
        (ke.prototype.imul = function (b, _) {
          return this._verify2(b, _), this.imod(b.imul(_));
        }),
        (ke.prototype.mul = function (b, _) {
          return this._verify2(b, _), this.imod(b.mul(_));
        }),
        (ke.prototype.isqr = function (b) {
          return this.imul(b, b.clone());
        }),
        (ke.prototype.sqr = function (b) {
          return this.mul(b, b);
        }),
        (ke.prototype.sqrt = function (b) {
          if (b.isZero()) return b.clone();
          var _ = this.m.andln(3);
          if ((r(_ % 2 === 1), _ === 3)) {
            var l = this.m.add(new i(1)).iushrn(2);
            return this.pow(b, l);
          }
          for (var f = this.m.subn(1), o = 0; !f.isZero() && f.andln(1) === 0; )
            o++, f.iushrn(1);
          r(!f.isZero());
          var c = new i(1).toRed(this),
            p = c.redNeg(),
            d = this.m.subn(1).iushrn(1),
            u = this.m.bitLength();
          for (u = new i(2 * u * u).toRed(this); this.pow(u, d).cmp(p) !== 0; )
            u.redIAdd(p);
          for (
            var y = this.pow(u, f),
              m = this.pow(b, f.addn(1).iushrn(1)),
              s = this.pow(b, f),
              w = o;
            s.cmp(c) !== 0;

          ) {
            for (var T = s, O = 0; T.cmp(c) !== 0; O++) T = T.redSqr();
            r(O < w);
            var P = this.pow(y, new i(1).iushln(w - O - 1));
            (m = m.redMul(P)), (y = P.redSqr()), (s = s.redMul(y)), (w = O);
          }
          return m;
        }),
        (ke.prototype.invm = function (b) {
          var _ = b._invmp(this.m);
          return _.negative !== 0
            ? ((_.negative = 0), this.imod(_).redNeg())
            : this.imod(_);
        }),
        (ke.prototype.pow = function (b, _) {
          if (_.isZero()) return new i(1).toRed(this);
          if (_.cmpn(1) === 0) return b.clone();
          var l = 4,
            f = new Array(1 << l);
          (f[0] = new i(1).toRed(this)), (f[1] = b);
          for (var o = 2; o < f.length; o++) f[o] = this.mul(f[o - 1], b);
          var c = f[0],
            p = 0,
            d = 0,
            u = _.bitLength() % 26;
          for (u === 0 && (u = 26), o = _.length - 1; o >= 0; o--) {
            for (var y = _.words[o], m = u - 1; m >= 0; m--) {
              var s = (y >> m) & 1;
              if ((c !== f[0] && (c = this.sqr(c)), s === 0 && p === 0)) {
                d = 0;
                continue;
              }
              (p <<= 1),
                (p |= s),
                d++,
                !(d !== l && (o !== 0 || m !== 0)) &&
                  ((c = this.mul(c, f[p])), (d = 0), (p = 0));
            }
            u = 26;
          }
          return c;
        }),
        (ke.prototype.convertTo = function (b) {
          var _ = b.umod(this.m);
          return _ === b ? _.clone() : _;
        }),
        (ke.prototype.convertFrom = function (b) {
          var _ = b.clone();
          return (_.red = null), _;
        }),
        (i.mont = function (b) {
          return new Ze(b);
        });
      function Ze(B) {
        ke.call(this, B),
          (this.shift = this.m.bitLength()),
          this.shift % 26 !== 0 && (this.shift += 26 - (this.shift % 26)),
          (this.r = new i(1).iushln(this.shift)),
          (this.r2 = this.imod(this.r.sqr())),
          (this.rinv = this.r._invmp(this.m)),
          (this.minv = this.rinv.mul(this.r).isubn(1).div(this.m)),
          (this.minv = this.minv.umod(this.r)),
          (this.minv = this.r.sub(this.minv));
      }
      n(Ze, ke),
        (Ze.prototype.convertTo = function (b) {
          return this.imod(b.ushln(this.shift));
        }),
        (Ze.prototype.convertFrom = function (b) {
          var _ = this.imod(b.mul(this.rinv));
          return (_.red = null), _;
        }),
        (Ze.prototype.imul = function (b, _) {
          if (b.isZero() || _.isZero())
            return (b.words[0] = 0), (b.length = 1), b;
          var l = b.imul(_),
            f = l
              .maskn(this.shift)
              .mul(this.minv)
              .imaskn(this.shift)
              .mul(this.m),
            o = l.isub(f).iushrn(this.shift),
            c = o;
          return (
            o.cmp(this.m) >= 0
              ? (c = o.isub(this.m))
              : o.cmpn(0) < 0 && (c = o.iadd(this.m)),
            c._forceRed(this)
          );
        }),
        (Ze.prototype.mul = function (b, _) {
          if (b.isZero() || _.isZero()) return new i(0)._forceRed(this);
          var l = b.mul(_),
            f = l
              .maskn(this.shift)
              .mul(this.minv)
              .imaskn(this.shift)
              .mul(this.m),
            o = l.isub(f).iushrn(this.shift),
            c = o;
          return (
            o.cmp(this.m) >= 0
              ? (c = o.isub(this.m))
              : o.cmpn(0) < 0 && (c = o.iadd(this.m)),
            c._forceRed(this)
          );
        }),
        (Ze.prototype.invm = function (b) {
          var _ = this.imod(b._invmp(this.m).mul(this.r2));
          return _._forceRed(this);
        });
    })(typeof fu > 'u' || fu, rb);
  });
  var is = R((OT, su) => {
    S();
    var au;
    su.exports = function (e) {
      return au || (au = new Bi(null)), au.generate(e);
    };
    function Bi(t) {
      this.rand = t;
    }
    su.exports.Rand = Bi;
    Bi.prototype.generate = function (e) {
      return this._rand(e);
    };
    Bi.prototype._rand = function (e) {
      if (this.rand.getBytes) return this.rand.getBytes(e);
      for (var r = new Uint8Array(e), n = 0; n < r.length; n++)
        r[n] = this.rand.getByte();
      return r;
    };
    if (typeof self == 'object')
      self.crypto && self.crypto.getRandomValues
        ? (Bi.prototype._rand = function (e) {
            var r = new Uint8Array(e);
            return self.crypto.getRandomValues(r), r;
          })
        : self.msCrypto && self.msCrypto.getRandomValues
          ? (Bi.prototype._rand = function (e) {
              var r = new Uint8Array(e);
              return self.msCrypto.getRandomValues(r), r;
            })
          : typeof window == 'object' &&
            (Bi.prototype._rand = function () {
              throw new Error('Not implemented yet');
            });
    else
      try {
        if (((ou = hu()), typeof ou.randomBytes != 'function'))
          throw new Error('Not supported');
        Bi.prototype._rand = function (e) {
          return ou.randomBytes(e);
        };
      } catch {}
    var ou;
  });
  var uu = R((NT, ib) => {
    S();
    var sn = it(),
      pw = is();
    function hn(t) {
      this.rand = t || new pw.Rand();
    }
    ib.exports = hn;
    hn.create = function (e) {
      return new hn(e);
    };
    hn.prototype._randbelow = function (e) {
      var r = e.bitLength(),
        n = Math.ceil(r / 8);
      do var i = new sn(this.rand.generate(n));
      while (i.cmp(e) >= 0);
      return i;
    };
    hn.prototype._randrange = function (e, r) {
      var n = r.sub(e);
      return e.add(this._randbelow(n));
    };
    hn.prototype.test = function (e, r, n) {
      var i = e.bitLength(),
        a = sn.mont(e),
        h = new sn(1).toRed(a);
      r || (r = Math.max(1, (i / 48) | 0));
      for (var v = e.subn(1), g = 0; !v.testn(g); g++);
      for (var M = e.shrn(g), x = v.toRed(a), E = !0; r > 0; r--) {
        var I = this._randrange(new sn(2), v);
        n && n(I);
        var q = I.toRed(a).redPow(M);
        if (!(q.cmp(h) === 0 || q.cmp(x) === 0)) {
          for (var k = 1; k < g; k++) {
            if (((q = q.redSqr()), q.cmp(h) === 0)) return !1;
            if (q.cmp(x) === 0) break;
          }
          if (k === g) return !1;
        }
      }
      return E;
    };
    hn.prototype.getDivisor = function (e, r) {
      var n = e.bitLength(),
        i = sn.mont(e),
        a = new sn(1).toRed(i);
      r || (r = Math.max(1, (n / 48) | 0));
      for (var h = e.subn(1), v = 0; !h.testn(v); v++);
      for (var g = e.shrn(v), M = h.toRed(i); r > 0; r--) {
        var x = this._randrange(new sn(2), h),
          E = e.gcd(x);
        if (E.cmpn(1) !== 0) return E;
        var I = x.toRed(i).redPow(g);
        if (!(I.cmp(a) === 0 || I.cmp(M) === 0)) {
          for (var q = 1; q < v; q++) {
            if (((I = I.redSqr()), I.cmp(a) === 0))
              return I.fromRed().subn(1).gcd(e);
            if (I.cmp(M) === 0) break;
          }
          if (q === v) return (I = I.redSqr()), I.fromRed().subn(1).gcd(e);
        }
      }
      return !1;
    };
  });
  var bu = R((zT, ab) => {
    S();
    var vw = Xi();
    ab.exports = vu;
    vu.simpleSieve = lu;
    vu.fermatTest = pu;
    var ht = it(),
      bw = new ht(24),
      yw = uu(),
      nb = new yw(),
      mw = new ht(1),
      du = new ht(2),
      gw = new ht(5),
      LT = new ht(16),
      FT = new ht(8),
      ww = new ht(10),
      _w = new ht(3),
      jT = new ht(7),
      xw = new ht(11),
      fb = new ht(4),
      UT = new ht(12),
      cu = null;
    function Mw() {
      if (cu !== null) return cu;
      var t = 1048576,
        e = [];
      e[0] = 2;
      for (var r = 1, n = 3; n < t; n += 2) {
        for (
          var i = Math.ceil(Math.sqrt(n)), a = 0;
          a < r && e[a] <= i && n % e[a] !== 0;
          a++
        );
        (r !== a && e[a] <= i) || (e[r++] = n);
      }
      return (cu = e), e;
    }
    function lu(t) {
      for (var e = Mw(), r = 0; r < e.length; r++)
        if (t.modn(e[r]) === 0) return t.cmpn(e[r]) === 0;
      return !0;
    }
    function pu(t) {
      var e = ht.mont(t);
      return du.toRed(e).redPow(t.subn(1)).fromRed().cmpn(1) === 0;
    }
    function vu(t, e) {
      if (t < 16)
        return e === 2 || e === 5 ? new ht([140, 123]) : new ht([140, 39]);
      e = new ht(e);
      for (var r, n; ; ) {
        for (r = new ht(vw(Math.ceil(t / 8))); r.bitLength() > t; ) r.ishrn(1);
        if ((r.isEven() && r.iadd(mw), r.testn(1) || r.iadd(du), e.cmp(du))) {
          if (!e.cmp(gw)) for (; r.mod(ww).cmp(_w); ) r.iadd(fb);
        } else for (; r.mod(bw).cmp(xw); ) r.iadd(fb);
        if (
          ((n = r.shrn(1)),
          lu(n) && lu(r) && pu(n) && pu(r) && nb.test(n) && nb.test(r))
        )
          return r;
      }
    }
  });
  var ob = R((KT, Sw) => {
    Sw.exports = {
      modp1: {
        gen: '02',
        prime:
          'ffffffffffffffffc90fdaa22168c234c4c6628b80dc1cd129024e088a67cc74020bbea63b139b22514a08798e3404ddef9519b3cd3a431b302b0a6df25f14374fe1356d6d51c245e485b576625e7ec6f44c42e9a63a3620ffffffffffffffff',
      },
      modp2: {
        gen: '02',
        prime:
          'ffffffffffffffffc90fdaa22168c234c4c6628b80dc1cd129024e088a67cc74020bbea63b139b22514a08798e3404ddef9519b3cd3a431b302b0a6df25f14374fe1356d6d51c245e485b576625e7ec6f44c42e9a637ed6b0bff5cb6f406b7edee386bfb5a899fa5ae9f24117c4b1fe649286651ece65381ffffffffffffffff',
      },
      modp5: {
        gen: '02',
        prime:
          'ffffffffffffffffc90fdaa22168c234c4c6628b80dc1cd129024e088a67cc74020bbea63b139b22514a08798e3404ddef9519b3cd3a431b302b0a6df25f14374fe1356d6d51c245e485b576625e7ec6f44c42e9a637ed6b0bff5cb6f406b7edee386bfb5a899fa5ae9f24117c4b1fe649286651ece45b3dc2007cb8a163bf0598da48361c55d39a69163fa8fd24cf5f83655d23dca3ad961c62f356208552bb9ed529077096966d670c354e4abc9804f1746c08ca237327ffffffffffffffff',
      },
      modp14: {
        gen: '02',
        prime:
          'ffffffffffffffffc90fdaa22168c234c4c6628b80dc1cd129024e088a67cc74020bbea63b139b22514a08798e3404ddef9519b3cd3a431b302b0a6df25f14374fe1356d6d51c245e485b576625e7ec6f44c42e9a637ed6b0bff5cb6f406b7edee386bfb5a899fa5ae9f24117c4b1fe649286651ece45b3dc2007cb8a163bf0598da48361c55d39a69163fa8fd24cf5f83655d23dca3ad961c62f356208552bb9ed529077096966d670c354e4abc9804f1746c08ca18217c32905e462e36ce3be39e772c180e86039b2783a2ec07a28fb5c55df06f4c52c9de2bcbf6955817183995497cea956ae515d2261898fa051015728e5a8aacaa68ffffffffffffffff',
      },
      modp15: {
        gen: '02',
        prime:
          'ffffffffffffffffc90fdaa22168c234c4c6628b80dc1cd129024e088a67cc74020bbea63b139b22514a08798e3404ddef9519b3cd3a431b302b0a6df25f14374fe1356d6d51c245e485b576625e7ec6f44c42e9a637ed6b0bff5cb6f406b7edee386bfb5a899fa5ae9f24117c4b1fe649286651ece45b3dc2007cb8a163bf0598da48361c55d39a69163fa8fd24cf5f83655d23dca3ad961c62f356208552bb9ed529077096966d670c354e4abc9804f1746c08ca18217c32905e462e36ce3be39e772c180e86039b2783a2ec07a28fb5c55df06f4c52c9de2bcbf6955817183995497cea956ae515d2261898fa051015728e5a8aaac42dad33170d04507a33a85521abdf1cba64ecfb850458dbef0a8aea71575d060c7db3970f85a6e1e4c7abf5ae8cdb0933d71e8c94e04a25619dcee3d2261ad2ee6bf12ffa06d98a0864d87602733ec86a64521f2b18177b200cbbe117577a615d6c770988c0bad946e208e24fa074e5ab3143db5bfce0fd108e4b82d120a93ad2caffffffffffffffff',
      },
      modp16: {
        gen: '02',
        prime:
          'ffffffffffffffffc90fdaa22168c234c4c6628b80dc1cd129024e088a67cc74020bbea63b139b22514a08798e3404ddef9519b3cd3a431b302b0a6df25f14374fe1356d6d51c245e485b576625e7ec6f44c42e9a637ed6b0bff5cb6f406b7edee386bfb5a899fa5ae9f24117c4b1fe649286651ece45b3dc2007cb8a163bf0598da48361c55d39a69163fa8fd24cf5f83655d23dca3ad961c62f356208552bb9ed529077096966d670c354e4abc9804f1746c08ca18217c32905e462e36ce3be39e772c180e86039b2783a2ec07a28fb5c55df06f4c52c9de2bcbf6955817183995497cea956ae515d2261898fa051015728e5a8aaac42dad33170d04507a33a85521abdf1cba64ecfb850458dbef0a8aea71575d060c7db3970f85a6e1e4c7abf5ae8cdb0933d71e8c94e04a25619dcee3d2261ad2ee6bf12ffa06d98a0864d87602733ec86a64521f2b18177b200cbbe117577a615d6c770988c0bad946e208e24fa074e5ab3143db5bfce0fd108e4b82d120a92108011a723c12a787e6d788719a10bdba5b2699c327186af4e23c1a946834b6150bda2583e9ca2ad44ce8dbbbc2db04de8ef92e8efc141fbecaa6287c59474e6bc05d99b2964fa090c3a2233ba186515be7ed1f612970cee2d7afb81bdd762170481cd0069127d5b05aa993b4ea988d8fddc186ffb7dc90a6c08f4df435c934063199ffffffffffffffff',
      },
      modp17: {
        gen: '02',
        prime:
          'ffffffffffffffffc90fdaa22168c234c4c6628b80dc1cd129024e088a67cc74020bbea63b139b22514a08798e3404ddef9519b3cd3a431b302b0a6df25f14374fe1356d6d51c245e485b576625e7ec6f44c42e9a637ed6b0bff5cb6f406b7edee386bfb5a899fa5ae9f24117c4b1fe649286651ece45b3dc2007cb8a163bf0598da48361c55d39a69163fa8fd24cf5f83655d23dca3ad961c62f356208552bb9ed529077096966d670c354e4abc9804f1746c08ca18217c32905e462e36ce3be39e772c180e86039b2783a2ec07a28fb5c55df06f4c52c9de2bcbf6955817183995497cea956ae515d2261898fa051015728e5a8aaac42dad33170d04507a33a85521abdf1cba64ecfb850458dbef0a8aea71575d060c7db3970f85a6e1e4c7abf5ae8cdb0933d71e8c94e04a25619dcee3d2261ad2ee6bf12ffa06d98a0864d87602733ec86a64521f2b18177b200cbbe117577a615d6c770988c0bad946e208e24fa074e5ab3143db5bfce0fd108e4b82d120a92108011a723c12a787e6d788719a10bdba5b2699c327186af4e23c1a946834b6150bda2583e9ca2ad44ce8dbbbc2db04de8ef92e8efc141fbecaa6287c59474e6bc05d99b2964fa090c3a2233ba186515be7ed1f612970cee2d7afb81bdd762170481cd0069127d5b05aa993b4ea988d8fddc186ffb7dc90a6c08f4df435c93402849236c3fab4d27c7026c1d4dcb2602646dec9751e763dba37bdf8ff9406ad9e530ee5db382f413001aeb06a53ed9027d831179727b0865a8918da3edbebcf9b14ed44ce6cbaced4bb1bdb7f1447e6cc254b332051512bd7af426fb8f401378cd2bf5983ca01c64b92ecf032ea15d1721d03f482d7ce6e74fef6d55e702f46980c82b5a84031900b1c9e59e7c97fbec7e8f323a97a7e36cc88be0f1d45b7ff585ac54bd407b22b4154aacc8f6d7ebf48e1d814cc5ed20f8037e0a79715eef29be32806a1d58bb7c5da76f550aa3d8a1fbff0eb19ccb1a313d55cda56c9ec2ef29632387fe8d76e3c0468043e8f663f4860ee12bf2d5b0b7474d6e694f91e6dcc4024ffffffffffffffff',
      },
      modp18: {
        gen: '02',
        prime:
          'ffffffffffffffffc90fdaa22168c234c4c6628b80dc1cd129024e088a67cc74020bbea63b139b22514a08798e3404ddef9519b3cd3a431b302b0a6df25f14374fe1356d6d51c245e485b576625e7ec6f44c42e9a637ed6b0bff5cb6f406b7edee386bfb5a899fa5ae9f24117c4b1fe649286651ece45b3dc2007cb8a163bf0598da48361c55d39a69163fa8fd24cf5f83655d23dca3ad961c62f356208552bb9ed529077096966d670c354e4abc9804f1746c08ca18217c32905e462e36ce3be39e772c180e86039b2783a2ec07a28fb5c55df06f4c52c9de2bcbf6955817183995497cea956ae515d2261898fa051015728e5a8aaac42dad33170d04507a33a85521abdf1cba64ecfb850458dbef0a8aea71575d060c7db3970f85a6e1e4c7abf5ae8cdb0933d71e8c94e04a25619dcee3d2261ad2ee6bf12ffa06d98a0864d87602733ec86a64521f2b18177b200cbbe117577a615d6c770988c0bad946e208e24fa074e5ab3143db5bfce0fd108e4b82d120a92108011a723c12a787e6d788719a10bdba5b2699c327186af4e23c1a946834b6150bda2583e9ca2ad44ce8dbbbc2db04de8ef92e8efc141fbecaa6287c59474e6bc05d99b2964fa090c3a2233ba186515be7ed1f612970cee2d7afb81bdd762170481cd0069127d5b05aa993b4ea988d8fddc186ffb7dc90a6c08f4df435c93402849236c3fab4d27c7026c1d4dcb2602646dec9751e763dba37bdf8ff9406ad9e530ee5db382f413001aeb06a53ed9027d831179727b0865a8918da3edbebcf9b14ed44ce6cbaced4bb1bdb7f1447e6cc254b332051512bd7af426fb8f401378cd2bf5983ca01c64b92ecf032ea15d1721d03f482d7ce6e74fef6d55e702f46980c82b5a84031900b1c9e59e7c97fbec7e8f323a97a7e36cc88be0f1d45b7ff585ac54bd407b22b4154aacc8f6d7ebf48e1d814cc5ed20f8037e0a79715eef29be32806a1d58bb7c5da76f550aa3d8a1fbff0eb19ccb1a313d55cda56c9ec2ef29632387fe8d76e3c0468043e8f663f4860ee12bf2d5b0b7474d6e694f91e6dbe115974a3926f12fee5e438777cb6a932df8cd8bec4d073b931ba3bc832b68d9dd300741fa7bf8afc47ed2576f6936ba424663aab639c5ae4f5683423b4742bf1c978238f16cbe39d652de3fdb8befc848ad922222e04a4037c0713eb57a81a23f0c73473fc646cea306b4bcbc8862f8385ddfa9d4b7fa2c087e879683303ed5bdd3a062b3cf5b3a278a66d2a13f83f44f82ddf310ee074ab6a364597e899a0255dc164f31cc50846851df9ab48195ded7ea1b1d510bd7ee74d73faf36bc31ecfa268359046f4eb879f924009438b481c6cd7889a002ed5ee382bc9190da6fc026e479558e4475677e9aa9e3050e2765694dfc81f56e880b96e7160c980dd98edd3dfffffffffffffffff',
      },
    };
  });
  var cb = R((VT, ub) => {
    S();
    var Lt = it(),
      Ew = uu(),
      sb = new Ew(),
      Aw = new Lt(24),
      Bw = new Lt(11),
      Iw = new Lt(10),
      Rw = new Lt(3),
      qw = new Lt(7),
      hb = bu(),
      Tw = Xi();
    ub.exports = Wr;
    function Pw(t, e) {
      return (
        (e = e || 'utf8'),
        Buffer.isBuffer(t) || (t = new Buffer(t, e)),
        (this._pub = new Lt(t)),
        this
      );
    }
    function kw(t, e) {
      return (
        (e = e || 'utf8'),
        Buffer.isBuffer(t) || (t = new Buffer(t, e)),
        (this._priv = new Lt(t)),
        this
      );
    }
    var ns = {};
    function Ow(t, e) {
      var r = e.toString('hex'),
        n = [r, t.toString(16)].join('_');
      if (n in ns) return ns[n];
      var i = 0;
      if (t.isEven() || !hb.simpleSieve || !hb.fermatTest(t) || !sb.test(t))
        return (
          (i += 1),
          r === '02' || r === '05' ? (i += 8) : (i += 4),
          (ns[n] = i),
          i
        );
      sb.test(t.shrn(1)) || (i += 2);
      var a;
      switch (r) {
        case '02':
          t.mod(Aw).cmp(Bw) && (i += 8);
          break;
        case '05':
          (a = t.mod(Iw)), a.cmp(Rw) && a.cmp(qw) && (i += 8);
          break;
        default:
          i += 4;
      }
      return (ns[n] = i), i;
    }
    function Wr(t, e, r) {
      this.setGenerator(e),
        (this.__prime = new Lt(t)),
        (this._prime = Lt.mont(this.__prime)),
        (this._primeLen = t.length),
        (this._pub = void 0),
        (this._priv = void 0),
        (this._primeCode = void 0),
        r
          ? ((this.setPublicKey = Pw), (this.setPrivateKey = kw))
          : (this._primeCode = 8);
    }
    Object.defineProperty(Wr.prototype, 'verifyError', {
      enumerable: !0,
      get: function () {
        return (
          typeof this._primeCode != 'number' &&
            (this._primeCode = Ow(this.__prime, this.__gen)),
          this._primeCode
        );
      },
    });
    Wr.prototype.generateKeys = function () {
      return (
        this._priv || (this._priv = new Lt(Tw(this._primeLen))),
        (this._pub = this._gen.toRed(this._prime).redPow(this._priv).fromRed()),
        this.getPublicKey()
      );
    };
    Wr.prototype.computeSecret = function (t) {
      (t = new Lt(t)), (t = t.toRed(this._prime));
      var e = t.redPow(this._priv).fromRed(),
        r = new Buffer(e.toArray()),
        n = this.getPrime();
      if (r.length < n.length) {
        var i = new Buffer(n.length - r.length);
        i.fill(0), (r = Buffer.concat([i, r]));
      }
      return r;
    };
    Wr.prototype.getPublicKey = function (e) {
      return fs(this._pub, e);
    };
    Wr.prototype.getPrivateKey = function (e) {
      return fs(this._priv, e);
    };
    Wr.prototype.getPrime = function (t) {
      return fs(this.__prime, t);
    };
    Wr.prototype.getGenerator = function (t) {
      return fs(this._gen, t);
    };
    Wr.prototype.setGenerator = function (t, e) {
      return (
        (e = e || 'utf8'),
        Buffer.isBuffer(t) || (t = new Buffer(t, e)),
        (this.__gen = t),
        (this._gen = new Lt(t)),
        this
      );
    };
    function fs(t, e) {
      var r = new Buffer(t.toArray());
      return e ? r.toString(e) : r;
    }
  });
  var pb = R(($n) => {
    S();
    var Cw = bu(),
      db = ob(),
      yu = cb();
    function Nw(t) {
      var e = new Buffer(db[t].prime, 'hex'),
        r = new Buffer(db[t].gen, 'hex');
      return new yu(e, r);
    }
    var Dw = { binary: !0, hex: !0, base64: !0 };
    function lb(t, e, r, n) {
      return Buffer.isBuffer(e) || Dw[e] === void 0
        ? lb(t, 'binary', e, r)
        : ((e = e || 'binary'),
          (n = n || 'binary'),
          (r = r || new Buffer([2])),
          Buffer.isBuffer(r) || (r = new Buffer(r, n)),
          typeof t == 'number'
            ? new yu(Cw(t, r), r, !0)
            : (Buffer.isBuffer(t) || (t = new Buffer(t, e)), new yu(t, r, !0)));
    }
    $n.DiffieHellmanGroup =
      $n.createDiffieHellmanGroup =
      $n.getDiffieHellman =
        Nw;
    $n.createDiffieHellman = $n.DiffieHellman = lb;
  });
  var bb = R((vb, mu) => {
    S();
    (function (t, e) {
      'use strict';
      function r(l, f) {
        if (!l) throw new Error(f || 'Assertion failed');
      }
      function n(l, f) {
        l.super_ = f;
        var o = function () {};
        (o.prototype = f.prototype),
          (l.prototype = new o()),
          (l.prototype.constructor = l);
      }
      function i(l, f, o) {
        if (i.isBN(l)) return l;
        (this.negative = 0),
          (this.words = null),
          (this.length = 0),
          (this.red = null),
          l !== null &&
            ((f === 'le' || f === 'be') && ((o = f), (f = 10)),
            this._init(l || 0, f || 10, o || 'be'));
      }
      typeof t == 'object' ? (t.exports = i) : (e.BN = i),
        (i.BN = i),
        (i.wordSize = 26);
      var a;
      try {
        typeof window < 'u' && typeof window.Buffer < 'u'
          ? (a = window.Buffer)
          : (a = Et().Buffer);
      } catch {}
      (i.isBN = function (f) {
        return f instanceof i
          ? !0
          : f !== null &&
              typeof f == 'object' &&
              f.constructor.wordSize === i.wordSize &&
              Array.isArray(f.words);
      }),
        (i.max = function (f, o) {
          return f.cmp(o) > 0 ? f : o;
        }),
        (i.min = function (f, o) {
          return f.cmp(o) < 0 ? f : o;
        }),
        (i.prototype._init = function (f, o, c) {
          if (typeof f == 'number') return this._initNumber(f, o, c);
          if (typeof f == 'object') return this._initArray(f, o, c);
          o === 'hex' && (o = 16),
            r(o === (o | 0) && o >= 2 && o <= 36),
            (f = f.toString().replace(/\s+/g, ''));
          var p = 0;
          f[0] === '-' && (p++, (this.negative = 1)),
            p < f.length &&
              (o === 16
                ? this._parseHex(f, p, c)
                : (this._parseBase(f, o, p),
                  c === 'le' && this._initArray(this.toArray(), o, c)));
        }),
        (i.prototype._initNumber = function (f, o, c) {
          f < 0 && ((this.negative = 1), (f = -f)),
            f < 67108864
              ? ((this.words = [f & 67108863]), (this.length = 1))
              : f < 4503599627370496
                ? ((this.words = [f & 67108863, (f / 67108864) & 67108863]),
                  (this.length = 2))
                : (r(f < 9007199254740992),
                  (this.words = [f & 67108863, (f / 67108864) & 67108863, 1]),
                  (this.length = 3)),
            c === 'le' && this._initArray(this.toArray(), o, c);
        }),
        (i.prototype._initArray = function (f, o, c) {
          if ((r(typeof f.length == 'number'), f.length <= 0))
            return (this.words = [0]), (this.length = 1), this;
          (this.length = Math.ceil(f.length / 3)),
            (this.words = new Array(this.length));
          for (var p = 0; p < this.length; p++) this.words[p] = 0;
          var d,
            u,
            y = 0;
          if (c === 'be')
            for (p = f.length - 1, d = 0; p >= 0; p -= 3)
              (u = f[p] | (f[p - 1] << 8) | (f[p - 2] << 16)),
                (this.words[d] |= (u << y) & 67108863),
                (this.words[d + 1] = (u >>> (26 - y)) & 67108863),
                (y += 24),
                y >= 26 && ((y -= 26), d++);
          else if (c === 'le')
            for (p = 0, d = 0; p < f.length; p += 3)
              (u = f[p] | (f[p + 1] << 8) | (f[p + 2] << 16)),
                (this.words[d] |= (u << y) & 67108863),
                (this.words[d + 1] = (u >>> (26 - y)) & 67108863),
                (y += 24),
                y >= 26 && ((y -= 26), d++);
          return this._strip();
        });
      function h(l, f) {
        var o = l.charCodeAt(f);
        if (o >= 48 && o <= 57) return o - 48;
        if (o >= 65 && o <= 70) return o - 55;
        if (o >= 97 && o <= 102) return o - 87;
        r(!1, 'Invalid character in ' + l);
      }
      function v(l, f, o) {
        var c = h(l, o);
        return o - 1 >= f && (c |= h(l, o - 1) << 4), c;
      }
      i.prototype._parseHex = function (f, o, c) {
        (this.length = Math.ceil((f.length - o) / 6)),
          (this.words = new Array(this.length));
        for (var p = 0; p < this.length; p++) this.words[p] = 0;
        var d = 0,
          u = 0,
          y;
        if (c === 'be')
          for (p = f.length - 1; p >= o; p -= 2)
            (y = v(f, o, p) << d),
              (this.words[u] |= y & 67108863),
              d >= 18
                ? ((d -= 18), (u += 1), (this.words[u] |= y >>> 26))
                : (d += 8);
        else {
          var m = f.length - o;
          for (p = m % 2 === 0 ? o + 1 : o; p < f.length; p += 2)
            (y = v(f, o, p) << d),
              (this.words[u] |= y & 67108863),
              d >= 18
                ? ((d -= 18), (u += 1), (this.words[u] |= y >>> 26))
                : (d += 8);
        }
        this._strip();
      };
      function g(l, f, o, c) {
        for (var p = 0, d = 0, u = Math.min(l.length, o), y = f; y < u; y++) {
          var m = l.charCodeAt(y) - 48;
          (p *= c),
            m >= 49 ? (d = m - 49 + 10) : m >= 17 ? (d = m - 17 + 10) : (d = m),
            r(m >= 0 && d < c, 'Invalid character'),
            (p += d);
        }
        return p;
      }
      (i.prototype._parseBase = function (f, o, c) {
        (this.words = [0]), (this.length = 1);
        for (var p = 0, d = 1; d <= 67108863; d *= o) p++;
        p--, (d = (d / o) | 0);
        for (
          var u = f.length - c,
            y = u % p,
            m = Math.min(u, u - y) + c,
            s = 0,
            w = c;
          w < m;
          w += p
        )
          (s = g(f, w, w + p, o)),
            this.imuln(d),
            this.words[0] + s < 67108864
              ? (this.words[0] += s)
              : this._iaddn(s);
        if (y !== 0) {
          var T = 1;
          for (s = g(f, w, f.length, o), w = 0; w < y; w++) T *= o;
          this.imuln(T),
            this.words[0] + s < 67108864
              ? (this.words[0] += s)
              : this._iaddn(s);
        }
        this._strip();
      }),
        (i.prototype.copy = function (f) {
          f.words = new Array(this.length);
          for (var o = 0; o < this.length; o++) f.words[o] = this.words[o];
          (f.length = this.length),
            (f.negative = this.negative),
            (f.red = this.red);
        });
      function M(l, f) {
        (l.words = f.words),
          (l.length = f.length),
          (l.negative = f.negative),
          (l.red = f.red);
      }
      if (
        ((i.prototype._move = function (f) {
          M(f, this);
        }),
        (i.prototype.clone = function () {
          var f = new i(null);
          return this.copy(f), f;
        }),
        (i.prototype._expand = function (f) {
          for (; this.length < f; ) this.words[this.length++] = 0;
          return this;
        }),
        (i.prototype._strip = function () {
          for (; this.length > 1 && this.words[this.length - 1] === 0; )
            this.length--;
          return this._normSign();
        }),
        (i.prototype._normSign = function () {
          return (
            this.length === 1 && this.words[0] === 0 && (this.negative = 0),
            this
          );
        }),
        typeof Symbol < 'u' && typeof Symbol.for == 'function')
      )
        try {
          i.prototype[Symbol.for('nodejs.util.inspect.custom')] = x;
        } catch {
          i.prototype.inspect = x;
        }
      else i.prototype.inspect = x;
      function x() {
        return (this.red ? '<BN-R: ' : '<BN: ') + this.toString(16) + '>';
      }
      var E = [
          '',
          '0',
          '00',
          '000',
          '0000',
          '00000',
          '000000',
          '0000000',
          '00000000',
          '000000000',
          '0000000000',
          '00000000000',
          '000000000000',
          '0000000000000',
          '00000000000000',
          '000000000000000',
          '0000000000000000',
          '00000000000000000',
          '000000000000000000',
          '0000000000000000000',
          '00000000000000000000',
          '000000000000000000000',
          '0000000000000000000000',
          '00000000000000000000000',
          '000000000000000000000000',
          '0000000000000000000000000',
        ],
        I = [
          0, 0, 25, 16, 12, 11, 10, 9, 8, 8, 7, 7, 7, 7, 6, 6, 6, 6, 6, 6, 6, 5,
          5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
        ],
        q = [
          0, 0, 33554432, 43046721, 16777216, 48828125, 60466176, 40353607,
          16777216, 43046721, 1e7, 19487171, 35831808, 62748517, 7529536,
          11390625, 16777216, 24137569, 34012224, 47045881, 64e6, 4084101,
          5153632, 6436343, 7962624, 9765625, 11881376, 14348907, 17210368,
          20511149, 243e5, 28629151, 33554432, 39135393, 45435424, 52521875,
          60466176,
        ];
      (i.prototype.toString = function (f, o) {
        (f = f || 10), (o = o | 0 || 1);
        var c;
        if (f === 16 || f === 'hex') {
          c = '';
          for (var p = 0, d = 0, u = 0; u < this.length; u++) {
            var y = this.words[u],
              m = (((y << p) | d) & 16777215).toString(16);
            (d = (y >>> (24 - p)) & 16777215),
              (p += 2),
              p >= 26 && ((p -= 26), u--),
              d !== 0 || u !== this.length - 1
                ? (c = E[6 - m.length] + m + c)
                : (c = m + c);
          }
          for (d !== 0 && (c = d.toString(16) + c); c.length % o !== 0; )
            c = '0' + c;
          return this.negative !== 0 && (c = '-' + c), c;
        }
        if (f === (f | 0) && f >= 2 && f <= 36) {
          var s = I[f],
            w = q[f];
          c = '';
          var T = this.clone();
          for (T.negative = 0; !T.isZero(); ) {
            var O = T.modrn(w).toString(f);
            (T = T.idivn(w)),
              T.isZero() ? (c = O + c) : (c = E[s - O.length] + O + c);
          }
          for (this.isZero() && (c = '0' + c); c.length % o !== 0; )
            c = '0' + c;
          return this.negative !== 0 && (c = '-' + c), c;
        }
        r(!1, 'Base should be between 2 and 36');
      }),
        (i.prototype.toNumber = function () {
          var f = this.words[0];
          return (
            this.length === 2
              ? (f += this.words[1] * 67108864)
              : this.length === 3 && this.words[2] === 1
                ? (f += 4503599627370496 + this.words[1] * 67108864)
                : this.length > 2 &&
                  r(!1, 'Number can only safely store up to 53 bits'),
            this.negative !== 0 ? -f : f
          );
        }),
        (i.prototype.toJSON = function () {
          return this.toString(16, 2);
        }),
        a &&
          (i.prototype.toBuffer = function (f, o) {
            return this.toArrayLike(a, f, o);
          }),
        (i.prototype.toArray = function (f, o) {
          return this.toArrayLike(Array, f, o);
        });
      var k = function (f, o) {
        return f.allocUnsafe ? f.allocUnsafe(o) : new f(o);
      };
      (i.prototype.toArrayLike = function (f, o, c) {
        this._strip();
        var p = this.byteLength(),
          d = c || Math.max(1, p);
        r(p <= d, 'byte array longer than desired length'),
          r(d > 0, 'Requested array length <= 0');
        var u = k(f, d),
          y = o === 'le' ? 'LE' : 'BE';
        return this['_toArrayLike' + y](u, p), u;
      }),
        (i.prototype._toArrayLikeLE = function (f, o) {
          for (var c = 0, p = 0, d = 0, u = 0; d < this.length; d++) {
            var y = (this.words[d] << u) | p;
            (f[c++] = y & 255),
              c < f.length && (f[c++] = (y >> 8) & 255),
              c < f.length && (f[c++] = (y >> 16) & 255),
              u === 6
                ? (c < f.length && (f[c++] = (y >> 24) & 255), (p = 0), (u = 0))
                : ((p = y >>> 24), (u += 2));
          }
          if (c < f.length) for (f[c++] = p; c < f.length; ) f[c++] = 0;
        }),
        (i.prototype._toArrayLikeBE = function (f, o) {
          for (
            var c = f.length - 1, p = 0, d = 0, u = 0;
            d < this.length;
            d++
          ) {
            var y = (this.words[d] << u) | p;
            (f[c--] = y & 255),
              c >= 0 && (f[c--] = (y >> 8) & 255),
              c >= 0 && (f[c--] = (y >> 16) & 255),
              u === 6
                ? (c >= 0 && (f[c--] = (y >> 24) & 255), (p = 0), (u = 0))
                : ((p = y >>> 24), (u += 2));
          }
          if (c >= 0) for (f[c--] = p; c >= 0; ) f[c--] = 0;
        }),
        Math.clz32
          ? (i.prototype._countBits = function (f) {
              return 32 - Math.clz32(f);
            })
          : (i.prototype._countBits = function (f) {
              var o = f,
                c = 0;
              return (
                o >= 4096 && ((c += 13), (o >>>= 13)),
                o >= 64 && ((c += 7), (o >>>= 7)),
                o >= 8 && ((c += 4), (o >>>= 4)),
                o >= 2 && ((c += 2), (o >>>= 2)),
                c + o
              );
            }),
        (i.prototype._zeroBits = function (f) {
          if (f === 0) return 26;
          var o = f,
            c = 0;
          return (
            (o & 8191) === 0 && ((c += 13), (o >>>= 13)),
            (o & 127) === 0 && ((c += 7), (o >>>= 7)),
            (o & 15) === 0 && ((c += 4), (o >>>= 4)),
            (o & 3) === 0 && ((c += 2), (o >>>= 2)),
            (o & 1) === 0 && c++,
            c
          );
        }),
        (i.prototype.bitLength = function () {
          var f = this.words[this.length - 1],
            o = this._countBits(f);
          return (this.length - 1) * 26 + o;
        });
      function L(l) {
        for (var f = new Array(l.bitLength()), o = 0; o < f.length; o++) {
          var c = (o / 26) | 0,
            p = o % 26;
          f[o] = (l.words[c] >>> p) & 1;
        }
        return f;
      }
      (i.prototype.zeroBits = function () {
        if (this.isZero()) return 0;
        for (var f = 0, o = 0; o < this.length; o++) {
          var c = this._zeroBits(this.words[o]);
          if (((f += c), c !== 26)) break;
        }
        return f;
      }),
        (i.prototype.byteLength = function () {
          return Math.ceil(this.bitLength() / 8);
        }),
        (i.prototype.toTwos = function (f) {
          return this.negative !== 0
            ? this.abs().inotn(f).iaddn(1)
            : this.clone();
        }),
        (i.prototype.fromTwos = function (f) {
          return this.testn(f - 1)
            ? this.notn(f).iaddn(1).ineg()
            : this.clone();
        }),
        (i.prototype.isNeg = function () {
          return this.negative !== 0;
        }),
        (i.prototype.neg = function () {
          return this.clone().ineg();
        }),
        (i.prototype.ineg = function () {
          return this.isZero() || (this.negative ^= 1), this;
        }),
        (i.prototype.iuor = function (f) {
          for (; this.length < f.length; ) this.words[this.length++] = 0;
          for (var o = 0; o < f.length; o++)
            this.words[o] = this.words[o] | f.words[o];
          return this._strip();
        }),
        (i.prototype.ior = function (f) {
          return r((this.negative | f.negative) === 0), this.iuor(f);
        }),
        (i.prototype.or = function (f) {
          return this.length > f.length
            ? this.clone().ior(f)
            : f.clone().ior(this);
        }),
        (i.prototype.uor = function (f) {
          return this.length > f.length
            ? this.clone().iuor(f)
            : f.clone().iuor(this);
        }),
        (i.prototype.iuand = function (f) {
          var o;
          this.length > f.length ? (o = f) : (o = this);
          for (var c = 0; c < o.length; c++)
            this.words[c] = this.words[c] & f.words[c];
          return (this.length = o.length), this._strip();
        }),
        (i.prototype.iand = function (f) {
          return r((this.negative | f.negative) === 0), this.iuand(f);
        }),
        (i.prototype.and = function (f) {
          return this.length > f.length
            ? this.clone().iand(f)
            : f.clone().iand(this);
        }),
        (i.prototype.uand = function (f) {
          return this.length > f.length
            ? this.clone().iuand(f)
            : f.clone().iuand(this);
        }),
        (i.prototype.iuxor = function (f) {
          var o, c;
          this.length > f.length
            ? ((o = this), (c = f))
            : ((o = f), (c = this));
          for (var p = 0; p < c.length; p++)
            this.words[p] = o.words[p] ^ c.words[p];
          if (this !== o) for (; p < o.length; p++) this.words[p] = o.words[p];
          return (this.length = o.length), this._strip();
        }),
        (i.prototype.ixor = function (f) {
          return r((this.negative | f.negative) === 0), this.iuxor(f);
        }),
        (i.prototype.xor = function (f) {
          return this.length > f.length
            ? this.clone().ixor(f)
            : f.clone().ixor(this);
        }),
        (i.prototype.uxor = function (f) {
          return this.length > f.length
            ? this.clone().iuxor(f)
            : f.clone().iuxor(this);
        }),
        (i.prototype.inotn = function (f) {
          r(typeof f == 'number' && f >= 0);
          var o = Math.ceil(f / 26) | 0,
            c = f % 26;
          this._expand(o), c > 0 && o--;
          for (var p = 0; p < o; p++) this.words[p] = ~this.words[p] & 67108863;
          return (
            c > 0 && (this.words[p] = ~this.words[p] & (67108863 >> (26 - c))),
            this._strip()
          );
        }),
        (i.prototype.notn = function (f) {
          return this.clone().inotn(f);
        }),
        (i.prototype.setn = function (f, o) {
          r(typeof f == 'number' && f >= 0);
          var c = (f / 26) | 0,
            p = f % 26;
          return (
            this._expand(c + 1),
            o
              ? (this.words[c] = this.words[c] | (1 << p))
              : (this.words[c] = this.words[c] & ~(1 << p)),
            this._strip()
          );
        }),
        (i.prototype.iadd = function (f) {
          var o;
          if (this.negative !== 0 && f.negative === 0)
            return (
              (this.negative = 0),
              (o = this.isub(f)),
              (this.negative ^= 1),
              this._normSign()
            );
          if (this.negative === 0 && f.negative !== 0)
            return (
              (f.negative = 0),
              (o = this.isub(f)),
              (f.negative = 1),
              o._normSign()
            );
          var c, p;
          this.length > f.length
            ? ((c = this), (p = f))
            : ((c = f), (p = this));
          for (var d = 0, u = 0; u < p.length; u++)
            (o = (c.words[u] | 0) + (p.words[u] | 0) + d),
              (this.words[u] = o & 67108863),
              (d = o >>> 26);
          for (; d !== 0 && u < c.length; u++)
            (o = (c.words[u] | 0) + d),
              (this.words[u] = o & 67108863),
              (d = o >>> 26);
          if (((this.length = c.length), d !== 0))
            (this.words[this.length] = d), this.length++;
          else if (c !== this)
            for (; u < c.length; u++) this.words[u] = c.words[u];
          return this;
        }),
        (i.prototype.add = function (f) {
          var o;
          return f.negative !== 0 && this.negative === 0
            ? ((f.negative = 0), (o = this.sub(f)), (f.negative ^= 1), o)
            : f.negative === 0 && this.negative !== 0
              ? ((this.negative = 0), (o = f.sub(this)), (this.negative = 1), o)
              : this.length > f.length
                ? this.clone().iadd(f)
                : f.clone().iadd(this);
        }),
        (i.prototype.isub = function (f) {
          if (f.negative !== 0) {
            f.negative = 0;
            var o = this.iadd(f);
            return (f.negative = 1), o._normSign();
          } else if (this.negative !== 0)
            return (
              (this.negative = 0),
              this.iadd(f),
              (this.negative = 1),
              this._normSign()
            );
          var c = this.cmp(f);
          if (c === 0)
            return (
              (this.negative = 0), (this.length = 1), (this.words[0] = 0), this
            );
          var p, d;
          c > 0 ? ((p = this), (d = f)) : ((p = f), (d = this));
          for (var u = 0, y = 0; y < d.length; y++)
            (o = (p.words[y] | 0) - (d.words[y] | 0) + u),
              (u = o >> 26),
              (this.words[y] = o & 67108863);
          for (; u !== 0 && y < p.length; y++)
            (o = (p.words[y] | 0) + u),
              (u = o >> 26),
              (this.words[y] = o & 67108863);
          if (u === 0 && y < p.length && p !== this)
            for (; y < p.length; y++) this.words[y] = p.words[y];
          return (
            (this.length = Math.max(this.length, y)),
            p !== this && (this.negative = 1),
            this._strip()
          );
        }),
        (i.prototype.sub = function (f) {
          return this.clone().isub(f);
        });
      function xe(l, f, o) {
        o.negative = f.negative ^ l.negative;
        var c = (l.length + f.length) | 0;
        (o.length = c), (c = (c - 1) | 0);
        var p = l.words[0] | 0,
          d = f.words[0] | 0,
          u = p * d,
          y = u & 67108863,
          m = (u / 67108864) | 0;
        o.words[0] = y;
        for (var s = 1; s < c; s++) {
          for (
            var w = m >>> 26,
              T = m & 67108863,
              O = Math.min(s, f.length - 1),
              P = Math.max(0, s - l.length + 1);
            P <= O;
            P++
          ) {
            var N = (s - P) | 0;
            (p = l.words[N] | 0),
              (d = f.words[P] | 0),
              (u = p * d + T),
              (w += (u / 67108864) | 0),
              (T = u & 67108863);
          }
          (o.words[s] = T | 0), (m = w | 0);
        }
        return m !== 0 ? (o.words[s] = m | 0) : o.length--, o._strip();
      }
      var U = function (f, o, c) {
        var p = f.words,
          d = o.words,
          u = c.words,
          y = 0,
          m,
          s,
          w,
          T = p[0] | 0,
          O = T & 8191,
          P = T >>> 13,
          N = p[1] | 0,
          F = N & 8191,
          j = N >>> 13,
          et = p[2] | 0,
          z = et & 8191,
          H = et >>> 13,
          ei = p[3] | 0,
          K = ei & 8191,
          V = ei >>> 13,
          ti = p[4] | 0,
          G = ti & 8191,
          W = ti >>> 13,
          ri = p[5] | 0,
          $ = ri & 8191,
          Z = ri >>> 13,
          ii = p[6] | 0,
          J = ii & 8191,
          X = ii >>> 13,
          ni = p[7] | 0,
          Y = ni & 8191,
          Q = ni >>> 13,
          fi = p[8] | 0,
          ee = fi & 8191,
          te = fi >>> 13,
          ai = p[9] | 0,
          re = ai & 8191,
          ie = ai >>> 13,
          oi = d[0] | 0,
          ne = oi & 8191,
          fe = oi >>> 13,
          si = d[1] | 0,
          ae = si & 8191,
          oe = si >>> 13,
          hi = d[2] | 0,
          se = hi & 8191,
          he = hi >>> 13,
          ui = d[3] | 0,
          ue = ui & 8191,
          ce = ui >>> 13,
          ci = d[4] | 0,
          de = ci & 8191,
          le = ci >>> 13,
          di = d[5] | 0,
          pe = di & 8191,
          ve = di >>> 13,
          li = d[6] | 0,
          be = li & 8191,
          ye = li >>> 13,
          pi = d[7] | 0,
          me = pi & 8191,
          ge = pi >>> 13,
          vi = d[8] | 0,
          we = vi & 8191,
          _e = vi >>> 13,
          Lr = d[9] | 0,
          Ae = Lr & 8191,
          Be = Lr >>> 13;
        (c.negative = f.negative ^ o.negative),
          (c.length = 19),
          (m = Math.imul(O, ne)),
          (s = Math.imul(O, fe)),
          (s = (s + Math.imul(P, ne)) | 0),
          (w = Math.imul(P, fe));
        var $t = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + ($t >>> 26)) | 0),
          ($t &= 67108863),
          (m = Math.imul(F, ne)),
          (s = Math.imul(F, fe)),
          (s = (s + Math.imul(j, ne)) | 0),
          (w = Math.imul(j, fe)),
          (m = (m + Math.imul(O, ae)) | 0),
          (s = (s + Math.imul(O, oe)) | 0),
          (s = (s + Math.imul(P, ae)) | 0),
          (w = (w + Math.imul(P, oe)) | 0);
        var Zt = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + (Zt >>> 26)) | 0),
          (Zt &= 67108863),
          (m = Math.imul(z, ne)),
          (s = Math.imul(z, fe)),
          (s = (s + Math.imul(H, ne)) | 0),
          (w = Math.imul(H, fe)),
          (m = (m + Math.imul(F, ae)) | 0),
          (s = (s + Math.imul(F, oe)) | 0),
          (s = (s + Math.imul(j, ae)) | 0),
          (w = (w + Math.imul(j, oe)) | 0),
          (m = (m + Math.imul(O, se)) | 0),
          (s = (s + Math.imul(O, he)) | 0),
          (s = (s + Math.imul(P, se)) | 0),
          (w = (w + Math.imul(P, he)) | 0);
        var Jt = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + (Jt >>> 26)) | 0),
          (Jt &= 67108863),
          (m = Math.imul(K, ne)),
          (s = Math.imul(K, fe)),
          (s = (s + Math.imul(V, ne)) | 0),
          (w = Math.imul(V, fe)),
          (m = (m + Math.imul(z, ae)) | 0),
          (s = (s + Math.imul(z, oe)) | 0),
          (s = (s + Math.imul(H, ae)) | 0),
          (w = (w + Math.imul(H, oe)) | 0),
          (m = (m + Math.imul(F, se)) | 0),
          (s = (s + Math.imul(F, he)) | 0),
          (s = (s + Math.imul(j, se)) | 0),
          (w = (w + Math.imul(j, he)) | 0),
          (m = (m + Math.imul(O, ue)) | 0),
          (s = (s + Math.imul(O, ce)) | 0),
          (s = (s + Math.imul(P, ue)) | 0),
          (w = (w + Math.imul(P, ce)) | 0);
        var Xt = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + (Xt >>> 26)) | 0),
          (Xt &= 67108863),
          (m = Math.imul(G, ne)),
          (s = Math.imul(G, fe)),
          (s = (s + Math.imul(W, ne)) | 0),
          (w = Math.imul(W, fe)),
          (m = (m + Math.imul(K, ae)) | 0),
          (s = (s + Math.imul(K, oe)) | 0),
          (s = (s + Math.imul(V, ae)) | 0),
          (w = (w + Math.imul(V, oe)) | 0),
          (m = (m + Math.imul(z, se)) | 0),
          (s = (s + Math.imul(z, he)) | 0),
          (s = (s + Math.imul(H, se)) | 0),
          (w = (w + Math.imul(H, he)) | 0),
          (m = (m + Math.imul(F, ue)) | 0),
          (s = (s + Math.imul(F, ce)) | 0),
          (s = (s + Math.imul(j, ue)) | 0),
          (w = (w + Math.imul(j, ce)) | 0),
          (m = (m + Math.imul(O, de)) | 0),
          (s = (s + Math.imul(O, le)) | 0),
          (s = (s + Math.imul(P, de)) | 0),
          (w = (w + Math.imul(P, le)) | 0);
        var Yt = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + (Yt >>> 26)) | 0),
          (Yt &= 67108863),
          (m = Math.imul($, ne)),
          (s = Math.imul($, fe)),
          (s = (s + Math.imul(Z, ne)) | 0),
          (w = Math.imul(Z, fe)),
          (m = (m + Math.imul(G, ae)) | 0),
          (s = (s + Math.imul(G, oe)) | 0),
          (s = (s + Math.imul(W, ae)) | 0),
          (w = (w + Math.imul(W, oe)) | 0),
          (m = (m + Math.imul(K, se)) | 0),
          (s = (s + Math.imul(K, he)) | 0),
          (s = (s + Math.imul(V, se)) | 0),
          (w = (w + Math.imul(V, he)) | 0),
          (m = (m + Math.imul(z, ue)) | 0),
          (s = (s + Math.imul(z, ce)) | 0),
          (s = (s + Math.imul(H, ue)) | 0),
          (w = (w + Math.imul(H, ce)) | 0),
          (m = (m + Math.imul(F, de)) | 0),
          (s = (s + Math.imul(F, le)) | 0),
          (s = (s + Math.imul(j, de)) | 0),
          (w = (w + Math.imul(j, le)) | 0),
          (m = (m + Math.imul(O, pe)) | 0),
          (s = (s + Math.imul(O, ve)) | 0),
          (s = (s + Math.imul(P, pe)) | 0),
          (w = (w + Math.imul(P, ve)) | 0);
        var Qt = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + (Qt >>> 26)) | 0),
          (Qt &= 67108863),
          (m = Math.imul(J, ne)),
          (s = Math.imul(J, fe)),
          (s = (s + Math.imul(X, ne)) | 0),
          (w = Math.imul(X, fe)),
          (m = (m + Math.imul($, ae)) | 0),
          (s = (s + Math.imul($, oe)) | 0),
          (s = (s + Math.imul(Z, ae)) | 0),
          (w = (w + Math.imul(Z, oe)) | 0),
          (m = (m + Math.imul(G, se)) | 0),
          (s = (s + Math.imul(G, he)) | 0),
          (s = (s + Math.imul(W, se)) | 0),
          (w = (w + Math.imul(W, he)) | 0),
          (m = (m + Math.imul(K, ue)) | 0),
          (s = (s + Math.imul(K, ce)) | 0),
          (s = (s + Math.imul(V, ue)) | 0),
          (w = (w + Math.imul(V, ce)) | 0),
          (m = (m + Math.imul(z, de)) | 0),
          (s = (s + Math.imul(z, le)) | 0),
          (s = (s + Math.imul(H, de)) | 0),
          (w = (w + Math.imul(H, le)) | 0),
          (m = (m + Math.imul(F, pe)) | 0),
          (s = (s + Math.imul(F, ve)) | 0),
          (s = (s + Math.imul(j, pe)) | 0),
          (w = (w + Math.imul(j, ve)) | 0),
          (m = (m + Math.imul(O, be)) | 0),
          (s = (s + Math.imul(O, ye)) | 0),
          (s = (s + Math.imul(P, be)) | 0),
          (w = (w + Math.imul(P, ye)) | 0);
        var er = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + (er >>> 26)) | 0),
          (er &= 67108863),
          (m = Math.imul(Y, ne)),
          (s = Math.imul(Y, fe)),
          (s = (s + Math.imul(Q, ne)) | 0),
          (w = Math.imul(Q, fe)),
          (m = (m + Math.imul(J, ae)) | 0),
          (s = (s + Math.imul(J, oe)) | 0),
          (s = (s + Math.imul(X, ae)) | 0),
          (w = (w + Math.imul(X, oe)) | 0),
          (m = (m + Math.imul($, se)) | 0),
          (s = (s + Math.imul($, he)) | 0),
          (s = (s + Math.imul(Z, se)) | 0),
          (w = (w + Math.imul(Z, he)) | 0),
          (m = (m + Math.imul(G, ue)) | 0),
          (s = (s + Math.imul(G, ce)) | 0),
          (s = (s + Math.imul(W, ue)) | 0),
          (w = (w + Math.imul(W, ce)) | 0),
          (m = (m + Math.imul(K, de)) | 0),
          (s = (s + Math.imul(K, le)) | 0),
          (s = (s + Math.imul(V, de)) | 0),
          (w = (w + Math.imul(V, le)) | 0),
          (m = (m + Math.imul(z, pe)) | 0),
          (s = (s + Math.imul(z, ve)) | 0),
          (s = (s + Math.imul(H, pe)) | 0),
          (w = (w + Math.imul(H, ve)) | 0),
          (m = (m + Math.imul(F, be)) | 0),
          (s = (s + Math.imul(F, ye)) | 0),
          (s = (s + Math.imul(j, be)) | 0),
          (w = (w + Math.imul(j, ye)) | 0),
          (m = (m + Math.imul(O, me)) | 0),
          (s = (s + Math.imul(O, ge)) | 0),
          (s = (s + Math.imul(P, me)) | 0),
          (w = (w + Math.imul(P, ge)) | 0);
        var tr = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + (tr >>> 26)) | 0),
          (tr &= 67108863),
          (m = Math.imul(ee, ne)),
          (s = Math.imul(ee, fe)),
          (s = (s + Math.imul(te, ne)) | 0),
          (w = Math.imul(te, fe)),
          (m = (m + Math.imul(Y, ae)) | 0),
          (s = (s + Math.imul(Y, oe)) | 0),
          (s = (s + Math.imul(Q, ae)) | 0),
          (w = (w + Math.imul(Q, oe)) | 0),
          (m = (m + Math.imul(J, se)) | 0),
          (s = (s + Math.imul(J, he)) | 0),
          (s = (s + Math.imul(X, se)) | 0),
          (w = (w + Math.imul(X, he)) | 0),
          (m = (m + Math.imul($, ue)) | 0),
          (s = (s + Math.imul($, ce)) | 0),
          (s = (s + Math.imul(Z, ue)) | 0),
          (w = (w + Math.imul(Z, ce)) | 0),
          (m = (m + Math.imul(G, de)) | 0),
          (s = (s + Math.imul(G, le)) | 0),
          (s = (s + Math.imul(W, de)) | 0),
          (w = (w + Math.imul(W, le)) | 0),
          (m = (m + Math.imul(K, pe)) | 0),
          (s = (s + Math.imul(K, ve)) | 0),
          (s = (s + Math.imul(V, pe)) | 0),
          (w = (w + Math.imul(V, ve)) | 0),
          (m = (m + Math.imul(z, be)) | 0),
          (s = (s + Math.imul(z, ye)) | 0),
          (s = (s + Math.imul(H, be)) | 0),
          (w = (w + Math.imul(H, ye)) | 0),
          (m = (m + Math.imul(F, me)) | 0),
          (s = (s + Math.imul(F, ge)) | 0),
          (s = (s + Math.imul(j, me)) | 0),
          (w = (w + Math.imul(j, ge)) | 0),
          (m = (m + Math.imul(O, we)) | 0),
          (s = (s + Math.imul(O, _e)) | 0),
          (s = (s + Math.imul(P, we)) | 0),
          (w = (w + Math.imul(P, _e)) | 0);
        var rr = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + (rr >>> 26)) | 0),
          (rr &= 67108863),
          (m = Math.imul(re, ne)),
          (s = Math.imul(re, fe)),
          (s = (s + Math.imul(ie, ne)) | 0),
          (w = Math.imul(ie, fe)),
          (m = (m + Math.imul(ee, ae)) | 0),
          (s = (s + Math.imul(ee, oe)) | 0),
          (s = (s + Math.imul(te, ae)) | 0),
          (w = (w + Math.imul(te, oe)) | 0),
          (m = (m + Math.imul(Y, se)) | 0),
          (s = (s + Math.imul(Y, he)) | 0),
          (s = (s + Math.imul(Q, se)) | 0),
          (w = (w + Math.imul(Q, he)) | 0),
          (m = (m + Math.imul(J, ue)) | 0),
          (s = (s + Math.imul(J, ce)) | 0),
          (s = (s + Math.imul(X, ue)) | 0),
          (w = (w + Math.imul(X, ce)) | 0),
          (m = (m + Math.imul($, de)) | 0),
          (s = (s + Math.imul($, le)) | 0),
          (s = (s + Math.imul(Z, de)) | 0),
          (w = (w + Math.imul(Z, le)) | 0),
          (m = (m + Math.imul(G, pe)) | 0),
          (s = (s + Math.imul(G, ve)) | 0),
          (s = (s + Math.imul(W, pe)) | 0),
          (w = (w + Math.imul(W, ve)) | 0),
          (m = (m + Math.imul(K, be)) | 0),
          (s = (s + Math.imul(K, ye)) | 0),
          (s = (s + Math.imul(V, be)) | 0),
          (w = (w + Math.imul(V, ye)) | 0),
          (m = (m + Math.imul(z, me)) | 0),
          (s = (s + Math.imul(z, ge)) | 0),
          (s = (s + Math.imul(H, me)) | 0),
          (w = (w + Math.imul(H, ge)) | 0),
          (m = (m + Math.imul(F, we)) | 0),
          (s = (s + Math.imul(F, _e)) | 0),
          (s = (s + Math.imul(j, we)) | 0),
          (w = (w + Math.imul(j, _e)) | 0),
          (m = (m + Math.imul(O, Ae)) | 0),
          (s = (s + Math.imul(O, Be)) | 0),
          (s = (s + Math.imul(P, Ae)) | 0),
          (w = (w + Math.imul(P, Be)) | 0);
        var ir = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + (ir >>> 26)) | 0),
          (ir &= 67108863),
          (m = Math.imul(re, ae)),
          (s = Math.imul(re, oe)),
          (s = (s + Math.imul(ie, ae)) | 0),
          (w = Math.imul(ie, oe)),
          (m = (m + Math.imul(ee, se)) | 0),
          (s = (s + Math.imul(ee, he)) | 0),
          (s = (s + Math.imul(te, se)) | 0),
          (w = (w + Math.imul(te, he)) | 0),
          (m = (m + Math.imul(Y, ue)) | 0),
          (s = (s + Math.imul(Y, ce)) | 0),
          (s = (s + Math.imul(Q, ue)) | 0),
          (w = (w + Math.imul(Q, ce)) | 0),
          (m = (m + Math.imul(J, de)) | 0),
          (s = (s + Math.imul(J, le)) | 0),
          (s = (s + Math.imul(X, de)) | 0),
          (w = (w + Math.imul(X, le)) | 0),
          (m = (m + Math.imul($, pe)) | 0),
          (s = (s + Math.imul($, ve)) | 0),
          (s = (s + Math.imul(Z, pe)) | 0),
          (w = (w + Math.imul(Z, ve)) | 0),
          (m = (m + Math.imul(G, be)) | 0),
          (s = (s + Math.imul(G, ye)) | 0),
          (s = (s + Math.imul(W, be)) | 0),
          (w = (w + Math.imul(W, ye)) | 0),
          (m = (m + Math.imul(K, me)) | 0),
          (s = (s + Math.imul(K, ge)) | 0),
          (s = (s + Math.imul(V, me)) | 0),
          (w = (w + Math.imul(V, ge)) | 0),
          (m = (m + Math.imul(z, we)) | 0),
          (s = (s + Math.imul(z, _e)) | 0),
          (s = (s + Math.imul(H, we)) | 0),
          (w = (w + Math.imul(H, _e)) | 0),
          (m = (m + Math.imul(F, Ae)) | 0),
          (s = (s + Math.imul(F, Be)) | 0),
          (s = (s + Math.imul(j, Ae)) | 0),
          (w = (w + Math.imul(j, Be)) | 0);
        var nr = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + (nr >>> 26)) | 0),
          (nr &= 67108863),
          (m = Math.imul(re, se)),
          (s = Math.imul(re, he)),
          (s = (s + Math.imul(ie, se)) | 0),
          (w = Math.imul(ie, he)),
          (m = (m + Math.imul(ee, ue)) | 0),
          (s = (s + Math.imul(ee, ce)) | 0),
          (s = (s + Math.imul(te, ue)) | 0),
          (w = (w + Math.imul(te, ce)) | 0),
          (m = (m + Math.imul(Y, de)) | 0),
          (s = (s + Math.imul(Y, le)) | 0),
          (s = (s + Math.imul(Q, de)) | 0),
          (w = (w + Math.imul(Q, le)) | 0),
          (m = (m + Math.imul(J, pe)) | 0),
          (s = (s + Math.imul(J, ve)) | 0),
          (s = (s + Math.imul(X, pe)) | 0),
          (w = (w + Math.imul(X, ve)) | 0),
          (m = (m + Math.imul($, be)) | 0),
          (s = (s + Math.imul($, ye)) | 0),
          (s = (s + Math.imul(Z, be)) | 0),
          (w = (w + Math.imul(Z, ye)) | 0),
          (m = (m + Math.imul(G, me)) | 0),
          (s = (s + Math.imul(G, ge)) | 0),
          (s = (s + Math.imul(W, me)) | 0),
          (w = (w + Math.imul(W, ge)) | 0),
          (m = (m + Math.imul(K, we)) | 0),
          (s = (s + Math.imul(K, _e)) | 0),
          (s = (s + Math.imul(V, we)) | 0),
          (w = (w + Math.imul(V, _e)) | 0),
          (m = (m + Math.imul(z, Ae)) | 0),
          (s = (s + Math.imul(z, Be)) | 0),
          (s = (s + Math.imul(H, Ae)) | 0),
          (w = (w + Math.imul(H, Be)) | 0);
        var fr = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + (fr >>> 26)) | 0),
          (fr &= 67108863),
          (m = Math.imul(re, ue)),
          (s = Math.imul(re, ce)),
          (s = (s + Math.imul(ie, ue)) | 0),
          (w = Math.imul(ie, ce)),
          (m = (m + Math.imul(ee, de)) | 0),
          (s = (s + Math.imul(ee, le)) | 0),
          (s = (s + Math.imul(te, de)) | 0),
          (w = (w + Math.imul(te, le)) | 0),
          (m = (m + Math.imul(Y, pe)) | 0),
          (s = (s + Math.imul(Y, ve)) | 0),
          (s = (s + Math.imul(Q, pe)) | 0),
          (w = (w + Math.imul(Q, ve)) | 0),
          (m = (m + Math.imul(J, be)) | 0),
          (s = (s + Math.imul(J, ye)) | 0),
          (s = (s + Math.imul(X, be)) | 0),
          (w = (w + Math.imul(X, ye)) | 0),
          (m = (m + Math.imul($, me)) | 0),
          (s = (s + Math.imul($, ge)) | 0),
          (s = (s + Math.imul(Z, me)) | 0),
          (w = (w + Math.imul(Z, ge)) | 0),
          (m = (m + Math.imul(G, we)) | 0),
          (s = (s + Math.imul(G, _e)) | 0),
          (s = (s + Math.imul(W, we)) | 0),
          (w = (w + Math.imul(W, _e)) | 0),
          (m = (m + Math.imul(K, Ae)) | 0),
          (s = (s + Math.imul(K, Be)) | 0),
          (s = (s + Math.imul(V, Ae)) | 0),
          (w = (w + Math.imul(V, Be)) | 0);
        var ar = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + (ar >>> 26)) | 0),
          (ar &= 67108863),
          (m = Math.imul(re, de)),
          (s = Math.imul(re, le)),
          (s = (s + Math.imul(ie, de)) | 0),
          (w = Math.imul(ie, le)),
          (m = (m + Math.imul(ee, pe)) | 0),
          (s = (s + Math.imul(ee, ve)) | 0),
          (s = (s + Math.imul(te, pe)) | 0),
          (w = (w + Math.imul(te, ve)) | 0),
          (m = (m + Math.imul(Y, be)) | 0),
          (s = (s + Math.imul(Y, ye)) | 0),
          (s = (s + Math.imul(Q, be)) | 0),
          (w = (w + Math.imul(Q, ye)) | 0),
          (m = (m + Math.imul(J, me)) | 0),
          (s = (s + Math.imul(J, ge)) | 0),
          (s = (s + Math.imul(X, me)) | 0),
          (w = (w + Math.imul(X, ge)) | 0),
          (m = (m + Math.imul($, we)) | 0),
          (s = (s + Math.imul($, _e)) | 0),
          (s = (s + Math.imul(Z, we)) | 0),
          (w = (w + Math.imul(Z, _e)) | 0),
          (m = (m + Math.imul(G, Ae)) | 0),
          (s = (s + Math.imul(G, Be)) | 0),
          (s = (s + Math.imul(W, Ae)) | 0),
          (w = (w + Math.imul(W, Be)) | 0);
        var or = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + (or >>> 26)) | 0),
          (or &= 67108863),
          (m = Math.imul(re, pe)),
          (s = Math.imul(re, ve)),
          (s = (s + Math.imul(ie, pe)) | 0),
          (w = Math.imul(ie, ve)),
          (m = (m + Math.imul(ee, be)) | 0),
          (s = (s + Math.imul(ee, ye)) | 0),
          (s = (s + Math.imul(te, be)) | 0),
          (w = (w + Math.imul(te, ye)) | 0),
          (m = (m + Math.imul(Y, me)) | 0),
          (s = (s + Math.imul(Y, ge)) | 0),
          (s = (s + Math.imul(Q, me)) | 0),
          (w = (w + Math.imul(Q, ge)) | 0),
          (m = (m + Math.imul(J, we)) | 0),
          (s = (s + Math.imul(J, _e)) | 0),
          (s = (s + Math.imul(X, we)) | 0),
          (w = (w + Math.imul(X, _e)) | 0),
          (m = (m + Math.imul($, Ae)) | 0),
          (s = (s + Math.imul($, Be)) | 0),
          (s = (s + Math.imul(Z, Ae)) | 0),
          (w = (w + Math.imul(Z, Be)) | 0);
        var sr = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + (sr >>> 26)) | 0),
          (sr &= 67108863),
          (m = Math.imul(re, be)),
          (s = Math.imul(re, ye)),
          (s = (s + Math.imul(ie, be)) | 0),
          (w = Math.imul(ie, ye)),
          (m = (m + Math.imul(ee, me)) | 0),
          (s = (s + Math.imul(ee, ge)) | 0),
          (s = (s + Math.imul(te, me)) | 0),
          (w = (w + Math.imul(te, ge)) | 0),
          (m = (m + Math.imul(Y, we)) | 0),
          (s = (s + Math.imul(Y, _e)) | 0),
          (s = (s + Math.imul(Q, we)) | 0),
          (w = (w + Math.imul(Q, _e)) | 0),
          (m = (m + Math.imul(J, Ae)) | 0),
          (s = (s + Math.imul(J, Be)) | 0),
          (s = (s + Math.imul(X, Ae)) | 0),
          (w = (w + Math.imul(X, Be)) | 0);
        var hr = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + (hr >>> 26)) | 0),
          (hr &= 67108863),
          (m = Math.imul(re, me)),
          (s = Math.imul(re, ge)),
          (s = (s + Math.imul(ie, me)) | 0),
          (w = Math.imul(ie, ge)),
          (m = (m + Math.imul(ee, we)) | 0),
          (s = (s + Math.imul(ee, _e)) | 0),
          (s = (s + Math.imul(te, we)) | 0),
          (w = (w + Math.imul(te, _e)) | 0),
          (m = (m + Math.imul(Y, Ae)) | 0),
          (s = (s + Math.imul(Y, Be)) | 0),
          (s = (s + Math.imul(Q, Ae)) | 0),
          (w = (w + Math.imul(Q, Be)) | 0);
        var Ni = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + (Ni >>> 26)) | 0),
          (Ni &= 67108863),
          (m = Math.imul(re, we)),
          (s = Math.imul(re, _e)),
          (s = (s + Math.imul(ie, we)) | 0),
          (w = Math.imul(ie, _e)),
          (m = (m + Math.imul(ee, Ae)) | 0),
          (s = (s + Math.imul(ee, Be)) | 0),
          (s = (s + Math.imul(te, Ae)) | 0),
          (w = (w + Math.imul(te, Be)) | 0);
        var Di = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + (Di >>> 26)) | 0),
          (Di &= 67108863),
          (m = Math.imul(re, Ae)),
          (s = Math.imul(re, Be)),
          (s = (s + Math.imul(ie, Ae)) | 0),
          (w = Math.imul(ie, Be));
        var Li = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        return (
          (y = (((w + (s >>> 13)) | 0) + (Li >>> 26)) | 0),
          (Li &= 67108863),
          (u[0] = $t),
          (u[1] = Zt),
          (u[2] = Jt),
          (u[3] = Xt),
          (u[4] = Yt),
          (u[5] = Qt),
          (u[6] = er),
          (u[7] = tr),
          (u[8] = rr),
          (u[9] = ir),
          (u[10] = nr),
          (u[11] = fr),
          (u[12] = ar),
          (u[13] = or),
          (u[14] = sr),
          (u[15] = hr),
          (u[16] = Ni),
          (u[17] = Di),
          (u[18] = Li),
          y !== 0 && ((u[19] = y), c.length++),
          c
        );
      };
      Math.imul || (U = xe);
      function Me(l, f, o) {
        (o.negative = f.negative ^ l.negative),
          (o.length = l.length + f.length);
        for (var c = 0, p = 0, d = 0; d < o.length - 1; d++) {
          var u = p;
          p = 0;
          for (
            var y = c & 67108863,
              m = Math.min(d, f.length - 1),
              s = Math.max(0, d - l.length + 1);
            s <= m;
            s++
          ) {
            var w = d - s,
              T = l.words[w] | 0,
              O = f.words[s] | 0,
              P = T * O,
              N = P & 67108863;
            (u = (u + ((P / 67108864) | 0)) | 0),
              (N = (N + y) | 0),
              (y = N & 67108863),
              (u = (u + (N >>> 26)) | 0),
              (p += u >>> 26),
              (u &= 67108863);
          }
          (o.words[d] = y), (c = u), (u = p);
        }
        return c !== 0 ? (o.words[d] = c) : o.length--, o._strip();
      }
      function Te(l, f, o) {
        return Me(l, f, o);
      }
      i.prototype.mulTo = function (f, o) {
        var c,
          p = this.length + f.length;
        return (
          this.length === 10 && f.length === 10
            ? (c = U(this, f, o))
            : p < 63
              ? (c = xe(this, f, o))
              : p < 1024
                ? (c = Me(this, f, o))
                : (c = Te(this, f, o)),
          c
        );
      };
      function Ee(l, f) {
        (this.x = l), (this.y = f);
      }
      (Ee.prototype.makeRBT = function (f) {
        for (
          var o = new Array(f), c = i.prototype._countBits(f) - 1, p = 0;
          p < f;
          p++
        )
          o[p] = this.revBin(p, c, f);
        return o;
      }),
        (Ee.prototype.revBin = function (f, o, c) {
          if (f === 0 || f === c - 1) return f;
          for (var p = 0, d = 0; d < o; d++)
            (p |= (f & 1) << (o - d - 1)), (f >>= 1);
          return p;
        }),
        (Ee.prototype.permute = function (f, o, c, p, d, u) {
          for (var y = 0; y < u; y++) (p[y] = o[f[y]]), (d[y] = c[f[y]]);
        }),
        (Ee.prototype.transform = function (f, o, c, p, d, u) {
          this.permute(u, f, o, c, p, d);
          for (var y = 1; y < d; y <<= 1)
            for (
              var m = y << 1,
                s = Math.cos((2 * Math.PI) / m),
                w = Math.sin((2 * Math.PI) / m),
                T = 0;
              T < d;
              T += m
            )
              for (var O = s, P = w, N = 0; N < y; N++) {
                var F = c[T + N],
                  j = p[T + N],
                  et = c[T + N + y],
                  z = p[T + N + y],
                  H = O * et - P * z;
                (z = O * z + P * et),
                  (et = H),
                  (c[T + N] = F + et),
                  (p[T + N] = j + z),
                  (c[T + N + y] = F - et),
                  (p[T + N + y] = j - z),
                  N !== m &&
                    ((H = s * O - w * P), (P = s * P + w * O), (O = H));
              }
        }),
        (Ee.prototype.guessLen13b = function (f, o) {
          var c = Math.max(o, f) | 1,
            p = c & 1,
            d = 0;
          for (c = (c / 2) | 0; c; c = c >>> 1) d++;
          return 1 << (d + 1 + p);
        }),
        (Ee.prototype.conjugate = function (f, o, c) {
          if (!(c <= 1))
            for (var p = 0; p < c / 2; p++) {
              var d = f[p];
              (f[p] = f[c - p - 1]),
                (f[c - p - 1] = d),
                (d = o[p]),
                (o[p] = -o[c - p - 1]),
                (o[c - p - 1] = -d);
            }
        }),
        (Ee.prototype.normalize13b = function (f, o) {
          for (var c = 0, p = 0; p < o / 2; p++) {
            var d =
              Math.round(f[2 * p + 1] / o) * 8192 +
              Math.round(f[2 * p] / o) +
              c;
            (f[p] = d & 67108863),
              d < 67108864 ? (c = 0) : (c = (d / 67108864) | 0);
          }
          return f;
        }),
        (Ee.prototype.convert13b = function (f, o, c, p) {
          for (var d = 0, u = 0; u < o; u++)
            (d = d + (f[u] | 0)),
              (c[2 * u] = d & 8191),
              (d = d >>> 13),
              (c[2 * u + 1] = d & 8191),
              (d = d >>> 13);
          for (u = 2 * o; u < p; ++u) c[u] = 0;
          r(d === 0), r((d & -8192) === 0);
        }),
        (Ee.prototype.stub = function (f) {
          for (var o = new Array(f), c = 0; c < f; c++) o[c] = 0;
          return o;
        }),
        (Ee.prototype.mulp = function (f, o, c) {
          var p = 2 * this.guessLen13b(f.length, o.length),
            d = this.makeRBT(p),
            u = this.stub(p),
            y = new Array(p),
            m = new Array(p),
            s = new Array(p),
            w = new Array(p),
            T = new Array(p),
            O = new Array(p),
            P = c.words;
          (P.length = p),
            this.convert13b(f.words, f.length, y, p),
            this.convert13b(o.words, o.length, w, p),
            this.transform(y, u, m, s, p, d),
            this.transform(w, u, T, O, p, d);
          for (var N = 0; N < p; N++) {
            var F = m[N] * T[N] - s[N] * O[N];
            (s[N] = m[N] * O[N] + s[N] * T[N]), (m[N] = F);
          }
          return (
            this.conjugate(m, s, p),
            this.transform(m, s, P, u, p, d),
            this.conjugate(P, u, p),
            this.normalize13b(P, p),
            (c.negative = f.negative ^ o.negative),
            (c.length = f.length + o.length),
            c._strip()
          );
        }),
        (i.prototype.mul = function (f) {
          var o = new i(null);
          return (
            (o.words = new Array(this.length + f.length)), this.mulTo(f, o)
          );
        }),
        (i.prototype.mulf = function (f) {
          var o = new i(null);
          return (o.words = new Array(this.length + f.length)), Te(this, f, o);
        }),
        (i.prototype.imul = function (f) {
          return this.clone().mulTo(f, this);
        }),
        (i.prototype.imuln = function (f) {
          var o = f < 0;
          o && (f = -f), r(typeof f == 'number'), r(f < 67108864);
          for (var c = 0, p = 0; p < this.length; p++) {
            var d = (this.words[p] | 0) * f,
              u = (d & 67108863) + (c & 67108863);
            (c >>= 26),
              (c += (d / 67108864) | 0),
              (c += u >>> 26),
              (this.words[p] = u & 67108863);
          }
          return (
            c !== 0 && ((this.words[p] = c), this.length++),
            o ? this.ineg() : this
          );
        }),
        (i.prototype.muln = function (f) {
          return this.clone().imuln(f);
        }),
        (i.prototype.sqr = function () {
          return this.mul(this);
        }),
        (i.prototype.isqr = function () {
          return this.imul(this.clone());
        }),
        (i.prototype.pow = function (f) {
          var o = L(f);
          if (o.length === 0) return new i(1);
          for (
            var c = this, p = 0;
            p < o.length && o[p] === 0;
            p++, c = c.sqr()
          );
          if (++p < o.length)
            for (var d = c.sqr(); p < o.length; p++, d = d.sqr())
              o[p] !== 0 && (c = c.mul(d));
          return c;
        }),
        (i.prototype.iushln = function (f) {
          r(typeof f == 'number' && f >= 0);
          var o = f % 26,
            c = (f - o) / 26,
            p = (67108863 >>> (26 - o)) << (26 - o),
            d;
          if (o !== 0) {
            var u = 0;
            for (d = 0; d < this.length; d++) {
              var y = this.words[d] & p,
                m = ((this.words[d] | 0) - y) << o;
              (this.words[d] = m | u), (u = y >>> (26 - o));
            }
            u && ((this.words[d] = u), this.length++);
          }
          if (c !== 0) {
            for (d = this.length - 1; d >= 0; d--)
              this.words[d + c] = this.words[d];
            for (d = 0; d < c; d++) this.words[d] = 0;
            this.length += c;
          }
          return this._strip();
        }),
        (i.prototype.ishln = function (f) {
          return r(this.negative === 0), this.iushln(f);
        }),
        (i.prototype.iushrn = function (f, o, c) {
          r(typeof f == 'number' && f >= 0);
          var p;
          o ? (p = (o - (o % 26)) / 26) : (p = 0);
          var d = f % 26,
            u = Math.min((f - d) / 26, this.length),
            y = 67108863 ^ ((67108863 >>> d) << d),
            m = c;
          if (((p -= u), (p = Math.max(0, p)), m)) {
            for (var s = 0; s < u; s++) m.words[s] = this.words[s];
            m.length = u;
          }
          if (u !== 0)
            if (this.length > u)
              for (this.length -= u, s = 0; s < this.length; s++)
                this.words[s] = this.words[s + u];
            else (this.words[0] = 0), (this.length = 1);
          var w = 0;
          for (s = this.length - 1; s >= 0 && (w !== 0 || s >= p); s--) {
            var T = this.words[s] | 0;
            (this.words[s] = (w << (26 - d)) | (T >>> d)), (w = T & y);
          }
          return (
            m && w !== 0 && (m.words[m.length++] = w),
            this.length === 0 && ((this.words[0] = 0), (this.length = 1)),
            this._strip()
          );
        }),
        (i.prototype.ishrn = function (f, o, c) {
          return r(this.negative === 0), this.iushrn(f, o, c);
        }),
        (i.prototype.shln = function (f) {
          return this.clone().ishln(f);
        }),
        (i.prototype.ushln = function (f) {
          return this.clone().iushln(f);
        }),
        (i.prototype.shrn = function (f) {
          return this.clone().ishrn(f);
        }),
        (i.prototype.ushrn = function (f) {
          return this.clone().iushrn(f);
        }),
        (i.prototype.testn = function (f) {
          r(typeof f == 'number' && f >= 0);
          var o = f % 26,
            c = (f - o) / 26,
            p = 1 << o;
          if (this.length <= c) return !1;
          var d = this.words[c];
          return !!(d & p);
        }),
        (i.prototype.imaskn = function (f) {
          r(typeof f == 'number' && f >= 0);
          var o = f % 26,
            c = (f - o) / 26;
          if (
            (r(this.negative === 0, 'imaskn works only with positive numbers'),
            this.length <= c)
          )
            return this;
          if (
            (o !== 0 && c++, (this.length = Math.min(c, this.length)), o !== 0)
          ) {
            var p = 67108863 ^ ((67108863 >>> o) << o);
            this.words[this.length - 1] &= p;
          }
          return this._strip();
        }),
        (i.prototype.maskn = function (f) {
          return this.clone().imaskn(f);
        }),
        (i.prototype.iaddn = function (f) {
          return (
            r(typeof f == 'number'),
            r(f < 67108864),
            f < 0
              ? this.isubn(-f)
              : this.negative !== 0
                ? this.length === 1 && (this.words[0] | 0) <= f
                  ? ((this.words[0] = f - (this.words[0] | 0)),
                    (this.negative = 0),
                    this)
                  : ((this.negative = 0),
                    this.isubn(f),
                    (this.negative = 1),
                    this)
                : this._iaddn(f)
          );
        }),
        (i.prototype._iaddn = function (f) {
          this.words[0] += f;
          for (var o = 0; o < this.length && this.words[o] >= 67108864; o++)
            (this.words[o] -= 67108864),
              o === this.length - 1
                ? (this.words[o + 1] = 1)
                : this.words[o + 1]++;
          return (this.length = Math.max(this.length, o + 1)), this;
        }),
        (i.prototype.isubn = function (f) {
          if ((r(typeof f == 'number'), r(f < 67108864), f < 0))
            return this.iaddn(-f);
          if (this.negative !== 0)
            return (
              (this.negative = 0), this.iaddn(f), (this.negative = 1), this
            );
          if (((this.words[0] -= f), this.length === 1 && this.words[0] < 0))
            (this.words[0] = -this.words[0]), (this.negative = 1);
          else
            for (var o = 0; o < this.length && this.words[o] < 0; o++)
              (this.words[o] += 67108864), (this.words[o + 1] -= 1);
          return this._strip();
        }),
        (i.prototype.addn = function (f) {
          return this.clone().iaddn(f);
        }),
        (i.prototype.subn = function (f) {
          return this.clone().isubn(f);
        }),
        (i.prototype.iabs = function () {
          return (this.negative = 0), this;
        }),
        (i.prototype.abs = function () {
          return this.clone().iabs();
        }),
        (i.prototype._ishlnsubmul = function (f, o, c) {
          var p = f.length + c,
            d;
          this._expand(p);
          var u,
            y = 0;
          for (d = 0; d < f.length; d++) {
            u = (this.words[d + c] | 0) + y;
            var m = (f.words[d] | 0) * o;
            (u -= m & 67108863),
              (y = (u >> 26) - ((m / 67108864) | 0)),
              (this.words[d + c] = u & 67108863);
          }
          for (; d < this.length - c; d++)
            (u = (this.words[d + c] | 0) + y),
              (y = u >> 26),
              (this.words[d + c] = u & 67108863);
          if (y === 0) return this._strip();
          for (r(y === -1), y = 0, d = 0; d < this.length; d++)
            (u = -(this.words[d] | 0) + y),
              (y = u >> 26),
              (this.words[d] = u & 67108863);
          return (this.negative = 1), this._strip();
        }),
        (i.prototype._wordDiv = function (f, o) {
          var c = this.length - f.length,
            p = this.clone(),
            d = f,
            u = d.words[d.length - 1] | 0,
            y = this._countBits(u);
          (c = 26 - y),
            c !== 0 &&
              ((d = d.ushln(c)), p.iushln(c), (u = d.words[d.length - 1] | 0));
          var m = p.length - d.length,
            s;
          if (o !== 'mod') {
            (s = new i(null)),
              (s.length = m + 1),
              (s.words = new Array(s.length));
            for (var w = 0; w < s.length; w++) s.words[w] = 0;
          }
          var T = p.clone()._ishlnsubmul(d, 1, m);
          T.negative === 0 && ((p = T), s && (s.words[m] = 1));
          for (var O = m - 1; O >= 0; O--) {
            var P =
              (p.words[d.length + O] | 0) * 67108864 +
              (p.words[d.length + O - 1] | 0);
            for (
              P = Math.min((P / u) | 0, 67108863), p._ishlnsubmul(d, P, O);
              p.negative !== 0;

            )
              P--,
                (p.negative = 0),
                p._ishlnsubmul(d, 1, O),
                p.isZero() || (p.negative ^= 1);
            s && (s.words[O] = P);
          }
          return (
            s && s._strip(),
            p._strip(),
            o !== 'div' && c !== 0 && p.iushrn(c),
            { div: s || null, mod: p }
          );
        }),
        (i.prototype.divmod = function (f, o, c) {
          if ((r(!f.isZero()), this.isZero()))
            return { div: new i(0), mod: new i(0) };
          var p, d, u;
          return this.negative !== 0 && f.negative === 0
            ? ((u = this.neg().divmod(f, o)),
              o !== 'mod' && (p = u.div.neg()),
              o !== 'div' &&
                ((d = u.mod.neg()), c && d.negative !== 0 && d.iadd(f)),
              { div: p, mod: d })
            : this.negative === 0 && f.negative !== 0
              ? ((u = this.divmod(f.neg(), o)),
                o !== 'mod' && (p = u.div.neg()),
                { div: p, mod: u.mod })
              : (this.negative & f.negative) !== 0
                ? ((u = this.neg().divmod(f.neg(), o)),
                  o !== 'div' &&
                    ((d = u.mod.neg()), c && d.negative !== 0 && d.isub(f)),
                  { div: u.div, mod: d })
                : f.length > this.length || this.cmp(f) < 0
                  ? { div: new i(0), mod: this }
                  : f.length === 1
                    ? o === 'div'
                      ? { div: this.divn(f.words[0]), mod: null }
                      : o === 'mod'
                        ? { div: null, mod: new i(this.modrn(f.words[0])) }
                        : {
                            div: this.divn(f.words[0]),
                            mod: new i(this.modrn(f.words[0])),
                          }
                    : this._wordDiv(f, o);
        }),
        (i.prototype.div = function (f) {
          return this.divmod(f, 'div', !1).div;
        }),
        (i.prototype.mod = function (f) {
          return this.divmod(f, 'mod', !1).mod;
        }),
        (i.prototype.umod = function (f) {
          return this.divmod(f, 'mod', !0).mod;
        }),
        (i.prototype.divRound = function (f) {
          var o = this.divmod(f);
          if (o.mod.isZero()) return o.div;
          var c = o.div.negative !== 0 ? o.mod.isub(f) : o.mod,
            p = f.ushrn(1),
            d = f.andln(1),
            u = c.cmp(p);
          return u < 0 || (d === 1 && u === 0)
            ? o.div
            : o.div.negative !== 0
              ? o.div.isubn(1)
              : o.div.iaddn(1);
        }),
        (i.prototype.modrn = function (f) {
          var o = f < 0;
          o && (f = -f), r(f <= 67108863);
          for (var c = (1 << 26) % f, p = 0, d = this.length - 1; d >= 0; d--)
            p = (c * p + (this.words[d] | 0)) % f;
          return o ? -p : p;
        }),
        (i.prototype.modn = function (f) {
          return this.modrn(f);
        }),
        (i.prototype.idivn = function (f) {
          var o = f < 0;
          o && (f = -f), r(f <= 67108863);
          for (var c = 0, p = this.length - 1; p >= 0; p--) {
            var d = (this.words[p] | 0) + c * 67108864;
            (this.words[p] = (d / f) | 0), (c = d % f);
          }
          return this._strip(), o ? this.ineg() : this;
        }),
        (i.prototype.divn = function (f) {
          return this.clone().idivn(f);
        }),
        (i.prototype.egcd = function (f) {
          r(f.negative === 0), r(!f.isZero());
          var o = this,
            c = f.clone();
          o.negative !== 0 ? (o = o.umod(f)) : (o = o.clone());
          for (
            var p = new i(1), d = new i(0), u = new i(0), y = new i(1), m = 0;
            o.isEven() && c.isEven();

          )
            o.iushrn(1), c.iushrn(1), ++m;
          for (var s = c.clone(), w = o.clone(); !o.isZero(); ) {
            for (
              var T = 0, O = 1;
              (o.words[0] & O) === 0 && T < 26;
              ++T, O <<= 1
            );
            if (T > 0)
              for (o.iushrn(T); T-- > 0; )
                (p.isOdd() || d.isOdd()) && (p.iadd(s), d.isub(w)),
                  p.iushrn(1),
                  d.iushrn(1);
            for (
              var P = 0, N = 1;
              (c.words[0] & N) === 0 && P < 26;
              ++P, N <<= 1
            );
            if (P > 0)
              for (c.iushrn(P); P-- > 0; )
                (u.isOdd() || y.isOdd()) && (u.iadd(s), y.isub(w)),
                  u.iushrn(1),
                  y.iushrn(1);
            o.cmp(c) >= 0
              ? (o.isub(c), p.isub(u), d.isub(y))
              : (c.isub(o), u.isub(p), y.isub(d));
          }
          return { a: u, b: y, gcd: c.iushln(m) };
        }),
        (i.prototype._invmp = function (f) {
          r(f.negative === 0), r(!f.isZero());
          var o = this,
            c = f.clone();
          o.negative !== 0 ? (o = o.umod(f)) : (o = o.clone());
          for (
            var p = new i(1), d = new i(0), u = c.clone();
            o.cmpn(1) > 0 && c.cmpn(1) > 0;

          ) {
            for (
              var y = 0, m = 1;
              (o.words[0] & m) === 0 && y < 26;
              ++y, m <<= 1
            );
            if (y > 0)
              for (o.iushrn(y); y-- > 0; ) p.isOdd() && p.iadd(u), p.iushrn(1);
            for (
              var s = 0, w = 1;
              (c.words[0] & w) === 0 && s < 26;
              ++s, w <<= 1
            );
            if (s > 0)
              for (c.iushrn(s); s-- > 0; ) d.isOdd() && d.iadd(u), d.iushrn(1);
            o.cmp(c) >= 0 ? (o.isub(c), p.isub(d)) : (c.isub(o), d.isub(p));
          }
          var T;
          return (
            o.cmpn(1) === 0 ? (T = p) : (T = d), T.cmpn(0) < 0 && T.iadd(f), T
          );
        }),
        (i.prototype.gcd = function (f) {
          if (this.isZero()) return f.abs();
          if (f.isZero()) return this.abs();
          var o = this.clone(),
            c = f.clone();
          (o.negative = 0), (c.negative = 0);
          for (var p = 0; o.isEven() && c.isEven(); p++)
            o.iushrn(1), c.iushrn(1);
          do {
            for (; o.isEven(); ) o.iushrn(1);
            for (; c.isEven(); ) c.iushrn(1);
            var d = o.cmp(c);
            if (d < 0) {
              var u = o;
              (o = c), (c = u);
            } else if (d === 0 || c.cmpn(1) === 0) break;
            o.isub(c);
          } while (!0);
          return c.iushln(p);
        }),
        (i.prototype.invm = function (f) {
          return this.egcd(f).a.umod(f);
        }),
        (i.prototype.isEven = function () {
          return (this.words[0] & 1) === 0;
        }),
        (i.prototype.isOdd = function () {
          return (this.words[0] & 1) === 1;
        }),
        (i.prototype.andln = function (f) {
          return this.words[0] & f;
        }),
        (i.prototype.bincn = function (f) {
          r(typeof f == 'number');
          var o = f % 26,
            c = (f - o) / 26,
            p = 1 << o;
          if (this.length <= c)
            return this._expand(c + 1), (this.words[c] |= p), this;
          for (var d = p, u = c; d !== 0 && u < this.length; u++) {
            var y = this.words[u] | 0;
            (y += d), (d = y >>> 26), (y &= 67108863), (this.words[u] = y);
          }
          return d !== 0 && ((this.words[u] = d), this.length++), this;
        }),
        (i.prototype.isZero = function () {
          return this.length === 1 && this.words[0] === 0;
        }),
        (i.prototype.cmpn = function (f) {
          var o = f < 0;
          if (this.negative !== 0 && !o) return -1;
          if (this.negative === 0 && o) return 1;
          this._strip();
          var c;
          if (this.length > 1) c = 1;
          else {
            o && (f = -f), r(f <= 67108863, 'Number is too big');
            var p = this.words[0] | 0;
            c = p === f ? 0 : p < f ? -1 : 1;
          }
          return this.negative !== 0 ? -c | 0 : c;
        }),
        (i.prototype.cmp = function (f) {
          if (this.negative !== 0 && f.negative === 0) return -1;
          if (this.negative === 0 && f.negative !== 0) return 1;
          var o = this.ucmp(f);
          return this.negative !== 0 ? -o | 0 : o;
        }),
        (i.prototype.ucmp = function (f) {
          if (this.length > f.length) return 1;
          if (this.length < f.length) return -1;
          for (var o = 0, c = this.length - 1; c >= 0; c--) {
            var p = this.words[c] | 0,
              d = f.words[c] | 0;
            if (p !== d) {
              p < d ? (o = -1) : p > d && (o = 1);
              break;
            }
          }
          return o;
        }),
        (i.prototype.gtn = function (f) {
          return this.cmpn(f) === 1;
        }),
        (i.prototype.gt = function (f) {
          return this.cmp(f) === 1;
        }),
        (i.prototype.gten = function (f) {
          return this.cmpn(f) >= 0;
        }),
        (i.prototype.gte = function (f) {
          return this.cmp(f) >= 0;
        }),
        (i.prototype.ltn = function (f) {
          return this.cmpn(f) === -1;
        }),
        (i.prototype.lt = function (f) {
          return this.cmp(f) === -1;
        }),
        (i.prototype.lten = function (f) {
          return this.cmpn(f) <= 0;
        }),
        (i.prototype.lte = function (f) {
          return this.cmp(f) <= 0;
        }),
        (i.prototype.eqn = function (f) {
          return this.cmpn(f) === 0;
        }),
        (i.prototype.eq = function (f) {
          return this.cmp(f) === 0;
        }),
        (i.red = function (f) {
          return new b(f);
        }),
        (i.prototype.toRed = function (f) {
          return (
            r(!this.red, 'Already a number in reduction context'),
            r(this.negative === 0, 'red works only with positives'),
            f.convertTo(this)._forceRed(f)
          );
        }),
        (i.prototype.fromRed = function () {
          return (
            r(this.red, 'fromRed works only with numbers in reduction context'),
            this.red.convertFrom(this)
          );
        }),
        (i.prototype._forceRed = function (f) {
          return (this.red = f), this;
        }),
        (i.prototype.forceRed = function (f) {
          return (
            r(!this.red, 'Already a number in reduction context'),
            this._forceRed(f)
          );
        }),
        (i.prototype.redAdd = function (f) {
          return (
            r(this.red, 'redAdd works only with red numbers'),
            this.red.add(this, f)
          );
        }),
        (i.prototype.redIAdd = function (f) {
          return (
            r(this.red, 'redIAdd works only with red numbers'),
            this.red.iadd(this, f)
          );
        }),
        (i.prototype.redSub = function (f) {
          return (
            r(this.red, 'redSub works only with red numbers'),
            this.red.sub(this, f)
          );
        }),
        (i.prototype.redISub = function (f) {
          return (
            r(this.red, 'redISub works only with red numbers'),
            this.red.isub(this, f)
          );
        }),
        (i.prototype.redShl = function (f) {
          return (
            r(this.red, 'redShl works only with red numbers'),
            this.red.shl(this, f)
          );
        }),
        (i.prototype.redMul = function (f) {
          return (
            r(this.red, 'redMul works only with red numbers'),
            this.red._verify2(this, f),
            this.red.mul(this, f)
          );
        }),
        (i.prototype.redIMul = function (f) {
          return (
            r(this.red, 'redMul works only with red numbers'),
            this.red._verify2(this, f),
            this.red.imul(this, f)
          );
        }),
        (i.prototype.redSqr = function () {
          return (
            r(this.red, 'redSqr works only with red numbers'),
            this.red._verify1(this),
            this.red.sqr(this)
          );
        }),
        (i.prototype.redISqr = function () {
          return (
            r(this.red, 'redISqr works only with red numbers'),
            this.red._verify1(this),
            this.red.isqr(this)
          );
        }),
        (i.prototype.redSqrt = function () {
          return (
            r(this.red, 'redSqrt works only with red numbers'),
            this.red._verify1(this),
            this.red.sqrt(this)
          );
        }),
        (i.prototype.redInvm = function () {
          return (
            r(this.red, 'redInvm works only with red numbers'),
            this.red._verify1(this),
            this.red.invm(this)
          );
        }),
        (i.prototype.redNeg = function () {
          return (
            r(this.red, 'redNeg works only with red numbers'),
            this.red._verify1(this),
            this.red.neg(this)
          );
        }),
        (i.prototype.redPow = function (f) {
          return (
            r(this.red && !f.red, 'redPow(normalNum)'),
            this.red._verify1(this),
            this.red.pow(this, f)
          );
        });
      var Fe = { k256: null, p224: null, p192: null, p25519: null };
      function Se(l, f) {
        (this.name = l),
          (this.p = new i(f, 16)),
          (this.n = this.p.bitLength()),
          (this.k = new i(1).iushln(this.n).isub(this.p)),
          (this.tmp = this._tmp());
      }
      (Se.prototype._tmp = function () {
        var f = new i(null);
        return (f.words = new Array(Math.ceil(this.n / 13))), f;
      }),
        (Se.prototype.ireduce = function (f) {
          var o = f,
            c;
          do
            this.split(o, this.tmp),
              (o = this.imulK(o)),
              (o = o.iadd(this.tmp)),
              (c = o.bitLength());
          while (c > this.n);
          var p = c < this.n ? -1 : o.ucmp(this.p);
          return (
            p === 0
              ? ((o.words[0] = 0), (o.length = 1))
              : p > 0
                ? o.isub(this.p)
                : o.strip !== void 0
                  ? o.strip()
                  : o._strip(),
            o
          );
        }),
        (Se.prototype.split = function (f, o) {
          f.iushrn(this.n, 0, o);
        }),
        (Se.prototype.imulK = function (f) {
          return f.imul(this.k);
        });
      function $e() {
        Se.call(
          this,
          'k256',
          'ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff fffffffe fffffc2f'
        );
      }
      n($e, Se),
        ($e.prototype.split = function (f, o) {
          for (var c = 4194303, p = Math.min(f.length, 9), d = 0; d < p; d++)
            o.words[d] = f.words[d];
          if (((o.length = p), f.length <= 9)) {
            (f.words[0] = 0), (f.length = 1);
            return;
          }
          var u = f.words[9];
          for (o.words[o.length++] = u & c, d = 10; d < f.length; d++) {
            var y = f.words[d] | 0;
            (f.words[d - 10] = ((y & c) << 4) | (u >>> 22)), (u = y);
          }
          (u >>>= 22),
            (f.words[d - 10] = u),
            u === 0 && f.length > 10 ? (f.length -= 10) : (f.length -= 9);
        }),
        ($e.prototype.imulK = function (f) {
          (f.words[f.length] = 0), (f.words[f.length + 1] = 0), (f.length += 2);
          for (var o = 0, c = 0; c < f.length; c++) {
            var p = f.words[c] | 0;
            (o += p * 977),
              (f.words[c] = o & 67108863),
              (o = p * 64 + ((o / 67108864) | 0));
          }
          return (
            f.words[f.length - 1] === 0 &&
              (f.length--, f.words[f.length - 1] === 0 && f.length--),
            f
          );
        });
      function ke() {
        Se.call(
          this,
          'p224',
          'ffffffff ffffffff ffffffff ffffffff 00000000 00000000 00000001'
        );
      }
      n(ke, Se);
      function Ze() {
        Se.call(
          this,
          'p192',
          'ffffffff ffffffff ffffffff fffffffe ffffffff ffffffff'
        );
      }
      n(Ze, Se);
      function B() {
        Se.call(
          this,
          '25519',
          '7fffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffed'
        );
      }
      n(B, Se),
        (B.prototype.imulK = function (f) {
          for (var o = 0, c = 0; c < f.length; c++) {
            var p = (f.words[c] | 0) * 19 + o,
              d = p & 67108863;
            (p >>>= 26), (f.words[c] = d), (o = p);
          }
          return o !== 0 && (f.words[f.length++] = o), f;
        }),
        (i._prime = function (f) {
          if (Fe[f]) return Fe[f];
          var o;
          if (f === 'k256') o = new $e();
          else if (f === 'p224') o = new ke();
          else if (f === 'p192') o = new Ze();
          else if (f === 'p25519') o = new B();
          else throw new Error('Unknown prime ' + f);
          return (Fe[f] = o), o;
        });
      function b(l) {
        if (typeof l == 'string') {
          var f = i._prime(l);
          (this.m = f.p), (this.prime = f);
        } else
          r(l.gtn(1), 'modulus must be greater than 1'),
            (this.m = l),
            (this.prime = null);
      }
      (b.prototype._verify1 = function (f) {
        r(f.negative === 0, 'red works only with positives'),
          r(f.red, 'red works only with red numbers');
      }),
        (b.prototype._verify2 = function (f, o) {
          r((f.negative | o.negative) === 0, 'red works only with positives'),
            r(f.red && f.red === o.red, 'red works only with red numbers');
        }),
        (b.prototype.imod = function (f) {
          return this.prime
            ? this.prime.ireduce(f)._forceRed(this)
            : (M(f, f.umod(this.m)._forceRed(this)), f);
        }),
        (b.prototype.neg = function (f) {
          return f.isZero() ? f.clone() : this.m.sub(f)._forceRed(this);
        }),
        (b.prototype.add = function (f, o) {
          this._verify2(f, o);
          var c = f.add(o);
          return c.cmp(this.m) >= 0 && c.isub(this.m), c._forceRed(this);
        }),
        (b.prototype.iadd = function (f, o) {
          this._verify2(f, o);
          var c = f.iadd(o);
          return c.cmp(this.m) >= 0 && c.isub(this.m), c;
        }),
        (b.prototype.sub = function (f, o) {
          this._verify2(f, o);
          var c = f.sub(o);
          return c.cmpn(0) < 0 && c.iadd(this.m), c._forceRed(this);
        }),
        (b.prototype.isub = function (f, o) {
          this._verify2(f, o);
          var c = f.isub(o);
          return c.cmpn(0) < 0 && c.iadd(this.m), c;
        }),
        (b.prototype.shl = function (f, o) {
          return this._verify1(f), this.imod(f.ushln(o));
        }),
        (b.prototype.imul = function (f, o) {
          return this._verify2(f, o), this.imod(f.imul(o));
        }),
        (b.prototype.mul = function (f, o) {
          return this._verify2(f, o), this.imod(f.mul(o));
        }),
        (b.prototype.isqr = function (f) {
          return this.imul(f, f.clone());
        }),
        (b.prototype.sqr = function (f) {
          return this.mul(f, f);
        }),
        (b.prototype.sqrt = function (f) {
          if (f.isZero()) return f.clone();
          var o = this.m.andln(3);
          if ((r(o % 2 === 1), o === 3)) {
            var c = this.m.add(new i(1)).iushrn(2);
            return this.pow(f, c);
          }
          for (var p = this.m.subn(1), d = 0; !p.isZero() && p.andln(1) === 0; )
            d++, p.iushrn(1);
          r(!p.isZero());
          var u = new i(1).toRed(this),
            y = u.redNeg(),
            m = this.m.subn(1).iushrn(1),
            s = this.m.bitLength();
          for (s = new i(2 * s * s).toRed(this); this.pow(s, m).cmp(y) !== 0; )
            s.redIAdd(y);
          for (
            var w = this.pow(s, p),
              T = this.pow(f, p.addn(1).iushrn(1)),
              O = this.pow(f, p),
              P = d;
            O.cmp(u) !== 0;

          ) {
            for (var N = O, F = 0; N.cmp(u) !== 0; F++) N = N.redSqr();
            r(F < P);
            var j = this.pow(w, new i(1).iushln(P - F - 1));
            (T = T.redMul(j)), (w = j.redSqr()), (O = O.redMul(w)), (P = F);
          }
          return T;
        }),
        (b.prototype.invm = function (f) {
          var o = f._invmp(this.m);
          return o.negative !== 0
            ? ((o.negative = 0), this.imod(o).redNeg())
            : this.imod(o);
        }),
        (b.prototype.pow = function (f, o) {
          if (o.isZero()) return new i(1).toRed(this);
          if (o.cmpn(1) === 0) return f.clone();
          var c = 4,
            p = new Array(1 << c);
          (p[0] = new i(1).toRed(this)), (p[1] = f);
          for (var d = 2; d < p.length; d++) p[d] = this.mul(p[d - 1], f);
          var u = p[0],
            y = 0,
            m = 0,
            s = o.bitLength() % 26;
          for (s === 0 && (s = 26), d = o.length - 1; d >= 0; d--) {
            for (var w = o.words[d], T = s - 1; T >= 0; T--) {
              var O = (w >> T) & 1;
              if ((u !== p[0] && (u = this.sqr(u)), O === 0 && y === 0)) {
                m = 0;
                continue;
              }
              (y <<= 1),
                (y |= O),
                m++,
                !(m !== c && (d !== 0 || T !== 0)) &&
                  ((u = this.mul(u, p[y])), (m = 0), (y = 0));
            }
            s = 26;
          }
          return u;
        }),
        (b.prototype.convertTo = function (f) {
          var o = f.umod(this.m);
          return o === f ? o.clone() : o;
        }),
        (b.prototype.convertFrom = function (f) {
          var o = f.clone();
          return (o.red = null), o;
        }),
        (i.mont = function (f) {
          return new _(f);
        });
      function _(l) {
        b.call(this, l),
          (this.shift = this.m.bitLength()),
          this.shift % 26 !== 0 && (this.shift += 26 - (this.shift % 26)),
          (this.r = new i(1).iushln(this.shift)),
          (this.r2 = this.imod(this.r.sqr())),
          (this.rinv = this.r._invmp(this.m)),
          (this.minv = this.rinv.mul(this.r).isubn(1).div(this.m)),
          (this.minv = this.minv.umod(this.r)),
          (this.minv = this.r.sub(this.minv));
      }
      n(_, b),
        (_.prototype.convertTo = function (f) {
          return this.imod(f.ushln(this.shift));
        }),
        (_.prototype.convertFrom = function (f) {
          var o = this.imod(f.mul(this.rinv));
          return (o.red = null), o;
        }),
        (_.prototype.imul = function (f, o) {
          if (f.isZero() || o.isZero())
            return (f.words[0] = 0), (f.length = 1), f;
          var c = f.imul(o),
            p = c
              .maskn(this.shift)
              .mul(this.minv)
              .imaskn(this.shift)
              .mul(this.m),
            d = c.isub(p).iushrn(this.shift),
            u = d;
          return (
            d.cmp(this.m) >= 0
              ? (u = d.isub(this.m))
              : d.cmpn(0) < 0 && (u = d.iadd(this.m)),
            u._forceRed(this)
          );
        }),
        (_.prototype.mul = function (f, o) {
          if (f.isZero() || o.isZero()) return new i(0)._forceRed(this);
          var c = f.mul(o),
            p = c
              .maskn(this.shift)
              .mul(this.minv)
              .imaskn(this.shift)
              .mul(this.m),
            d = c.isub(p).iushrn(this.shift),
            u = d;
          return (
            d.cmp(this.m) >= 0
              ? (u = d.isub(this.m))
              : d.cmpn(0) < 0 && (u = d.iadd(this.m)),
            u._forceRed(this)
          );
        }),
        (_.prototype.invm = function (f) {
          var o = this.imod(f._invmp(this.m).mul(this.r2));
          return o._forceRed(this);
        });
    })(typeof mu > 'u' || mu, vb);
  });
  var as = R((JT, gb) => {
    S();
    var Zn = bb(),
      Lw = Xi();
    function Fw(t) {
      var e = yb(t),
        r = e
          .toRed(Zn.mont(t.modulus))
          .redPow(new Zn(t.publicExponent))
          .fromRed();
      return { blinder: r, unblinder: e.invm(t.modulus) };
    }
    function yb(t) {
      var e = t.modulus.byteLength(),
        r;
      do r = new Zn(Lw(e));
      while (r.cmp(t.modulus) >= 0 || !r.umod(t.prime1) || !r.umod(t.prime2));
      return r;
    }
    function mb(t, e) {
      var r = Fw(e),
        n = e.modulus.byteLength(),
        i = new Zn(t).mul(r.blinder).umod(e.modulus),
        a = i.toRed(Zn.mont(e.prime1)),
        h = i.toRed(Zn.mont(e.prime2)),
        v = e.coefficient,
        g = e.prime1,
        M = e.prime2,
        x = a.redPow(e.exponent1).fromRed(),
        E = h.redPow(e.exponent2).fromRed(),
        I = x.isub(E).imul(v).umod(g).imul(M);
      return E.iadd(I)
        .imul(r.unblinder)
        .umod(e.modulus)
        .toArrayLike(Buffer, 'be', n);
    }
    mb.getr = yb;
    gb.exports = mb;
  });
  var wb = R((YT, jw) => {
    jw.exports = {
      name: 'elliptic',
      version: '6.5.4',
      description: 'EC cryptography',
      main: 'lib/elliptic.js',
      files: ['lib'],
      scripts: {
        lint: 'eslint lib test',
        'lint:fix': 'npm run lint -- --fix',
        unit: 'istanbul test _mocha --reporter=spec test/index.js',
        test: 'npm run lint && npm run unit',
        version: 'grunt dist && git add dist/',
      },
      repository: { type: 'git', url: 'git@github.com:indutny/elliptic' },
      keywords: ['EC', 'Elliptic', 'curve', 'Cryptography'],
      author: 'Fedor Indutny <fedor@indutny.com>',
      license: 'MIT',
      bugs: { url: 'https://github.com/indutny/elliptic/issues' },
      homepage: 'https://github.com/indutny/elliptic',
      devDependencies: {
        brfs: '^2.0.2',
        coveralls: '^3.1.0',
        eslint: '^7.6.0',
        grunt: '^1.2.1',
        'grunt-browserify': '^5.3.0',
        'grunt-cli': '^1.3.2',
        'grunt-contrib-connect': '^3.0.0',
        'grunt-contrib-copy': '^1.0.0',
        'grunt-contrib-uglify': '^5.0.0',
        'grunt-mocha-istanbul': '^5.0.2',
        'grunt-saucelabs': '^9.0.1',
        istanbul: '^0.4.5',
        mocha: '^8.0.1',
      },
      dependencies: {
        'bn.js': '^4.11.9',
        brorand: '^1.1.0',
        'hash.js': '^1.0.0',
        'hmac-drbg': '^1.0.1',
        inherits: '^2.0.4',
        'minimalistic-assert': '^1.0.1',
        'minimalistic-crypto-utils': '^1.0.1',
      },
    };
  });
  var gu = R((Mb) => {
    'use strict';
    S();
    var os = Mb;
    function Uw(t, e) {
      if (Array.isArray(t)) return t.slice();
      if (!t) return [];
      var r = [];
      if (typeof t != 'string') {
        for (var n = 0; n < t.length; n++) r[n] = t[n] | 0;
        return r;
      }
      if (e === 'hex') {
        (t = t.replace(/[^a-z0-9]+/gi, '')),
          t.length % 2 !== 0 && (t = '0' + t);
        for (var n = 0; n < t.length; n += 2)
          r.push(parseInt(t[n] + t[n + 1], 16));
      } else
        for (var n = 0; n < t.length; n++) {
          var i = t.charCodeAt(n),
            a = i >> 8,
            h = i & 255;
          a ? r.push(a, h) : r.push(h);
        }
      return r;
    }
    os.toArray = Uw;
    function _b(t) {
      return t.length === 1 ? '0' + t : t;
    }
    os.zero2 = _b;
    function xb(t) {
      for (var e = '', r = 0; r < t.length; r++) e += _b(t[r].toString(16));
      return e;
    }
    os.toHex = xb;
    os.encode = function (e, r) {
      return r === 'hex' ? xb(e) : e;
    };
  });
  var Bt = R((Sb) => {
    'use strict';
    S();
    var pr = Sb,
      zw = it(),
      Hw = At(),
      ss = gu();
    pr.assert = Hw;
    pr.toArray = ss.toArray;
    pr.zero2 = ss.zero2;
    pr.toHex = ss.toHex;
    pr.encode = ss.encode;
    function Kw(t, e, r) {
      var n = new Array(Math.max(t.bitLength(), r) + 1);
      n.fill(0);
      for (var i = 1 << (e + 1), a = t.clone(), h = 0; h < n.length; h++) {
        var v,
          g = a.andln(i - 1);
        a.isOdd()
          ? (g > (i >> 1) - 1 ? (v = (i >> 1) - g) : (v = g), a.isubn(v))
          : (v = 0),
          (n[h] = v),
          a.iushrn(1);
      }
      return n;
    }
    pr.getNAF = Kw;
    function Vw(t, e) {
      var r = [[], []];
      (t = t.clone()), (e = e.clone());
      for (var n = 0, i = 0, a; t.cmpn(-n) > 0 || e.cmpn(-i) > 0; ) {
        var h = (t.andln(3) + n) & 3,
          v = (e.andln(3) + i) & 3;
        h === 3 && (h = -1), v === 3 && (v = -1);
        var g;
        (h & 1) === 0
          ? (g = 0)
          : ((a = (t.andln(7) + n) & 7),
            (a === 3 || a === 5) && v === 2 ? (g = -h) : (g = h)),
          r[0].push(g);
        var M;
        (v & 1) === 0
          ? (M = 0)
          : ((a = (e.andln(7) + i) & 7),
            (a === 3 || a === 5) && h === 2 ? (M = -v) : (M = v)),
          r[1].push(M),
          2 * n === g + 1 && (n = 1 - n),
          2 * i === M + 1 && (i = 1 - i),
          t.iushrn(1),
          e.iushrn(1);
      }
      return r;
    }
    pr.getJSF = Vw;
    function Gw(t, e, r) {
      var n = '_' + e;
      t.prototype[e] = function () {
        return this[n] !== void 0 ? this[n] : (this[n] = r.call(this));
      };
    }
    pr.cachedProperty = Gw;
    function Ww(t) {
      return typeof t == 'string' ? pr.toArray(t, 'hex') : t;
    }
    pr.parseBytes = Ww;
    function $w(t) {
      return new zw(t, 'hex', 'le');
    }
    pr.intFromLE = $w;
  });
  var da = R((iP, Eb) => {
    'use strict';
    S();
    var un = it(),
      ca = Bt(),
      hs = ca.getNAF,
      Zw = ca.getJSF,
      us = ca.assert;
    function Ii(t, e) {
      (this.type = t),
        (this.p = new un(e.p, 16)),
        (this.red = e.prime ? un.red(e.prime) : un.mont(this.p)),
        (this.zero = new un(0).toRed(this.red)),
        (this.one = new un(1).toRed(this.red)),
        (this.two = new un(2).toRed(this.red)),
        (this.n = e.n && new un(e.n, 16)),
        (this.g = e.g && this.pointFromJSON(e.g, e.gRed)),
        (this._wnafT1 = new Array(4)),
        (this._wnafT2 = new Array(4)),
        (this._wnafT3 = new Array(4)),
        (this._wnafT4 = new Array(4)),
        (this._bitLength = this.n ? this.n.bitLength() : 0);
      var r = this.n && this.p.div(this.n);
      !r || r.cmpn(100) > 0
        ? (this.redN = null)
        : ((this._maxwellTrick = !0), (this.redN = this.n.toRed(this.red)));
    }
    Eb.exports = Ii;
    Ii.prototype.point = function () {
      throw new Error('Not implemented');
    };
    Ii.prototype.validate = function () {
      throw new Error('Not implemented');
    };
    Ii.prototype._fixedNafMul = function (e, r) {
      us(e.precomputed);
      var n = e._getDoubles(),
        i = hs(r, 1, this._bitLength),
        a = (1 << (n.step + 1)) - (n.step % 2 === 0 ? 2 : 1);
      a /= 3;
      var h = [],
        v,
        g;
      for (v = 0; v < i.length; v += n.step) {
        g = 0;
        for (var M = v + n.step - 1; M >= v; M--) g = (g << 1) + i[M];
        h.push(g);
      }
      for (
        var x = this.jpoint(null, null, null),
          E = this.jpoint(null, null, null),
          I = a;
        I > 0;
        I--
      ) {
        for (v = 0; v < h.length; v++)
          (g = h[v]),
            g === I
              ? (E = E.mixedAdd(n.points[v]))
              : g === -I && (E = E.mixedAdd(n.points[v].neg()));
        x = x.add(E);
      }
      return x.toP();
    };
    Ii.prototype._wnafMul = function (e, r) {
      var n = 4,
        i = e._getNAFPoints(n);
      n = i.wnd;
      for (
        var a = i.points,
          h = hs(r, n, this._bitLength),
          v = this.jpoint(null, null, null),
          g = h.length - 1;
        g >= 0;
        g--
      ) {
        for (var M = 0; g >= 0 && h[g] === 0; g--) M++;
        if ((g >= 0 && M++, (v = v.dblp(M)), g < 0)) break;
        var x = h[g];
        us(x !== 0),
          e.type === 'affine'
            ? x > 0
              ? (v = v.mixedAdd(a[(x - 1) >> 1]))
              : (v = v.mixedAdd(a[(-x - 1) >> 1].neg()))
            : x > 0
              ? (v = v.add(a[(x - 1) >> 1]))
              : (v = v.add(a[(-x - 1) >> 1].neg()));
      }
      return e.type === 'affine' ? v.toP() : v;
    };
    Ii.prototype._wnafMulAdd = function (e, r, n, i, a) {
      var h = this._wnafT1,
        v = this._wnafT2,
        g = this._wnafT3,
        M = 0,
        x,
        E,
        I;
      for (x = 0; x < i; x++) {
        I = r[x];
        var q = I._getNAFPoints(e);
        (h[x] = q.wnd), (v[x] = q.points);
      }
      for (x = i - 1; x >= 1; x -= 2) {
        var k = x - 1,
          L = x;
        if (h[k] !== 1 || h[L] !== 1) {
          (g[k] = hs(n[k], h[k], this._bitLength)),
            (g[L] = hs(n[L], h[L], this._bitLength)),
            (M = Math.max(g[k].length, M)),
            (M = Math.max(g[L].length, M));
          continue;
        }
        var xe = [r[k], null, null, r[L]];
        r[k].y.cmp(r[L].y) === 0
          ? ((xe[1] = r[k].add(r[L])),
            (xe[2] = r[k].toJ().mixedAdd(r[L].neg())))
          : r[k].y.cmp(r[L].y.redNeg()) === 0
            ? ((xe[1] = r[k].toJ().mixedAdd(r[L])),
              (xe[2] = r[k].add(r[L].neg())))
            : ((xe[1] = r[k].toJ().mixedAdd(r[L])),
              (xe[2] = r[k].toJ().mixedAdd(r[L].neg())));
        var U = [-3, -1, -5, -7, 0, 7, 5, 1, 3],
          Me = Zw(n[k], n[L]);
        for (
          M = Math.max(Me[0].length, M),
            g[k] = new Array(M),
            g[L] = new Array(M),
            E = 0;
          E < M;
          E++
        ) {
          var Te = Me[0][E] | 0,
            Ee = Me[1][E] | 0;
          (g[k][E] = U[(Te + 1) * 3 + (Ee + 1)]), (g[L][E] = 0), (v[k] = xe);
        }
      }
      var Fe = this.jpoint(null, null, null),
        Se = this._wnafT4;
      for (x = M; x >= 0; x--) {
        for (var $e = 0; x >= 0; ) {
          var ke = !0;
          for (E = 0; E < i; E++)
            (Se[E] = g[E][x] | 0), Se[E] !== 0 && (ke = !1);
          if (!ke) break;
          $e++, x--;
        }
        if ((x >= 0 && $e++, (Fe = Fe.dblp($e)), x < 0)) break;
        for (E = 0; E < i; E++) {
          var Ze = Se[E];
          Ze !== 0 &&
            (Ze > 0
              ? (I = v[E][(Ze - 1) >> 1])
              : Ze < 0 && (I = v[E][(-Ze - 1) >> 1].neg()),
            I.type === 'affine' ? (Fe = Fe.mixedAdd(I)) : (Fe = Fe.add(I)));
        }
      }
      for (x = 0; x < i; x++) v[x] = null;
      return a ? Fe : Fe.toP();
    };
    function Ft(t, e) {
      (this.curve = t), (this.type = e), (this.precomputed = null);
    }
    Ii.BasePoint = Ft;
    Ft.prototype.eq = function () {
      throw new Error('Not implemented');
    };
    Ft.prototype.validate = function () {
      return this.curve.validate(this);
    };
    Ii.prototype.decodePoint = function (e, r) {
      e = ca.toArray(e, r);
      var n = this.p.byteLength();
      if ((e[0] === 4 || e[0] === 6 || e[0] === 7) && e.length - 1 === 2 * n) {
        e[0] === 6
          ? us(e[e.length - 1] % 2 === 0)
          : e[0] === 7 && us(e[e.length - 1] % 2 === 1);
        var i = this.point(e.slice(1, 1 + n), e.slice(1 + n, 1 + 2 * n));
        return i;
      } else if ((e[0] === 2 || e[0] === 3) && e.length - 1 === n)
        return this.pointFromX(e.slice(1, 1 + n), e[0] === 3);
      throw new Error('Unknown point format');
    };
    Ft.prototype.encodeCompressed = function (e) {
      return this.encode(e, !0);
    };
    Ft.prototype._encode = function (e) {
      var r = this.curve.p.byteLength(),
        n = this.getX().toArray('be', r);
      return e
        ? [this.getY().isEven() ? 2 : 3].concat(n)
        : [4].concat(n, this.getY().toArray('be', r));
    };
    Ft.prototype.encode = function (e, r) {
      return ca.encode(this._encode(r), e);
    };
    Ft.prototype.precompute = function (e) {
      if (this.precomputed) return this;
      var r = { doubles: null, naf: null, beta: null };
      return (
        (r.naf = this._getNAFPoints(8)),
        (r.doubles = this._getDoubles(4, e)),
        (r.beta = this._getBeta()),
        (this.precomputed = r),
        this
      );
    };
    Ft.prototype._hasDoubles = function (e) {
      if (!this.precomputed) return !1;
      var r = this.precomputed.doubles;
      return r
        ? r.points.length >= Math.ceil((e.bitLength() + 1) / r.step)
        : !1;
    };
    Ft.prototype._getDoubles = function (e, r) {
      if (this.precomputed && this.precomputed.doubles)
        return this.precomputed.doubles;
      for (var n = [this], i = this, a = 0; a < r; a += e) {
        for (var h = 0; h < e; h++) i = i.dbl();
        n.push(i);
      }
      return { step: e, points: n };
    };
    Ft.prototype._getNAFPoints = function (e) {
      if (this.precomputed && this.precomputed.naf) return this.precomputed.naf;
      for (
        var r = [this],
          n = (1 << e) - 1,
          i = n === 1 ? null : this.dbl(),
          a = 1;
        a < n;
        a++
      )
        r[a] = r[a - 1].add(i);
      return { wnd: e, points: r };
    };
    Ft.prototype._getBeta = function () {
      return null;
    };
    Ft.prototype.dblp = function (e) {
      for (var r = this, n = 0; n < e; n++) r = r.dbl();
      return r;
    };
  });
  var Bb = R((fP, Ab) => {
    'use strict';
    S();
    var Jw = Bt(),
      Ye = it(),
      wu = qe(),
      Jn = da(),
      Xw = Jw.assert;
    function jt(t) {
      Jn.call(this, 'short', t),
        (this.a = new Ye(t.a, 16).toRed(this.red)),
        (this.b = new Ye(t.b, 16).toRed(this.red)),
        (this.tinv = this.two.redInvm()),
        (this.zeroA = this.a.fromRed().cmpn(0) === 0),
        (this.threeA = this.a.fromRed().sub(this.p).cmpn(-3) === 0),
        (this.endo = this._getEndomorphism(t)),
        (this._endoWnafT1 = new Array(4)),
        (this._endoWnafT2 = new Array(4));
    }
    wu(jt, Jn);
    Ab.exports = jt;
    jt.prototype._getEndomorphism = function (e) {
      if (!(!this.zeroA || !this.g || !this.n || this.p.modn(3) !== 1)) {
        var r, n;
        if (e.beta) r = new Ye(e.beta, 16).toRed(this.red);
        else {
          var i = this._getEndoRoots(this.p);
          (r = i[0].cmp(i[1]) < 0 ? i[0] : i[1]), (r = r.toRed(this.red));
        }
        if (e.lambda) n = new Ye(e.lambda, 16);
        else {
          var a = this._getEndoRoots(this.n);
          this.g.mul(a[0]).x.cmp(this.g.x.redMul(r)) === 0
            ? (n = a[0])
            : ((n = a[1]), Xw(this.g.mul(n).x.cmp(this.g.x.redMul(r)) === 0));
        }
        var h;
        return (
          e.basis
            ? (h = e.basis.map(function (v) {
                return { a: new Ye(v.a, 16), b: new Ye(v.b, 16) };
              }))
            : (h = this._getEndoBasis(n)),
          { beta: r, lambda: n, basis: h }
        );
      }
    };
    jt.prototype._getEndoRoots = function (e) {
      var r = e === this.p ? this.red : Ye.mont(e),
        n = new Ye(2).toRed(r).redInvm(),
        i = n.redNeg(),
        a = new Ye(3).toRed(r).redNeg().redSqrt().redMul(n),
        h = i.redAdd(a).fromRed(),
        v = i.redSub(a).fromRed();
      return [h, v];
    };
    jt.prototype._getEndoBasis = function (e) {
      for (
        var r = this.n.ushrn(Math.floor(this.n.bitLength() / 2)),
          n = e,
          i = this.n.clone(),
          a = new Ye(1),
          h = new Ye(0),
          v = new Ye(0),
          g = new Ye(1),
          M,
          x,
          E,
          I,
          q,
          k,
          L,
          xe = 0,
          U,
          Me;
        n.cmpn(0) !== 0;

      ) {
        var Te = i.div(n);
        (U = i.sub(Te.mul(n))), (Me = v.sub(Te.mul(a)));
        var Ee = g.sub(Te.mul(h));
        if (!E && U.cmp(r) < 0) (M = L.neg()), (x = a), (E = U.neg()), (I = Me);
        else if (E && ++xe === 2) break;
        (L = U), (i = n), (n = U), (v = a), (a = Me), (g = h), (h = Ee);
      }
      (q = U.neg()), (k = Me);
      var Fe = E.sqr().add(I.sqr()),
        Se = q.sqr().add(k.sqr());
      return (
        Se.cmp(Fe) >= 0 && ((q = M), (k = x)),
        E.negative && ((E = E.neg()), (I = I.neg())),
        q.negative && ((q = q.neg()), (k = k.neg())),
        [
          { a: E, b: I },
          { a: q, b: k },
        ]
      );
    };
    jt.prototype._endoSplit = function (e) {
      var r = this.endo.basis,
        n = r[0],
        i = r[1],
        a = i.b.mul(e).divRound(this.n),
        h = n.b.neg().mul(e).divRound(this.n),
        v = a.mul(n.a),
        g = h.mul(i.a),
        M = a.mul(n.b),
        x = h.mul(i.b),
        E = e.sub(v).sub(g),
        I = M.add(x).neg();
      return { k1: E, k2: I };
    };
    jt.prototype.pointFromX = function (e, r) {
      (e = new Ye(e, 16)), e.red || (e = e.toRed(this.red));
      var n = e.redSqr().redMul(e).redIAdd(e.redMul(this.a)).redIAdd(this.b),
        i = n.redSqrt();
      if (i.redSqr().redSub(n).cmp(this.zero) !== 0)
        throw new Error('invalid point');
      var a = i.fromRed().isOdd();
      return ((r && !a) || (!r && a)) && (i = i.redNeg()), this.point(e, i);
    };
    jt.prototype.validate = function (e) {
      if (e.inf) return !0;
      var r = e.x,
        n = e.y,
        i = this.a.redMul(r),
        a = r.redSqr().redMul(r).redIAdd(i).redIAdd(this.b);
      return n.redSqr().redISub(a).cmpn(0) === 0;
    };
    jt.prototype._endoWnafMulAdd = function (e, r, n) {
      for (
        var i = this._endoWnafT1, a = this._endoWnafT2, h = 0;
        h < e.length;
        h++
      ) {
        var v = this._endoSplit(r[h]),
          g = e[h],
          M = g._getBeta();
        v.k1.negative && (v.k1.ineg(), (g = g.neg(!0))),
          v.k2.negative && (v.k2.ineg(), (M = M.neg(!0))),
          (i[h * 2] = g),
          (i[h * 2 + 1] = M),
          (a[h * 2] = v.k1),
          (a[h * 2 + 1] = v.k2);
      }
      for (var x = this._wnafMulAdd(1, i, a, h * 2, n), E = 0; E < h * 2; E++)
        (i[E] = null), (a[E] = null);
      return x;
    };
    function at(t, e, r, n) {
      Jn.BasePoint.call(this, t, 'affine'),
        e === null && r === null
          ? ((this.x = null), (this.y = null), (this.inf = !0))
          : ((this.x = new Ye(e, 16)),
            (this.y = new Ye(r, 16)),
            n &&
              (this.x.forceRed(this.curve.red),
              this.y.forceRed(this.curve.red)),
            this.x.red || (this.x = this.x.toRed(this.curve.red)),
            this.y.red || (this.y = this.y.toRed(this.curve.red)),
            (this.inf = !1));
    }
    wu(at, Jn.BasePoint);
    jt.prototype.point = function (e, r, n) {
      return new at(this, e, r, n);
    };
    jt.prototype.pointFromJSON = function (e, r) {
      return at.fromJSON(this, e, r);
    };
    at.prototype._getBeta = function () {
      if (!!this.curve.endo) {
        var e = this.precomputed;
        if (e && e.beta) return e.beta;
        var r = this.curve.point(this.x.redMul(this.curve.endo.beta), this.y);
        if (e) {
          var n = this.curve,
            i = function (a) {
              return n.point(a.x.redMul(n.endo.beta), a.y);
            };
          (e.beta = r),
            (r.precomputed = {
              beta: null,
              naf: e.naf && { wnd: e.naf.wnd, points: e.naf.points.map(i) },
              doubles: e.doubles && {
                step: e.doubles.step,
                points: e.doubles.points.map(i),
              },
            });
        }
        return r;
      }
    };
    at.prototype.toJSON = function () {
      return this.precomputed
        ? [
            this.x,
            this.y,
            this.precomputed && {
              doubles: this.precomputed.doubles && {
                step: this.precomputed.doubles.step,
                points: this.precomputed.doubles.points.slice(1),
              },
              naf: this.precomputed.naf && {
                wnd: this.precomputed.naf.wnd,
                points: this.precomputed.naf.points.slice(1),
              },
            },
          ]
        : [this.x, this.y];
    };
    at.fromJSON = function (e, r, n) {
      typeof r == 'string' && (r = JSON.parse(r));
      var i = e.point(r[0], r[1], n);
      if (!r[2]) return i;
      function a(v) {
        return e.point(v[0], v[1], n);
      }
      var h = r[2];
      return (
        (i.precomputed = {
          beta: null,
          doubles: h.doubles && {
            step: h.doubles.step,
            points: [i].concat(h.doubles.points.map(a)),
          },
          naf: h.naf && {
            wnd: h.naf.wnd,
            points: [i].concat(h.naf.points.map(a)),
          },
        }),
        i
      );
    };
    at.prototype.inspect = function () {
      return this.isInfinity()
        ? '<EC Point Infinity>'
        : '<EC Point x: ' +
            this.x.fromRed().toString(16, 2) +
            ' y: ' +
            this.y.fromRed().toString(16, 2) +
            '>';
    };
    at.prototype.isInfinity = function () {
      return this.inf;
    };
    at.prototype.add = function (e) {
      if (this.inf) return e;
      if (e.inf) return this;
      if (this.eq(e)) return this.dbl();
      if (this.neg().eq(e)) return this.curve.point(null, null);
      if (this.x.cmp(e.x) === 0) return this.curve.point(null, null);
      var r = this.y.redSub(e.y);
      r.cmpn(0) !== 0 && (r = r.redMul(this.x.redSub(e.x).redInvm()));
      var n = r.redSqr().redISub(this.x).redISub(e.x),
        i = r.redMul(this.x.redSub(n)).redISub(this.y);
      return this.curve.point(n, i);
    };
    at.prototype.dbl = function () {
      if (this.inf) return this;
      var e = this.y.redAdd(this.y);
      if (e.cmpn(0) === 0) return this.curve.point(null, null);
      var r = this.curve.a,
        n = this.x.redSqr(),
        i = e.redInvm(),
        a = n.redAdd(n).redIAdd(n).redIAdd(r).redMul(i),
        h = a.redSqr().redISub(this.x.redAdd(this.x)),
        v = a.redMul(this.x.redSub(h)).redISub(this.y);
      return this.curve.point(h, v);
    };
    at.prototype.getX = function () {
      return this.x.fromRed();
    };
    at.prototype.getY = function () {
      return this.y.fromRed();
    };
    at.prototype.mul = function (e) {
      return (
        (e = new Ye(e, 16)),
        this.isInfinity()
          ? this
          : this._hasDoubles(e)
            ? this.curve._fixedNafMul(this, e)
            : this.curve.endo
              ? this.curve._endoWnafMulAdd([this], [e])
              : this.curve._wnafMul(this, e)
      );
    };
    at.prototype.mulAdd = function (e, r, n) {
      var i = [this, r],
        a = [e, n];
      return this.curve.endo
        ? this.curve._endoWnafMulAdd(i, a)
        : this.curve._wnafMulAdd(1, i, a, 2);
    };
    at.prototype.jmulAdd = function (e, r, n) {
      var i = [this, r],
        a = [e, n];
      return this.curve.endo
        ? this.curve._endoWnafMulAdd(i, a, !0)
        : this.curve._wnafMulAdd(1, i, a, 2, !0);
    };
    at.prototype.eq = function (e) {
      return (
        this === e ||
        (this.inf === e.inf &&
          (this.inf || (this.x.cmp(e.x) === 0 && this.y.cmp(e.y) === 0)))
      );
    };
    at.prototype.neg = function (e) {
      if (this.inf) return this;
      var r = this.curve.point(this.x, this.y.redNeg());
      if (e && this.precomputed) {
        var n = this.precomputed,
          i = function (a) {
            return a.neg();
          };
        r.precomputed = {
          naf: n.naf && { wnd: n.naf.wnd, points: n.naf.points.map(i) },
          doubles: n.doubles && {
            step: n.doubles.step,
            points: n.doubles.points.map(i),
          },
        };
      }
      return r;
    };
    at.prototype.toJ = function () {
      if (this.inf) return this.curve.jpoint(null, null, null);
      var e = this.curve.jpoint(this.x, this.y, this.curve.one);
      return e;
    };
    function ut(t, e, r, n) {
      Jn.BasePoint.call(this, t, 'jacobian'),
        e === null && r === null && n === null
          ? ((this.x = this.curve.one),
            (this.y = this.curve.one),
            (this.z = new Ye(0)))
          : ((this.x = new Ye(e, 16)),
            (this.y = new Ye(r, 16)),
            (this.z = new Ye(n, 16))),
        this.x.red || (this.x = this.x.toRed(this.curve.red)),
        this.y.red || (this.y = this.y.toRed(this.curve.red)),
        this.z.red || (this.z = this.z.toRed(this.curve.red)),
        (this.zOne = this.z === this.curve.one);
    }
    wu(ut, Jn.BasePoint);
    jt.prototype.jpoint = function (e, r, n) {
      return new ut(this, e, r, n);
    };
    ut.prototype.toP = function () {
      if (this.isInfinity()) return this.curve.point(null, null);
      var e = this.z.redInvm(),
        r = e.redSqr(),
        n = this.x.redMul(r),
        i = this.y.redMul(r).redMul(e);
      return this.curve.point(n, i);
    };
    ut.prototype.neg = function () {
      return this.curve.jpoint(this.x, this.y.redNeg(), this.z);
    };
    ut.prototype.add = function (e) {
      if (this.isInfinity()) return e;
      if (e.isInfinity()) return this;
      var r = e.z.redSqr(),
        n = this.z.redSqr(),
        i = this.x.redMul(r),
        a = e.x.redMul(n),
        h = this.y.redMul(r.redMul(e.z)),
        v = e.y.redMul(n.redMul(this.z)),
        g = i.redSub(a),
        M = h.redSub(v);
      if (g.cmpn(0) === 0)
        return M.cmpn(0) !== 0
          ? this.curve.jpoint(null, null, null)
          : this.dbl();
      var x = g.redSqr(),
        E = x.redMul(g),
        I = i.redMul(x),
        q = M.redSqr().redIAdd(E).redISub(I).redISub(I),
        k = M.redMul(I.redISub(q)).redISub(h.redMul(E)),
        L = this.z.redMul(e.z).redMul(g);
      return this.curve.jpoint(q, k, L);
    };
    ut.prototype.mixedAdd = function (e) {
      if (this.isInfinity()) return e.toJ();
      if (e.isInfinity()) return this;
      var r = this.z.redSqr(),
        n = this.x,
        i = e.x.redMul(r),
        a = this.y,
        h = e.y.redMul(r).redMul(this.z),
        v = n.redSub(i),
        g = a.redSub(h);
      if (v.cmpn(0) === 0)
        return g.cmpn(0) !== 0
          ? this.curve.jpoint(null, null, null)
          : this.dbl();
      var M = v.redSqr(),
        x = M.redMul(v),
        E = n.redMul(M),
        I = g.redSqr().redIAdd(x).redISub(E).redISub(E),
        q = g.redMul(E.redISub(I)).redISub(a.redMul(x)),
        k = this.z.redMul(v);
      return this.curve.jpoint(I, q, k);
    };
    ut.prototype.dblp = function (e) {
      if (e === 0) return this;
      if (this.isInfinity()) return this;
      if (!e) return this.dbl();
      var r;
      if (this.curve.zeroA || this.curve.threeA) {
        var n = this;
        for (r = 0; r < e; r++) n = n.dbl();
        return n;
      }
      var i = this.curve.a,
        a = this.curve.tinv,
        h = this.x,
        v = this.y,
        g = this.z,
        M = g.redSqr().redSqr(),
        x = v.redAdd(v);
      for (r = 0; r < e; r++) {
        var E = h.redSqr(),
          I = x.redSqr(),
          q = I.redSqr(),
          k = E.redAdd(E).redIAdd(E).redIAdd(i.redMul(M)),
          L = h.redMul(I),
          xe = k.redSqr().redISub(L.redAdd(L)),
          U = L.redISub(xe),
          Me = k.redMul(U);
        Me = Me.redIAdd(Me).redISub(q);
        var Te = x.redMul(g);
        r + 1 < e && (M = M.redMul(q)), (h = xe), (g = Te), (x = Me);
      }
      return this.curve.jpoint(h, x.redMul(a), g);
    };
    ut.prototype.dbl = function () {
      return this.isInfinity()
        ? this
        : this.curve.zeroA
          ? this._zeroDbl()
          : this.curve.threeA
            ? this._threeDbl()
            : this._dbl();
    };
    ut.prototype._zeroDbl = function () {
      var e, r, n;
      if (this.zOne) {
        var i = this.x.redSqr(),
          a = this.y.redSqr(),
          h = a.redSqr(),
          v = this.x.redAdd(a).redSqr().redISub(i).redISub(h);
        v = v.redIAdd(v);
        var g = i.redAdd(i).redIAdd(i),
          M = g.redSqr().redISub(v).redISub(v),
          x = h.redIAdd(h);
        (x = x.redIAdd(x)),
          (x = x.redIAdd(x)),
          (e = M),
          (r = g.redMul(v.redISub(M)).redISub(x)),
          (n = this.y.redAdd(this.y));
      } else {
        var E = this.x.redSqr(),
          I = this.y.redSqr(),
          q = I.redSqr(),
          k = this.x.redAdd(I).redSqr().redISub(E).redISub(q);
        k = k.redIAdd(k);
        var L = E.redAdd(E).redIAdd(E),
          xe = L.redSqr(),
          U = q.redIAdd(q);
        (U = U.redIAdd(U)),
          (U = U.redIAdd(U)),
          (e = xe.redISub(k).redISub(k)),
          (r = L.redMul(k.redISub(e)).redISub(U)),
          (n = this.y.redMul(this.z)),
          (n = n.redIAdd(n));
      }
      return this.curve.jpoint(e, r, n);
    };
    ut.prototype._threeDbl = function () {
      var e, r, n;
      if (this.zOne) {
        var i = this.x.redSqr(),
          a = this.y.redSqr(),
          h = a.redSqr(),
          v = this.x.redAdd(a).redSqr().redISub(i).redISub(h);
        v = v.redIAdd(v);
        var g = i.redAdd(i).redIAdd(i).redIAdd(this.curve.a),
          M = g.redSqr().redISub(v).redISub(v);
        e = M;
        var x = h.redIAdd(h);
        (x = x.redIAdd(x)),
          (x = x.redIAdd(x)),
          (r = g.redMul(v.redISub(M)).redISub(x)),
          (n = this.y.redAdd(this.y));
      } else {
        var E = this.z.redSqr(),
          I = this.y.redSqr(),
          q = this.x.redMul(I),
          k = this.x.redSub(E).redMul(this.x.redAdd(E));
        k = k.redAdd(k).redIAdd(k);
        var L = q.redIAdd(q);
        L = L.redIAdd(L);
        var xe = L.redAdd(L);
        (e = k.redSqr().redISub(xe)),
          (n = this.y.redAdd(this.z).redSqr().redISub(I).redISub(E));
        var U = I.redSqr();
        (U = U.redIAdd(U)),
          (U = U.redIAdd(U)),
          (U = U.redIAdd(U)),
          (r = k.redMul(L.redISub(e)).redISub(U));
      }
      return this.curve.jpoint(e, r, n);
    };
    ut.prototype._dbl = function () {
      var e = this.curve.a,
        r = this.x,
        n = this.y,
        i = this.z,
        a = i.redSqr().redSqr(),
        h = r.redSqr(),
        v = n.redSqr(),
        g = h.redAdd(h).redIAdd(h).redIAdd(e.redMul(a)),
        M = r.redAdd(r);
      M = M.redIAdd(M);
      var x = M.redMul(v),
        E = g.redSqr().redISub(x.redAdd(x)),
        I = x.redISub(E),
        q = v.redSqr();
      (q = q.redIAdd(q)), (q = q.redIAdd(q)), (q = q.redIAdd(q));
      var k = g.redMul(I).redISub(q),
        L = n.redAdd(n).redMul(i);
      return this.curve.jpoint(E, k, L);
    };
    ut.prototype.trpl = function () {
      if (!this.curve.zeroA) return this.dbl().add(this);
      var e = this.x.redSqr(),
        r = this.y.redSqr(),
        n = this.z.redSqr(),
        i = r.redSqr(),
        a = e.redAdd(e).redIAdd(e),
        h = a.redSqr(),
        v = this.x.redAdd(r).redSqr().redISub(e).redISub(i);
      (v = v.redIAdd(v)), (v = v.redAdd(v).redIAdd(v)), (v = v.redISub(h));
      var g = v.redSqr(),
        M = i.redIAdd(i);
      (M = M.redIAdd(M)), (M = M.redIAdd(M)), (M = M.redIAdd(M));
      var x = a.redIAdd(v).redSqr().redISub(h).redISub(g).redISub(M),
        E = r.redMul(x);
      (E = E.redIAdd(E)), (E = E.redIAdd(E));
      var I = this.x.redMul(g).redISub(E);
      (I = I.redIAdd(I)), (I = I.redIAdd(I));
      var q = this.y.redMul(x.redMul(M.redISub(x)).redISub(v.redMul(g)));
      (q = q.redIAdd(q)), (q = q.redIAdd(q)), (q = q.redIAdd(q));
      var k = this.z.redAdd(v).redSqr().redISub(n).redISub(g);
      return this.curve.jpoint(I, q, k);
    };
    ut.prototype.mul = function (e, r) {
      return (e = new Ye(e, r)), this.curve._wnafMul(this, e);
    };
    ut.prototype.eq = function (e) {
      if (e.type === 'affine') return this.eq(e.toJ());
      if (this === e) return !0;
      var r = this.z.redSqr(),
        n = e.z.redSqr();
      if (this.x.redMul(n).redISub(e.x.redMul(r)).cmpn(0) !== 0) return !1;
      var i = r.redMul(this.z),
        a = n.redMul(e.z);
      return this.y.redMul(a).redISub(e.y.redMul(i)).cmpn(0) === 0;
    };
    ut.prototype.eqXToP = function (e) {
      var r = this.z.redSqr(),
        n = e.toRed(this.curve.red).redMul(r);
      if (this.x.cmp(n) === 0) return !0;
      for (var i = e.clone(), a = this.curve.redN.redMul(r); ; ) {
        if ((i.iadd(this.curve.n), i.cmp(this.curve.p) >= 0)) return !1;
        if ((n.redIAdd(a), this.x.cmp(n) === 0)) return !0;
      }
    };
    ut.prototype.inspect = function () {
      return this.isInfinity()
        ? '<EC JPoint Infinity>'
        : '<EC JPoint x: ' +
            this.x.toString(16, 2) +
            ' y: ' +
            this.y.toString(16, 2) +
            ' z: ' +
            this.z.toString(16, 2) +
            '>';
    };
    ut.prototype.isInfinity = function () {
      return this.z.cmpn(0) === 0;
    };
  });
  var qb = R((oP, Rb) => {
    'use strict';
    S();
    var Xn = it(),
      Ib = qe(),
      cs = da(),
      Yw = Bt();
    function Yn(t) {
      cs.call(this, 'mont', t),
        (this.a = new Xn(t.a, 16).toRed(this.red)),
        (this.b = new Xn(t.b, 16).toRed(this.red)),
        (this.i4 = new Xn(4).toRed(this.red).redInvm()),
        (this.two = new Xn(2).toRed(this.red)),
        (this.a24 = this.i4.redMul(this.a.redAdd(this.two)));
    }
    Ib(Yn, cs);
    Rb.exports = Yn;
    Yn.prototype.validate = function (e) {
      var r = e.normalize().x,
        n = r.redSqr(),
        i = n.redMul(r).redAdd(n.redMul(this.a)).redAdd(r),
        a = i.redSqrt();
      return a.redSqr().cmp(i) === 0;
    };
    function ot(t, e, r) {
      cs.BasePoint.call(this, t, 'projective'),
        e === null && r === null
          ? ((this.x = this.curve.one), (this.z = this.curve.zero))
          : ((this.x = new Xn(e, 16)),
            (this.z = new Xn(r, 16)),
            this.x.red || (this.x = this.x.toRed(this.curve.red)),
            this.z.red || (this.z = this.z.toRed(this.curve.red)));
    }
    Ib(ot, cs.BasePoint);
    Yn.prototype.decodePoint = function (e, r) {
      return this.point(Yw.toArray(e, r), 1);
    };
    Yn.prototype.point = function (e, r) {
      return new ot(this, e, r);
    };
    Yn.prototype.pointFromJSON = function (e) {
      return ot.fromJSON(this, e);
    };
    ot.prototype.precompute = function () {};
    ot.prototype._encode = function () {
      return this.getX().toArray('be', this.curve.p.byteLength());
    };
    ot.fromJSON = function (e, r) {
      return new ot(e, r[0], r[1] || e.one);
    };
    ot.prototype.inspect = function () {
      return this.isInfinity()
        ? '<EC Point Infinity>'
        : '<EC Point x: ' +
            this.x.fromRed().toString(16, 2) +
            ' z: ' +
            this.z.fromRed().toString(16, 2) +
            '>';
    };
    ot.prototype.isInfinity = function () {
      return this.z.cmpn(0) === 0;
    };
    ot.prototype.dbl = function () {
      var e = this.x.redAdd(this.z),
        r = e.redSqr(),
        n = this.x.redSub(this.z),
        i = n.redSqr(),
        a = r.redSub(i),
        h = r.redMul(i),
        v = a.redMul(i.redAdd(this.curve.a24.redMul(a)));
      return this.curve.point(h, v);
    };
    ot.prototype.add = function () {
      throw new Error('Not supported on Montgomery curve');
    };
    ot.prototype.diffAdd = function (e, r) {
      var n = this.x.redAdd(this.z),
        i = this.x.redSub(this.z),
        a = e.x.redAdd(e.z),
        h = e.x.redSub(e.z),
        v = h.redMul(n),
        g = a.redMul(i),
        M = r.z.redMul(v.redAdd(g).redSqr()),
        x = r.x.redMul(v.redISub(g).redSqr());
      return this.curve.point(M, x);
    };
    ot.prototype.mul = function (e) {
      for (
        var r = e.clone(),
          n = this,
          i = this.curve.point(null, null),
          a = this,
          h = [];
        r.cmpn(0) !== 0;
        r.iushrn(1)
      )
        h.push(r.andln(1));
      for (var v = h.length - 1; v >= 0; v--)
        h[v] === 0
          ? ((n = n.diffAdd(i, a)), (i = i.dbl()))
          : ((i = n.diffAdd(i, a)), (n = n.dbl()));
      return i;
    };
    ot.prototype.mulAdd = function () {
      throw new Error('Not supported on Montgomery curve');
    };
    ot.prototype.jumlAdd = function () {
      throw new Error('Not supported on Montgomery curve');
    };
    ot.prototype.eq = function (e) {
      return this.getX().cmp(e.getX()) === 0;
    };
    ot.prototype.normalize = function () {
      return (
        (this.x = this.x.redMul(this.z.redInvm())),
        (this.z = this.curve.one),
        this
      );
    };
    ot.prototype.getX = function () {
      return this.normalize(), this.x.fromRed();
    };
  });
  var kb = R((hP, Pb) => {
    'use strict';
    S();
    var Qw = Bt(),
      $r = it(),
      Tb = qe(),
      ds = da(),
      e_ = Qw.assert;
    function Rr(t) {
      (this.twisted = (t.a | 0) !== 1),
        (this.mOneA = this.twisted && (t.a | 0) === -1),
        (this.extended = this.mOneA),
        ds.call(this, 'edwards', t),
        (this.a = new $r(t.a, 16).umod(this.red.m)),
        (this.a = this.a.toRed(this.red)),
        (this.c = new $r(t.c, 16).toRed(this.red)),
        (this.c2 = this.c.redSqr()),
        (this.d = new $r(t.d, 16).toRed(this.red)),
        (this.dd = this.d.redAdd(this.d)),
        e_(!this.twisted || this.c.fromRed().cmpn(1) === 0),
        (this.oneC = (t.c | 0) === 1);
    }
    Tb(Rr, ds);
    Pb.exports = Rr;
    Rr.prototype._mulA = function (e) {
      return this.mOneA ? e.redNeg() : this.a.redMul(e);
    };
    Rr.prototype._mulC = function (e) {
      return this.oneC ? e : this.c.redMul(e);
    };
    Rr.prototype.jpoint = function (e, r, n, i) {
      return this.point(e, r, n, i);
    };
    Rr.prototype.pointFromX = function (e, r) {
      (e = new $r(e, 16)), e.red || (e = e.toRed(this.red));
      var n = e.redSqr(),
        i = this.c2.redSub(this.a.redMul(n)),
        a = this.one.redSub(this.c2.redMul(this.d).redMul(n)),
        h = i.redMul(a.redInvm()),
        v = h.redSqrt();
      if (v.redSqr().redSub(h).cmp(this.zero) !== 0)
        throw new Error('invalid point');
      var g = v.fromRed().isOdd();
      return ((r && !g) || (!r && g)) && (v = v.redNeg()), this.point(e, v);
    };
    Rr.prototype.pointFromY = function (e, r) {
      (e = new $r(e, 16)), e.red || (e = e.toRed(this.red));
      var n = e.redSqr(),
        i = n.redSub(this.c2),
        a = n.redMul(this.d).redMul(this.c2).redSub(this.a),
        h = i.redMul(a.redInvm());
      if (h.cmp(this.zero) === 0) {
        if (r) throw new Error('invalid point');
        return this.point(this.zero, e);
      }
      var v = h.redSqrt();
      if (v.redSqr().redSub(h).cmp(this.zero) !== 0)
        throw new Error('invalid point');
      return v.fromRed().isOdd() !== r && (v = v.redNeg()), this.point(v, e);
    };
    Rr.prototype.validate = function (e) {
      if (e.isInfinity()) return !0;
      e.normalize();
      var r = e.x.redSqr(),
        n = e.y.redSqr(),
        i = r.redMul(this.a).redAdd(n),
        a = this.c2.redMul(this.one.redAdd(this.d.redMul(r).redMul(n)));
      return i.cmp(a) === 0;
    };
    function We(t, e, r, n, i) {
      ds.BasePoint.call(this, t, 'projective'),
        e === null && r === null && n === null
          ? ((this.x = this.curve.zero),
            (this.y = this.curve.one),
            (this.z = this.curve.one),
            (this.t = this.curve.zero),
            (this.zOne = !0))
          : ((this.x = new $r(e, 16)),
            (this.y = new $r(r, 16)),
            (this.z = n ? new $r(n, 16) : this.curve.one),
            (this.t = i && new $r(i, 16)),
            this.x.red || (this.x = this.x.toRed(this.curve.red)),
            this.y.red || (this.y = this.y.toRed(this.curve.red)),
            this.z.red || (this.z = this.z.toRed(this.curve.red)),
            this.t && !this.t.red && (this.t = this.t.toRed(this.curve.red)),
            (this.zOne = this.z === this.curve.one),
            this.curve.extended &&
              !this.t &&
              ((this.t = this.x.redMul(this.y)),
              this.zOne || (this.t = this.t.redMul(this.z.redInvm()))));
    }
    Tb(We, ds.BasePoint);
    Rr.prototype.pointFromJSON = function (e) {
      return We.fromJSON(this, e);
    };
    Rr.prototype.point = function (e, r, n, i) {
      return new We(this, e, r, n, i);
    };
    We.fromJSON = function (e, r) {
      return new We(e, r[0], r[1], r[2]);
    };
    We.prototype.inspect = function () {
      return this.isInfinity()
        ? '<EC Point Infinity>'
        : '<EC Point x: ' +
            this.x.fromRed().toString(16, 2) +
            ' y: ' +
            this.y.fromRed().toString(16, 2) +
            ' z: ' +
            this.z.fromRed().toString(16, 2) +
            '>';
    };
    We.prototype.isInfinity = function () {
      return (
        this.x.cmpn(0) === 0 &&
        (this.y.cmp(this.z) === 0 ||
          (this.zOne && this.y.cmp(this.curve.c) === 0))
      );
    };
    We.prototype._extDbl = function () {
      var e = this.x.redSqr(),
        r = this.y.redSqr(),
        n = this.z.redSqr();
      n = n.redIAdd(n);
      var i = this.curve._mulA(e),
        a = this.x.redAdd(this.y).redSqr().redISub(e).redISub(r),
        h = i.redAdd(r),
        v = h.redSub(n),
        g = i.redSub(r),
        M = a.redMul(v),
        x = h.redMul(g),
        E = a.redMul(g),
        I = v.redMul(h);
      return this.curve.point(M, x, I, E);
    };
    We.prototype._projDbl = function () {
      var e = this.x.redAdd(this.y).redSqr(),
        r = this.x.redSqr(),
        n = this.y.redSqr(),
        i,
        a,
        h,
        v,
        g,
        M;
      if (this.curve.twisted) {
        v = this.curve._mulA(r);
        var x = v.redAdd(n);
        this.zOne
          ? ((i = e.redSub(r).redSub(n).redMul(x.redSub(this.curve.two))),
            (a = x.redMul(v.redSub(n))),
            (h = x.redSqr().redSub(x).redSub(x)))
          : ((g = this.z.redSqr()),
            (M = x.redSub(g).redISub(g)),
            (i = e.redSub(r).redISub(n).redMul(M)),
            (a = x.redMul(v.redSub(n))),
            (h = x.redMul(M)));
      } else
        (v = r.redAdd(n)),
          (g = this.curve._mulC(this.z).redSqr()),
          (M = v.redSub(g).redSub(g)),
          (i = this.curve._mulC(e.redISub(v)).redMul(M)),
          (a = this.curve._mulC(v).redMul(r.redISub(n))),
          (h = v.redMul(M));
      return this.curve.point(i, a, h);
    };
    We.prototype.dbl = function () {
      return this.isInfinity()
        ? this
        : this.curve.extended
          ? this._extDbl()
          : this._projDbl();
    };
    We.prototype._extAdd = function (e) {
      var r = this.y.redSub(this.x).redMul(e.y.redSub(e.x)),
        n = this.y.redAdd(this.x).redMul(e.y.redAdd(e.x)),
        i = this.t.redMul(this.curve.dd).redMul(e.t),
        a = this.z.redMul(e.z.redAdd(e.z)),
        h = n.redSub(r),
        v = a.redSub(i),
        g = a.redAdd(i),
        M = n.redAdd(r),
        x = h.redMul(v),
        E = g.redMul(M),
        I = h.redMul(M),
        q = v.redMul(g);
      return this.curve.point(x, E, q, I);
    };
    We.prototype._projAdd = function (e) {
      var r = this.z.redMul(e.z),
        n = r.redSqr(),
        i = this.x.redMul(e.x),
        a = this.y.redMul(e.y),
        h = this.curve.d.redMul(i).redMul(a),
        v = n.redSub(h),
        g = n.redAdd(h),
        M = this.x.redAdd(this.y).redMul(e.x.redAdd(e.y)).redISub(i).redISub(a),
        x = r.redMul(v).redMul(M),
        E,
        I;
      return (
        this.curve.twisted
          ? ((E = r.redMul(g).redMul(a.redSub(this.curve._mulA(i)))),
            (I = v.redMul(g)))
          : ((E = r.redMul(g).redMul(a.redSub(i))),
            (I = this.curve._mulC(v).redMul(g))),
        this.curve.point(x, E, I)
      );
    };
    We.prototype.add = function (e) {
      return this.isInfinity()
        ? e
        : e.isInfinity()
          ? this
          : this.curve.extended
            ? this._extAdd(e)
            : this._projAdd(e);
    };
    We.prototype.mul = function (e) {
      return this._hasDoubles(e)
        ? this.curve._fixedNafMul(this, e)
        : this.curve._wnafMul(this, e);
    };
    We.prototype.mulAdd = function (e, r, n) {
      return this.curve._wnafMulAdd(1, [this, r], [e, n], 2, !1);
    };
    We.prototype.jmulAdd = function (e, r, n) {
      return this.curve._wnafMulAdd(1, [this, r], [e, n], 2, !0);
    };
    We.prototype.normalize = function () {
      if (this.zOne) return this;
      var e = this.z.redInvm();
      return (
        (this.x = this.x.redMul(e)),
        (this.y = this.y.redMul(e)),
        this.t && (this.t = this.t.redMul(e)),
        (this.z = this.curve.one),
        (this.zOne = !0),
        this
      );
    };
    We.prototype.neg = function () {
      return this.curve.point(
        this.x.redNeg(),
        this.y,
        this.z,
        this.t && this.t.redNeg()
      );
    };
    We.prototype.getX = function () {
      return this.normalize(), this.x.fromRed();
    };
    We.prototype.getY = function () {
      return this.normalize(), this.y.fromRed();
    };
    We.prototype.eq = function (e) {
      return (
        this === e ||
        (this.getX().cmp(e.getX()) === 0 && this.getY().cmp(e.getY()) === 0)
      );
    };
    We.prototype.eqXToP = function (e) {
      var r = e.toRed(this.curve.red).redMul(this.z);
      if (this.x.cmp(r) === 0) return !0;
      for (var n = e.clone(), i = this.curve.redN.redMul(this.z); ; ) {
        if ((n.iadd(this.curve.n), n.cmp(this.curve.p) >= 0)) return !1;
        if ((r.redIAdd(i), this.x.cmp(r) === 0)) return !0;
      }
    };
    We.prototype.toP = We.prototype.normalize;
    We.prototype.mixedAdd = We.prototype.add;
  });
  var _u = R((Ob) => {
    'use strict';
    S();
    var ls = Ob;
    ls.base = da();
    ls.short = Bb();
    ls.mont = qb();
    ls.edwards = kb();
  });
  var vr = R((Ve) => {
    'use strict';
    S();
    var t_ = At(),
      r_ = qe();
    Ve.inherits = r_;
    function i_(t, e) {
      return (t.charCodeAt(e) & 64512) !== 55296 || e < 0 || e + 1 >= t.length
        ? !1
        : (t.charCodeAt(e + 1) & 64512) === 56320;
    }
    function n_(t, e) {
      if (Array.isArray(t)) return t.slice();
      if (!t) return [];
      var r = [];
      if (typeof t == 'string')
        if (e) {
          if (e === 'hex')
            for (
              t = t.replace(/[^a-z0-9]+/gi, ''),
                t.length % 2 !== 0 && (t = '0' + t),
                i = 0;
              i < t.length;
              i += 2
            )
              r.push(parseInt(t[i] + t[i + 1], 16));
        } else
          for (var n = 0, i = 0; i < t.length; i++) {
            var a = t.charCodeAt(i);
            a < 128
              ? (r[n++] = a)
              : a < 2048
                ? ((r[n++] = (a >> 6) | 192), (r[n++] = (a & 63) | 128))
                : i_(t, i)
                  ? ((a =
                      65536 + ((a & 1023) << 10) + (t.charCodeAt(++i) & 1023)),
                    (r[n++] = (a >> 18) | 240),
                    (r[n++] = ((a >> 12) & 63) | 128),
                    (r[n++] = ((a >> 6) & 63) | 128),
                    (r[n++] = (a & 63) | 128))
                  : ((r[n++] = (a >> 12) | 224),
                    (r[n++] = ((a >> 6) & 63) | 128),
                    (r[n++] = (a & 63) | 128));
          }
      else for (i = 0; i < t.length; i++) r[i] = t[i] | 0;
      return r;
    }
    Ve.toArray = n_;
    function f_(t) {
      for (var e = '', r = 0; r < t.length; r++) e += Nb(t[r].toString(16));
      return e;
    }
    Ve.toHex = f_;
    function Cb(t) {
      var e =
        (t >>> 24) |
        ((t >>> 8) & 65280) |
        ((t << 8) & 16711680) |
        ((t & 255) << 24);
      return e >>> 0;
    }
    Ve.htonl = Cb;
    function a_(t, e) {
      for (var r = '', n = 0; n < t.length; n++) {
        var i = t[n];
        e === 'little' && (i = Cb(i)), (r += Db(i.toString(16)));
      }
      return r;
    }
    Ve.toHex32 = a_;
    function Nb(t) {
      return t.length === 1 ? '0' + t : t;
    }
    Ve.zero2 = Nb;
    function Db(t) {
      return t.length === 7
        ? '0' + t
        : t.length === 6
          ? '00' + t
          : t.length === 5
            ? '000' + t
            : t.length === 4
              ? '0000' + t
              : t.length === 3
                ? '00000' + t
                : t.length === 2
                  ? '000000' + t
                  : t.length === 1
                    ? '0000000' + t
                    : t;
    }
    Ve.zero8 = Db;
    function o_(t, e, r, n) {
      var i = r - e;
      t_(i % 4 === 0);
      for (var a = new Array(i / 4), h = 0, v = e; h < a.length; h++, v += 4) {
        var g;
        n === 'big'
          ? (g = (t[v] << 24) | (t[v + 1] << 16) | (t[v + 2] << 8) | t[v + 3])
          : (g = (t[v + 3] << 24) | (t[v + 2] << 16) | (t[v + 1] << 8) | t[v]),
          (a[h] = g >>> 0);
      }
      return a;
    }
    Ve.join32 = o_;
    function s_(t, e) {
      for (
        var r = new Array(t.length * 4), n = 0, i = 0;
        n < t.length;
        n++, i += 4
      ) {
        var a = t[n];
        e === 'big'
          ? ((r[i] = a >>> 24),
            (r[i + 1] = (a >>> 16) & 255),
            (r[i + 2] = (a >>> 8) & 255),
            (r[i + 3] = a & 255))
          : ((r[i + 3] = a >>> 24),
            (r[i + 2] = (a >>> 16) & 255),
            (r[i + 1] = (a >>> 8) & 255),
            (r[i] = a & 255));
      }
      return r;
    }
    Ve.split32 = s_;
    function h_(t, e) {
      return (t >>> e) | (t << (32 - e));
    }
    Ve.rotr32 = h_;
    function u_(t, e) {
      return (t << e) | (t >>> (32 - e));
    }
    Ve.rotl32 = u_;
    function c_(t, e) {
      return (t + e) >>> 0;
    }
    Ve.sum32 = c_;
    function d_(t, e, r) {
      return (t + e + r) >>> 0;
    }
    Ve.sum32_3 = d_;
    function l_(t, e, r, n) {
      return (t + e + r + n) >>> 0;
    }
    Ve.sum32_4 = l_;
    function p_(t, e, r, n, i) {
      return (t + e + r + n + i) >>> 0;
    }
    Ve.sum32_5 = p_;
    function v_(t, e, r, n) {
      var i = t[e],
        a = t[e + 1],
        h = (n + a) >>> 0,
        v = (h < n ? 1 : 0) + r + i;
      (t[e] = v >>> 0), (t[e + 1] = h);
    }
    Ve.sum64 = v_;
    function b_(t, e, r, n) {
      var i = (e + n) >>> 0,
        a = (i < e ? 1 : 0) + t + r;
      return a >>> 0;
    }
    Ve.sum64_hi = b_;
    function y_(t, e, r, n) {
      var i = e + n;
      return i >>> 0;
    }
    Ve.sum64_lo = y_;
    function m_(t, e, r, n, i, a, h, v) {
      var g = 0,
        M = e;
      (M = (M + n) >>> 0),
        (g += M < e ? 1 : 0),
        (M = (M + a) >>> 0),
        (g += M < a ? 1 : 0),
        (M = (M + v) >>> 0),
        (g += M < v ? 1 : 0);
      var x = t + r + i + h + g;
      return x >>> 0;
    }
    Ve.sum64_4_hi = m_;
    function g_(t, e, r, n, i, a, h, v) {
      var g = e + n + a + v;
      return g >>> 0;
    }
    Ve.sum64_4_lo = g_;
    function w_(t, e, r, n, i, a, h, v, g, M) {
      var x = 0,
        E = e;
      (E = (E + n) >>> 0),
        (x += E < e ? 1 : 0),
        (E = (E + a) >>> 0),
        (x += E < a ? 1 : 0),
        (E = (E + v) >>> 0),
        (x += E < v ? 1 : 0),
        (E = (E + M) >>> 0),
        (x += E < M ? 1 : 0);
      var I = t + r + i + h + g + x;
      return I >>> 0;
    }
    Ve.sum64_5_hi = w_;
    function __(t, e, r, n, i, a, h, v, g, M) {
      var x = e + n + a + v + M;
      return x >>> 0;
    }
    Ve.sum64_5_lo = __;
    function x_(t, e, r) {
      var n = (e << (32 - r)) | (t >>> r);
      return n >>> 0;
    }
    Ve.rotr64_hi = x_;
    function M_(t, e, r) {
      var n = (t << (32 - r)) | (e >>> r);
      return n >>> 0;
    }
    Ve.rotr64_lo = M_;
    function S_(t, e, r) {
      return t >>> r;
    }
    Ve.shr64_hi = S_;
    function E_(t, e, r) {
      var n = (t << (32 - r)) | (e >>> r);
      return n >>> 0;
    }
    Ve.shr64_lo = E_;
  });
  var Qn = R((Fb) => {
    'use strict';
    S();
    var Lb = vr(),
      A_ = At();
    function ps() {
      (this.pending = null),
        (this.pendingTotal = 0),
        (this.blockSize = this.constructor.blockSize),
        (this.outSize = this.constructor.outSize),
        (this.hmacStrength = this.constructor.hmacStrength),
        (this.padLength = this.constructor.padLength / 8),
        (this.endian = 'big'),
        (this._delta8 = this.blockSize / 8),
        (this._delta32 = this.blockSize / 32);
    }
    Fb.BlockHash = ps;
    ps.prototype.update = function (e, r) {
      if (
        ((e = Lb.toArray(e, r)),
        this.pending
          ? (this.pending = this.pending.concat(e))
          : (this.pending = e),
        (this.pendingTotal += e.length),
        this.pending.length >= this._delta8)
      ) {
        e = this.pending;
        var n = e.length % this._delta8;
        (this.pending = e.slice(e.length - n, e.length)),
          this.pending.length === 0 && (this.pending = null),
          (e = Lb.join32(e, 0, e.length - n, this.endian));
        for (var i = 0; i < e.length; i += this._delta32)
          this._update(e, i, i + this._delta32);
      }
      return this;
    };
    ps.prototype.digest = function (e) {
      return (
        this.update(this._pad()), A_(this.pending === null), this._digest(e)
      );
    };
    ps.prototype._pad = function () {
      var e = this.pendingTotal,
        r = this._delta8,
        n = r - ((e + this.padLength) % r),
        i = new Array(n + this.padLength);
      i[0] = 128;
      for (var a = 1; a < n; a++) i[a] = 0;
      if (((e <<= 3), this.endian === 'big')) {
        for (var h = 8; h < this.padLength; h++) i[a++] = 0;
        (i[a++] = 0),
          (i[a++] = 0),
          (i[a++] = 0),
          (i[a++] = 0),
          (i[a++] = (e >>> 24) & 255),
          (i[a++] = (e >>> 16) & 255),
          (i[a++] = (e >>> 8) & 255),
          (i[a++] = e & 255);
      } else
        for (
          i[a++] = e & 255,
            i[a++] = (e >>> 8) & 255,
            i[a++] = (e >>> 16) & 255,
            i[a++] = (e >>> 24) & 255,
            i[a++] = 0,
            i[a++] = 0,
            i[a++] = 0,
            i[a++] = 0,
            h = 8;
          h < this.padLength;
          h++
        )
          i[a++] = 0;
      return i;
    };
  });
  var xu = R((Zr) => {
    'use strict';
    S();
    var B_ = vr(),
      qr = B_.rotr32;
    function I_(t, e, r, n) {
      if (t === 0) return jb(e, r, n);
      if (t === 1 || t === 3) return zb(e, r, n);
      if (t === 2) return Ub(e, r, n);
    }
    Zr.ft_1 = I_;
    function jb(t, e, r) {
      return (t & e) ^ (~t & r);
    }
    Zr.ch32 = jb;
    function Ub(t, e, r) {
      return (t & e) ^ (t & r) ^ (e & r);
    }
    Zr.maj32 = Ub;
    function zb(t, e, r) {
      return t ^ e ^ r;
    }
    Zr.p32 = zb;
    function R_(t) {
      return qr(t, 2) ^ qr(t, 13) ^ qr(t, 22);
    }
    Zr.s0_256 = R_;
    function q_(t) {
      return qr(t, 6) ^ qr(t, 11) ^ qr(t, 25);
    }
    Zr.s1_256 = q_;
    function T_(t) {
      return qr(t, 7) ^ qr(t, 18) ^ (t >>> 3);
    }
    Zr.g0_256 = T_;
    function P_(t) {
      return qr(t, 17) ^ qr(t, 19) ^ (t >>> 10);
    }
    Zr.g1_256 = P_;
  });
  var Vb = R((gP, Kb) => {
    'use strict';
    S();
    var ef = vr(),
      k_ = Qn(),
      O_ = xu(),
      Mu = ef.rotl32,
      la = ef.sum32,
      C_ = ef.sum32_5,
      N_ = O_.ft_1,
      Hb = k_.BlockHash,
      D_ = [1518500249, 1859775393, 2400959708, 3395469782];
    function Tr() {
      if (!(this instanceof Tr)) return new Tr();
      Hb.call(this),
        (this.h = [1732584193, 4023233417, 2562383102, 271733878, 3285377520]),
        (this.W = new Array(80));
    }
    ef.inherits(Tr, Hb);
    Kb.exports = Tr;
    Tr.blockSize = 512;
    Tr.outSize = 160;
    Tr.hmacStrength = 80;
    Tr.padLength = 64;
    Tr.prototype._update = function (e, r) {
      for (var n = this.W, i = 0; i < 16; i++) n[i] = e[r + i];
      for (; i < n.length; i++)
        n[i] = Mu(n[i - 3] ^ n[i - 8] ^ n[i - 14] ^ n[i - 16], 1);
      var a = this.h[0],
        h = this.h[1],
        v = this.h[2],
        g = this.h[3],
        M = this.h[4];
      for (i = 0; i < n.length; i++) {
        var x = ~~(i / 20),
          E = C_(Mu(a, 5), N_(x, h, v, g), M, n[i], D_[x]);
        (M = g), (g = v), (v = Mu(h, 30)), (h = a), (a = E);
      }
      (this.h[0] = la(this.h[0], a)),
        (this.h[1] = la(this.h[1], h)),
        (this.h[2] = la(this.h[2], v)),
        (this.h[3] = la(this.h[3], g)),
        (this.h[4] = la(this.h[4], M));
    };
    Tr.prototype._digest = function (e) {
      return e === 'hex'
        ? ef.toHex32(this.h, 'big')
        : ef.split32(this.h, 'big');
    };
  });
  var Su = R((_P, Wb) => {
    'use strict';
    S();
    var tf = vr(),
      L_ = Qn(),
      rf = xu(),
      F_ = At(),
      br = tf.sum32,
      j_ = tf.sum32_4,
      U_ = tf.sum32_5,
      z_ = rf.ch32,
      H_ = rf.maj32,
      K_ = rf.s0_256,
      V_ = rf.s1_256,
      G_ = rf.g0_256,
      W_ = rf.g1_256,
      Gb = L_.BlockHash,
      $_ = [
        1116352408, 1899447441, 3049323471, 3921009573, 961987163, 1508970993,
        2453635748, 2870763221, 3624381080, 310598401, 607225278, 1426881987,
        1925078388, 2162078206, 2614888103, 3248222580, 3835390401, 4022224774,
        264347078, 604807628, 770255983, 1249150122, 1555081692, 1996064986,
        2554220882, 2821834349, 2952996808, 3210313671, 3336571891, 3584528711,
        113926993, 338241895, 666307205, 773529912, 1294757372, 1396182291,
        1695183700, 1986661051, 2177026350, 2456956037, 2730485921, 2820302411,
        3259730800, 3345764771, 3516065817, 3600352804, 4094571909, 275423344,
        430227734, 506948616, 659060556, 883997877, 958139571, 1322822218,
        1537002063, 1747873779, 1955562222, 2024104815, 2227730452, 2361852424,
        2428436474, 2756734187, 3204031479, 3329325298,
      ];
    function Pr() {
      if (!(this instanceof Pr)) return new Pr();
      Gb.call(this),
        (this.h = [
          1779033703, 3144134277, 1013904242, 2773480762, 1359893119,
          2600822924, 528734635, 1541459225,
        ]),
        (this.k = $_),
        (this.W = new Array(64));
    }
    tf.inherits(Pr, Gb);
    Wb.exports = Pr;
    Pr.blockSize = 512;
    Pr.outSize = 256;
    Pr.hmacStrength = 192;
    Pr.padLength = 64;
    Pr.prototype._update = function (e, r) {
      for (var n = this.W, i = 0; i < 16; i++) n[i] = e[r + i];
      for (; i < n.length; i++)
        n[i] = j_(W_(n[i - 2]), n[i - 7], G_(n[i - 15]), n[i - 16]);
      var a = this.h[0],
        h = this.h[1],
        v = this.h[2],
        g = this.h[3],
        M = this.h[4],
        x = this.h[5],
        E = this.h[6],
        I = this.h[7];
      for (F_(this.k.length === n.length), i = 0; i < n.length; i++) {
        var q = U_(I, V_(M), z_(M, x, E), this.k[i], n[i]),
          k = br(K_(a), H_(a, h, v));
        (I = E),
          (E = x),
          (x = M),
          (M = br(g, q)),
          (g = v),
          (v = h),
          (h = a),
          (a = br(q, k));
      }
      (this.h[0] = br(this.h[0], a)),
        (this.h[1] = br(this.h[1], h)),
        (this.h[2] = br(this.h[2], v)),
        (this.h[3] = br(this.h[3], g)),
        (this.h[4] = br(this.h[4], M)),
        (this.h[5] = br(this.h[5], x)),
        (this.h[6] = br(this.h[6], E)),
        (this.h[7] = br(this.h[7], I));
    };
    Pr.prototype._digest = function (e) {
      return e === 'hex'
        ? tf.toHex32(this.h, 'big')
        : tf.split32(this.h, 'big');
    };
  });
  var Jb = R((MP, Zb) => {
    'use strict';
    S();
    var Eu = vr(),
      $b = Su();
    function Jr() {
      if (!(this instanceof Jr)) return new Jr();
      $b.call(this),
        (this.h = [
          3238371032, 914150663, 812702999, 4144912697, 4290775857, 1750603025,
          1694076839, 3204075428,
        ]);
    }
    Eu.inherits(Jr, $b);
    Zb.exports = Jr;
    Jr.blockSize = 512;
    Jr.outSize = 224;
    Jr.hmacStrength = 192;
    Jr.padLength = 64;
    Jr.prototype._digest = function (e) {
      return e === 'hex'
        ? Eu.toHex32(this.h.slice(0, 7), 'big')
        : Eu.split32(this.h.slice(0, 7), 'big');
    };
  });
  var Iu = R((EP, e2) => {
    'use strict';
    S();
    var xt = vr(),
      Z_ = Qn(),
      J_ = At(),
      kr = xt.rotr64_hi,
      Or = xt.rotr64_lo,
      Xb = xt.shr64_hi,
      Yb = xt.shr64_lo,
      Ri = xt.sum64,
      Au = xt.sum64_hi,
      Bu = xt.sum64_lo,
      X_ = xt.sum64_4_hi,
      Y_ = xt.sum64_4_lo,
      Q_ = xt.sum64_5_hi,
      ex = xt.sum64_5_lo,
      Qb = Z_.BlockHash,
      tx = [
        1116352408, 3609767458, 1899447441, 602891725, 3049323471, 3964484399,
        3921009573, 2173295548, 961987163, 4081628472, 1508970993, 3053834265,
        2453635748, 2937671579, 2870763221, 3664609560, 3624381080, 2734883394,
        310598401, 1164996542, 607225278, 1323610764, 1426881987, 3590304994,
        1925078388, 4068182383, 2162078206, 991336113, 2614888103, 633803317,
        3248222580, 3479774868, 3835390401, 2666613458, 4022224774, 944711139,
        264347078, 2341262773, 604807628, 2007800933, 770255983, 1495990901,
        1249150122, 1856431235, 1555081692, 3175218132, 1996064986, 2198950837,
        2554220882, 3999719339, 2821834349, 766784016, 2952996808, 2566594879,
        3210313671, 3203337956, 3336571891, 1034457026, 3584528711, 2466948901,
        113926993, 3758326383, 338241895, 168717936, 666307205, 1188179964,
        773529912, 1546045734, 1294757372, 1522805485, 1396182291, 2643833823,
        1695183700, 2343527390, 1986661051, 1014477480, 2177026350, 1206759142,
        2456956037, 344077627, 2730485921, 1290863460, 2820302411, 3158454273,
        3259730800, 3505952657, 3345764771, 106217008, 3516065817, 3606008344,
        3600352804, 1432725776, 4094571909, 1467031594, 275423344, 851169720,
        430227734, 3100823752, 506948616, 1363258195, 659060556, 3750685593,
        883997877, 3785050280, 958139571, 3318307427, 1322822218, 3812723403,
        1537002063, 2003034995, 1747873779, 3602036899, 1955562222, 1575990012,
        2024104815, 1125592928, 2227730452, 2716904306, 2361852424, 442776044,
        2428436474, 593698344, 2756734187, 3733110249, 3204031479, 2999351573,
        3329325298, 3815920427, 3391569614, 3928383900, 3515267271, 566280711,
        3940187606, 3454069534, 4118630271, 4000239992, 116418474, 1914138554,
        174292421, 2731055270, 289380356, 3203993006, 460393269, 320620315,
        685471733, 587496836, 852142971, 1086792851, 1017036298, 365543100,
        1126000580, 2618297676, 1288033470, 3409855158, 1501505948, 4234509866,
        1607167915, 987167468, 1816402316, 1246189591,
      ];
    function yr() {
      if (!(this instanceof yr)) return new yr();
      Qb.call(this),
        (this.h = [
          1779033703, 4089235720, 3144134277, 2227873595, 1013904242,
          4271175723, 2773480762, 1595750129, 1359893119, 2917565137,
          2600822924, 725511199, 528734635, 4215389547, 1541459225, 327033209,
        ]),
        (this.k = tx),
        (this.W = new Array(160));
    }
    xt.inherits(yr, Qb);
    e2.exports = yr;
    yr.blockSize = 1024;
    yr.outSize = 512;
    yr.hmacStrength = 192;
    yr.padLength = 128;
    yr.prototype._prepareBlock = function (e, r) {
      for (var n = this.W, i = 0; i < 32; i++) n[i] = e[r + i];
      for (; i < n.length; i += 2) {
        var a = dx(n[i - 4], n[i - 3]),
          h = lx(n[i - 4], n[i - 3]),
          v = n[i - 14],
          g = n[i - 13],
          M = ux(n[i - 30], n[i - 29]),
          x = cx(n[i - 30], n[i - 29]),
          E = n[i - 32],
          I = n[i - 31];
        (n[i] = X_(a, h, v, g, M, x, E, I)),
          (n[i + 1] = Y_(a, h, v, g, M, x, E, I));
      }
    };
    yr.prototype._update = function (e, r) {
      this._prepareBlock(e, r);
      var n = this.W,
        i = this.h[0],
        a = this.h[1],
        h = this.h[2],
        v = this.h[3],
        g = this.h[4],
        M = this.h[5],
        x = this.h[6],
        E = this.h[7],
        I = this.h[8],
        q = this.h[9],
        k = this.h[10],
        L = this.h[11],
        xe = this.h[12],
        U = this.h[13],
        Me = this.h[14],
        Te = this.h[15];
      J_(this.k.length === n.length);
      for (var Ee = 0; Ee < n.length; Ee += 2) {
        var Fe = Me,
          Se = Te,
          $e = sx(I, q),
          ke = hx(I, q),
          Ze = rx(I, q, k, L, xe, U),
          B = ix(I, q, k, L, xe, U),
          b = this.k[Ee],
          _ = this.k[Ee + 1],
          l = n[Ee],
          f = n[Ee + 1],
          o = Q_(Fe, Se, $e, ke, Ze, B, b, _, l, f),
          c = ex(Fe, Se, $e, ke, Ze, B, b, _, l, f);
        (Fe = ax(i, a)),
          (Se = ox(i, a)),
          ($e = nx(i, a, h, v, g, M)),
          (ke = fx(i, a, h, v, g, M));
        var p = Au(Fe, Se, $e, ke),
          d = Bu(Fe, Se, $e, ke);
        (Me = xe),
          (Te = U),
          (xe = k),
          (U = L),
          (k = I),
          (L = q),
          (I = Au(x, E, o, c)),
          (q = Bu(E, E, o, c)),
          (x = g),
          (E = M),
          (g = h),
          (M = v),
          (h = i),
          (v = a),
          (i = Au(o, c, p, d)),
          (a = Bu(o, c, p, d));
      }
      Ri(this.h, 0, i, a),
        Ri(this.h, 2, h, v),
        Ri(this.h, 4, g, M),
        Ri(this.h, 6, x, E),
        Ri(this.h, 8, I, q),
        Ri(this.h, 10, k, L),
        Ri(this.h, 12, xe, U),
        Ri(this.h, 14, Me, Te);
    };
    yr.prototype._digest = function (e) {
      return e === 'hex'
        ? xt.toHex32(this.h, 'big')
        : xt.split32(this.h, 'big');
    };
    function rx(t, e, r, n, i) {
      var a = (t & r) ^ (~t & i);
      return a < 0 && (a += 4294967296), a;
    }
    function ix(t, e, r, n, i, a) {
      var h = (e & n) ^ (~e & a);
      return h < 0 && (h += 4294967296), h;
    }
    function nx(t, e, r, n, i) {
      var a = (t & r) ^ (t & i) ^ (r & i);
      return a < 0 && (a += 4294967296), a;
    }
    function fx(t, e, r, n, i, a) {
      var h = (e & n) ^ (e & a) ^ (n & a);
      return h < 0 && (h += 4294967296), h;
    }
    function ax(t, e) {
      var r = kr(t, e, 28),
        n = kr(e, t, 2),
        i = kr(e, t, 7),
        a = r ^ n ^ i;
      return a < 0 && (a += 4294967296), a;
    }
    function ox(t, e) {
      var r = Or(t, e, 28),
        n = Or(e, t, 2),
        i = Or(e, t, 7),
        a = r ^ n ^ i;
      return a < 0 && (a += 4294967296), a;
    }
    function sx(t, e) {
      var r = kr(t, e, 14),
        n = kr(t, e, 18),
        i = kr(e, t, 9),
        a = r ^ n ^ i;
      return a < 0 && (a += 4294967296), a;
    }
    function hx(t, e) {
      var r = Or(t, e, 14),
        n = Or(t, e, 18),
        i = Or(e, t, 9),
        a = r ^ n ^ i;
      return a < 0 && (a += 4294967296), a;
    }
    function ux(t, e) {
      var r = kr(t, e, 1),
        n = kr(t, e, 8),
        i = Xb(t, e, 7),
        a = r ^ n ^ i;
      return a < 0 && (a += 4294967296), a;
    }
    function cx(t, e) {
      var r = Or(t, e, 1),
        n = Or(t, e, 8),
        i = Yb(t, e, 7),
        a = r ^ n ^ i;
      return a < 0 && (a += 4294967296), a;
    }
    function dx(t, e) {
      var r = kr(t, e, 19),
        n = kr(e, t, 29),
        i = Xb(t, e, 6),
        a = r ^ n ^ i;
      return a < 0 && (a += 4294967296), a;
    }
    function lx(t, e) {
      var r = Or(t, e, 19),
        n = Or(e, t, 29),
        i = Yb(t, e, 6),
        a = r ^ n ^ i;
      return a < 0 && (a += 4294967296), a;
    }
  });
  var i2 = R((BP, r2) => {
    'use strict';
    S();
    var Ru = vr(),
      t2 = Iu();
    function Xr() {
      if (!(this instanceof Xr)) return new Xr();
      t2.call(this),
        (this.h = [
          3418070365, 3238371032, 1654270250, 914150663, 2438529370, 812702999,
          355462360, 4144912697, 1731405415, 4290775857, 2394180231, 1750603025,
          3675008525, 1694076839, 1203062813, 3204075428,
        ]);
    }
    Ru.inherits(Xr, t2);
    r2.exports = Xr;
    Xr.blockSize = 1024;
    Xr.outSize = 384;
    Xr.hmacStrength = 192;
    Xr.padLength = 128;
    Xr.prototype._digest = function (e) {
      return e === 'hex'
        ? Ru.toHex32(this.h.slice(0, 12), 'big')
        : Ru.split32(this.h.slice(0, 12), 'big');
    };
  });
  var n2 = R((nf) => {
    'use strict';
    S();
    nf.sha1 = Vb();
    nf.sha224 = Jb();
    nf.sha256 = Su();
    nf.sha384 = i2();
    nf.sha512 = Iu();
  });
  var u2 = R((h2) => {
    'use strict';
    S();
    var cn = vr(),
      px = Qn(),
      vs = cn.rotl32,
      f2 = cn.sum32,
      pa = cn.sum32_3,
      a2 = cn.sum32_4,
      s2 = px.BlockHash;
    function Cr() {
      if (!(this instanceof Cr)) return new Cr();
      s2.call(this),
        (this.h = [1732584193, 4023233417, 2562383102, 271733878, 3285377520]),
        (this.endian = 'little');
    }
    cn.inherits(Cr, s2);
    h2.ripemd160 = Cr;
    Cr.blockSize = 512;
    Cr.outSize = 160;
    Cr.hmacStrength = 192;
    Cr.padLength = 64;
    Cr.prototype._update = function (e, r) {
      for (
        var n = this.h[0],
          i = this.h[1],
          a = this.h[2],
          h = this.h[3],
          v = this.h[4],
          g = n,
          M = i,
          x = a,
          E = h,
          I = v,
          q = 0;
        q < 80;
        q++
      ) {
        var k = f2(vs(a2(n, o2(q, i, a, h), e[yx[q] + r], vx(q)), gx[q]), v);
        (n = v),
          (v = h),
          (h = vs(a, 10)),
          (a = i),
          (i = k),
          (k = f2(
            vs(a2(g, o2(79 - q, M, x, E), e[mx[q] + r], bx(q)), wx[q]),
            I
          )),
          (g = I),
          (I = E),
          (E = vs(x, 10)),
          (x = M),
          (M = k);
      }
      (k = pa(this.h[1], a, E)),
        (this.h[1] = pa(this.h[2], h, I)),
        (this.h[2] = pa(this.h[3], v, g)),
        (this.h[3] = pa(this.h[4], n, M)),
        (this.h[4] = pa(this.h[0], i, x)),
        (this.h[0] = k);
    };
    Cr.prototype._digest = function (e) {
      return e === 'hex'
        ? cn.toHex32(this.h, 'little')
        : cn.split32(this.h, 'little');
    };
    function o2(t, e, r, n) {
      return t <= 15
        ? e ^ r ^ n
        : t <= 31
          ? (e & r) | (~e & n)
          : t <= 47
            ? (e | ~r) ^ n
            : t <= 63
              ? (e & n) | (r & ~n)
              : e ^ (r | ~n);
    }
    function vx(t) {
      return t <= 15
        ? 0
        : t <= 31
          ? 1518500249
          : t <= 47
            ? 1859775393
            : t <= 63
              ? 2400959708
              : 2840853838;
    }
    function bx(t) {
      return t <= 15
        ? 1352829926
        : t <= 31
          ? 1548603684
          : t <= 47
            ? 1836072691
            : t <= 63
              ? 2053994217
              : 0;
    }
    var yx = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 7, 4, 13, 1, 10,
        6, 15, 3, 12, 0, 9, 5, 2, 14, 11, 8, 3, 10, 14, 4, 9, 15, 8, 1, 2, 7, 0,
        6, 13, 11, 5, 12, 1, 9, 11, 10, 0, 8, 12, 4, 13, 3, 7, 15, 14, 5, 6, 2,
        4, 0, 5, 9, 7, 12, 2, 10, 14, 1, 3, 8, 11, 6, 15, 13,
      ],
      mx = [
        5, 14, 7, 0, 9, 2, 11, 4, 13, 6, 15, 8, 1, 10, 3, 12, 6, 11, 3, 7, 0,
        13, 5, 10, 14, 15, 8, 12, 4, 9, 1, 2, 15, 5, 1, 3, 7, 14, 6, 9, 11, 8,
        12, 2, 10, 0, 4, 13, 8, 6, 4, 1, 3, 11, 15, 0, 5, 12, 2, 13, 9, 7, 10,
        14, 12, 15, 10, 4, 1, 5, 8, 7, 6, 2, 13, 14, 0, 3, 9, 11,
      ],
      gx = [
        11, 14, 15, 12, 5, 8, 7, 9, 11, 13, 14, 15, 6, 7, 9, 8, 7, 6, 8, 13, 11,
        9, 7, 15, 7, 12, 15, 9, 11, 7, 13, 12, 11, 13, 6, 7, 14, 9, 13, 15, 14,
        8, 13, 6, 5, 12, 7, 5, 11, 12, 14, 15, 14, 15, 9, 8, 9, 14, 5, 6, 8, 6,
        5, 12, 9, 15, 5, 11, 6, 8, 13, 12, 5, 12, 13, 14, 11, 8, 5, 6,
      ],
      wx = [
        8, 9, 9, 11, 13, 15, 15, 5, 7, 7, 8, 11, 14, 14, 12, 6, 9, 13, 15, 7,
        12, 8, 9, 11, 7, 7, 12, 7, 6, 15, 13, 11, 9, 7, 15, 11, 8, 6, 6, 14, 12,
        13, 5, 14, 13, 13, 7, 5, 15, 5, 8, 11, 14, 14, 6, 14, 6, 9, 12, 9, 12,
        5, 15, 8, 8, 5, 12, 9, 12, 5, 14, 6, 8, 13, 6, 5, 15, 13, 11, 11,
      ];
  });
  var d2 = R((kP, c2) => {
    'use strict';
    S();
    var _x = vr(),
      xx = At();
    function ff(t, e, r) {
      if (!(this instanceof ff)) return new ff(t, e, r);
      (this.Hash = t),
        (this.blockSize = t.blockSize / 8),
        (this.outSize = t.outSize / 8),
        (this.inner = null),
        (this.outer = null),
        this._init(_x.toArray(e, r));
    }
    c2.exports = ff;
    ff.prototype._init = function (e) {
      e.length > this.blockSize && (e = new this.Hash().update(e).digest()),
        xx(e.length <= this.blockSize);
      for (var r = e.length; r < this.blockSize; r++) e.push(0);
      for (r = 0; r < e.length; r++) e[r] ^= 54;
      for (this.inner = new this.Hash().update(e), r = 0; r < e.length; r++)
        e[r] ^= 106;
      this.outer = new this.Hash().update(e);
    };
    ff.prototype.update = function (e, r) {
      return this.inner.update(e, r), this;
    };
    ff.prototype.digest = function (e) {
      return this.outer.update(this.inner.digest()), this.outer.digest(e);
    };
  });
  var bs = R((l2) => {
    S();
    var ct = l2;
    ct.utils = vr();
    ct.common = Qn();
    ct.sha = n2();
    ct.ripemd = u2();
    ct.hmac = d2();
    ct.sha1 = ct.sha.sha1;
    ct.sha256 = ct.sha.sha256;
    ct.sha224 = ct.sha.sha224;
    ct.sha384 = ct.sha.sha384;
    ct.sha512 = ct.sha.sha512;
    ct.ripemd160 = ct.ripemd.ripemd160;
  });
  var v2 = R((DP, p2) => {
    S();
    p2.exports = {
      doubles: {
        step: 4,
        points: [
          [
            'e60fce93b59e9ec53011aabc21c23e97b2a31369b87a5ae9c44ee89e2a6dec0a',
            'f7e3507399e595929db99f34f57937101296891e44d23f0be1f32cce69616821',
          ],
          [
            '8282263212c609d9ea2a6e3e172de238d8c39cabd5ac1ca10646e23fd5f51508',
            '11f8a8098557dfe45e8256e830b60ace62d613ac2f7b17bed31b6eaff6e26caf',
          ],
          [
            '175e159f728b865a72f99cc6c6fc846de0b93833fd2222ed73fce5b551e5b739',
            'd3506e0d9e3c79eba4ef97a51ff71f5eacb5955add24345c6efa6ffee9fed695',
          ],
          [
            '363d90d447b00c9c99ceac05b6262ee053441c7e55552ffe526bad8f83ff4640',
            '4e273adfc732221953b445397f3363145b9a89008199ecb62003c7f3bee9de9',
          ],
          [
            '8b4b5f165df3c2be8c6244b5b745638843e4a781a15bcd1b69f79a55dffdf80c',
            '4aad0a6f68d308b4b3fbd7813ab0da04f9e336546162ee56b3eff0c65fd4fd36',
          ],
          [
            '723cbaa6e5db996d6bf771c00bd548c7b700dbffa6c0e77bcb6115925232fcda',
            '96e867b5595cc498a921137488824d6e2660a0653779494801dc069d9eb39f5f',
          ],
          [
            'eebfa4d493bebf98ba5feec812c2d3b50947961237a919839a533eca0e7dd7fa',
            '5d9a8ca3970ef0f269ee7edaf178089d9ae4cdc3a711f712ddfd4fdae1de8999',
          ],
          [
            '100f44da696e71672791d0a09b7bde459f1215a29b3c03bfefd7835b39a48db0',
            'cdd9e13192a00b772ec8f3300c090666b7ff4a18ff5195ac0fbd5cd62bc65a09',
          ],
          [
            'e1031be262c7ed1b1dc9227a4a04c017a77f8d4464f3b3852c8acde6e534fd2d',
            '9d7061928940405e6bb6a4176597535af292dd419e1ced79a44f18f29456a00d',
          ],
          [
            'feea6cae46d55b530ac2839f143bd7ec5cf8b266a41d6af52d5e688d9094696d',
            'e57c6b6c97dce1bab06e4e12bf3ecd5c981c8957cc41442d3155debf18090088',
          ],
          [
            'da67a91d91049cdcb367be4be6ffca3cfeed657d808583de33fa978bc1ec6cb1',
            '9bacaa35481642bc41f463f7ec9780e5dec7adc508f740a17e9ea8e27a68be1d',
          ],
          [
            '53904faa0b334cdda6e000935ef22151ec08d0f7bb11069f57545ccc1a37b7c0',
            '5bc087d0bc80106d88c9eccac20d3c1c13999981e14434699dcb096b022771c8',
          ],
          [
            '8e7bcd0bd35983a7719cca7764ca906779b53a043a9b8bcaeff959f43ad86047',
            '10b7770b2a3da4b3940310420ca9514579e88e2e47fd68b3ea10047e8460372a',
          ],
          [
            '385eed34c1cdff21e6d0818689b81bde71a7f4f18397e6690a841e1599c43862',
            '283bebc3e8ea23f56701de19e9ebf4576b304eec2086dc8cc0458fe5542e5453',
          ],
          [
            '6f9d9b803ecf191637c73a4413dfa180fddf84a5947fbc9c606ed86c3fac3a7',
            '7c80c68e603059ba69b8e2a30e45c4d47ea4dd2f5c281002d86890603a842160',
          ],
          [
            '3322d401243c4e2582a2147c104d6ecbf774d163db0f5e5313b7e0e742d0e6bd',
            '56e70797e9664ef5bfb019bc4ddaf9b72805f63ea2873af624f3a2e96c28b2a0',
          ],
          [
            '85672c7d2de0b7da2bd1770d89665868741b3f9af7643397721d74d28134ab83',
            '7c481b9b5b43b2eb6374049bfa62c2e5e77f17fcc5298f44c8e3094f790313a6',
          ],
          [
            '948bf809b1988a46b06c9f1919413b10f9226c60f668832ffd959af60c82a0a',
            '53a562856dcb6646dc6b74c5d1c3418c6d4dff08c97cd2bed4cb7f88d8c8e589',
          ],
          [
            '6260ce7f461801c34f067ce0f02873a8f1b0e44dfc69752accecd819f38fd8e8',
            'bc2da82b6fa5b571a7f09049776a1ef7ecd292238051c198c1a84e95b2b4ae17',
          ],
          [
            'e5037de0afc1d8d43d8348414bbf4103043ec8f575bfdc432953cc8d2037fa2d',
            '4571534baa94d3b5f9f98d09fb990bddbd5f5b03ec481f10e0e5dc841d755bda',
          ],
          [
            'e06372b0f4a207adf5ea905e8f1771b4e7e8dbd1c6a6c5b725866a0ae4fce725',
            '7a908974bce18cfe12a27bb2ad5a488cd7484a7787104870b27034f94eee31dd',
          ],
          [
            '213c7a715cd5d45358d0bbf9dc0ce02204b10bdde2a3f58540ad6908d0559754',
            '4b6dad0b5ae462507013ad06245ba190bb4850f5f36a7eeddff2c27534b458f2',
          ],
          [
            '4e7c272a7af4b34e8dbb9352a5419a87e2838c70adc62cddf0cc3a3b08fbd53c',
            '17749c766c9d0b18e16fd09f6def681b530b9614bff7dd33e0b3941817dcaae6',
          ],
          [
            'fea74e3dbe778b1b10f238ad61686aa5c76e3db2be43057632427e2840fb27b6',
            '6e0568db9b0b13297cf674deccb6af93126b596b973f7b77701d3db7f23cb96f',
          ],
          [
            '76e64113f677cf0e10a2570d599968d31544e179b760432952c02a4417bdde39',
            'c90ddf8dee4e95cf577066d70681f0d35e2a33d2b56d2032b4b1752d1901ac01',
          ],
          [
            'c738c56b03b2abe1e8281baa743f8f9a8f7cc643df26cbee3ab150242bcbb891',
            '893fb578951ad2537f718f2eacbfbbbb82314eef7880cfe917e735d9699a84c3',
          ],
          [
            'd895626548b65b81e264c7637c972877d1d72e5f3a925014372e9f6588f6c14b',
            'febfaa38f2bc7eae728ec60818c340eb03428d632bb067e179363ed75d7d991f',
          ],
          [
            'b8da94032a957518eb0f6433571e8761ceffc73693e84edd49150a564f676e03',
            '2804dfa44805a1e4d7c99cc9762808b092cc584d95ff3b511488e4e74efdf6e7',
          ],
          [
            'e80fea14441fb33a7d8adab9475d7fab2019effb5156a792f1a11778e3c0df5d',
            'eed1de7f638e00771e89768ca3ca94472d155e80af322ea9fcb4291b6ac9ec78',
          ],
          [
            'a301697bdfcd704313ba48e51d567543f2a182031efd6915ddc07bbcc4e16070',
            '7370f91cfb67e4f5081809fa25d40f9b1735dbf7c0a11a130c0d1a041e177ea1',
          ],
          [
            '90ad85b389d6b936463f9d0512678de208cc330b11307fffab7ac63e3fb04ed4',
            'e507a3620a38261affdcbd9427222b839aefabe1582894d991d4d48cb6ef150',
          ],
          [
            '8f68b9d2f63b5f339239c1ad981f162ee88c5678723ea3351b7b444c9ec4c0da',
            '662a9f2dba063986de1d90c2b6be215dbbea2cfe95510bfdf23cbf79501fff82',
          ],
          [
            'e4f3fb0176af85d65ff99ff9198c36091f48e86503681e3e6686fd5053231e11',
            '1e63633ad0ef4f1c1661a6d0ea02b7286cc7e74ec951d1c9822c38576feb73bc',
          ],
          [
            '8c00fa9b18ebf331eb961537a45a4266c7034f2f0d4e1d0716fb6eae20eae29e',
            'efa47267fea521a1a9dc343a3736c974c2fadafa81e36c54e7d2a4c66702414b',
          ],
          [
            'e7a26ce69dd4829f3e10cec0a9e98ed3143d084f308b92c0997fddfc60cb3e41',
            '2a758e300fa7984b471b006a1aafbb18d0a6b2c0420e83e20e8a9421cf2cfd51',
          ],
          [
            'b6459e0ee3662ec8d23540c223bcbdc571cbcb967d79424f3cf29eb3de6b80ef',
            '67c876d06f3e06de1dadf16e5661db3c4b3ae6d48e35b2ff30bf0b61a71ba45',
          ],
          [
            'd68a80c8280bb840793234aa118f06231d6f1fc67e73c5a5deda0f5b496943e8',
            'db8ba9fff4b586d00c4b1f9177b0e28b5b0e7b8f7845295a294c84266b133120',
          ],
          [
            '324aed7df65c804252dc0270907a30b09612aeb973449cea4095980fc28d3d5d',
            '648a365774b61f2ff130c0c35aec1f4f19213b0c7e332843967224af96ab7c84',
          ],
          [
            '4df9c14919cde61f6d51dfdbe5fee5dceec4143ba8d1ca888e8bd373fd054c96',
            '35ec51092d8728050974c23a1d85d4b5d506cdc288490192ebac06cad10d5d',
          ],
          [
            '9c3919a84a474870faed8a9c1cc66021523489054d7f0308cbfc99c8ac1f98cd',
            'ddb84f0f4a4ddd57584f044bf260e641905326f76c64c8e6be7e5e03d4fc599d',
          ],
          [
            '6057170b1dd12fdf8de05f281d8e06bb91e1493a8b91d4cc5a21382120a959e5',
            '9a1af0b26a6a4807add9a2daf71df262465152bc3ee24c65e899be932385a2a8',
          ],
          [
            'a576df8e23a08411421439a4518da31880cef0fba7d4df12b1a6973eecb94266',
            '40a6bf20e76640b2c92b97afe58cd82c432e10a7f514d9f3ee8be11ae1b28ec8',
          ],
          [
            '7778a78c28dec3e30a05fe9629de8c38bb30d1f5cf9a3a208f763889be58ad71',
            '34626d9ab5a5b22ff7098e12f2ff580087b38411ff24ac563b513fc1fd9f43ac',
          ],
          [
            '928955ee637a84463729fd30e7afd2ed5f96274e5ad7e5cb09eda9c06d903ac',
            'c25621003d3f42a827b78a13093a95eeac3d26efa8a8d83fc5180e935bcd091f',
          ],
          [
            '85d0fef3ec6db109399064f3a0e3b2855645b4a907ad354527aae75163d82751',
            '1f03648413a38c0be29d496e582cf5663e8751e96877331582c237a24eb1f962',
          ],
          [
            'ff2b0dce97eece97c1c9b6041798b85dfdfb6d8882da20308f5404824526087e',
            '493d13fef524ba188af4c4dc54d07936c7b7ed6fb90e2ceb2c951e01f0c29907',
          ],
          [
            '827fbbe4b1e880ea9ed2b2e6301b212b57f1ee148cd6dd28780e5e2cf856e241',
            'c60f9c923c727b0b71bef2c67d1d12687ff7a63186903166d605b68baec293ec',
          ],
          [
            'eaa649f21f51bdbae7be4ae34ce6e5217a58fdce7f47f9aa7f3b58fa2120e2b3',
            'be3279ed5bbbb03ac69a80f89879aa5a01a6b965f13f7e59d47a5305ba5ad93d',
          ],
          [
            'e4a42d43c5cf169d9391df6decf42ee541b6d8f0c9a137401e23632dda34d24f',
            '4d9f92e716d1c73526fc99ccfb8ad34ce886eedfa8d8e4f13a7f7131deba9414',
          ],
          [
            '1ec80fef360cbdd954160fadab352b6b92b53576a88fea4947173b9d4300bf19',
            'aeefe93756b5340d2f3a4958a7abbf5e0146e77f6295a07b671cdc1cc107cefd',
          ],
          [
            '146a778c04670c2f91b00af4680dfa8bce3490717d58ba889ddb5928366642be',
            'b318e0ec3354028add669827f9d4b2870aaa971d2f7e5ed1d0b297483d83efd0',
          ],
          [
            'fa50c0f61d22e5f07e3acebb1aa07b128d0012209a28b9776d76a8793180eef9',
            '6b84c6922397eba9b72cd2872281a68a5e683293a57a213b38cd8d7d3f4f2811',
          ],
          [
            'da1d61d0ca721a11b1a5bf6b7d88e8421a288ab5d5bba5220e53d32b5f067ec2',
            '8157f55a7c99306c79c0766161c91e2966a73899d279b48a655fba0f1ad836f1',
          ],
          [
            'a8e282ff0c9706907215ff98e8fd416615311de0446f1e062a73b0610d064e13',
            '7f97355b8db81c09abfb7f3c5b2515888b679a3e50dd6bd6cef7c73111f4cc0c',
          ],
          [
            '174a53b9c9a285872d39e56e6913cab15d59b1fa512508c022f382de8319497c',
            'ccc9dc37abfc9c1657b4155f2c47f9e6646b3a1d8cb9854383da13ac079afa73',
          ],
          [
            '959396981943785c3d3e57edf5018cdbe039e730e4918b3d884fdff09475b7ba',
            '2e7e552888c331dd8ba0386a4b9cd6849c653f64c8709385e9b8abf87524f2fd',
          ],
          [
            'd2a63a50ae401e56d645a1153b109a8fcca0a43d561fba2dbb51340c9d82b151',
            'e82d86fb6443fcb7565aee58b2948220a70f750af484ca52d4142174dcf89405',
          ],
          [
            '64587e2335471eb890ee7896d7cfdc866bacbdbd3839317b3436f9b45617e073',
            'd99fcdd5bf6902e2ae96dd6447c299a185b90a39133aeab358299e5e9faf6589',
          ],
          [
            '8481bde0e4e4d885b3a546d3e549de042f0aa6cea250e7fd358d6c86dd45e458',
            '38ee7b8cba5404dd84a25bf39cecb2ca900a79c42b262e556d64b1b59779057e',
          ],
          [
            '13464a57a78102aa62b6979ae817f4637ffcfed3c4b1ce30bcd6303f6caf666b',
            '69be159004614580ef7e433453ccb0ca48f300a81d0942e13f495a907f6ecc27',
          ],
          [
            'bc4a9df5b713fe2e9aef430bcc1dc97a0cd9ccede2f28588cada3a0d2d83f366',
            'd3a81ca6e785c06383937adf4b798caa6e8a9fbfa547b16d758d666581f33c1',
          ],
          [
            '8c28a97bf8298bc0d23d8c749452a32e694b65e30a9472a3954ab30fe5324caa',
            '40a30463a3305193378fedf31f7cc0eb7ae784f0451cb9459e71dc73cbef9482',
          ],
          [
            '8ea9666139527a8c1dd94ce4f071fd23c8b350c5a4bb33748c4ba111faccae0',
            '620efabbc8ee2782e24e7c0cfb95c5d735b783be9cf0f8e955af34a30e62b945',
          ],
          [
            'dd3625faef5ba06074669716bbd3788d89bdde815959968092f76cc4eb9a9787',
            '7a188fa3520e30d461da2501045731ca941461982883395937f68d00c644a573',
          ],
          [
            'f710d79d9eb962297e4f6232b40e8f7feb2bc63814614d692c12de752408221e',
            'ea98e67232d3b3295d3b535532115ccac8612c721851617526ae47a9c77bfc82',
          ],
        ],
      },
      naf: {
        wnd: 7,
        points: [
          [
            'f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9',
            '388f7b0f632de8140fe337e62a37f3566500a99934c2231b6cb9fd7584b8e672',
          ],
          [
            '2f8bde4d1a07209355b4a7250a5c5128e88b84bddc619ab7cba8d569b240efe4',
            'd8ac222636e5e3d6d4dba9dda6c9c426f788271bab0d6840dca87d3aa6ac62d6',
          ],
          [
            '5cbdf0646e5db4eaa398f365f2ea7a0e3d419b7e0330e39ce92bddedcac4f9bc',
            '6aebca40ba255960a3178d6d861a54dba813d0b813fde7b5a5082628087264da',
          ],
          [
            'acd484e2f0c7f65309ad178a9f559abde09796974c57e714c35f110dfc27ccbe',
            'cc338921b0a7d9fd64380971763b61e9add888a4375f8e0f05cc262ac64f9c37',
          ],
          [
            '774ae7f858a9411e5ef4246b70c65aac5649980be5c17891bbec17895da008cb',
            'd984a032eb6b5e190243dd56d7b7b365372db1e2dff9d6a8301d74c9c953c61b',
          ],
          [
            'f28773c2d975288bc7d1d205c3748651b075fbc6610e58cddeeddf8f19405aa8',
            'ab0902e8d880a89758212eb65cdaf473a1a06da521fa91f29b5cb52db03ed81',
          ],
          [
            'd7924d4f7d43ea965a465ae3095ff41131e5946f3c85f79e44adbcf8e27e080e',
            '581e2872a86c72a683842ec228cc6defea40af2bd896d3a5c504dc9ff6a26b58',
          ],
          [
            'defdea4cdb677750a420fee807eacf21eb9898ae79b9768766e4faa04a2d4a34',
            '4211ab0694635168e997b0ead2a93daeced1f4a04a95c0f6cfb199f69e56eb77',
          ],
          [
            '2b4ea0a797a443d293ef5cff444f4979f06acfebd7e86d277475656138385b6c',
            '85e89bc037945d93b343083b5a1c86131a01f60c50269763b570c854e5c09b7a',
          ],
          [
            '352bbf4a4cdd12564f93fa332ce333301d9ad40271f8107181340aef25be59d5',
            '321eb4075348f534d59c18259dda3e1f4a1b3b2e71b1039c67bd3d8bcf81998c',
          ],
          [
            '2fa2104d6b38d11b0230010559879124e42ab8dfeff5ff29dc9cdadd4ecacc3f',
            '2de1068295dd865b64569335bd5dd80181d70ecfc882648423ba76b532b7d67',
          ],
          [
            '9248279b09b4d68dab21a9b066edda83263c3d84e09572e269ca0cd7f5453714',
            '73016f7bf234aade5d1aa71bdea2b1ff3fc0de2a887912ffe54a32ce97cb3402',
          ],
          [
            'daed4f2be3a8bf278e70132fb0beb7522f570e144bf615c07e996d443dee8729',
            'a69dce4a7d6c98e8d4a1aca87ef8d7003f83c230f3afa726ab40e52290be1c55',
          ],
          [
            'c44d12c7065d812e8acf28d7cbb19f9011ecd9e9fdf281b0e6a3b5e87d22e7db',
            '2119a460ce326cdc76c45926c982fdac0e106e861edf61c5a039063f0e0e6482',
          ],
          [
            '6a245bf6dc698504c89a20cfded60853152b695336c28063b61c65cbd269e6b4',
            'e022cf42c2bd4a708b3f5126f16a24ad8b33ba48d0423b6efd5e6348100d8a82',
          ],
          [
            '1697ffa6fd9de627c077e3d2fe541084ce13300b0bec1146f95ae57f0d0bd6a5',
            'b9c398f186806f5d27561506e4557433a2cf15009e498ae7adee9d63d01b2396',
          ],
          [
            '605bdb019981718b986d0f07e834cb0d9deb8360ffb7f61df982345ef27a7479',
            '2972d2de4f8d20681a78d93ec96fe23c26bfae84fb14db43b01e1e9056b8c49',
          ],
          [
            '62d14dab4150bf497402fdc45a215e10dcb01c354959b10cfe31c7e9d87ff33d',
            '80fc06bd8cc5b01098088a1950eed0db01aa132967ab472235f5642483b25eaf',
          ],
          [
            '80c60ad0040f27dade5b4b06c408e56b2c50e9f56b9b8b425e555c2f86308b6f',
            '1c38303f1cc5c30f26e66bad7fe72f70a65eed4cbe7024eb1aa01f56430bd57a',
          ],
          [
            '7a9375ad6167ad54aa74c6348cc54d344cc5dc9487d847049d5eabb0fa03c8fb',
            'd0e3fa9eca8726909559e0d79269046bdc59ea10c70ce2b02d499ec224dc7f7',
          ],
          [
            'd528ecd9b696b54c907a9ed045447a79bb408ec39b68df504bb51f459bc3ffc9',
            'eecf41253136e5f99966f21881fd656ebc4345405c520dbc063465b521409933',
          ],
          [
            '49370a4b5f43412ea25f514e8ecdad05266115e4a7ecb1387231808f8b45963',
            '758f3f41afd6ed428b3081b0512fd62a54c3f3afbb5b6764b653052a12949c9a',
          ],
          [
            '77f230936ee88cbbd73df930d64702ef881d811e0e1498e2f1c13eb1fc345d74',
            '958ef42a7886b6400a08266e9ba1b37896c95330d97077cbbe8eb3c7671c60d6',
          ],
          [
            'f2dac991cc4ce4b9ea44887e5c7c0bce58c80074ab9d4dbaeb28531b7739f530',
            'e0dedc9b3b2f8dad4da1f32dec2531df9eb5fbeb0598e4fd1a117dba703a3c37',
          ],
          [
            '463b3d9f662621fb1b4be8fbbe2520125a216cdfc9dae3debcba4850c690d45b',
            '5ed430d78c296c3543114306dd8622d7c622e27c970a1de31cb377b01af7307e',
          ],
          [
            'f16f804244e46e2a09232d4aff3b59976b98fac14328a2d1a32496b49998f247',
            'cedabd9b82203f7e13d206fcdf4e33d92a6c53c26e5cce26d6579962c4e31df6',
          ],
          [
            'caf754272dc84563b0352b7a14311af55d245315ace27c65369e15f7151d41d1',
            'cb474660ef35f5f2a41b643fa5e460575f4fa9b7962232a5c32f908318a04476',
          ],
          [
            '2600ca4b282cb986f85d0f1709979d8b44a09c07cb86d7c124497bc86f082120',
            '4119b88753c15bd6a693b03fcddbb45d5ac6be74ab5f0ef44b0be9475a7e4b40',
          ],
          [
            '7635ca72d7e8432c338ec53cd12220bc01c48685e24f7dc8c602a7746998e435',
            '91b649609489d613d1d5e590f78e6d74ecfc061d57048bad9e76f302c5b9c61',
          ],
          [
            '754e3239f325570cdbbf4a87deee8a66b7f2b33479d468fbc1a50743bf56cc18',
            '673fb86e5bda30fb3cd0ed304ea49a023ee33d0197a695d0c5d98093c536683',
          ],
          [
            'e3e6bd1071a1e96aff57859c82d570f0330800661d1c952f9fe2694691d9b9e8',
            '59c9e0bba394e76f40c0aa58379a3cb6a5a2283993e90c4167002af4920e37f5',
          ],
          [
            '186b483d056a033826ae73d88f732985c4ccb1f32ba35f4b4cc47fdcf04aa6eb',
            '3b952d32c67cf77e2e17446e204180ab21fb8090895138b4a4a797f86e80888b',
          ],
          [
            'df9d70a6b9876ce544c98561f4be4f725442e6d2b737d9c91a8321724ce0963f',
            '55eb2dafd84d6ccd5f862b785dc39d4ab157222720ef9da217b8c45cf2ba2417',
          ],
          [
            '5edd5cc23c51e87a497ca815d5dce0f8ab52554f849ed8995de64c5f34ce7143',
            'efae9c8dbc14130661e8cec030c89ad0c13c66c0d17a2905cdc706ab7399a868',
          ],
          [
            '290798c2b6476830da12fe02287e9e777aa3fba1c355b17a722d362f84614fba',
            'e38da76dcd440621988d00bcf79af25d5b29c094db2a23146d003afd41943e7a',
          ],
          [
            'af3c423a95d9f5b3054754efa150ac39cd29552fe360257362dfdecef4053b45',
            'f98a3fd831eb2b749a93b0e6f35cfb40c8cd5aa667a15581bc2feded498fd9c6',
          ],
          [
            '766dbb24d134e745cccaa28c99bf274906bb66b26dcf98df8d2fed50d884249a',
            '744b1152eacbe5e38dcc887980da38b897584a65fa06cedd2c924f97cbac5996',
          ],
          [
            '59dbf46f8c94759ba21277c33784f41645f7b44f6c596a58ce92e666191abe3e',
            'c534ad44175fbc300f4ea6ce648309a042ce739a7919798cd85e216c4a307f6e',
          ],
          [
            'f13ada95103c4537305e691e74e9a4a8dd647e711a95e73cb62dc6018cfd87b8',
            'e13817b44ee14de663bf4bc808341f326949e21a6a75c2570778419bdaf5733d',
          ],
          [
            '7754b4fa0e8aced06d4167a2c59cca4cda1869c06ebadfb6488550015a88522c',
            '30e93e864e669d82224b967c3020b8fa8d1e4e350b6cbcc537a48b57841163a2',
          ],
          [
            '948dcadf5990e048aa3874d46abef9d701858f95de8041d2a6828c99e2262519',
            'e491a42537f6e597d5d28a3224b1bc25df9154efbd2ef1d2cbba2cae5347d57e',
          ],
          [
            '7962414450c76c1689c7b48f8202ec37fb224cf5ac0bfa1570328a8a3d7c77ab',
            '100b610ec4ffb4760d5c1fc133ef6f6b12507a051f04ac5760afa5b29db83437',
          ],
          [
            '3514087834964b54b15b160644d915485a16977225b8847bb0dd085137ec47ca',
            'ef0afbb2056205448e1652c48e8127fc6039e77c15c2378b7e7d15a0de293311',
          ],
          [
            'd3cc30ad6b483e4bc79ce2c9dd8bc54993e947eb8df787b442943d3f7b527eaf',
            '8b378a22d827278d89c5e9be8f9508ae3c2ad46290358630afb34db04eede0a4',
          ],
          [
            '1624d84780732860ce1c78fcbfefe08b2b29823db913f6493975ba0ff4847610',
            '68651cf9b6da903e0914448c6cd9d4ca896878f5282be4c8cc06e2a404078575',
          ],
          [
            '733ce80da955a8a26902c95633e62a985192474b5af207da6df7b4fd5fc61cd4',
            'f5435a2bd2badf7d485a4d8b8db9fcce3e1ef8e0201e4578c54673bc1dc5ea1d',
          ],
          [
            '15d9441254945064cf1a1c33bbd3b49f8966c5092171e699ef258dfab81c045c',
            'd56eb30b69463e7234f5137b73b84177434800bacebfc685fc37bbe9efe4070d',
          ],
          [
            'a1d0fcf2ec9de675b612136e5ce70d271c21417c9d2b8aaaac138599d0717940',
            'edd77f50bcb5a3cab2e90737309667f2641462a54070f3d519212d39c197a629',
          ],
          [
            'e22fbe15c0af8ccc5780c0735f84dbe9a790badee8245c06c7ca37331cb36980',
            'a855babad5cd60c88b430a69f53a1a7a38289154964799be43d06d77d31da06',
          ],
          [
            '311091dd9860e8e20ee13473c1155f5f69635e394704eaa74009452246cfa9b3',
            '66db656f87d1f04fffd1f04788c06830871ec5a64feee685bd80f0b1286d8374',
          ],
          [
            '34c1fd04d301be89b31c0442d3e6ac24883928b45a9340781867d4232ec2dbdf',
            '9414685e97b1b5954bd46f730174136d57f1ceeb487443dc5321857ba73abee',
          ],
          [
            'f219ea5d6b54701c1c14de5b557eb42a8d13f3abbcd08affcc2a5e6b049b8d63',
            '4cb95957e83d40b0f73af4544cccf6b1f4b08d3c07b27fb8d8c2962a400766d1',
          ],
          [
            'd7b8740f74a8fbaab1f683db8f45de26543a5490bca627087236912469a0b448',
            'fa77968128d9c92ee1010f337ad4717eff15db5ed3c049b3411e0315eaa4593b',
          ],
          [
            '32d31c222f8f6f0ef86f7c98d3a3335ead5bcd32abdd94289fe4d3091aa824bf',
            '5f3032f5892156e39ccd3d7915b9e1da2e6dac9e6f26e961118d14b8462e1661',
          ],
          [
            '7461f371914ab32671045a155d9831ea8793d77cd59592c4340f86cbc18347b5',
            '8ec0ba238b96bec0cbdddcae0aa442542eee1ff50c986ea6b39847b3cc092ff6',
          ],
          [
            'ee079adb1df1860074356a25aa38206a6d716b2c3e67453d287698bad7b2b2d6',
            '8dc2412aafe3be5c4c5f37e0ecc5f9f6a446989af04c4e25ebaac479ec1c8c1e',
          ],
          [
            '16ec93e447ec83f0467b18302ee620f7e65de331874c9dc72bfd8616ba9da6b5',
            '5e4631150e62fb40d0e8c2a7ca5804a39d58186a50e497139626778e25b0674d',
          ],
          [
            'eaa5f980c245f6f038978290afa70b6bd8855897f98b6aa485b96065d537bd99',
            'f65f5d3e292c2e0819a528391c994624d784869d7e6ea67fb18041024edc07dc',
          ],
          [
            '78c9407544ac132692ee1910a02439958ae04877151342ea96c4b6b35a49f51',
            'f3e0319169eb9b85d5404795539a5e68fa1fbd583c064d2462b675f194a3ddb4',
          ],
          [
            '494f4be219a1a77016dcd838431aea0001cdc8ae7a6fc688726578d9702857a5',
            '42242a969283a5f339ba7f075e36ba2af925ce30d767ed6e55f4b031880d562c',
          ],
          [
            'a598a8030da6d86c6bc7f2f5144ea549d28211ea58faa70ebf4c1e665c1fe9b5',
            '204b5d6f84822c307e4b4a7140737aec23fc63b65b35f86a10026dbd2d864e6b',
          ],
          [
            'c41916365abb2b5d09192f5f2dbeafec208f020f12570a184dbadc3e58595997',
            '4f14351d0087efa49d245b328984989d5caf9450f34bfc0ed16e96b58fa9913',
          ],
          [
            '841d6063a586fa475a724604da03bc5b92a2e0d2e0a36acfe4c73a5514742881',
            '73867f59c0659e81904f9a1c7543698e62562d6744c169ce7a36de01a8d6154',
          ],
          [
            '5e95bb399a6971d376026947f89bde2f282b33810928be4ded112ac4d70e20d5',
            '39f23f366809085beebfc71181313775a99c9aed7d8ba38b161384c746012865',
          ],
          [
            '36e4641a53948fd476c39f8a99fd974e5ec07564b5315d8bf99471bca0ef2f66',
            'd2424b1b1abe4eb8164227b085c9aa9456ea13493fd563e06fd51cf5694c78fc',
          ],
          [
            '336581ea7bfbbb290c191a2f507a41cf5643842170e914faeab27c2c579f726',
            'ead12168595fe1be99252129b6e56b3391f7ab1410cd1e0ef3dcdcabd2fda224',
          ],
          [
            '8ab89816dadfd6b6a1f2634fcf00ec8403781025ed6890c4849742706bd43ede',
            '6fdcef09f2f6d0a044e654aef624136f503d459c3e89845858a47a9129cdd24e',
          ],
          [
            '1e33f1a746c9c5778133344d9299fcaa20b0938e8acff2544bb40284b8c5fb94',
            '60660257dd11b3aa9c8ed618d24edff2306d320f1d03010e33a7d2057f3b3b6',
          ],
          [
            '85b7c1dcb3cec1b7ee7f30ded79dd20a0ed1f4cc18cbcfcfa410361fd8f08f31',
            '3d98a9cdd026dd43f39048f25a8847f4fcafad1895d7a633c6fed3c35e999511',
          ],
          [
            '29df9fbd8d9e46509275f4b125d6d45d7fbe9a3b878a7af872a2800661ac5f51',
            'b4c4fe99c775a606e2d8862179139ffda61dc861c019e55cd2876eb2a27d84b',
          ],
          [
            'a0b1cae06b0a847a3fea6e671aaf8adfdfe58ca2f768105c8082b2e449fce252',
            'ae434102edde0958ec4b19d917a6a28e6b72da1834aff0e650f049503a296cf2',
          ],
          [
            '4e8ceafb9b3e9a136dc7ff67e840295b499dfb3b2133e4ba113f2e4c0e121e5',
            'cf2174118c8b6d7a4b48f6d534ce5c79422c086a63460502b827ce62a326683c',
          ],
          [
            'd24a44e047e19b6f5afb81c7ca2f69080a5076689a010919f42725c2b789a33b',
            '6fb8d5591b466f8fc63db50f1c0f1c69013f996887b8244d2cdec417afea8fa3',
          ],
          [
            'ea01606a7a6c9cdd249fdfcfacb99584001edd28abbab77b5104e98e8e3b35d4',
            '322af4908c7312b0cfbfe369f7a7b3cdb7d4494bc2823700cfd652188a3ea98d',
          ],
          [
            'af8addbf2b661c8a6c6328655eb96651252007d8c5ea31be4ad196de8ce2131f',
            '6749e67c029b85f52a034eafd096836b2520818680e26ac8f3dfbcdb71749700',
          ],
          [
            'e3ae1974566ca06cc516d47e0fb165a674a3dabcfca15e722f0e3450f45889',
            '2aeabe7e4531510116217f07bf4d07300de97e4874f81f533420a72eeb0bd6a4',
          ],
          [
            '591ee355313d99721cf6993ffed1e3e301993ff3ed258802075ea8ced397e246',
            'b0ea558a113c30bea60fc4775460c7901ff0b053d25ca2bdeee98f1a4be5d196',
          ],
          [
            '11396d55fda54c49f19aa97318d8da61fa8584e47b084945077cf03255b52984',
            '998c74a8cd45ac01289d5833a7beb4744ff536b01b257be4c5767bea93ea57a4',
          ],
          [
            '3c5d2a1ba39c5a1790000738c9e0c40b8dcdfd5468754b6405540157e017aa7a',
            'b2284279995a34e2f9d4de7396fc18b80f9b8b9fdd270f6661f79ca4c81bd257',
          ],
          [
            'cc8704b8a60a0defa3a99a7299f2e9c3fbc395afb04ac078425ef8a1793cc030',
            'bdd46039feed17881d1e0862db347f8cf395b74fc4bcdc4e940b74e3ac1f1b13',
          ],
          [
            'c533e4f7ea8555aacd9777ac5cad29b97dd4defccc53ee7ea204119b2889b197',
            '6f0a256bc5efdf429a2fb6242f1a43a2d9b925bb4a4b3a26bb8e0f45eb596096',
          ],
          [
            'c14f8f2ccb27d6f109f6d08d03cc96a69ba8c34eec07bbcf566d48e33da6593',
            'c359d6923bb398f7fd4473e16fe1c28475b740dd098075e6c0e8649113dc3a38',
          ],
          [
            'a6cbc3046bc6a450bac24789fa17115a4c9739ed75f8f21ce441f72e0b90e6ef',
            '21ae7f4680e889bb130619e2c0f95a360ceb573c70603139862afd617fa9b9f',
          ],
          [
            '347d6d9a02c48927ebfb86c1359b1caf130a3c0267d11ce6344b39f99d43cc38',
            '60ea7f61a353524d1c987f6ecec92f086d565ab687870cb12689ff1e31c74448',
          ],
          [
            'da6545d2181db8d983f7dcb375ef5866d47c67b1bf31c8cf855ef7437b72656a',
            '49b96715ab6878a79e78f07ce5680c5d6673051b4935bd897fea824b77dc208a',
          ],
          [
            'c40747cc9d012cb1a13b8148309c6de7ec25d6945d657146b9d5994b8feb1111',
            '5ca560753be2a12fc6de6caf2cb489565db936156b9514e1bb5e83037e0fa2d4',
          ],
          [
            '4e42c8ec82c99798ccf3a610be870e78338c7f713348bd34c8203ef4037f3502',
            '7571d74ee5e0fb92a7a8b33a07783341a5492144cc54bcc40a94473693606437',
          ],
          [
            '3775ab7089bc6af823aba2e1af70b236d251cadb0c86743287522a1b3b0dedea',
            'be52d107bcfa09d8bcb9736a828cfa7fac8db17bf7a76a2c42ad961409018cf7',
          ],
          [
            'cee31cbf7e34ec379d94fb814d3d775ad954595d1314ba8846959e3e82f74e26',
            '8fd64a14c06b589c26b947ae2bcf6bfa0149ef0be14ed4d80f448a01c43b1c6d',
          ],
          [
            'b4f9eaea09b6917619f6ea6a4eb5464efddb58fd45b1ebefcdc1a01d08b47986',
            '39e5c9925b5a54b07433a4f18c61726f8bb131c012ca542eb24a8ac07200682a',
          ],
          [
            'd4263dfc3d2df923a0179a48966d30ce84e2515afc3dccc1b77907792ebcc60e',
            '62dfaf07a0f78feb30e30d6295853ce189e127760ad6cf7fae164e122a208d54',
          ],
          [
            '48457524820fa65a4f8d35eb6930857c0032acc0a4a2de422233eeda897612c4',
            '25a748ab367979d98733c38a1fa1c2e7dc6cc07db2d60a9ae7a76aaa49bd0f77',
          ],
          [
            'dfeeef1881101f2cb11644f3a2afdfc2045e19919152923f367a1767c11cceda',
            'ecfb7056cf1de042f9420bab396793c0c390bde74b4bbdff16a83ae09a9a7517',
          ],
          [
            '6d7ef6b17543f8373c573f44e1f389835d89bcbc6062ced36c82df83b8fae859',
            'cd450ec335438986dfefa10c57fea9bcc521a0959b2d80bbf74b190dca712d10',
          ],
          [
            'e75605d59102a5a2684500d3b991f2e3f3c88b93225547035af25af66e04541f',
            'f5c54754a8f71ee540b9b48728473e314f729ac5308b06938360990e2bfad125',
          ],
          [
            'eb98660f4c4dfaa06a2be453d5020bc99a0c2e60abe388457dd43fefb1ed620c',
            '6cb9a8876d9cb8520609af3add26cd20a0a7cd8a9411131ce85f44100099223e',
          ],
          [
            '13e87b027d8514d35939f2e6892b19922154596941888336dc3563e3b8dba942',
            'fef5a3c68059a6dec5d624114bf1e91aac2b9da568d6abeb2570d55646b8adf1',
          ],
          [
            'ee163026e9fd6fe017c38f06a5be6fc125424b371ce2708e7bf4491691e5764a',
            '1acb250f255dd61c43d94ccc670d0f58f49ae3fa15b96623e5430da0ad6c62b2',
          ],
          [
            'b268f5ef9ad51e4d78de3a750c2dc89b1e626d43505867999932e5db33af3d80',
            '5f310d4b3c99b9ebb19f77d41c1dee018cf0d34fd4191614003e945a1216e423',
          ],
          [
            'ff07f3118a9df035e9fad85eb6c7bfe42b02f01ca99ceea3bf7ffdba93c4750d',
            '438136d603e858a3a5c440c38eccbaddc1d2942114e2eddd4740d098ced1f0d8',
          ],
          [
            '8d8b9855c7c052a34146fd20ffb658bea4b9f69e0d825ebec16e8c3ce2b526a1',
            'cdb559eedc2d79f926baf44fb84ea4d44bcf50fee51d7ceb30e2e7f463036758',
          ],
          [
            '52db0b5384dfbf05bfa9d472d7ae26dfe4b851ceca91b1eba54263180da32b63',
            'c3b997d050ee5d423ebaf66a6db9f57b3180c902875679de924b69d84a7b375',
          ],
          [
            'e62f9490d3d51da6395efd24e80919cc7d0f29c3f3fa48c6fff543becbd43352',
            '6d89ad7ba4876b0b22c2ca280c682862f342c8591f1daf5170e07bfd9ccafa7d',
          ],
          [
            '7f30ea2476b399b4957509c88f77d0191afa2ff5cb7b14fd6d8e7d65aaab1193',
            'ca5ef7d4b231c94c3b15389a5f6311e9daff7bb67b103e9880ef4bff637acaec',
          ],
          [
            '5098ff1e1d9f14fb46a210fada6c903fef0fb7b4a1dd1d9ac60a0361800b7a00',
            '9731141d81fc8f8084d37c6e7542006b3ee1b40d60dfe5362a5b132fd17ddc0',
          ],
          [
            '32b78c7de9ee512a72895be6b9cbefa6e2f3c4ccce445c96b9f2c81e2778ad58',
            'ee1849f513df71e32efc3896ee28260c73bb80547ae2275ba497237794c8753c',
          ],
          [
            'e2cb74fddc8e9fbcd076eef2a7c72b0ce37d50f08269dfc074b581550547a4f7',
            'd3aa2ed71c9dd2247a62df062736eb0baddea9e36122d2be8641abcb005cc4a4',
          ],
          [
            '8438447566d4d7bedadc299496ab357426009a35f235cb141be0d99cd10ae3a8',
            'c4e1020916980a4da5d01ac5e6ad330734ef0d7906631c4f2390426b2edd791f',
          ],
          [
            '4162d488b89402039b584c6fc6c308870587d9c46f660b878ab65c82c711d67e',
            '67163e903236289f776f22c25fb8a3afc1732f2b84b4e95dbda47ae5a0852649',
          ],
          [
            '3fad3fa84caf0f34f0f89bfd2dcf54fc175d767aec3e50684f3ba4a4bf5f683d',
            'cd1bc7cb6cc407bb2f0ca647c718a730cf71872e7d0d2a53fa20efcdfe61826',
          ],
          [
            '674f2600a3007a00568c1a7ce05d0816c1fb84bf1370798f1c69532faeb1a86b',
            '299d21f9413f33b3edf43b257004580b70db57da0b182259e09eecc69e0d38a5',
          ],
          [
            'd32f4da54ade74abb81b815ad1fb3b263d82d6c692714bcff87d29bd5ee9f08f',
            'f9429e738b8e53b968e99016c059707782e14f4535359d582fc416910b3eea87',
          ],
          [
            '30e4e670435385556e593657135845d36fbb6931f72b08cb1ed954f1e3ce3ff6',
            '462f9bce619898638499350113bbc9b10a878d35da70740dc695a559eb88db7b',
          ],
          [
            'be2062003c51cc3004682904330e4dee7f3dcd10b01e580bf1971b04d4cad297',
            '62188bc49d61e5428573d48a74e1c655b1c61090905682a0d5558ed72dccb9bc',
          ],
          [
            '93144423ace3451ed29e0fb9ac2af211cb6e84a601df5993c419859fff5df04a',
            '7c10dfb164c3425f5c71a3f9d7992038f1065224f72bb9d1d902a6d13037b47c',
          ],
          [
            'b015f8044f5fcbdcf21ca26d6c34fb8197829205c7b7d2a7cb66418c157b112c',
            'ab8c1e086d04e813744a655b2df8d5f83b3cdc6faa3088c1d3aea1454e3a1d5f',
          ],
          [
            'd5e9e1da649d97d89e4868117a465a3a4f8a18de57a140d36b3f2af341a21b52',
            '4cb04437f391ed73111a13cc1d4dd0db1693465c2240480d8955e8592f27447a',
          ],
          [
            'd3ae41047dd7ca065dbf8ed77b992439983005cd72e16d6f996a5316d36966bb',
            'bd1aeb21ad22ebb22a10f0303417c6d964f8cdd7df0aca614b10dc14d125ac46',
          ],
          [
            '463e2763d885f958fc66cdd22800f0a487197d0a82e377b49f80af87c897b065',
            'bfefacdb0e5d0fd7df3a311a94de062b26b80c61fbc97508b79992671ef7ca7f',
          ],
          [
            '7985fdfd127c0567c6f53ec1bb63ec3158e597c40bfe747c83cddfc910641917',
            '603c12daf3d9862ef2b25fe1de289aed24ed291e0ec6708703a5bd567f32ed03',
          ],
          [
            '74a1ad6b5f76e39db2dd249410eac7f99e74c59cb83d2d0ed5ff1543da7703e9',
            'cc6157ef18c9c63cd6193d83631bbea0093e0968942e8c33d5737fd790e0db08',
          ],
          [
            '30682a50703375f602d416664ba19b7fc9bab42c72747463a71d0896b22f6da3',
            '553e04f6b018b4fa6c8f39e7f311d3176290d0e0f19ca73f17714d9977a22ff8',
          ],
          [
            '9e2158f0d7c0d5f26c3791efefa79597654e7a2b2464f52b1ee6c1347769ef57',
            '712fcdd1b9053f09003a3481fa7762e9ffd7c8ef35a38509e2fbf2629008373',
          ],
          [
            '176e26989a43c9cfeba4029c202538c28172e566e3c4fce7322857f3be327d66',
            'ed8cc9d04b29eb877d270b4878dc43c19aefd31f4eee09ee7b47834c1fa4b1c3',
          ],
          [
            '75d46efea3771e6e68abb89a13ad747ecf1892393dfc4f1b7004788c50374da8',
            '9852390a99507679fd0b86fd2b39a868d7efc22151346e1a3ca4726586a6bed8',
          ],
          [
            '809a20c67d64900ffb698c4c825f6d5f2310fb0451c869345b7319f645605721',
            '9e994980d9917e22b76b061927fa04143d096ccc54963e6a5ebfa5f3f8e286c1',
          ],
          [
            '1b38903a43f7f114ed4500b4eac7083fdefece1cf29c63528d563446f972c180',
            '4036edc931a60ae889353f77fd53de4a2708b26b6f5da72ad3394119daf408f9',
          ],
        ],
      },
    };
  });
  var ys = R((m2) => {
    'use strict';
    S();
    var Tu = m2,
      qi = bs(),
      qu = _u(),
      Mx = Bt(),
      b2 = Mx.assert;
    function y2(t) {
      t.type === 'short'
        ? (this.curve = new qu.short(t))
        : t.type === 'edwards'
          ? (this.curve = new qu.edwards(t))
          : (this.curve = new qu.mont(t)),
        (this.g = this.curve.g),
        (this.n = this.curve.n),
        (this.hash = t.hash),
        b2(this.g.validate(), 'Invalid curve'),
        b2(this.g.mul(this.n).isInfinity(), 'Invalid curve, G*N != O');
    }
    Tu.PresetCurve = y2;
    function Ti(t, e) {
      Object.defineProperty(Tu, t, {
        configurable: !0,
        enumerable: !0,
        get: function () {
          var r = new y2(e);
          return (
            Object.defineProperty(Tu, t, {
              configurable: !0,
              enumerable: !0,
              value: r,
            }),
            r
          );
        },
      });
    }
    Ti('p192', {
      type: 'short',
      prime: 'p192',
      p: 'ffffffff ffffffff ffffffff fffffffe ffffffff ffffffff',
      a: 'ffffffff ffffffff ffffffff fffffffe ffffffff fffffffc',
      b: '64210519 e59c80e7 0fa7e9ab 72243049 feb8deec c146b9b1',
      n: 'ffffffff ffffffff ffffffff 99def836 146bc9b1 b4d22831',
      hash: qi.sha256,
      gRed: !1,
      g: [
        '188da80e b03090f6 7cbf20eb 43a18800 f4ff0afd 82ff1012',
        '07192b95 ffc8da78 631011ed 6b24cdd5 73f977a1 1e794811',
      ],
    });
    Ti('p224', {
      type: 'short',
      prime: 'p224',
      p: 'ffffffff ffffffff ffffffff ffffffff 00000000 00000000 00000001',
      a: 'ffffffff ffffffff ffffffff fffffffe ffffffff ffffffff fffffffe',
      b: 'b4050a85 0c04b3ab f5413256 5044b0b7 d7bfd8ba 270b3943 2355ffb4',
      n: 'ffffffff ffffffff ffffffff ffff16a2 e0b8f03e 13dd2945 5c5c2a3d',
      hash: qi.sha256,
      gRed: !1,
      g: [
        'b70e0cbd 6bb4bf7f 321390b9 4a03c1d3 56c21122 343280d6 115c1d21',
        'bd376388 b5f723fb 4c22dfe6 cd4375a0 5a074764 44d58199 85007e34',
      ],
    });
    Ti('p256', {
      type: 'short',
      prime: null,
      p: 'ffffffff 00000001 00000000 00000000 00000000 ffffffff ffffffff ffffffff',
      a: 'ffffffff 00000001 00000000 00000000 00000000 ffffffff ffffffff fffffffc',
      b: '5ac635d8 aa3a93e7 b3ebbd55 769886bc 651d06b0 cc53b0f6 3bce3c3e 27d2604b',
      n: 'ffffffff 00000000 ffffffff ffffffff bce6faad a7179e84 f3b9cac2 fc632551',
      hash: qi.sha256,
      gRed: !1,
      g: [
        '6b17d1f2 e12c4247 f8bce6e5 63a440f2 77037d81 2deb33a0 f4a13945 d898c296',
        '4fe342e2 fe1a7f9b 8ee7eb4a 7c0f9e16 2bce3357 6b315ece cbb64068 37bf51f5',
      ],
    });
    Ti('p384', {
      type: 'short',
      prime: null,
      p: 'ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff fffffffe ffffffff 00000000 00000000 ffffffff',
      a: 'ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff fffffffe ffffffff 00000000 00000000 fffffffc',
      b: 'b3312fa7 e23ee7e4 988e056b e3f82d19 181d9c6e fe814112 0314088f 5013875a c656398d 8a2ed19d 2a85c8ed d3ec2aef',
      n: 'ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff c7634d81 f4372ddf 581a0db2 48b0a77a ecec196a ccc52973',
      hash: qi.sha384,
      gRed: !1,
      g: [
        'aa87ca22 be8b0537 8eb1c71e f320ad74 6e1d3b62 8ba79b98 59f741e0 82542a38 5502f25d bf55296c 3a545e38 72760ab7',
        '3617de4a 96262c6f 5d9e98bf 9292dc29 f8f41dbd 289a147c e9da3113 b5f0b8c0 0a60b1ce 1d7e819d 7a431d7c 90ea0e5f',
      ],
    });
    Ti('p521', {
      type: 'short',
      prime: null,
      p: '000001ff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff',
      a: '000001ff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff fffffffc',
      b: '00000051 953eb961 8e1c9a1f 929a21a0 b68540ee a2da725b 99b315f3 b8b48991 8ef109e1 56193951 ec7e937b 1652c0bd 3bb1bf07 3573df88 3d2c34f1 ef451fd4 6b503f00',
      n: '000001ff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff fffffffa 51868783 bf2f966b 7fcc0148 f709a5d0 3bb5c9b8 899c47ae bb6fb71e 91386409',
      hash: qi.sha512,
      gRed: !1,
      g: [
        '000000c6 858e06b7 0404e9cd 9e3ecb66 2395b442 9c648139 053fb521 f828af60 6b4d3dba a14b5e77 efe75928 fe1dc127 a2ffa8de 3348b3c1 856a429b f97e7e31 c2e5bd66',
        '00000118 39296a78 9a3bc004 5c8a5fb4 2c7d1bd9 98f54449 579b4468 17afbd17 273e662c 97ee7299 5ef42640 c550b901 3fad0761 353c7086 a272c240 88be9476 9fd16650',
      ],
    });
    Ti('curve25519', {
      type: 'mont',
      prime: 'p25519',
      p: '7fffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffed',
      a: '76d06',
      b: '1',
      n: '1000000000000000 0000000000000000 14def9dea2f79cd6 5812631a5cf5d3ed',
      hash: qi.sha256,
      gRed: !1,
      g: ['9'],
    });
    Ti('ed25519', {
      type: 'edwards',
      prime: 'p25519',
      p: '7fffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffed',
      a: '-1',
      c: '1',
      d: '52036cee2b6ffe73 8cc740797779e898 00700a4d4141d8ab 75eb4dca135978a3',
      n: '1000000000000000 0000000000000000 14def9dea2f79cd6 5812631a5cf5d3ed',
      hash: qi.sha256,
      gRed: !1,
      g: [
        '216936d3cd6e53fec0a4e231fdd6dc5c692cc7609525a7b2c9562d608f25d51a',
        '6666666666666666666666666666666666666666666666666666666666666658',
      ],
    });
    var Pu;
    try {
      Pu = v2();
    } catch {
      Pu = void 0;
    }
    Ti('secp256k1', {
      type: 'short',
      prime: 'k256',
      p: 'ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff fffffffe fffffc2f',
      a: '0',
      b: '7',
      n: 'ffffffff ffffffff ffffffff fffffffe baaedce6 af48a03b bfd25e8c d0364141',
      h: '1',
      hash: qi.sha256,
      beta: '7ae96a2b657c07106e64479eac3434e99cf0497512f58995c1396c28719501ee',
      lambda:
        '5363ad4cc05c30e0a5261c028812645a122e22ea20816678df02967c1b23bd72',
      basis: [
        {
          a: '3086d221a7d46bcde86c90e49284eb15',
          b: '-e4437ed6010e88286f547fa90abfe4c3',
        },
        {
          a: '114ca50f7a8e2f3f657c1108d9d44cfd8',
          b: '3086d221a7d46bcde86c90e49284eb15',
        },
      ],
      gRed: !1,
      g: [
        '79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798',
        '483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8',
        Pu,
      ],
    });
  });
  var _2 = R((UP, w2) => {
    'use strict';
    S();
    var Sx = bs(),
      dn = gu(),
      g2 = At();
    function Pi(t) {
      if (!(this instanceof Pi)) return new Pi(t);
      (this.hash = t.hash),
        (this.predResist = !!t.predResist),
        (this.outLen = this.hash.outSize),
        (this.minEntropy = t.minEntropy || this.hash.hmacStrength),
        (this._reseed = null),
        (this.reseedInterval = null),
        (this.K = null),
        (this.V = null);
      var e = dn.toArray(t.entropy, t.entropyEnc || 'hex'),
        r = dn.toArray(t.nonce, t.nonceEnc || 'hex'),
        n = dn.toArray(t.pers, t.persEnc || 'hex');
      g2(
        e.length >= this.minEntropy / 8,
        'Not enough entropy. Minimum is: ' + this.minEntropy + ' bits'
      ),
        this._init(e, r, n);
    }
    w2.exports = Pi;
    Pi.prototype._init = function (e, r, n) {
      var i = e.concat(r).concat(n);
      (this.K = new Array(this.outLen / 8)),
        (this.V = new Array(this.outLen / 8));
      for (var a = 0; a < this.V.length; a++) (this.K[a] = 0), (this.V[a] = 1);
      this._update(i),
        (this._reseed = 1),
        (this.reseedInterval = 281474976710656);
    };
    Pi.prototype._hmac = function () {
      return new Sx.hmac(this.hash, this.K);
    };
    Pi.prototype._update = function (e) {
      var r = this._hmac().update(this.V).update([0]);
      e && (r = r.update(e)),
        (this.K = r.digest()),
        (this.V = this._hmac().update(this.V).digest()),
        e &&
          ((this.K = this._hmac()
            .update(this.V)
            .update([1])
            .update(e)
            .digest()),
          (this.V = this._hmac().update(this.V).digest()));
    };
    Pi.prototype.reseed = function (e, r, n, i) {
      typeof r != 'string' && ((i = n), (n = r), (r = null)),
        (e = dn.toArray(e, r)),
        (n = dn.toArray(n, i)),
        g2(
          e.length >= this.minEntropy / 8,
          'Not enough entropy. Minimum is: ' + this.minEntropy + ' bits'
        ),
        this._update(e.concat(n || [])),
        (this._reseed = 1);
    };
    Pi.prototype.generate = function (e, r, n, i) {
      if (this._reseed > this.reseedInterval)
        throw new Error('Reseed is required');
      typeof r != 'string' && ((i = n), (n = r), (r = null)),
        n && ((n = dn.toArray(n, i || 'hex')), this._update(n));
      for (var a = []; a.length < e; )
        (this.V = this._hmac().update(this.V).digest()), (a = a.concat(this.V));
      var h = a.slice(0, e);
      return this._update(n), this._reseed++, dn.encode(h, r);
    };
  });
  var M2 = R((HP, x2) => {
    'use strict';
    S();
    var Ex = it(),
      Ax = Bt(),
      ku = Ax.assert;
    function bt(t, e) {
      (this.ec = t),
        (this.priv = null),
        (this.pub = null),
        e.priv && this._importPrivate(e.priv, e.privEnc),
        e.pub && this._importPublic(e.pub, e.pubEnc);
    }
    x2.exports = bt;
    bt.fromPublic = function (e, r, n) {
      return r instanceof bt ? r : new bt(e, { pub: r, pubEnc: n });
    };
    bt.fromPrivate = function (e, r, n) {
      return r instanceof bt ? r : new bt(e, { priv: r, privEnc: n });
    };
    bt.prototype.validate = function () {
      var e = this.getPublic();
      return e.isInfinity()
        ? { result: !1, reason: 'Invalid public key' }
        : e.validate()
          ? e.mul(this.ec.curve.n).isInfinity()
            ? { result: !0, reason: null }
            : { result: !1, reason: 'Public key * N != O' }
          : { result: !1, reason: 'Public key is not a point' };
    };
    bt.prototype.getPublic = function (e, r) {
      return (
        typeof e == 'string' && ((r = e), (e = null)),
        this.pub || (this.pub = this.ec.g.mul(this.priv)),
        r ? this.pub.encode(r, e) : this.pub
      );
    };
    bt.prototype.getPrivate = function (e) {
      return e === 'hex' ? this.priv.toString(16, 2) : this.priv;
    };
    bt.prototype._importPrivate = function (e, r) {
      (this.priv = new Ex(e, r || 16)),
        (this.priv = this.priv.umod(this.ec.curve.n));
    };
    bt.prototype._importPublic = function (e, r) {
      if (e.x || e.y) {
        this.ec.curve.type === 'mont'
          ? ku(e.x, 'Need x coordinate')
          : (this.ec.curve.type === 'short' ||
              this.ec.curve.type === 'edwards') &&
            ku(e.x && e.y, 'Need both x and y coordinate'),
          (this.pub = this.ec.curve.point(e.x, e.y));
        return;
      }
      this.pub = this.ec.curve.decodePoint(e, r);
    };
    bt.prototype.derive = function (e) {
      return (
        e.validate() || ku(e.validate(), 'public point not validated'),
        e.mul(this.priv).getX()
      );
    };
    bt.prototype.sign = function (e, r, n) {
      return this.ec.sign(e, this, r, n);
    };
    bt.prototype.verify = function (e, r) {
      return this.ec.verify(e, r, this);
    };
    bt.prototype.inspect = function () {
      return (
        '<Key priv: ' +
        (this.priv && this.priv.toString(16, 2)) +
        ' pub: ' +
        (this.pub && this.pub.inspect()) +
        ' >'
      );
    };
  });
  var A2 = R((VP, E2) => {
    'use strict';
    S();
    var ms = it(),
      Nu = Bt(),
      Bx = Nu.assert;
    function gs(t, e) {
      if (t instanceof gs) return t;
      this._importDER(t, e) ||
        (Bx(t.r && t.s, 'Signature without r or s'),
        (this.r = new ms(t.r, 16)),
        (this.s = new ms(t.s, 16)),
        t.recoveryParam === void 0
          ? (this.recoveryParam = null)
          : (this.recoveryParam = t.recoveryParam));
    }
    E2.exports = gs;
    function Ix() {
      this.place = 0;
    }
    function Ou(t, e) {
      var r = t[e.place++];
      if (!(r & 128)) return r;
      var n = r & 15;
      if (n === 0 || n > 4) return !1;
      for (var i = 0, a = 0, h = e.place; a < n; a++, h++)
        (i <<= 8), (i |= t[h]), (i >>>= 0);
      return i <= 127 ? !1 : ((e.place = h), i);
    }
    function S2(t) {
      for (var e = 0, r = t.length - 1; !t[e] && !(t[e + 1] & 128) && e < r; )
        e++;
      return e === 0 ? t : t.slice(e);
    }
    gs.prototype._importDER = function (e, r) {
      e = Nu.toArray(e, r);
      var n = new Ix();
      if (e[n.place++] !== 48) return !1;
      var i = Ou(e, n);
      if (i === !1 || i + n.place !== e.length || e[n.place++] !== 2) return !1;
      var a = Ou(e, n);
      if (a === !1) return !1;
      var h = e.slice(n.place, a + n.place);
      if (((n.place += a), e[n.place++] !== 2)) return !1;
      var v = Ou(e, n);
      if (v === !1 || e.length !== v + n.place) return !1;
      var g = e.slice(n.place, v + n.place);
      if (h[0] === 0)
        if (h[1] & 128) h = h.slice(1);
        else return !1;
      if (g[0] === 0)
        if (g[1] & 128) g = g.slice(1);
        else return !1;
      return (
        (this.r = new ms(h)),
        (this.s = new ms(g)),
        (this.recoveryParam = null),
        !0
      );
    };
    function Cu(t, e) {
      if (e < 128) {
        t.push(e);
        return;
      }
      var r = 1 + ((Math.log(e) / Math.LN2) >>> 3);
      for (t.push(r | 128); --r; ) t.push((e >>> (r << 3)) & 255);
      t.push(e);
    }
    gs.prototype.toDER = function (e) {
      var r = this.r.toArray(),
        n = this.s.toArray();
      for (
        r[0] & 128 && (r = [0].concat(r)),
          n[0] & 128 && (n = [0].concat(n)),
          r = S2(r),
          n = S2(n);
        !n[0] && !(n[1] & 128);

      )
        n = n.slice(1);
      var i = [2];
      Cu(i, r.length), (i = i.concat(r)), i.push(2), Cu(i, n.length);
      var a = i.concat(n),
        h = [48];
      return Cu(h, a.length), (h = h.concat(a)), Nu.encode(h, e);
    };
  });
  var q2 = R((WP, R2) => {
    'use strict';
    S();
    var ln = it(),
      B2 = _2(),
      Rx = Bt(),
      Du = ys(),
      qx = is(),
      I2 = Rx.assert,
      Lu = M2(),
      ws = A2();
    function Ut(t) {
      if (!(this instanceof Ut)) return new Ut(t);
      typeof t == 'string' &&
        (I2(Object.prototype.hasOwnProperty.call(Du, t), 'Unknown curve ' + t),
        (t = Du[t])),
        t instanceof Du.PresetCurve && (t = { curve: t }),
        (this.curve = t.curve.curve),
        (this.n = this.curve.n),
        (this.nh = this.n.ushrn(1)),
        (this.g = this.curve.g),
        (this.g = t.curve.g),
        this.g.precompute(t.curve.n.bitLength() + 1),
        (this.hash = t.hash || t.curve.hash);
    }
    R2.exports = Ut;
    Ut.prototype.keyPair = function (e) {
      return new Lu(this, e);
    };
    Ut.prototype.keyFromPrivate = function (e, r) {
      return Lu.fromPrivate(this, e, r);
    };
    Ut.prototype.keyFromPublic = function (e, r) {
      return Lu.fromPublic(this, e, r);
    };
    Ut.prototype.genKeyPair = function (e) {
      e || (e = {});
      for (
        var r = new B2({
            hash: this.hash,
            pers: e.pers,
            persEnc: e.persEnc || 'utf8',
            entropy: e.entropy || qx(this.hash.hmacStrength),
            entropyEnc: (e.entropy && e.entropyEnc) || 'utf8',
            nonce: this.n.toArray(),
          }),
          n = this.n.byteLength(),
          i = this.n.sub(new ln(2));
        ;

      ) {
        var a = new ln(r.generate(n));
        if (!(a.cmp(i) > 0)) return a.iaddn(1), this.keyFromPrivate(a);
      }
    };
    Ut.prototype._truncateToN = function (e, r) {
      var n = e.byteLength() * 8 - this.n.bitLength();
      return (
        n > 0 && (e = e.ushrn(n)), !r && e.cmp(this.n) >= 0 ? e.sub(this.n) : e
      );
    };
    Ut.prototype.sign = function (e, r, n, i) {
      typeof n == 'object' && ((i = n), (n = null)),
        i || (i = {}),
        (r = this.keyFromPrivate(r, n)),
        (e = this._truncateToN(new ln(e, 16)));
      for (
        var a = this.n.byteLength(),
          h = r.getPrivate().toArray('be', a),
          v = e.toArray('be', a),
          g = new B2({
            hash: this.hash,
            entropy: h,
            nonce: v,
            pers: i.pers,
            persEnc: i.persEnc || 'utf8',
          }),
          M = this.n.sub(new ln(1)),
          x = 0;
        ;
        x++
      ) {
        var E = i.k ? i.k(x) : new ln(g.generate(this.n.byteLength()));
        if (
          ((E = this._truncateToN(E, !0)), !(E.cmpn(1) <= 0 || E.cmp(M) >= 0))
        ) {
          var I = this.g.mul(E);
          if (!I.isInfinity()) {
            var q = I.getX(),
              k = q.umod(this.n);
            if (k.cmpn(0) !== 0) {
              var L = E.invm(this.n).mul(k.mul(r.getPrivate()).iadd(e));
              if (((L = L.umod(this.n)), L.cmpn(0) !== 0)) {
                var xe = (I.getY().isOdd() ? 1 : 0) | (q.cmp(k) !== 0 ? 2 : 0);
                return (
                  i.canonical &&
                    L.cmp(this.nh) > 0 &&
                    ((L = this.n.sub(L)), (xe ^= 1)),
                  new ws({ r: k, s: L, recoveryParam: xe })
                );
              }
            }
          }
        }
      }
    };
    Ut.prototype.verify = function (e, r, n, i) {
      (e = this._truncateToN(new ln(e, 16))),
        (n = this.keyFromPublic(n, i)),
        (r = new ws(r, 'hex'));
      var a = r.r,
        h = r.s;
      if (
        a.cmpn(1) < 0 ||
        a.cmp(this.n) >= 0 ||
        h.cmpn(1) < 0 ||
        h.cmp(this.n) >= 0
      )
        return !1;
      var v = h.invm(this.n),
        g = v.mul(e).umod(this.n),
        M = v.mul(a).umod(this.n),
        x;
      return this.curve._maxwellTrick
        ? ((x = this.g.jmulAdd(g, n.getPublic(), M)),
          x.isInfinity() ? !1 : x.eqXToP(a))
        : ((x = this.g.mulAdd(g, n.getPublic(), M)),
          x.isInfinity() ? !1 : x.getX().umod(this.n).cmp(a) === 0);
    };
    Ut.prototype.recoverPubKey = function (t, e, r, n) {
      I2((3 & r) === r, 'The recovery param is more than two bits'),
        (e = new ws(e, n));
      var i = this.n,
        a = new ln(t),
        h = e.r,
        v = e.s,
        g = r & 1,
        M = r >> 1;
      if (h.cmp(this.curve.p.umod(this.curve.n)) >= 0 && M)
        throw new Error('Unable to find sencond key candinate');
      M
        ? (h = this.curve.pointFromX(h.add(this.curve.n), g))
        : (h = this.curve.pointFromX(h, g));
      var x = e.r.invm(i),
        E = i.sub(a).mul(x).umod(i),
        I = v.mul(x).umod(i);
      return this.g.mulAdd(E, h, I);
    };
    Ut.prototype.getKeyRecoveryParam = function (t, e, r, n) {
      if (((e = new ws(e, n)), e.recoveryParam !== null))
        return e.recoveryParam;
      for (var i = 0; i < 4; i++) {
        var a;
        try {
          a = this.recoverPubKey(t, e, i);
        } catch {
          continue;
        }
        if (a.eq(r)) return i;
      }
      throw new Error('Unable to find valid recovery factor');
    };
  });
  var O2 = R((ZP, k2) => {
    'use strict';
    S();
    var va = Bt(),
      P2 = va.assert,
      T2 = va.parseBytes,
      af = va.cachedProperty;
    function st(t, e) {
      (this.eddsa = t),
        (this._secret = T2(e.secret)),
        t.isPoint(e.pub) ? (this._pub = e.pub) : (this._pubBytes = T2(e.pub));
    }
    st.fromPublic = function (e, r) {
      return r instanceof st ? r : new st(e, { pub: r });
    };
    st.fromSecret = function (e, r) {
      return r instanceof st ? r : new st(e, { secret: r });
    };
    st.prototype.secret = function () {
      return this._secret;
    };
    af(st, 'pubBytes', function () {
      return this.eddsa.encodePoint(this.pub());
    });
    af(st, 'pub', function () {
      return this._pubBytes
        ? this.eddsa.decodePoint(this._pubBytes)
        : this.eddsa.g.mul(this.priv());
    });
    af(st, 'privBytes', function () {
      var e = this.eddsa,
        r = this.hash(),
        n = e.encodingLength - 1,
        i = r.slice(0, e.encodingLength);
      return (i[0] &= 248), (i[n] &= 127), (i[n] |= 64), i;
    });
    af(st, 'priv', function () {
      return this.eddsa.decodeInt(this.privBytes());
    });
    af(st, 'hash', function () {
      return this.eddsa.hash().update(this.secret()).digest();
    });
    af(st, 'messagePrefix', function () {
      return this.hash().slice(this.eddsa.encodingLength);
    });
    st.prototype.sign = function (e) {
      return (
        P2(this._secret, 'KeyPair can only verify'), this.eddsa.sign(e, this)
      );
    };
    st.prototype.verify = function (e, r) {
      return this.eddsa.verify(e, r, this);
    };
    st.prototype.getSecret = function (e) {
      return (
        P2(this._secret, 'KeyPair is public only'), va.encode(this.secret(), e)
      );
    };
    st.prototype.getPublic = function (e) {
      return va.encode(this.pubBytes(), e);
    };
    k2.exports = st;
  });
  var N2 = R((XP, C2) => {
    'use strict';
    S();
    var Tx = it(),
      _s = Bt(),
      Px = _s.assert,
      xs = _s.cachedProperty,
      kx = _s.parseBytes;
    function pn(t, e) {
      (this.eddsa = t),
        typeof e != 'object' && (e = kx(e)),
        Array.isArray(e) &&
          (e = {
            R: e.slice(0, t.encodingLength),
            S: e.slice(t.encodingLength),
          }),
        Px(e.R && e.S, 'Signature without R or S'),
        t.isPoint(e.R) && (this._R = e.R),
        e.S instanceof Tx && (this._S = e.S),
        (this._Rencoded = Array.isArray(e.R) ? e.R : e.Rencoded),
        (this._Sencoded = Array.isArray(e.S) ? e.S : e.Sencoded);
    }
    xs(pn, 'S', function () {
      return this.eddsa.decodeInt(this.Sencoded());
    });
    xs(pn, 'R', function () {
      return this.eddsa.decodePoint(this.Rencoded());
    });
    xs(pn, 'Rencoded', function () {
      return this.eddsa.encodePoint(this.R());
    });
    xs(pn, 'Sencoded', function () {
      return this.eddsa.encodeInt(this.S());
    });
    pn.prototype.toBytes = function () {
      return this.Rencoded().concat(this.Sencoded());
    };
    pn.prototype.toHex = function () {
      return _s.encode(this.toBytes(), 'hex').toUpperCase();
    };
    C2.exports = pn;
  });
  var U2 = R((QP, j2) => {
    'use strict';
    S();
    var Ox = bs(),
      Cx = ys(),
      of = Bt(),
      Nx = of.assert,
      L2 = of.parseBytes,
      F2 = O2(),
      D2 = N2();
    function Mt(t) {
      if (
        (Nx(t === 'ed25519', 'only tested with ed25519 so far'),
        !(this instanceof Mt))
      )
        return new Mt(t);
      (t = Cx[t].curve),
        (this.curve = t),
        (this.g = t.g),
        this.g.precompute(t.n.bitLength() + 1),
        (this.pointClass = t.point().constructor),
        (this.encodingLength = Math.ceil(t.n.bitLength() / 8)),
        (this.hash = Ox.sha512);
    }
    j2.exports = Mt;
    Mt.prototype.sign = function (e, r) {
      e = L2(e);
      var n = this.keyFromSecret(r),
        i = this.hashInt(n.messagePrefix(), e),
        a = this.g.mul(i),
        h = this.encodePoint(a),
        v = this.hashInt(h, n.pubBytes(), e).mul(n.priv()),
        g = i.add(v).umod(this.curve.n);
      return this.makeSignature({ R: a, S: g, Rencoded: h });
    };
    Mt.prototype.verify = function (e, r, n) {
      (e = L2(e)), (r = this.makeSignature(r));
      var i = this.keyFromPublic(n),
        a = this.hashInt(r.Rencoded(), i.pubBytes(), e),
        h = this.g.mul(r.S()),
        v = r.R().add(i.pub().mul(a));
      return v.eq(h);
    };
    Mt.prototype.hashInt = function () {
      for (var e = this.hash(), r = 0; r < arguments.length; r++)
        e.update(arguments[r]);
      return of.intFromLE(e.digest()).umod(this.curve.n);
    };
    Mt.prototype.keyFromPublic = function (e) {
      return F2.fromPublic(this, e);
    };
    Mt.prototype.keyFromSecret = function (e) {
      return F2.fromSecret(this, e);
    };
    Mt.prototype.makeSignature = function (e) {
      return e instanceof D2 ? e : new D2(this, e);
    };
    Mt.prototype.encodePoint = function (e) {
      var r = e.getY().toArray('le', this.encodingLength);
      return (r[this.encodingLength - 1] |= e.getX().isOdd() ? 128 : 0), r;
    };
    Mt.prototype.decodePoint = function (e) {
      e = of.parseBytes(e);
      var r = e.length - 1,
        n = e.slice(0, r).concat(e[r] & -129),
        i = (e[r] & 128) !== 0,
        a = of.intFromLE(n);
      return this.curve.pointFromY(a, i);
    };
    Mt.prototype.encodeInt = function (e) {
      return e.toArray('le', this.encodingLength);
    };
    Mt.prototype.decodeInt = function (e) {
      return of.intFromLE(e);
    };
    Mt.prototype.isPoint = function (e) {
      return e instanceof this.pointClass;
    };
  });
  var Ms = R((z2) => {
    'use strict';
    S();
    var vn = z2;
    vn.version = wb().version;
    vn.utils = Bt();
    vn.rand = is();
    vn.curve = _u();
    vn.curves = ys();
    vn.ec = q2();
    vn.eddsa = U2();
  });
  var ju = R((H2, Fu) => {
    S();
    (function (t, e) {
      'use strict';
      function r(l, f) {
        if (!l) throw new Error(f || 'Assertion failed');
      }
      function n(l, f) {
        l.super_ = f;
        var o = function () {};
        (o.prototype = f.prototype),
          (l.prototype = new o()),
          (l.prototype.constructor = l);
      }
      function i(l, f, o) {
        if (i.isBN(l)) return l;
        (this.negative = 0),
          (this.words = null),
          (this.length = 0),
          (this.red = null),
          l !== null &&
            ((f === 'le' || f === 'be') && ((o = f), (f = 10)),
            this._init(l || 0, f || 10, o || 'be'));
      }
      typeof t == 'object' ? (t.exports = i) : (e.BN = i),
        (i.BN = i),
        (i.wordSize = 26);
      var a;
      try {
        typeof window < 'u' && typeof window.Buffer < 'u'
          ? (a = window.Buffer)
          : (a = Et().Buffer);
      } catch {}
      (i.isBN = function (f) {
        return f instanceof i
          ? !0
          : f !== null &&
              typeof f == 'object' &&
              f.constructor.wordSize === i.wordSize &&
              Array.isArray(f.words);
      }),
        (i.max = function (f, o) {
          return f.cmp(o) > 0 ? f : o;
        }),
        (i.min = function (f, o) {
          return f.cmp(o) < 0 ? f : o;
        }),
        (i.prototype._init = function (f, o, c) {
          if (typeof f == 'number') return this._initNumber(f, o, c);
          if (typeof f == 'object') return this._initArray(f, o, c);
          o === 'hex' && (o = 16),
            r(o === (o | 0) && o >= 2 && o <= 36),
            (f = f.toString().replace(/\s+/g, ''));
          var p = 0;
          f[0] === '-' && (p++, (this.negative = 1)),
            p < f.length &&
              (o === 16
                ? this._parseHex(f, p, c)
                : (this._parseBase(f, o, p),
                  c === 'le' && this._initArray(this.toArray(), o, c)));
        }),
        (i.prototype._initNumber = function (f, o, c) {
          f < 0 && ((this.negative = 1), (f = -f)),
            f < 67108864
              ? ((this.words = [f & 67108863]), (this.length = 1))
              : f < 4503599627370496
                ? ((this.words = [f & 67108863, (f / 67108864) & 67108863]),
                  (this.length = 2))
                : (r(f < 9007199254740992),
                  (this.words = [f & 67108863, (f / 67108864) & 67108863, 1]),
                  (this.length = 3)),
            c === 'le' && this._initArray(this.toArray(), o, c);
        }),
        (i.prototype._initArray = function (f, o, c) {
          if ((r(typeof f.length == 'number'), f.length <= 0))
            return (this.words = [0]), (this.length = 1), this;
          (this.length = Math.ceil(f.length / 3)),
            (this.words = new Array(this.length));
          for (var p = 0; p < this.length; p++) this.words[p] = 0;
          var d,
            u,
            y = 0;
          if (c === 'be')
            for (p = f.length - 1, d = 0; p >= 0; p -= 3)
              (u = f[p] | (f[p - 1] << 8) | (f[p - 2] << 16)),
                (this.words[d] |= (u << y) & 67108863),
                (this.words[d + 1] = (u >>> (26 - y)) & 67108863),
                (y += 24),
                y >= 26 && ((y -= 26), d++);
          else if (c === 'le')
            for (p = 0, d = 0; p < f.length; p += 3)
              (u = f[p] | (f[p + 1] << 8) | (f[p + 2] << 16)),
                (this.words[d] |= (u << y) & 67108863),
                (this.words[d + 1] = (u >>> (26 - y)) & 67108863),
                (y += 24),
                y >= 26 && ((y -= 26), d++);
          return this._strip();
        });
      function h(l, f) {
        var o = l.charCodeAt(f);
        if (o >= 48 && o <= 57) return o - 48;
        if (o >= 65 && o <= 70) return o - 55;
        if (o >= 97 && o <= 102) return o - 87;
        r(!1, 'Invalid character in ' + l);
      }
      function v(l, f, o) {
        var c = h(l, o);
        return o - 1 >= f && (c |= h(l, o - 1) << 4), c;
      }
      i.prototype._parseHex = function (f, o, c) {
        (this.length = Math.ceil((f.length - o) / 6)),
          (this.words = new Array(this.length));
        for (var p = 0; p < this.length; p++) this.words[p] = 0;
        var d = 0,
          u = 0,
          y;
        if (c === 'be')
          for (p = f.length - 1; p >= o; p -= 2)
            (y = v(f, o, p) << d),
              (this.words[u] |= y & 67108863),
              d >= 18
                ? ((d -= 18), (u += 1), (this.words[u] |= y >>> 26))
                : (d += 8);
        else {
          var m = f.length - o;
          for (p = m % 2 === 0 ? o + 1 : o; p < f.length; p += 2)
            (y = v(f, o, p) << d),
              (this.words[u] |= y & 67108863),
              d >= 18
                ? ((d -= 18), (u += 1), (this.words[u] |= y >>> 26))
                : (d += 8);
        }
        this._strip();
      };
      function g(l, f, o, c) {
        for (var p = 0, d = 0, u = Math.min(l.length, o), y = f; y < u; y++) {
          var m = l.charCodeAt(y) - 48;
          (p *= c),
            m >= 49 ? (d = m - 49 + 10) : m >= 17 ? (d = m - 17 + 10) : (d = m),
            r(m >= 0 && d < c, 'Invalid character'),
            (p += d);
        }
        return p;
      }
      (i.prototype._parseBase = function (f, o, c) {
        (this.words = [0]), (this.length = 1);
        for (var p = 0, d = 1; d <= 67108863; d *= o) p++;
        p--, (d = (d / o) | 0);
        for (
          var u = f.length - c,
            y = u % p,
            m = Math.min(u, u - y) + c,
            s = 0,
            w = c;
          w < m;
          w += p
        )
          (s = g(f, w, w + p, o)),
            this.imuln(d),
            this.words[0] + s < 67108864
              ? (this.words[0] += s)
              : this._iaddn(s);
        if (y !== 0) {
          var T = 1;
          for (s = g(f, w, f.length, o), w = 0; w < y; w++) T *= o;
          this.imuln(T),
            this.words[0] + s < 67108864
              ? (this.words[0] += s)
              : this._iaddn(s);
        }
        this._strip();
      }),
        (i.prototype.copy = function (f) {
          f.words = new Array(this.length);
          for (var o = 0; o < this.length; o++) f.words[o] = this.words[o];
          (f.length = this.length),
            (f.negative = this.negative),
            (f.red = this.red);
        });
      function M(l, f) {
        (l.words = f.words),
          (l.length = f.length),
          (l.negative = f.negative),
          (l.red = f.red);
      }
      if (
        ((i.prototype._move = function (f) {
          M(f, this);
        }),
        (i.prototype.clone = function () {
          var f = new i(null);
          return this.copy(f), f;
        }),
        (i.prototype._expand = function (f) {
          for (; this.length < f; ) this.words[this.length++] = 0;
          return this;
        }),
        (i.prototype._strip = function () {
          for (; this.length > 1 && this.words[this.length - 1] === 0; )
            this.length--;
          return this._normSign();
        }),
        (i.prototype._normSign = function () {
          return (
            this.length === 1 && this.words[0] === 0 && (this.negative = 0),
            this
          );
        }),
        typeof Symbol < 'u' && typeof Symbol.for == 'function')
      )
        try {
          i.prototype[Symbol.for('nodejs.util.inspect.custom')] = x;
        } catch {
          i.prototype.inspect = x;
        }
      else i.prototype.inspect = x;
      function x() {
        return (this.red ? '<BN-R: ' : '<BN: ') + this.toString(16) + '>';
      }
      var E = [
          '',
          '0',
          '00',
          '000',
          '0000',
          '00000',
          '000000',
          '0000000',
          '00000000',
          '000000000',
          '0000000000',
          '00000000000',
          '000000000000',
          '0000000000000',
          '00000000000000',
          '000000000000000',
          '0000000000000000',
          '00000000000000000',
          '000000000000000000',
          '0000000000000000000',
          '00000000000000000000',
          '000000000000000000000',
          '0000000000000000000000',
          '00000000000000000000000',
          '000000000000000000000000',
          '0000000000000000000000000',
        ],
        I = [
          0, 0, 25, 16, 12, 11, 10, 9, 8, 8, 7, 7, 7, 7, 6, 6, 6, 6, 6, 6, 6, 5,
          5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
        ],
        q = [
          0, 0, 33554432, 43046721, 16777216, 48828125, 60466176, 40353607,
          16777216, 43046721, 1e7, 19487171, 35831808, 62748517, 7529536,
          11390625, 16777216, 24137569, 34012224, 47045881, 64e6, 4084101,
          5153632, 6436343, 7962624, 9765625, 11881376, 14348907, 17210368,
          20511149, 243e5, 28629151, 33554432, 39135393, 45435424, 52521875,
          60466176,
        ];
      (i.prototype.toString = function (f, o) {
        (f = f || 10), (o = o | 0 || 1);
        var c;
        if (f === 16 || f === 'hex') {
          c = '';
          for (var p = 0, d = 0, u = 0; u < this.length; u++) {
            var y = this.words[u],
              m = (((y << p) | d) & 16777215).toString(16);
            (d = (y >>> (24 - p)) & 16777215),
              (p += 2),
              p >= 26 && ((p -= 26), u--),
              d !== 0 || u !== this.length - 1
                ? (c = E[6 - m.length] + m + c)
                : (c = m + c);
          }
          for (d !== 0 && (c = d.toString(16) + c); c.length % o !== 0; )
            c = '0' + c;
          return this.negative !== 0 && (c = '-' + c), c;
        }
        if (f === (f | 0) && f >= 2 && f <= 36) {
          var s = I[f],
            w = q[f];
          c = '';
          var T = this.clone();
          for (T.negative = 0; !T.isZero(); ) {
            var O = T.modrn(w).toString(f);
            (T = T.idivn(w)),
              T.isZero() ? (c = O + c) : (c = E[s - O.length] + O + c);
          }
          for (this.isZero() && (c = '0' + c); c.length % o !== 0; )
            c = '0' + c;
          return this.negative !== 0 && (c = '-' + c), c;
        }
        r(!1, 'Base should be between 2 and 36');
      }),
        (i.prototype.toNumber = function () {
          var f = this.words[0];
          return (
            this.length === 2
              ? (f += this.words[1] * 67108864)
              : this.length === 3 && this.words[2] === 1
                ? (f += 4503599627370496 + this.words[1] * 67108864)
                : this.length > 2 &&
                  r(!1, 'Number can only safely store up to 53 bits'),
            this.negative !== 0 ? -f : f
          );
        }),
        (i.prototype.toJSON = function () {
          return this.toString(16, 2);
        }),
        a &&
          (i.prototype.toBuffer = function (f, o) {
            return this.toArrayLike(a, f, o);
          }),
        (i.prototype.toArray = function (f, o) {
          return this.toArrayLike(Array, f, o);
        });
      var k = function (f, o) {
        return f.allocUnsafe ? f.allocUnsafe(o) : new f(o);
      };
      (i.prototype.toArrayLike = function (f, o, c) {
        this._strip();
        var p = this.byteLength(),
          d = c || Math.max(1, p);
        r(p <= d, 'byte array longer than desired length'),
          r(d > 0, 'Requested array length <= 0');
        var u = k(f, d),
          y = o === 'le' ? 'LE' : 'BE';
        return this['_toArrayLike' + y](u, p), u;
      }),
        (i.prototype._toArrayLikeLE = function (f, o) {
          for (var c = 0, p = 0, d = 0, u = 0; d < this.length; d++) {
            var y = (this.words[d] << u) | p;
            (f[c++] = y & 255),
              c < f.length && (f[c++] = (y >> 8) & 255),
              c < f.length && (f[c++] = (y >> 16) & 255),
              u === 6
                ? (c < f.length && (f[c++] = (y >> 24) & 255), (p = 0), (u = 0))
                : ((p = y >>> 24), (u += 2));
          }
          if (c < f.length) for (f[c++] = p; c < f.length; ) f[c++] = 0;
        }),
        (i.prototype._toArrayLikeBE = function (f, o) {
          for (
            var c = f.length - 1, p = 0, d = 0, u = 0;
            d < this.length;
            d++
          ) {
            var y = (this.words[d] << u) | p;
            (f[c--] = y & 255),
              c >= 0 && (f[c--] = (y >> 8) & 255),
              c >= 0 && (f[c--] = (y >> 16) & 255),
              u === 6
                ? (c >= 0 && (f[c--] = (y >> 24) & 255), (p = 0), (u = 0))
                : ((p = y >>> 24), (u += 2));
          }
          if (c >= 0) for (f[c--] = p; c >= 0; ) f[c--] = 0;
        }),
        Math.clz32
          ? (i.prototype._countBits = function (f) {
              return 32 - Math.clz32(f);
            })
          : (i.prototype._countBits = function (f) {
              var o = f,
                c = 0;
              return (
                o >= 4096 && ((c += 13), (o >>>= 13)),
                o >= 64 && ((c += 7), (o >>>= 7)),
                o >= 8 && ((c += 4), (o >>>= 4)),
                o >= 2 && ((c += 2), (o >>>= 2)),
                c + o
              );
            }),
        (i.prototype._zeroBits = function (f) {
          if (f === 0) return 26;
          var o = f,
            c = 0;
          return (
            (o & 8191) === 0 && ((c += 13), (o >>>= 13)),
            (o & 127) === 0 && ((c += 7), (o >>>= 7)),
            (o & 15) === 0 && ((c += 4), (o >>>= 4)),
            (o & 3) === 0 && ((c += 2), (o >>>= 2)),
            (o & 1) === 0 && c++,
            c
          );
        }),
        (i.prototype.bitLength = function () {
          var f = this.words[this.length - 1],
            o = this._countBits(f);
          return (this.length - 1) * 26 + o;
        });
      function L(l) {
        for (var f = new Array(l.bitLength()), o = 0; o < f.length; o++) {
          var c = (o / 26) | 0,
            p = o % 26;
          f[o] = (l.words[c] >>> p) & 1;
        }
        return f;
      }
      (i.prototype.zeroBits = function () {
        if (this.isZero()) return 0;
        for (var f = 0, o = 0; o < this.length; o++) {
          var c = this._zeroBits(this.words[o]);
          if (((f += c), c !== 26)) break;
        }
        return f;
      }),
        (i.prototype.byteLength = function () {
          return Math.ceil(this.bitLength() / 8);
        }),
        (i.prototype.toTwos = function (f) {
          return this.negative !== 0
            ? this.abs().inotn(f).iaddn(1)
            : this.clone();
        }),
        (i.prototype.fromTwos = function (f) {
          return this.testn(f - 1)
            ? this.notn(f).iaddn(1).ineg()
            : this.clone();
        }),
        (i.prototype.isNeg = function () {
          return this.negative !== 0;
        }),
        (i.prototype.neg = function () {
          return this.clone().ineg();
        }),
        (i.prototype.ineg = function () {
          return this.isZero() || (this.negative ^= 1), this;
        }),
        (i.prototype.iuor = function (f) {
          for (; this.length < f.length; ) this.words[this.length++] = 0;
          for (var o = 0; o < f.length; o++)
            this.words[o] = this.words[o] | f.words[o];
          return this._strip();
        }),
        (i.prototype.ior = function (f) {
          return r((this.negative | f.negative) === 0), this.iuor(f);
        }),
        (i.prototype.or = function (f) {
          return this.length > f.length
            ? this.clone().ior(f)
            : f.clone().ior(this);
        }),
        (i.prototype.uor = function (f) {
          return this.length > f.length
            ? this.clone().iuor(f)
            : f.clone().iuor(this);
        }),
        (i.prototype.iuand = function (f) {
          var o;
          this.length > f.length ? (o = f) : (o = this);
          for (var c = 0; c < o.length; c++)
            this.words[c] = this.words[c] & f.words[c];
          return (this.length = o.length), this._strip();
        }),
        (i.prototype.iand = function (f) {
          return r((this.negative | f.negative) === 0), this.iuand(f);
        }),
        (i.prototype.and = function (f) {
          return this.length > f.length
            ? this.clone().iand(f)
            : f.clone().iand(this);
        }),
        (i.prototype.uand = function (f) {
          return this.length > f.length
            ? this.clone().iuand(f)
            : f.clone().iuand(this);
        }),
        (i.prototype.iuxor = function (f) {
          var o, c;
          this.length > f.length
            ? ((o = this), (c = f))
            : ((o = f), (c = this));
          for (var p = 0; p < c.length; p++)
            this.words[p] = o.words[p] ^ c.words[p];
          if (this !== o) for (; p < o.length; p++) this.words[p] = o.words[p];
          return (this.length = o.length), this._strip();
        }),
        (i.prototype.ixor = function (f) {
          return r((this.negative | f.negative) === 0), this.iuxor(f);
        }),
        (i.prototype.xor = function (f) {
          return this.length > f.length
            ? this.clone().ixor(f)
            : f.clone().ixor(this);
        }),
        (i.prototype.uxor = function (f) {
          return this.length > f.length
            ? this.clone().iuxor(f)
            : f.clone().iuxor(this);
        }),
        (i.prototype.inotn = function (f) {
          r(typeof f == 'number' && f >= 0);
          var o = Math.ceil(f / 26) | 0,
            c = f % 26;
          this._expand(o), c > 0 && o--;
          for (var p = 0; p < o; p++) this.words[p] = ~this.words[p] & 67108863;
          return (
            c > 0 && (this.words[p] = ~this.words[p] & (67108863 >> (26 - c))),
            this._strip()
          );
        }),
        (i.prototype.notn = function (f) {
          return this.clone().inotn(f);
        }),
        (i.prototype.setn = function (f, o) {
          r(typeof f == 'number' && f >= 0);
          var c = (f / 26) | 0,
            p = f % 26;
          return (
            this._expand(c + 1),
            o
              ? (this.words[c] = this.words[c] | (1 << p))
              : (this.words[c] = this.words[c] & ~(1 << p)),
            this._strip()
          );
        }),
        (i.prototype.iadd = function (f) {
          var o;
          if (this.negative !== 0 && f.negative === 0)
            return (
              (this.negative = 0),
              (o = this.isub(f)),
              (this.negative ^= 1),
              this._normSign()
            );
          if (this.negative === 0 && f.negative !== 0)
            return (
              (f.negative = 0),
              (o = this.isub(f)),
              (f.negative = 1),
              o._normSign()
            );
          var c, p;
          this.length > f.length
            ? ((c = this), (p = f))
            : ((c = f), (p = this));
          for (var d = 0, u = 0; u < p.length; u++)
            (o = (c.words[u] | 0) + (p.words[u] | 0) + d),
              (this.words[u] = o & 67108863),
              (d = o >>> 26);
          for (; d !== 0 && u < c.length; u++)
            (o = (c.words[u] | 0) + d),
              (this.words[u] = o & 67108863),
              (d = o >>> 26);
          if (((this.length = c.length), d !== 0))
            (this.words[this.length] = d), this.length++;
          else if (c !== this)
            for (; u < c.length; u++) this.words[u] = c.words[u];
          return this;
        }),
        (i.prototype.add = function (f) {
          var o;
          return f.negative !== 0 && this.negative === 0
            ? ((f.negative = 0), (o = this.sub(f)), (f.negative ^= 1), o)
            : f.negative === 0 && this.negative !== 0
              ? ((this.negative = 0), (o = f.sub(this)), (this.negative = 1), o)
              : this.length > f.length
                ? this.clone().iadd(f)
                : f.clone().iadd(this);
        }),
        (i.prototype.isub = function (f) {
          if (f.negative !== 0) {
            f.negative = 0;
            var o = this.iadd(f);
            return (f.negative = 1), o._normSign();
          } else if (this.negative !== 0)
            return (
              (this.negative = 0),
              this.iadd(f),
              (this.negative = 1),
              this._normSign()
            );
          var c = this.cmp(f);
          if (c === 0)
            return (
              (this.negative = 0), (this.length = 1), (this.words[0] = 0), this
            );
          var p, d;
          c > 0 ? ((p = this), (d = f)) : ((p = f), (d = this));
          for (var u = 0, y = 0; y < d.length; y++)
            (o = (p.words[y] | 0) - (d.words[y] | 0) + u),
              (u = o >> 26),
              (this.words[y] = o & 67108863);
          for (; u !== 0 && y < p.length; y++)
            (o = (p.words[y] | 0) + u),
              (u = o >> 26),
              (this.words[y] = o & 67108863);
          if (u === 0 && y < p.length && p !== this)
            for (; y < p.length; y++) this.words[y] = p.words[y];
          return (
            (this.length = Math.max(this.length, y)),
            p !== this && (this.negative = 1),
            this._strip()
          );
        }),
        (i.prototype.sub = function (f) {
          return this.clone().isub(f);
        });
      function xe(l, f, o) {
        o.negative = f.negative ^ l.negative;
        var c = (l.length + f.length) | 0;
        (o.length = c), (c = (c - 1) | 0);
        var p = l.words[0] | 0,
          d = f.words[0] | 0,
          u = p * d,
          y = u & 67108863,
          m = (u / 67108864) | 0;
        o.words[0] = y;
        for (var s = 1; s < c; s++) {
          for (
            var w = m >>> 26,
              T = m & 67108863,
              O = Math.min(s, f.length - 1),
              P = Math.max(0, s - l.length + 1);
            P <= O;
            P++
          ) {
            var N = (s - P) | 0;
            (p = l.words[N] | 0),
              (d = f.words[P] | 0),
              (u = p * d + T),
              (w += (u / 67108864) | 0),
              (T = u & 67108863);
          }
          (o.words[s] = T | 0), (m = w | 0);
        }
        return m !== 0 ? (o.words[s] = m | 0) : o.length--, o._strip();
      }
      var U = function (f, o, c) {
        var p = f.words,
          d = o.words,
          u = c.words,
          y = 0,
          m,
          s,
          w,
          T = p[0] | 0,
          O = T & 8191,
          P = T >>> 13,
          N = p[1] | 0,
          F = N & 8191,
          j = N >>> 13,
          et = p[2] | 0,
          z = et & 8191,
          H = et >>> 13,
          ei = p[3] | 0,
          K = ei & 8191,
          V = ei >>> 13,
          ti = p[4] | 0,
          G = ti & 8191,
          W = ti >>> 13,
          ri = p[5] | 0,
          $ = ri & 8191,
          Z = ri >>> 13,
          ii = p[6] | 0,
          J = ii & 8191,
          X = ii >>> 13,
          ni = p[7] | 0,
          Y = ni & 8191,
          Q = ni >>> 13,
          fi = p[8] | 0,
          ee = fi & 8191,
          te = fi >>> 13,
          ai = p[9] | 0,
          re = ai & 8191,
          ie = ai >>> 13,
          oi = d[0] | 0,
          ne = oi & 8191,
          fe = oi >>> 13,
          si = d[1] | 0,
          ae = si & 8191,
          oe = si >>> 13,
          hi = d[2] | 0,
          se = hi & 8191,
          he = hi >>> 13,
          ui = d[3] | 0,
          ue = ui & 8191,
          ce = ui >>> 13,
          ci = d[4] | 0,
          de = ci & 8191,
          le = ci >>> 13,
          di = d[5] | 0,
          pe = di & 8191,
          ve = di >>> 13,
          li = d[6] | 0,
          be = li & 8191,
          ye = li >>> 13,
          pi = d[7] | 0,
          me = pi & 8191,
          ge = pi >>> 13,
          vi = d[8] | 0,
          we = vi & 8191,
          _e = vi >>> 13,
          Lr = d[9] | 0,
          Ae = Lr & 8191,
          Be = Lr >>> 13;
        (c.negative = f.negative ^ o.negative),
          (c.length = 19),
          (m = Math.imul(O, ne)),
          (s = Math.imul(O, fe)),
          (s = (s + Math.imul(P, ne)) | 0),
          (w = Math.imul(P, fe));
        var $t = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + ($t >>> 26)) | 0),
          ($t &= 67108863),
          (m = Math.imul(F, ne)),
          (s = Math.imul(F, fe)),
          (s = (s + Math.imul(j, ne)) | 0),
          (w = Math.imul(j, fe)),
          (m = (m + Math.imul(O, ae)) | 0),
          (s = (s + Math.imul(O, oe)) | 0),
          (s = (s + Math.imul(P, ae)) | 0),
          (w = (w + Math.imul(P, oe)) | 0);
        var Zt = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + (Zt >>> 26)) | 0),
          (Zt &= 67108863),
          (m = Math.imul(z, ne)),
          (s = Math.imul(z, fe)),
          (s = (s + Math.imul(H, ne)) | 0),
          (w = Math.imul(H, fe)),
          (m = (m + Math.imul(F, ae)) | 0),
          (s = (s + Math.imul(F, oe)) | 0),
          (s = (s + Math.imul(j, ae)) | 0),
          (w = (w + Math.imul(j, oe)) | 0),
          (m = (m + Math.imul(O, se)) | 0),
          (s = (s + Math.imul(O, he)) | 0),
          (s = (s + Math.imul(P, se)) | 0),
          (w = (w + Math.imul(P, he)) | 0);
        var Jt = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + (Jt >>> 26)) | 0),
          (Jt &= 67108863),
          (m = Math.imul(K, ne)),
          (s = Math.imul(K, fe)),
          (s = (s + Math.imul(V, ne)) | 0),
          (w = Math.imul(V, fe)),
          (m = (m + Math.imul(z, ae)) | 0),
          (s = (s + Math.imul(z, oe)) | 0),
          (s = (s + Math.imul(H, ae)) | 0),
          (w = (w + Math.imul(H, oe)) | 0),
          (m = (m + Math.imul(F, se)) | 0),
          (s = (s + Math.imul(F, he)) | 0),
          (s = (s + Math.imul(j, se)) | 0),
          (w = (w + Math.imul(j, he)) | 0),
          (m = (m + Math.imul(O, ue)) | 0),
          (s = (s + Math.imul(O, ce)) | 0),
          (s = (s + Math.imul(P, ue)) | 0),
          (w = (w + Math.imul(P, ce)) | 0);
        var Xt = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + (Xt >>> 26)) | 0),
          (Xt &= 67108863),
          (m = Math.imul(G, ne)),
          (s = Math.imul(G, fe)),
          (s = (s + Math.imul(W, ne)) | 0),
          (w = Math.imul(W, fe)),
          (m = (m + Math.imul(K, ae)) | 0),
          (s = (s + Math.imul(K, oe)) | 0),
          (s = (s + Math.imul(V, ae)) | 0),
          (w = (w + Math.imul(V, oe)) | 0),
          (m = (m + Math.imul(z, se)) | 0),
          (s = (s + Math.imul(z, he)) | 0),
          (s = (s + Math.imul(H, se)) | 0),
          (w = (w + Math.imul(H, he)) | 0),
          (m = (m + Math.imul(F, ue)) | 0),
          (s = (s + Math.imul(F, ce)) | 0),
          (s = (s + Math.imul(j, ue)) | 0),
          (w = (w + Math.imul(j, ce)) | 0),
          (m = (m + Math.imul(O, de)) | 0),
          (s = (s + Math.imul(O, le)) | 0),
          (s = (s + Math.imul(P, de)) | 0),
          (w = (w + Math.imul(P, le)) | 0);
        var Yt = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + (Yt >>> 26)) | 0),
          (Yt &= 67108863),
          (m = Math.imul($, ne)),
          (s = Math.imul($, fe)),
          (s = (s + Math.imul(Z, ne)) | 0),
          (w = Math.imul(Z, fe)),
          (m = (m + Math.imul(G, ae)) | 0),
          (s = (s + Math.imul(G, oe)) | 0),
          (s = (s + Math.imul(W, ae)) | 0),
          (w = (w + Math.imul(W, oe)) | 0),
          (m = (m + Math.imul(K, se)) | 0),
          (s = (s + Math.imul(K, he)) | 0),
          (s = (s + Math.imul(V, se)) | 0),
          (w = (w + Math.imul(V, he)) | 0),
          (m = (m + Math.imul(z, ue)) | 0),
          (s = (s + Math.imul(z, ce)) | 0),
          (s = (s + Math.imul(H, ue)) | 0),
          (w = (w + Math.imul(H, ce)) | 0),
          (m = (m + Math.imul(F, de)) | 0),
          (s = (s + Math.imul(F, le)) | 0),
          (s = (s + Math.imul(j, de)) | 0),
          (w = (w + Math.imul(j, le)) | 0),
          (m = (m + Math.imul(O, pe)) | 0),
          (s = (s + Math.imul(O, ve)) | 0),
          (s = (s + Math.imul(P, pe)) | 0),
          (w = (w + Math.imul(P, ve)) | 0);
        var Qt = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + (Qt >>> 26)) | 0),
          (Qt &= 67108863),
          (m = Math.imul(J, ne)),
          (s = Math.imul(J, fe)),
          (s = (s + Math.imul(X, ne)) | 0),
          (w = Math.imul(X, fe)),
          (m = (m + Math.imul($, ae)) | 0),
          (s = (s + Math.imul($, oe)) | 0),
          (s = (s + Math.imul(Z, ae)) | 0),
          (w = (w + Math.imul(Z, oe)) | 0),
          (m = (m + Math.imul(G, se)) | 0),
          (s = (s + Math.imul(G, he)) | 0),
          (s = (s + Math.imul(W, se)) | 0),
          (w = (w + Math.imul(W, he)) | 0),
          (m = (m + Math.imul(K, ue)) | 0),
          (s = (s + Math.imul(K, ce)) | 0),
          (s = (s + Math.imul(V, ue)) | 0),
          (w = (w + Math.imul(V, ce)) | 0),
          (m = (m + Math.imul(z, de)) | 0),
          (s = (s + Math.imul(z, le)) | 0),
          (s = (s + Math.imul(H, de)) | 0),
          (w = (w + Math.imul(H, le)) | 0),
          (m = (m + Math.imul(F, pe)) | 0),
          (s = (s + Math.imul(F, ve)) | 0),
          (s = (s + Math.imul(j, pe)) | 0),
          (w = (w + Math.imul(j, ve)) | 0),
          (m = (m + Math.imul(O, be)) | 0),
          (s = (s + Math.imul(O, ye)) | 0),
          (s = (s + Math.imul(P, be)) | 0),
          (w = (w + Math.imul(P, ye)) | 0);
        var er = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + (er >>> 26)) | 0),
          (er &= 67108863),
          (m = Math.imul(Y, ne)),
          (s = Math.imul(Y, fe)),
          (s = (s + Math.imul(Q, ne)) | 0),
          (w = Math.imul(Q, fe)),
          (m = (m + Math.imul(J, ae)) | 0),
          (s = (s + Math.imul(J, oe)) | 0),
          (s = (s + Math.imul(X, ae)) | 0),
          (w = (w + Math.imul(X, oe)) | 0),
          (m = (m + Math.imul($, se)) | 0),
          (s = (s + Math.imul($, he)) | 0),
          (s = (s + Math.imul(Z, se)) | 0),
          (w = (w + Math.imul(Z, he)) | 0),
          (m = (m + Math.imul(G, ue)) | 0),
          (s = (s + Math.imul(G, ce)) | 0),
          (s = (s + Math.imul(W, ue)) | 0),
          (w = (w + Math.imul(W, ce)) | 0),
          (m = (m + Math.imul(K, de)) | 0),
          (s = (s + Math.imul(K, le)) | 0),
          (s = (s + Math.imul(V, de)) | 0),
          (w = (w + Math.imul(V, le)) | 0),
          (m = (m + Math.imul(z, pe)) | 0),
          (s = (s + Math.imul(z, ve)) | 0),
          (s = (s + Math.imul(H, pe)) | 0),
          (w = (w + Math.imul(H, ve)) | 0),
          (m = (m + Math.imul(F, be)) | 0),
          (s = (s + Math.imul(F, ye)) | 0),
          (s = (s + Math.imul(j, be)) | 0),
          (w = (w + Math.imul(j, ye)) | 0),
          (m = (m + Math.imul(O, me)) | 0),
          (s = (s + Math.imul(O, ge)) | 0),
          (s = (s + Math.imul(P, me)) | 0),
          (w = (w + Math.imul(P, ge)) | 0);
        var tr = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + (tr >>> 26)) | 0),
          (tr &= 67108863),
          (m = Math.imul(ee, ne)),
          (s = Math.imul(ee, fe)),
          (s = (s + Math.imul(te, ne)) | 0),
          (w = Math.imul(te, fe)),
          (m = (m + Math.imul(Y, ae)) | 0),
          (s = (s + Math.imul(Y, oe)) | 0),
          (s = (s + Math.imul(Q, ae)) | 0),
          (w = (w + Math.imul(Q, oe)) | 0),
          (m = (m + Math.imul(J, se)) | 0),
          (s = (s + Math.imul(J, he)) | 0),
          (s = (s + Math.imul(X, se)) | 0),
          (w = (w + Math.imul(X, he)) | 0),
          (m = (m + Math.imul($, ue)) | 0),
          (s = (s + Math.imul($, ce)) | 0),
          (s = (s + Math.imul(Z, ue)) | 0),
          (w = (w + Math.imul(Z, ce)) | 0),
          (m = (m + Math.imul(G, de)) | 0),
          (s = (s + Math.imul(G, le)) | 0),
          (s = (s + Math.imul(W, de)) | 0),
          (w = (w + Math.imul(W, le)) | 0),
          (m = (m + Math.imul(K, pe)) | 0),
          (s = (s + Math.imul(K, ve)) | 0),
          (s = (s + Math.imul(V, pe)) | 0),
          (w = (w + Math.imul(V, ve)) | 0),
          (m = (m + Math.imul(z, be)) | 0),
          (s = (s + Math.imul(z, ye)) | 0),
          (s = (s + Math.imul(H, be)) | 0),
          (w = (w + Math.imul(H, ye)) | 0),
          (m = (m + Math.imul(F, me)) | 0),
          (s = (s + Math.imul(F, ge)) | 0),
          (s = (s + Math.imul(j, me)) | 0),
          (w = (w + Math.imul(j, ge)) | 0),
          (m = (m + Math.imul(O, we)) | 0),
          (s = (s + Math.imul(O, _e)) | 0),
          (s = (s + Math.imul(P, we)) | 0),
          (w = (w + Math.imul(P, _e)) | 0);
        var rr = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + (rr >>> 26)) | 0),
          (rr &= 67108863),
          (m = Math.imul(re, ne)),
          (s = Math.imul(re, fe)),
          (s = (s + Math.imul(ie, ne)) | 0),
          (w = Math.imul(ie, fe)),
          (m = (m + Math.imul(ee, ae)) | 0),
          (s = (s + Math.imul(ee, oe)) | 0),
          (s = (s + Math.imul(te, ae)) | 0),
          (w = (w + Math.imul(te, oe)) | 0),
          (m = (m + Math.imul(Y, se)) | 0),
          (s = (s + Math.imul(Y, he)) | 0),
          (s = (s + Math.imul(Q, se)) | 0),
          (w = (w + Math.imul(Q, he)) | 0),
          (m = (m + Math.imul(J, ue)) | 0),
          (s = (s + Math.imul(J, ce)) | 0),
          (s = (s + Math.imul(X, ue)) | 0),
          (w = (w + Math.imul(X, ce)) | 0),
          (m = (m + Math.imul($, de)) | 0),
          (s = (s + Math.imul($, le)) | 0),
          (s = (s + Math.imul(Z, de)) | 0),
          (w = (w + Math.imul(Z, le)) | 0),
          (m = (m + Math.imul(G, pe)) | 0),
          (s = (s + Math.imul(G, ve)) | 0),
          (s = (s + Math.imul(W, pe)) | 0),
          (w = (w + Math.imul(W, ve)) | 0),
          (m = (m + Math.imul(K, be)) | 0),
          (s = (s + Math.imul(K, ye)) | 0),
          (s = (s + Math.imul(V, be)) | 0),
          (w = (w + Math.imul(V, ye)) | 0),
          (m = (m + Math.imul(z, me)) | 0),
          (s = (s + Math.imul(z, ge)) | 0),
          (s = (s + Math.imul(H, me)) | 0),
          (w = (w + Math.imul(H, ge)) | 0),
          (m = (m + Math.imul(F, we)) | 0),
          (s = (s + Math.imul(F, _e)) | 0),
          (s = (s + Math.imul(j, we)) | 0),
          (w = (w + Math.imul(j, _e)) | 0),
          (m = (m + Math.imul(O, Ae)) | 0),
          (s = (s + Math.imul(O, Be)) | 0),
          (s = (s + Math.imul(P, Ae)) | 0),
          (w = (w + Math.imul(P, Be)) | 0);
        var ir = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + (ir >>> 26)) | 0),
          (ir &= 67108863),
          (m = Math.imul(re, ae)),
          (s = Math.imul(re, oe)),
          (s = (s + Math.imul(ie, ae)) | 0),
          (w = Math.imul(ie, oe)),
          (m = (m + Math.imul(ee, se)) | 0),
          (s = (s + Math.imul(ee, he)) | 0),
          (s = (s + Math.imul(te, se)) | 0),
          (w = (w + Math.imul(te, he)) | 0),
          (m = (m + Math.imul(Y, ue)) | 0),
          (s = (s + Math.imul(Y, ce)) | 0),
          (s = (s + Math.imul(Q, ue)) | 0),
          (w = (w + Math.imul(Q, ce)) | 0),
          (m = (m + Math.imul(J, de)) | 0),
          (s = (s + Math.imul(J, le)) | 0),
          (s = (s + Math.imul(X, de)) | 0),
          (w = (w + Math.imul(X, le)) | 0),
          (m = (m + Math.imul($, pe)) | 0),
          (s = (s + Math.imul($, ve)) | 0),
          (s = (s + Math.imul(Z, pe)) | 0),
          (w = (w + Math.imul(Z, ve)) | 0),
          (m = (m + Math.imul(G, be)) | 0),
          (s = (s + Math.imul(G, ye)) | 0),
          (s = (s + Math.imul(W, be)) | 0),
          (w = (w + Math.imul(W, ye)) | 0),
          (m = (m + Math.imul(K, me)) | 0),
          (s = (s + Math.imul(K, ge)) | 0),
          (s = (s + Math.imul(V, me)) | 0),
          (w = (w + Math.imul(V, ge)) | 0),
          (m = (m + Math.imul(z, we)) | 0),
          (s = (s + Math.imul(z, _e)) | 0),
          (s = (s + Math.imul(H, we)) | 0),
          (w = (w + Math.imul(H, _e)) | 0),
          (m = (m + Math.imul(F, Ae)) | 0),
          (s = (s + Math.imul(F, Be)) | 0),
          (s = (s + Math.imul(j, Ae)) | 0),
          (w = (w + Math.imul(j, Be)) | 0);
        var nr = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + (nr >>> 26)) | 0),
          (nr &= 67108863),
          (m = Math.imul(re, se)),
          (s = Math.imul(re, he)),
          (s = (s + Math.imul(ie, se)) | 0),
          (w = Math.imul(ie, he)),
          (m = (m + Math.imul(ee, ue)) | 0),
          (s = (s + Math.imul(ee, ce)) | 0),
          (s = (s + Math.imul(te, ue)) | 0),
          (w = (w + Math.imul(te, ce)) | 0),
          (m = (m + Math.imul(Y, de)) | 0),
          (s = (s + Math.imul(Y, le)) | 0),
          (s = (s + Math.imul(Q, de)) | 0),
          (w = (w + Math.imul(Q, le)) | 0),
          (m = (m + Math.imul(J, pe)) | 0),
          (s = (s + Math.imul(J, ve)) | 0),
          (s = (s + Math.imul(X, pe)) | 0),
          (w = (w + Math.imul(X, ve)) | 0),
          (m = (m + Math.imul($, be)) | 0),
          (s = (s + Math.imul($, ye)) | 0),
          (s = (s + Math.imul(Z, be)) | 0),
          (w = (w + Math.imul(Z, ye)) | 0),
          (m = (m + Math.imul(G, me)) | 0),
          (s = (s + Math.imul(G, ge)) | 0),
          (s = (s + Math.imul(W, me)) | 0),
          (w = (w + Math.imul(W, ge)) | 0),
          (m = (m + Math.imul(K, we)) | 0),
          (s = (s + Math.imul(K, _e)) | 0),
          (s = (s + Math.imul(V, we)) | 0),
          (w = (w + Math.imul(V, _e)) | 0),
          (m = (m + Math.imul(z, Ae)) | 0),
          (s = (s + Math.imul(z, Be)) | 0),
          (s = (s + Math.imul(H, Ae)) | 0),
          (w = (w + Math.imul(H, Be)) | 0);
        var fr = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + (fr >>> 26)) | 0),
          (fr &= 67108863),
          (m = Math.imul(re, ue)),
          (s = Math.imul(re, ce)),
          (s = (s + Math.imul(ie, ue)) | 0),
          (w = Math.imul(ie, ce)),
          (m = (m + Math.imul(ee, de)) | 0),
          (s = (s + Math.imul(ee, le)) | 0),
          (s = (s + Math.imul(te, de)) | 0),
          (w = (w + Math.imul(te, le)) | 0),
          (m = (m + Math.imul(Y, pe)) | 0),
          (s = (s + Math.imul(Y, ve)) | 0),
          (s = (s + Math.imul(Q, pe)) | 0),
          (w = (w + Math.imul(Q, ve)) | 0),
          (m = (m + Math.imul(J, be)) | 0),
          (s = (s + Math.imul(J, ye)) | 0),
          (s = (s + Math.imul(X, be)) | 0),
          (w = (w + Math.imul(X, ye)) | 0),
          (m = (m + Math.imul($, me)) | 0),
          (s = (s + Math.imul($, ge)) | 0),
          (s = (s + Math.imul(Z, me)) | 0),
          (w = (w + Math.imul(Z, ge)) | 0),
          (m = (m + Math.imul(G, we)) | 0),
          (s = (s + Math.imul(G, _e)) | 0),
          (s = (s + Math.imul(W, we)) | 0),
          (w = (w + Math.imul(W, _e)) | 0),
          (m = (m + Math.imul(K, Ae)) | 0),
          (s = (s + Math.imul(K, Be)) | 0),
          (s = (s + Math.imul(V, Ae)) | 0),
          (w = (w + Math.imul(V, Be)) | 0);
        var ar = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + (ar >>> 26)) | 0),
          (ar &= 67108863),
          (m = Math.imul(re, de)),
          (s = Math.imul(re, le)),
          (s = (s + Math.imul(ie, de)) | 0),
          (w = Math.imul(ie, le)),
          (m = (m + Math.imul(ee, pe)) | 0),
          (s = (s + Math.imul(ee, ve)) | 0),
          (s = (s + Math.imul(te, pe)) | 0),
          (w = (w + Math.imul(te, ve)) | 0),
          (m = (m + Math.imul(Y, be)) | 0),
          (s = (s + Math.imul(Y, ye)) | 0),
          (s = (s + Math.imul(Q, be)) | 0),
          (w = (w + Math.imul(Q, ye)) | 0),
          (m = (m + Math.imul(J, me)) | 0),
          (s = (s + Math.imul(J, ge)) | 0),
          (s = (s + Math.imul(X, me)) | 0),
          (w = (w + Math.imul(X, ge)) | 0),
          (m = (m + Math.imul($, we)) | 0),
          (s = (s + Math.imul($, _e)) | 0),
          (s = (s + Math.imul(Z, we)) | 0),
          (w = (w + Math.imul(Z, _e)) | 0),
          (m = (m + Math.imul(G, Ae)) | 0),
          (s = (s + Math.imul(G, Be)) | 0),
          (s = (s + Math.imul(W, Ae)) | 0),
          (w = (w + Math.imul(W, Be)) | 0);
        var or = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + (or >>> 26)) | 0),
          (or &= 67108863),
          (m = Math.imul(re, pe)),
          (s = Math.imul(re, ve)),
          (s = (s + Math.imul(ie, pe)) | 0),
          (w = Math.imul(ie, ve)),
          (m = (m + Math.imul(ee, be)) | 0),
          (s = (s + Math.imul(ee, ye)) | 0),
          (s = (s + Math.imul(te, be)) | 0),
          (w = (w + Math.imul(te, ye)) | 0),
          (m = (m + Math.imul(Y, me)) | 0),
          (s = (s + Math.imul(Y, ge)) | 0),
          (s = (s + Math.imul(Q, me)) | 0),
          (w = (w + Math.imul(Q, ge)) | 0),
          (m = (m + Math.imul(J, we)) | 0),
          (s = (s + Math.imul(J, _e)) | 0),
          (s = (s + Math.imul(X, we)) | 0),
          (w = (w + Math.imul(X, _e)) | 0),
          (m = (m + Math.imul($, Ae)) | 0),
          (s = (s + Math.imul($, Be)) | 0),
          (s = (s + Math.imul(Z, Ae)) | 0),
          (w = (w + Math.imul(Z, Be)) | 0);
        var sr = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + (sr >>> 26)) | 0),
          (sr &= 67108863),
          (m = Math.imul(re, be)),
          (s = Math.imul(re, ye)),
          (s = (s + Math.imul(ie, be)) | 0),
          (w = Math.imul(ie, ye)),
          (m = (m + Math.imul(ee, me)) | 0),
          (s = (s + Math.imul(ee, ge)) | 0),
          (s = (s + Math.imul(te, me)) | 0),
          (w = (w + Math.imul(te, ge)) | 0),
          (m = (m + Math.imul(Y, we)) | 0),
          (s = (s + Math.imul(Y, _e)) | 0),
          (s = (s + Math.imul(Q, we)) | 0),
          (w = (w + Math.imul(Q, _e)) | 0),
          (m = (m + Math.imul(J, Ae)) | 0),
          (s = (s + Math.imul(J, Be)) | 0),
          (s = (s + Math.imul(X, Ae)) | 0),
          (w = (w + Math.imul(X, Be)) | 0);
        var hr = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + (hr >>> 26)) | 0),
          (hr &= 67108863),
          (m = Math.imul(re, me)),
          (s = Math.imul(re, ge)),
          (s = (s + Math.imul(ie, me)) | 0),
          (w = Math.imul(ie, ge)),
          (m = (m + Math.imul(ee, we)) | 0),
          (s = (s + Math.imul(ee, _e)) | 0),
          (s = (s + Math.imul(te, we)) | 0),
          (w = (w + Math.imul(te, _e)) | 0),
          (m = (m + Math.imul(Y, Ae)) | 0),
          (s = (s + Math.imul(Y, Be)) | 0),
          (s = (s + Math.imul(Q, Ae)) | 0),
          (w = (w + Math.imul(Q, Be)) | 0);
        var Ni = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + (Ni >>> 26)) | 0),
          (Ni &= 67108863),
          (m = Math.imul(re, we)),
          (s = Math.imul(re, _e)),
          (s = (s + Math.imul(ie, we)) | 0),
          (w = Math.imul(ie, _e)),
          (m = (m + Math.imul(ee, Ae)) | 0),
          (s = (s + Math.imul(ee, Be)) | 0),
          (s = (s + Math.imul(te, Ae)) | 0),
          (w = (w + Math.imul(te, Be)) | 0);
        var Di = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        (y = (((w + (s >>> 13)) | 0) + (Di >>> 26)) | 0),
          (Di &= 67108863),
          (m = Math.imul(re, Ae)),
          (s = Math.imul(re, Be)),
          (s = (s + Math.imul(ie, Ae)) | 0),
          (w = Math.imul(ie, Be));
        var Li = (((y + m) | 0) + ((s & 8191) << 13)) | 0;
        return (
          (y = (((w + (s >>> 13)) | 0) + (Li >>> 26)) | 0),
          (Li &= 67108863),
          (u[0] = $t),
          (u[1] = Zt),
          (u[2] = Jt),
          (u[3] = Xt),
          (u[4] = Yt),
          (u[5] = Qt),
          (u[6] = er),
          (u[7] = tr),
          (u[8] = rr),
          (u[9] = ir),
          (u[10] = nr),
          (u[11] = fr),
          (u[12] = ar),
          (u[13] = or),
          (u[14] = sr),
          (u[15] = hr),
          (u[16] = Ni),
          (u[17] = Di),
          (u[18] = Li),
          y !== 0 && ((u[19] = y), c.length++),
          c
        );
      };
      Math.imul || (U = xe);
      function Me(l, f, o) {
        (o.negative = f.negative ^ l.negative),
          (o.length = l.length + f.length);
        for (var c = 0, p = 0, d = 0; d < o.length - 1; d++) {
          var u = p;
          p = 0;
          for (
            var y = c & 67108863,
              m = Math.min(d, f.length - 1),
              s = Math.max(0, d - l.length + 1);
            s <= m;
            s++
          ) {
            var w = d - s,
              T = l.words[w] | 0,
              O = f.words[s] | 0,
              P = T * O,
              N = P & 67108863;
            (u = (u + ((P / 67108864) | 0)) | 0),
              (N = (N + y) | 0),
              (y = N & 67108863),
              (u = (u + (N >>> 26)) | 0),
              (p += u >>> 26),
              (u &= 67108863);
          }
          (o.words[d] = y), (c = u), (u = p);
        }
        return c !== 0 ? (o.words[d] = c) : o.length--, o._strip();
      }
      function Te(l, f, o) {
        return Me(l, f, o);
      }
      i.prototype.mulTo = function (f, o) {
        var c,
          p = this.length + f.length;
        return (
          this.length === 10 && f.length === 10
            ? (c = U(this, f, o))
            : p < 63
              ? (c = xe(this, f, o))
              : p < 1024
                ? (c = Me(this, f, o))
                : (c = Te(this, f, o)),
          c
        );
      };
      function Ee(l, f) {
        (this.x = l), (this.y = f);
      }
      (Ee.prototype.makeRBT = function (f) {
        for (
          var o = new Array(f), c = i.prototype._countBits(f) - 1, p = 0;
          p < f;
          p++
        )
          o[p] = this.revBin(p, c, f);
        return o;
      }),
        (Ee.prototype.revBin = function (f, o, c) {
          if (f === 0 || f === c - 1) return f;
          for (var p = 0, d = 0; d < o; d++)
            (p |= (f & 1) << (o - d - 1)), (f >>= 1);
          return p;
        }),
        (Ee.prototype.permute = function (f, o, c, p, d, u) {
          for (var y = 0; y < u; y++) (p[y] = o[f[y]]), (d[y] = c[f[y]]);
        }),
        (Ee.prototype.transform = function (f, o, c, p, d, u) {
          this.permute(u, f, o, c, p, d);
          for (var y = 1; y < d; y <<= 1)
            for (
              var m = y << 1,
                s = Math.cos((2 * Math.PI) / m),
                w = Math.sin((2 * Math.PI) / m),
                T = 0;
              T < d;
              T += m
            )
              for (var O = s, P = w, N = 0; N < y; N++) {
                var F = c[T + N],
                  j = p[T + N],
                  et = c[T + N + y],
                  z = p[T + N + y],
                  H = O * et - P * z;
                (z = O * z + P * et),
                  (et = H),
                  (c[T + N] = F + et),
                  (p[T + N] = j + z),
                  (c[T + N + y] = F - et),
                  (p[T + N + y] = j - z),
                  N !== m &&
                    ((H = s * O - w * P), (P = s * P + w * O), (O = H));
              }
        }),
        (Ee.prototype.guessLen13b = function (f, o) {
          var c = Math.max(o, f) | 1,
            p = c & 1,
            d = 0;
          for (c = (c / 2) | 0; c; c = c >>> 1) d++;
          return 1 << (d + 1 + p);
        }),
        (Ee.prototype.conjugate = function (f, o, c) {
          if (!(c <= 1))
            for (var p = 0; p < c / 2; p++) {
              var d = f[p];
              (f[p] = f[c - p - 1]),
                (f[c - p - 1] = d),
                (d = o[p]),
                (o[p] = -o[c - p - 1]),
                (o[c - p - 1] = -d);
            }
        }),
        (Ee.prototype.normalize13b = function (f, o) {
          for (var c = 0, p = 0; p < o / 2; p++) {
            var d =
              Math.round(f[2 * p + 1] / o) * 8192 +
              Math.round(f[2 * p] / o) +
              c;
            (f[p] = d & 67108863),
              d < 67108864 ? (c = 0) : (c = (d / 67108864) | 0);
          }
          return f;
        }),
        (Ee.prototype.convert13b = function (f, o, c, p) {
          for (var d = 0, u = 0; u < o; u++)
            (d = d + (f[u] | 0)),
              (c[2 * u] = d & 8191),
              (d = d >>> 13),
              (c[2 * u + 1] = d & 8191),
              (d = d >>> 13);
          for (u = 2 * o; u < p; ++u) c[u] = 0;
          r(d === 0), r((d & -8192) === 0);
        }),
        (Ee.prototype.stub = function (f) {
          for (var o = new Array(f), c = 0; c < f; c++) o[c] = 0;
          return o;
        }),
        (Ee.prototype.mulp = function (f, o, c) {
          var p = 2 * this.guessLen13b(f.length, o.length),
            d = this.makeRBT(p),
            u = this.stub(p),
            y = new Array(p),
            m = new Array(p),
            s = new Array(p),
            w = new Array(p),
            T = new Array(p),
            O = new Array(p),
            P = c.words;
          (P.length = p),
            this.convert13b(f.words, f.length, y, p),
            this.convert13b(o.words, o.length, w, p),
            this.transform(y, u, m, s, p, d),
            this.transform(w, u, T, O, p, d);
          for (var N = 0; N < p; N++) {
            var F = m[N] * T[N] - s[N] * O[N];
            (s[N] = m[N] * O[N] + s[N] * T[N]), (m[N] = F);
          }
          return (
            this.conjugate(m, s, p),
            this.transform(m, s, P, u, p, d),
            this.conjugate(P, u, p),
            this.normalize13b(P, p),
            (c.negative = f.negative ^ o.negative),
            (c.length = f.length + o.length),
            c._strip()
          );
        }),
        (i.prototype.mul = function (f) {
          var o = new i(null);
          return (
            (o.words = new Array(this.length + f.length)), this.mulTo(f, o)
          );
        }),
        (i.prototype.mulf = function (f) {
          var o = new i(null);
          return (o.words = new Array(this.length + f.length)), Te(this, f, o);
        }),
        (i.prototype.imul = function (f) {
          return this.clone().mulTo(f, this);
        }),
        (i.prototype.imuln = function (f) {
          var o = f < 0;
          o && (f = -f), r(typeof f == 'number'), r(f < 67108864);
          for (var c = 0, p = 0; p < this.length; p++) {
            var d = (this.words[p] | 0) * f,
              u = (d & 67108863) + (c & 67108863);
            (c >>= 26),
              (c += (d / 67108864) | 0),
              (c += u >>> 26),
              (this.words[p] = u & 67108863);
          }
          return (
            c !== 0 && ((this.words[p] = c), this.length++),
            o ? this.ineg() : this
          );
        }),
        (i.prototype.muln = function (f) {
          return this.clone().imuln(f);
        }),
        (i.prototype.sqr = function () {
          return this.mul(this);
        }),
        (i.prototype.isqr = function () {
          return this.imul(this.clone());
        }),
        (i.prototype.pow = function (f) {
          var o = L(f);
          if (o.length === 0) return new i(1);
          for (
            var c = this, p = 0;
            p < o.length && o[p] === 0;
            p++, c = c.sqr()
          );
          if (++p < o.length)
            for (var d = c.sqr(); p < o.length; p++, d = d.sqr())
              o[p] !== 0 && (c = c.mul(d));
          return c;
        }),
        (i.prototype.iushln = function (f) {
          r(typeof f == 'number' && f >= 0);
          var o = f % 26,
            c = (f - o) / 26,
            p = (67108863 >>> (26 - o)) << (26 - o),
            d;
          if (o !== 0) {
            var u = 0;
            for (d = 0; d < this.length; d++) {
              var y = this.words[d] & p,
                m = ((this.words[d] | 0) - y) << o;
              (this.words[d] = m | u), (u = y >>> (26 - o));
            }
            u && ((this.words[d] = u), this.length++);
          }
          if (c !== 0) {
            for (d = this.length - 1; d >= 0; d--)
              this.words[d + c] = this.words[d];
            for (d = 0; d < c; d++) this.words[d] = 0;
            this.length += c;
          }
          return this._strip();
        }),
        (i.prototype.ishln = function (f) {
          return r(this.negative === 0), this.iushln(f);
        }),
        (i.prototype.iushrn = function (f, o, c) {
          r(typeof f == 'number' && f >= 0);
          var p;
          o ? (p = (o - (o % 26)) / 26) : (p = 0);
          var d = f % 26,
            u = Math.min((f - d) / 26, this.length),
            y = 67108863 ^ ((67108863 >>> d) << d),
            m = c;
          if (((p -= u), (p = Math.max(0, p)), m)) {
            for (var s = 0; s < u; s++) m.words[s] = this.words[s];
            m.length = u;
          }
          if (u !== 0)
            if (this.length > u)
              for (this.length -= u, s = 0; s < this.length; s++)
                this.words[s] = this.words[s + u];
            else (this.words[0] = 0), (this.length = 1);
          var w = 0;
          for (s = this.length - 1; s >= 0 && (w !== 0 || s >= p); s--) {
            var T = this.words[s] | 0;
            (this.words[s] = (w << (26 - d)) | (T >>> d)), (w = T & y);
          }
          return (
            m && w !== 0 && (m.words[m.length++] = w),
            this.length === 0 && ((this.words[0] = 0), (this.length = 1)),
            this._strip()
          );
        }),
        (i.prototype.ishrn = function (f, o, c) {
          return r(this.negative === 0), this.iushrn(f, o, c);
        }),
        (i.prototype.shln = function (f) {
          return this.clone().ishln(f);
        }),
        (i.prototype.ushln = function (f) {
          return this.clone().iushln(f);
        }),
        (i.prototype.shrn = function (f) {
          return this.clone().ishrn(f);
        }),
        (i.prototype.ushrn = function (f) {
          return this.clone().iushrn(f);
        }),
        (i.prototype.testn = function (f) {
          r(typeof f == 'number' && f >= 0);
          var o = f % 26,
            c = (f - o) / 26,
            p = 1 << o;
          if (this.length <= c) return !1;
          var d = this.words[c];
          return !!(d & p);
        }),
        (i.prototype.imaskn = function (f) {
          r(typeof f == 'number' && f >= 0);
          var o = f % 26,
            c = (f - o) / 26;
          if (
            (r(this.negative === 0, 'imaskn works only with positive numbers'),
            this.length <= c)
          )
            return this;
          if (
            (o !== 0 && c++, (this.length = Math.min(c, this.length)), o !== 0)
          ) {
            var p = 67108863 ^ ((67108863 >>> o) << o);
            this.words[this.length - 1] &= p;
          }
          return this._strip();
        }),
        (i.prototype.maskn = function (f) {
          return this.clone().imaskn(f);
        }),
        (i.prototype.iaddn = function (f) {
          return (
            r(typeof f == 'number'),
            r(f < 67108864),
            f < 0
              ? this.isubn(-f)
              : this.negative !== 0
                ? this.length === 1 && (this.words[0] | 0) <= f
                  ? ((this.words[0] = f - (this.words[0] | 0)),
                    (this.negative = 0),
                    this)
                  : ((this.negative = 0),
                    this.isubn(f),
                    (this.negative = 1),
                    this)
                : this._iaddn(f)
          );
        }),
        (i.prototype._iaddn = function (f) {
          this.words[0] += f;
          for (var o = 0; o < this.length && this.words[o] >= 67108864; o++)
            (this.words[o] -= 67108864),
              o === this.length - 1
                ? (this.words[o + 1] = 1)
                : this.words[o + 1]++;
          return (this.length = Math.max(this.length, o + 1)), this;
        }),
        (i.prototype.isubn = function (f) {
          if ((r(typeof f == 'number'), r(f < 67108864), f < 0))
            return this.iaddn(-f);
          if (this.negative !== 0)
            return (
              (this.negative = 0), this.iaddn(f), (this.negative = 1), this
            );
          if (((this.words[0] -= f), this.length === 1 && this.words[0] < 0))
            (this.words[0] = -this.words[0]), (this.negative = 1);
          else
            for (var o = 0; o < this.length && this.words[o] < 0; o++)
              (this.words[o] += 67108864), (this.words[o + 1] -= 1);
          return this._strip();
        }),
        (i.prototype.addn = function (f) {
          return this.clone().iaddn(f);
        }),
        (i.prototype.subn = function (f) {
          return this.clone().isubn(f);
        }),
        (i.prototype.iabs = function () {
          return (this.negative = 0), this;
        }),
        (i.prototype.abs = function () {
          return this.clone().iabs();
        }),
        (i.prototype._ishlnsubmul = function (f, o, c) {
          var p = f.length + c,
            d;
          this._expand(p);
          var u,
            y = 0;
          for (d = 0; d < f.length; d++) {
            u = (this.words[d + c] | 0) + y;
            var m = (f.words[d] | 0) * o;
            (u -= m & 67108863),
              (y = (u >> 26) - ((m / 67108864) | 0)),
              (this.words[d + c] = u & 67108863);
          }
          for (; d < this.length - c; d++)
            (u = (this.words[d + c] | 0) + y),
              (y = u >> 26),
              (this.words[d + c] = u & 67108863);
          if (y === 0) return this._strip();
          for (r(y === -1), y = 0, d = 0; d < this.length; d++)
            (u = -(this.words[d] | 0) + y),
              (y = u >> 26),
              (this.words[d] = u & 67108863);
          return (this.negative = 1), this._strip();
        }),
        (i.prototype._wordDiv = function (f, o) {
          var c = this.length - f.length,
            p = this.clone(),
            d = f,
            u = d.words[d.length - 1] | 0,
            y = this._countBits(u);
          (c = 26 - y),
            c !== 0 &&
              ((d = d.ushln(c)), p.iushln(c), (u = d.words[d.length - 1] | 0));
          var m = p.length - d.length,
            s;
          if (o !== 'mod') {
            (s = new i(null)),
              (s.length = m + 1),
              (s.words = new Array(s.length));
            for (var w = 0; w < s.length; w++) s.words[w] = 0;
          }
          var T = p.clone()._ishlnsubmul(d, 1, m);
          T.negative === 0 && ((p = T), s && (s.words[m] = 1));
          for (var O = m - 1; O >= 0; O--) {
            var P =
              (p.words[d.length + O] | 0) * 67108864 +
              (p.words[d.length + O - 1] | 0);
            for (
              P = Math.min((P / u) | 0, 67108863), p._ishlnsubmul(d, P, O);
              p.negative !== 0;

            )
              P--,
                (p.negative = 0),
                p._ishlnsubmul(d, 1, O),
                p.isZero() || (p.negative ^= 1);
            s && (s.words[O] = P);
          }
          return (
            s && s._strip(),
            p._strip(),
            o !== 'div' && c !== 0 && p.iushrn(c),
            { div: s || null, mod: p }
          );
        }),
        (i.prototype.divmod = function (f, o, c) {
          if ((r(!f.isZero()), this.isZero()))
            return { div: new i(0), mod: new i(0) };
          var p, d, u;
          return this.negative !== 0 && f.negative === 0
            ? ((u = this.neg().divmod(f, o)),
              o !== 'mod' && (p = u.div.neg()),
              o !== 'div' &&
                ((d = u.mod.neg()), c && d.negative !== 0 && d.iadd(f)),
              { div: p, mod: d })
            : this.negative === 0 && f.negative !== 0
              ? ((u = this.divmod(f.neg(), o)),
                o !== 'mod' && (p = u.div.neg()),
                { div: p, mod: u.mod })
              : (this.negative & f.negative) !== 0
                ? ((u = this.neg().divmod(f.neg(), o)),
                  o !== 'div' &&
                    ((d = u.mod.neg()), c && d.negative !== 0 && d.isub(f)),
                  { div: u.div, mod: d })
                : f.length > this.length || this.cmp(f) < 0
                  ? { div: new i(0), mod: this }
                  : f.length === 1
                    ? o === 'div'
                      ? { div: this.divn(f.words[0]), mod: null }
                      : o === 'mod'
                        ? { div: null, mod: new i(this.modrn(f.words[0])) }
                        : {
                            div: this.divn(f.words[0]),
                            mod: new i(this.modrn(f.words[0])),
                          }
                    : this._wordDiv(f, o);
        }),
        (i.prototype.div = function (f) {
          return this.divmod(f, 'div', !1).div;
        }),
        (i.prototype.mod = function (f) {
          return this.divmod(f, 'mod', !1).mod;
        }),
        (i.prototype.umod = function (f) {
          return this.divmod(f, 'mod', !0).mod;
        }),
        (i.prototype.divRound = function (f) {
          var o = this.divmod(f);
          if (o.mod.isZero()) return o.div;
          var c = o.div.negative !== 0 ? o.mod.isub(f) : o.mod,
            p = f.ushrn(1),
            d = f.andln(1),
            u = c.cmp(p);
          return u < 0 || (d === 1 && u === 0)
            ? o.div
            : o.div.negative !== 0
              ? o.div.isubn(1)
              : o.div.iaddn(1);
        }),
        (i.prototype.modrn = function (f) {
          var o = f < 0;
          o && (f = -f), r(f <= 67108863);
          for (var c = (1 << 26) % f, p = 0, d = this.length - 1; d >= 0; d--)
            p = (c * p + (this.words[d] | 0)) % f;
          return o ? -p : p;
        }),
        (i.prototype.modn = function (f) {
          return this.modrn(f);
        }),
        (i.prototype.idivn = function (f) {
          var o = f < 0;
          o && (f = -f), r(f <= 67108863);
          for (var c = 0, p = this.length - 1; p >= 0; p--) {
            var d = (this.words[p] | 0) + c * 67108864;
            (this.words[p] = (d / f) | 0), (c = d % f);
          }
          return this._strip(), o ? this.ineg() : this;
        }),
        (i.prototype.divn = function (f) {
          return this.clone().idivn(f);
        }),
        (i.prototype.egcd = function (f) {
          r(f.negative === 0), r(!f.isZero());
          var o = this,
            c = f.clone();
          o.negative !== 0 ? (o = o.umod(f)) : (o = o.clone());
          for (
            var p = new i(1), d = new i(0), u = new i(0), y = new i(1), m = 0;
            o.isEven() && c.isEven();

          )
            o.iushrn(1), c.iushrn(1), ++m;
          for (var s = c.clone(), w = o.clone(); !o.isZero(); ) {
            for (
              var T = 0, O = 1;
              (o.words[0] & O) === 0 && T < 26;
              ++T, O <<= 1
            );
            if (T > 0)
              for (o.iushrn(T); T-- > 0; )
                (p.isOdd() || d.isOdd()) && (p.iadd(s), d.isub(w)),
                  p.iushrn(1),
                  d.iushrn(1);
            for (
              var P = 0, N = 1;
              (c.words[0] & N) === 0 && P < 26;
              ++P, N <<= 1
            );
            if (P > 0)
              for (c.iushrn(P); P-- > 0; )
                (u.isOdd() || y.isOdd()) && (u.iadd(s), y.isub(w)),
                  u.iushrn(1),
                  y.iushrn(1);
            o.cmp(c) >= 0
              ? (o.isub(c), p.isub(u), d.isub(y))
              : (c.isub(o), u.isub(p), y.isub(d));
          }
          return { a: u, b: y, gcd: c.iushln(m) };
        }),
        (i.prototype._invmp = function (f) {
          r(f.negative === 0), r(!f.isZero());
          var o = this,
            c = f.clone();
          o.negative !== 0 ? (o = o.umod(f)) : (o = o.clone());
          for (
            var p = new i(1), d = new i(0), u = c.clone();
            o.cmpn(1) > 0 && c.cmpn(1) > 0;

          ) {
            for (
              var y = 0, m = 1;
              (o.words[0] & m) === 0 && y < 26;
              ++y, m <<= 1
            );
            if (y > 0)
              for (o.iushrn(y); y-- > 0; ) p.isOdd() && p.iadd(u), p.iushrn(1);
            for (
              var s = 0, w = 1;
              (c.words[0] & w) === 0 && s < 26;
              ++s, w <<= 1
            );
            if (s > 0)
              for (c.iushrn(s); s-- > 0; ) d.isOdd() && d.iadd(u), d.iushrn(1);
            o.cmp(c) >= 0 ? (o.isub(c), p.isub(d)) : (c.isub(o), d.isub(p));
          }
          var T;
          return (
            o.cmpn(1) === 0 ? (T = p) : (T = d), T.cmpn(0) < 0 && T.iadd(f), T
          );
        }),
        (i.prototype.gcd = function (f) {
          if (this.isZero()) return f.abs();
          if (f.isZero()) return this.abs();
          var o = this.clone(),
            c = f.clone();
          (o.negative = 0), (c.negative = 0);
          for (var p = 0; o.isEven() && c.isEven(); p++)
            o.iushrn(1), c.iushrn(1);
          do {
            for (; o.isEven(); ) o.iushrn(1);
            for (; c.isEven(); ) c.iushrn(1);
            var d = o.cmp(c);
            if (d < 0) {
              var u = o;
              (o = c), (c = u);
            } else if (d === 0 || c.cmpn(1) === 0) break;
            o.isub(c);
          } while (!0);
          return c.iushln(p);
        }),
        (i.prototype.invm = function (f) {
          return this.egcd(f).a.umod(f);
        }),
        (i.prototype.isEven = function () {
          return (this.words[0] & 1) === 0;
        }),
        (i.prototype.isOdd = function () {
          return (this.words[0] & 1) === 1;
        }),
        (i.prototype.andln = function (f) {
          return this.words[0] & f;
        }),
        (i.prototype.bincn = function (f) {
          r(typeof f == 'number');
          var o = f % 26,
            c = (f - o) / 26,
            p = 1 << o;
          if (this.length <= c)
            return this._expand(c + 1), (this.words[c] |= p), this;
          for (var d = p, u = c; d !== 0 && u < this.length; u++) {
            var y = this.words[u] | 0;
            (y += d), (d = y >>> 26), (y &= 67108863), (this.words[u] = y);
          }
          return d !== 0 && ((this.words[u] = d), this.length++), this;
        }),
        (i.prototype.isZero = function () {
          return this.length === 1 && this.words[0] === 0;
        }),
        (i.prototype.cmpn = function (f) {
          var o = f < 0;
          if (this.negative !== 0 && !o) return -1;
          if (this.negative === 0 && o) return 1;
          this._strip();
          var c;
          if (this.length > 1) c = 1;
          else {
            o && (f = -f), r(f <= 67108863, 'Number is too big');
            var p = this.words[0] | 0;
            c = p === f ? 0 : p < f ? -1 : 1;
          }
          return this.negative !== 0 ? -c | 0 : c;
        }),
        (i.prototype.cmp = function (f) {
          if (this.negative !== 0 && f.negative === 0) return -1;
          if (this.negative === 0 && f.negative !== 0) return 1;
          var o = this.ucmp(f);
          return this.negative !== 0 ? -o | 0 : o;
        }),
        (i.prototype.ucmp = function (f) {
          if (this.length > f.length) return 1;
          if (this.length < f.length) return -1;
          for (var o = 0, c = this.length - 1; c >= 0; c--) {
            var p = this.words[c] | 0,
              d = f.words[c] | 0;
            if (p !== d) {
              p < d ? (o = -1) : p > d && (o = 1);
              break;
            }
          }
          return o;
        }),
        (i.prototype.gtn = function (f) {
          return this.cmpn(f) === 1;
        }),
        (i.prototype.gt = function (f) {
          return this.cmp(f) === 1;
        }),
        (i.prototype.gten = function (f) {
          return this.cmpn(f) >= 0;
        }),
        (i.prototype.gte = function (f) {
          return this.cmp(f) >= 0;
        }),
        (i.prototype.ltn = function (f) {
          return this.cmpn(f) === -1;
        }),
        (i.prototype.lt = function (f) {
          return this.cmp(f) === -1;
        }),
        (i.prototype.lten = function (f) {
          return this.cmpn(f) <= 0;
        }),
        (i.prototype.lte = function (f) {
          return this.cmp(f) <= 0;
        }),
        (i.prototype.eqn = function (f) {
          return this.cmpn(f) === 0;
        }),
        (i.prototype.eq = function (f) {
          return this.cmp(f) === 0;
        }),
        (i.red = function (f) {
          return new b(f);
        }),
        (i.prototype.toRed = function (f) {
          return (
            r(!this.red, 'Already a number in reduction context'),
            r(this.negative === 0, 'red works only with positives'),
            f.convertTo(this)._forceRed(f)
          );
        }),
        (i.prototype.fromRed = function () {
          return (
            r(this.red, 'fromRed works only with numbers in reduction context'),
            this.red.convertFrom(this)
          );
        }),
        (i.prototype._forceRed = function (f) {
          return (this.red = f), this;
        }),
        (i.prototype.forceRed = function (f) {
          return (
            r(!this.red, 'Already a number in reduction context'),
            this._forceRed(f)
          );
        }),
        (i.prototype.redAdd = function (f) {
          return (
            r(this.red, 'redAdd works only with red numbers'),
            this.red.add(this, f)
          );
        }),
        (i.prototype.redIAdd = function (f) {
          return (
            r(this.red, 'redIAdd works only with red numbers'),
            this.red.iadd(this, f)
          );
        }),
        (i.prototype.redSub = function (f) {
          return (
            r(this.red, 'redSub works only with red numbers'),
            this.red.sub(this, f)
          );
        }),
        (i.prototype.redISub = function (f) {
          return (
            r(this.red, 'redISub works only with red numbers'),
            this.red.isub(this, f)
          );
        }),
        (i.prototype.redShl = function (f) {
          return (
            r(this.red, 'redShl works only with red numbers'),
            this.red.shl(this, f)
          );
        }),
        (i.prototype.redMul = function (f) {
          return (
            r(this.red, 'redMul works only with red numbers'),
            this.red._verify2(this, f),
            this.red.mul(this, f)
          );
        }),
        (i.prototype.redIMul = function (f) {
          return (
            r(this.red, 'redMul works only with red numbers'),
            this.red._verify2(this, f),
            this.red.imul(this, f)
          );
        }),
        (i.prototype.redSqr = function () {
          return (
            r(this.red, 'redSqr works only with red numbers'),
            this.red._verify1(this),
            this.red.sqr(this)
          );
        }),
        (i.prototype.redISqr = function () {
          return (
            r(this.red, 'redISqr works only with red numbers'),
            this.red._verify1(this),
            this.red.isqr(this)
          );
        }),
        (i.prototype.redSqrt = function () {
          return (
            r(this.red, 'redSqrt works only with red numbers'),
            this.red._verify1(this),
            this.red.sqrt(this)
          );
        }),
        (i.prototype.redInvm = function () {
          return (
            r(this.red, 'redInvm works only with red numbers'),
            this.red._verify1(this),
            this.red.invm(this)
          );
        }),
        (i.prototype.redNeg = function () {
          return (
            r(this.red, 'redNeg works only with red numbers'),
            this.red._verify1(this),
            this.red.neg(this)
          );
        }),
        (i.prototype.redPow = function (f) {
          return (
            r(this.red && !f.red, 'redPow(normalNum)'),
            this.red._verify1(this),
            this.red.pow(this, f)
          );
        });
      var Fe = { k256: null, p224: null, p192: null, p25519: null };
      function Se(l, f) {
        (this.name = l),
          (this.p = new i(f, 16)),
          (this.n = this.p.bitLength()),
          (this.k = new i(1).iushln(this.n).isub(this.p)),
          (this.tmp = this._tmp());
      }
      (Se.prototype._tmp = function () {
        var f = new i(null);
        return (f.words = new Array(Math.ceil(this.n / 13))), f;
      }),
        (Se.prototype.ireduce = function (f) {
          var o = f,
            c;
          do
            this.split(o, this.tmp),
              (o = this.imulK(o)),
              (o = o.iadd(this.tmp)),
              (c = o.bitLength());
          while (c > this.n);
          var p = c < this.n ? -1 : o.ucmp(this.p);
          return (
            p === 0
              ? ((o.words[0] = 0), (o.length = 1))
              : p > 0
                ? o.isub(this.p)
                : o.strip !== void 0
                  ? o.strip()
                  : o._strip(),
            o
          );
        }),
        (Se.prototype.split = function (f, o) {
          f.iushrn(this.n, 0, o);
        }),
        (Se.prototype.imulK = function (f) {
          return f.imul(this.k);
        });
      function $e() {
        Se.call(
          this,
          'k256',
          'ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff fffffffe fffffc2f'
        );
      }
      n($e, Se),
        ($e.prototype.split = function (f, o) {
          for (var c = 4194303, p = Math.min(f.length, 9), d = 0; d < p; d++)
            o.words[d] = f.words[d];
          if (((o.length = p), f.length <= 9)) {
            (f.words[0] = 0), (f.length = 1);
            return;
          }
          var u = f.words[9];
          for (o.words[o.length++] = u & c, d = 10; d < f.length; d++) {
            var y = f.words[d] | 0;
            (f.words[d - 10] = ((y & c) << 4) | (u >>> 22)), (u = y);
          }
          (u >>>= 22),
            (f.words[d - 10] = u),
            u === 0 && f.length > 10 ? (f.length -= 10) : (f.length -= 9);
        }),
        ($e.prototype.imulK = function (f) {
          (f.words[f.length] = 0), (f.words[f.length + 1] = 0), (f.length += 2);
          for (var o = 0, c = 0; c < f.length; c++) {
            var p = f.words[c] | 0;
            (o += p * 977),
              (f.words[c] = o & 67108863),
              (o = p * 64 + ((o / 67108864) | 0));
          }
          return (
            f.words[f.length - 1] === 0 &&
              (f.length--, f.words[f.length - 1] === 0 && f.length--),
            f
          );
        });
      function ke() {
        Se.call(
          this,
          'p224',
          'ffffffff ffffffff ffffffff ffffffff 00000000 00000000 00000001'
        );
      }
      n(ke, Se);
      function Ze() {
        Se.call(
          this,
          'p192',
          'ffffffff ffffffff ffffffff fffffffe ffffffff ffffffff'
        );
      }
      n(Ze, Se);
      function B() {
        Se.call(
          this,
          '25519',
          '7fffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffed'
        );
      }
      n(B, Se),
        (B.prototype.imulK = function (f) {
          for (var o = 0, c = 0; c < f.length; c++) {
            var p = (f.words[c] | 0) * 19 + o,
              d = p & 67108863;
            (p >>>= 26), (f.words[c] = d), (o = p);
          }
          return o !== 0 && (f.words[f.length++] = o), f;
        }),
        (i._prime = function (f) {
          if (Fe[f]) return Fe[f];
          var o;
          if (f === 'k256') o = new $e();
          else if (f === 'p224') o = new ke();
          else if (f === 'p192') o = new Ze();
          else if (f === 'p25519') o = new B();
          else throw new Error('Unknown prime ' + f);
          return (Fe[f] = o), o;
        });
      function b(l) {
        if (typeof l == 'string') {
          var f = i._prime(l);
          (this.m = f.p), (this.prime = f);
        } else
          r(l.gtn(1), 'modulus must be greater than 1'),
            (this.m = l),
            (this.prime = null);
      }
      (b.prototype._verify1 = function (f) {
        r(f.negative === 0, 'red works only with positives'),
          r(f.red, 'red works only with red numbers');
      }),
        (b.prototype._verify2 = function (f, o) {
          r((f.negative | o.negative) === 0, 'red works only with positives'),
            r(f.red && f.red === o.red, 'red works only with red numbers');
        }),
        (b.prototype.imod = function (f) {
          return this.prime
            ? this.prime.ireduce(f)._forceRed(this)
            : (M(f, f.umod(this.m)._forceRed(this)), f);
        }),
        (b.prototype.neg = function (f) {
          return f.isZero() ? f.clone() : this.m.sub(f)._forceRed(this);
        }),
        (b.prototype.add = function (f, o) {
          this._verify2(f, o);
          var c = f.add(o);
          return c.cmp(this.m) >= 0 && c.isub(this.m), c._forceRed(this);
        }),
        (b.prototype.iadd = function (f, o) {
          this._verify2(f, o);
          var c = f.iadd(o);
          return c.cmp(this.m) >= 0 && c.isub(this.m), c;
        }),
        (b.prototype.sub = function (f, o) {
          this._verify2(f, o);
          var c = f.sub(o);
          return c.cmpn(0) < 0 && c.iadd(this.m), c._forceRed(this);
        }),
        (b.prototype.isub = function (f, o) {
          this._verify2(f, o);
          var c = f.isub(o);
          return c.cmpn(0) < 0 && c.iadd(this.m), c;
        }),
        (b.prototype.shl = function (f, o) {
          return this._verify1(f), this.imod(f.ushln(o));
        }),
        (b.prototype.imul = function (f, o) {
          return this._verify2(f, o), this.imod(f.imul(o));
        }),
        (b.prototype.mul = function (f, o) {
          return this._verify2(f, o), this.imod(f.mul(o));
        }),
        (b.prototype.isqr = function (f) {
          return this.imul(f, f.clone());
        }),
        (b.prototype.sqr = function (f) {
          return this.mul(f, f);
        }),
        (b.prototype.sqrt = function (f) {
          if (f.isZero()) return f.clone();
          var o = this.m.andln(3);
          if ((r(o % 2 === 1), o === 3)) {
            var c = this.m.add(new i(1)).iushrn(2);
            return this.pow(f, c);
          }
          for (var p = this.m.subn(1), d = 0; !p.isZero() && p.andln(1) === 0; )
            d++, p.iushrn(1);
          r(!p.isZero());
          var u = new i(1).toRed(this),
            y = u.redNeg(),
            m = this.m.subn(1).iushrn(1),
            s = this.m.bitLength();
          for (s = new i(2 * s * s).toRed(this); this.pow(s, m).cmp(y) !== 0; )
            s.redIAdd(y);
          for (
            var w = this.pow(s, p),
              T = this.pow(f, p.addn(1).iushrn(1)),
              O = this.pow(f, p),
              P = d;
            O.cmp(u) !== 0;

          ) {
            for (var N = O, F = 0; N.cmp(u) !== 0; F++) N = N.redSqr();
            r(F < P);
            var j = this.pow(w, new i(1).iushln(P - F - 1));
            (T = T.redMul(j)), (w = j.redSqr()), (O = O.redMul(w)), (P = F);
          }
          return T;
        }),
        (b.prototype.invm = function (f) {
          var o = f._invmp(this.m);
          return o.negative !== 0
            ? ((o.negative = 0), this.imod(o).redNeg())
            : this.imod(o);
        }),
        (b.prototype.pow = function (f, o) {
          if (o.isZero()) return new i(1).toRed(this);
          if (o.cmpn(1) === 0) return f.clone();
          var c = 4,
            p = new Array(1 << c);
          (p[0] = new i(1).toRed(this)), (p[1] = f);
          for (var d = 2; d < p.length; d++) p[d] = this.mul(p[d - 1], f);
          var u = p[0],
            y = 0,
            m = 0,
            s = o.bitLength() % 26;
          for (s === 0 && (s = 26), d = o.length - 1; d >= 0; d--) {
            for (var w = o.words[d], T = s - 1; T >= 0; T--) {
              var O = (w >> T) & 1;
              if ((u !== p[0] && (u = this.sqr(u)), O === 0 && y === 0)) {
                m = 0;
                continue;
              }
              (y <<= 1),
                (y |= O),
                m++,
                !(m !== c && (d !== 0 || T !== 0)) &&
                  ((u = this.mul(u, p[y])), (m = 0), (y = 0));
            }
            s = 26;
          }
          return u;
        }),
        (b.prototype.convertTo = function (f) {
          var o = f.umod(this.m);
          return o === f ? o.clone() : o;
        }),
        (b.prototype.convertFrom = function (f) {
          var o = f.clone();
          return (o.red = null), o;
        }),
        (i.mont = function (f) {
          return new _(f);
        });
      function _(l) {
        b.call(this, l),
          (this.shift = this.m.bitLength()),
          this.shift % 26 !== 0 && (this.shift += 26 - (this.shift % 26)),
          (this.r = new i(1).iushln(this.shift)),
          (this.r2 = this.imod(this.r.sqr())),
          (this.rinv = this.r._invmp(this.m)),
          (this.minv = this.rinv.mul(this.r).isubn(1).div(this.m)),
          (this.minv = this.minv.umod(this.r)),
          (this.minv = this.r.sub(this.minv));
      }
      n(_, b),
        (_.prototype.convertTo = function (f) {
          return this.imod(f.ushln(this.shift));
        }),
        (_.prototype.convertFrom = function (f) {
          var o = this.imod(f.mul(this.rinv));
          return (o.red = null), o;
        }),
        (_.prototype.imul = function (f, o) {
          if (f.isZero() || o.isZero())
            return (f.words[0] = 0), (f.length = 1), f;
          var c = f.imul(o),
            p = c
              .maskn(this.shift)
              .mul(this.minv)
              .imaskn(this.shift)
              .mul(this.m),
            d = c.isub(p).iushrn(this.shift),
            u = d;
          return (
            d.cmp(this.m) >= 0
              ? (u = d.isub(this.m))
              : d.cmpn(0) < 0 && (u = d.iadd(this.m)),
            u._forceRed(this)
          );
        }),
        (_.prototype.mul = function (f, o) {
          if (f.isZero() || o.isZero()) return new i(0)._forceRed(this);
          var c = f.mul(o),
            p = c
              .maskn(this.shift)
              .mul(this.minv)
              .imaskn(this.shift)
              .mul(this.m),
            d = c.isub(p).iushrn(this.shift),
            u = d;
          return (
            d.cmp(this.m) >= 0
              ? (u = d.isub(this.m))
              : d.cmpn(0) < 0 && (u = d.iadd(this.m)),
            u._forceRed(this)
          );
        }),
        (_.prototype.invm = function (f) {
          var o = this.imod(f._invmp(this.m).mul(this.r2));
          return o._forceRed(this);
        });
    })(typeof Fu > 'u' || Fu, H2);
  });
  var Es = R((nk, K2) => {
    'use strict';
    S();
    var Ss = Et(),
      sf = Ss.Buffer,
      zt = {},
      Ht;
    for (Ht in Ss)
      !Ss.hasOwnProperty(Ht) ||
        Ht === 'SlowBuffer' ||
        Ht === 'Buffer' ||
        (zt[Ht] = Ss[Ht]);
    var hf = (zt.Buffer = {});
    for (Ht in sf)
      !sf.hasOwnProperty(Ht) ||
        Ht === 'allocUnsafe' ||
        Ht === 'allocUnsafeSlow' ||
        (hf[Ht] = sf[Ht]);
    zt.Buffer.prototype = sf.prototype;
    (!hf.from || hf.from === Uint8Array.from) &&
      (hf.from = function (t, e, r) {
        if (typeof t == 'number')
          throw new TypeError(
            'The "value" argument must not be of type number. Received type ' +
              typeof t
          );
        if (t && typeof t.length > 'u')
          throw new TypeError(
            'The first argument must be one of type string, Buffer, ArrayBuffer, Array, or Array-like Object. Received type ' +
              typeof t
          );
        return sf(t, e, r);
      });
    hf.alloc ||
      (hf.alloc = function (t, e, r) {
        if (typeof t != 'number')
          throw new TypeError(
            'The "size" argument must be of type number. Received type ' +
              typeof t
          );
        if (t < 0 || t >= 2 * (1 << 30))
          throw new RangeError(
            'The value "' + t + '" is invalid for option "size"'
          );
        var n = sf(t);
        return (
          !e || e.length === 0
            ? n.fill(0)
            : typeof r == 'string'
              ? n.fill(e, r)
              : n.fill(e),
          n
        );
      });
    if (!zt.kStringMaxLength)
      try {
        zt.kStringMaxLength = A.binding('buffer').kStringMaxLength;
      } catch {}
    zt.constants ||
      ((zt.constants = { MAX_LENGTH: zt.kMaxLength }),
      zt.kStringMaxLength &&
        (zt.constants.MAX_STRING_LENGTH = zt.kStringMaxLength));
    K2.exports = zt;
  });
  var As = R((V2) => {
    'use strict';
    S();
    var Dx = qe();
    function Kt(t) {
      this._reporterState = {
        obj: null,
        path: [],
        options: t || {},
        errors: [],
      };
    }
    V2.Reporter = Kt;
    Kt.prototype.isError = function (e) {
      return e instanceof uf;
    };
    Kt.prototype.save = function () {
      let e = this._reporterState;
      return { obj: e.obj, pathLen: e.path.length };
    };
    Kt.prototype.restore = function (e) {
      let r = this._reporterState;
      (r.obj = e.obj), (r.path = r.path.slice(0, e.pathLen));
    };
    Kt.prototype.enterKey = function (e) {
      return this._reporterState.path.push(e);
    };
    Kt.prototype.exitKey = function (e) {
      let r = this._reporterState;
      r.path = r.path.slice(0, e - 1);
    };
    Kt.prototype.leaveKey = function (e, r, n) {
      let i = this._reporterState;
      this.exitKey(e), i.obj !== null && (i.obj[r] = n);
    };
    Kt.prototype.path = function () {
      return this._reporterState.path.join('/');
    };
    Kt.prototype.enterObject = function () {
      let e = this._reporterState,
        r = e.obj;
      return (e.obj = {}), r;
    };
    Kt.prototype.leaveObject = function (e) {
      let r = this._reporterState,
        n = r.obj;
      return (r.obj = e), n;
    };
    Kt.prototype.error = function (e) {
      let r,
        n = this._reporterState,
        i = e instanceof uf;
      if (
        (i
          ? (r = e)
          : (r = new uf(
              n.path
                .map(function (a) {
                  return '[' + JSON.stringify(a) + ']';
                })
                .join(''),
              e.message || e,
              e.stack
            )),
        !n.options.partial)
      )
        throw r;
      return i || n.errors.push(r), r;
    };
    Kt.prototype.wrapResult = function (e) {
      let r = this._reporterState;
      return r.options.partial
        ? { result: this.isError(e) ? null : e, errors: r.errors }
        : e;
    };
    function uf(t, e) {
      (this.path = t), this.rethrow(e);
    }
    Dx(uf, Error);
    uf.prototype.rethrow = function (e) {
      if (
        ((this.message = e + ' at: ' + (this.path || '(shallow)')),
        Error.captureStackTrace && Error.captureStackTrace(this, uf),
        !this.stack)
      )
        try {
          throw new Error(this.message);
        } catch (r) {
          this.stack = r.stack;
        }
      return this;
    };
  });
  var lf = R((Uu) => {
    'use strict';
    S();
    var Lx = qe(),
      Bs = As().Reporter,
      cf = Es().Buffer;
    function Vt(t, e) {
      if ((Bs.call(this, e), !cf.isBuffer(t))) {
        this.error('Input not Buffer');
        return;
      }
      (this.base = t), (this.offset = 0), (this.length = t.length);
    }
    Lx(Vt, Bs);
    Uu.DecoderBuffer = Vt;
    Vt.isDecoderBuffer = function (e) {
      return e instanceof Vt
        ? !0
        : typeof e == 'object' &&
            cf.isBuffer(e.base) &&
            e.constructor.name === 'DecoderBuffer' &&
            typeof e.offset == 'number' &&
            typeof e.length == 'number' &&
            typeof e.save == 'function' &&
            typeof e.restore == 'function' &&
            typeof e.isEmpty == 'function' &&
            typeof e.readUInt8 == 'function' &&
            typeof e.skip == 'function' &&
            typeof e.raw == 'function';
    };
    Vt.prototype.save = function () {
      return { offset: this.offset, reporter: Bs.prototype.save.call(this) };
    };
    Vt.prototype.restore = function (e) {
      let r = new Vt(this.base);
      return (
        (r.offset = e.offset),
        (r.length = this.offset),
        (this.offset = e.offset),
        Bs.prototype.restore.call(this, e.reporter),
        r
      );
    };
    Vt.prototype.isEmpty = function () {
      return this.offset === this.length;
    };
    Vt.prototype.readUInt8 = function (e) {
      return this.offset + 1 <= this.length
        ? this.base.readUInt8(this.offset++, !0)
        : this.error(e || 'DecoderBuffer overrun');
    };
    Vt.prototype.skip = function (e, r) {
      if (!(this.offset + e <= this.length))
        return this.error(r || 'DecoderBuffer overrun');
      let n = new Vt(this.base);
      return (
        (n._reporterState = this._reporterState),
        (n.offset = this.offset),
        (n.length = this.offset + e),
        (this.offset += e),
        n
      );
    };
    Vt.prototype.raw = function (e) {
      return this.base.slice(e ? e.offset : this.offset, this.length);
    };
    function df(t, e) {
      if (Array.isArray(t))
        (this.length = 0),
          (this.value = t.map(function (r) {
            return (
              df.isEncoderBuffer(r) || (r = new df(r, e)),
              (this.length += r.length),
              r
            );
          }, this));
      else if (typeof t == 'number') {
        if (!(0 <= t && t <= 255))
          return e.error('non-byte EncoderBuffer value');
        (this.value = t), (this.length = 1);
      } else if (typeof t == 'string')
        (this.value = t), (this.length = cf.byteLength(t));
      else if (cf.isBuffer(t)) (this.value = t), (this.length = t.length);
      else return e.error('Unsupported type: ' + typeof t);
    }
    Uu.EncoderBuffer = df;
    df.isEncoderBuffer = function (e) {
      return e instanceof df
        ? !0
        : typeof e == 'object' &&
            e.constructor.name === 'EncoderBuffer' &&
            typeof e.length == 'number' &&
            typeof e.join == 'function';
    };
    df.prototype.join = function (e, r) {
      return (
        e || (e = cf.alloc(this.length)),
        r || (r = 0),
        this.length === 0 ||
          (Array.isArray(this.value)
            ? this.value.forEach(function (n) {
                n.join(e, r), (r += n.length);
              })
            : (typeof this.value == 'number'
                ? (e[r] = this.value)
                : typeof this.value == 'string'
                  ? e.write(this.value, r)
                  : cf.isBuffer(this.value) && this.value.copy(e, r),
              (r += this.length))),
        e
      );
    };
  });
  var Is = R((uk, W2) => {
    'use strict';
    S();
    var Fx = As().Reporter,
      jx = lf().EncoderBuffer,
      Ux = lf().DecoderBuffer,
      mt = At(),
      G2 = [
        'seq',
        'seqof',
        'set',
        'setof',
        'objid',
        'bool',
        'gentime',
        'utctime',
        'null_',
        'enum',
        'int',
        'objDesc',
        'bitstr',
        'bmpstr',
        'charstr',
        'genstr',
        'graphstr',
        'ia5str',
        'iso646str',
        'numstr',
        'octstr',
        'printstr',
        't61str',
        'unistr',
        'utf8str',
        'videostr',
      ],
      zx = [
        'key',
        'obj',
        'use',
        'optional',
        'explicit',
        'implicit',
        'def',
        'choice',
        'any',
        'contains',
      ].concat(G2),
      Hx = [
        '_peekTag',
        '_decodeTag',
        '_use',
        '_decodeStr',
        '_decodeObjid',
        '_decodeTime',
        '_decodeNull',
        '_decodeInt',
        '_decodeBool',
        '_decodeList',
        '_encodeComposite',
        '_encodeStr',
        '_encodeObjid',
        '_encodeTime',
        '_encodeNull',
        '_encodeInt',
        '_encodeBool',
      ];
    function Ke(t, e, r) {
      let n = {};
      (this._baseState = n),
        (n.name = r),
        (n.enc = t),
        (n.parent = e || null),
        (n.children = null),
        (n.tag = null),
        (n.args = null),
        (n.reverseArgs = null),
        (n.choice = null),
        (n.optional = !1),
        (n.any = !1),
        (n.obj = !1),
        (n.use = null),
        (n.useDecoder = null),
        (n.key = null),
        (n.default = null),
        (n.explicit = null),
        (n.implicit = null),
        (n.contains = null),
        n.parent || ((n.children = []), this._wrap());
    }
    W2.exports = Ke;
    var Kx = [
      'enc',
      'parent',
      'children',
      'tag',
      'args',
      'reverseArgs',
      'choice',
      'optional',
      'any',
      'obj',
      'use',
      'alteredUse',
      'key',
      'default',
      'explicit',
      'implicit',
      'contains',
    ];
    Ke.prototype.clone = function () {
      let e = this._baseState,
        r = {};
      Kx.forEach(function (i) {
        r[i] = e[i];
      });
      let n = new this.constructor(r.parent);
      return (n._baseState = r), n;
    };
    Ke.prototype._wrap = function () {
      let e = this._baseState;
      zx.forEach(function (r) {
        this[r] = function () {
          let i = new this.constructor(this);
          return e.children.push(i), i[r].apply(i, arguments);
        };
      }, this);
    };
    Ke.prototype._init = function (e) {
      let r = this._baseState;
      mt(r.parent === null),
        e.call(this),
        (r.children = r.children.filter(function (n) {
          return n._baseState.parent === this;
        }, this)),
        mt.equal(r.children.length, 1, 'Root node can have only one child');
    };
    Ke.prototype._useArgs = function (e) {
      let r = this._baseState,
        n = e.filter(function (i) {
          return i instanceof this.constructor;
        }, this);
      (e = e.filter(function (i) {
        return !(i instanceof this.constructor);
      }, this)),
        n.length !== 0 &&
          (mt(r.children === null),
          (r.children = n),
          n.forEach(function (i) {
            i._baseState.parent = this;
          }, this)),
        e.length !== 0 &&
          (mt(r.args === null),
          (r.args = e),
          (r.reverseArgs = e.map(function (i) {
            if (typeof i != 'object' || i.constructor !== Object) return i;
            let a = {};
            return (
              Object.keys(i).forEach(function (h) {
                h == (h | 0) && (h |= 0);
                let v = i[h];
                a[v] = h;
              }),
              a
            );
          })));
    };
    Hx.forEach(function (t) {
      Ke.prototype[t] = function () {
        let r = this._baseState;
        throw new Error(t + ' not implemented for encoding: ' + r.enc);
      };
    });
    G2.forEach(function (t) {
      Ke.prototype[t] = function () {
        let r = this._baseState,
          n = Array.prototype.slice.call(arguments);
        return mt(r.tag === null), (r.tag = t), this._useArgs(n), this;
      };
    });
    Ke.prototype.use = function (e) {
      mt(e);
      let r = this._baseState;
      return mt(r.use === null), (r.use = e), this;
    };
    Ke.prototype.optional = function () {
      let e = this._baseState;
      return (e.optional = !0), this;
    };
    Ke.prototype.def = function (e) {
      let r = this._baseState;
      return mt(r.default === null), (r.default = e), (r.optional = !0), this;
    };
    Ke.prototype.explicit = function (e) {
      let r = this._baseState;
      return (
        mt(r.explicit === null && r.implicit === null), (r.explicit = e), this
      );
    };
    Ke.prototype.implicit = function (e) {
      let r = this._baseState;
      return (
        mt(r.explicit === null && r.implicit === null), (r.implicit = e), this
      );
    };
    Ke.prototype.obj = function () {
      let e = this._baseState,
        r = Array.prototype.slice.call(arguments);
      return (e.obj = !0), r.length !== 0 && this._useArgs(r), this;
    };
    Ke.prototype.key = function (e) {
      let r = this._baseState;
      return mt(r.key === null), (r.key = e), this;
    };
    Ke.prototype.any = function () {
      let e = this._baseState;
      return (e.any = !0), this;
    };
    Ke.prototype.choice = function (e) {
      let r = this._baseState;
      return (
        mt(r.choice === null),
        (r.choice = e),
        this._useArgs(
          Object.keys(e).map(function (n) {
            return e[n];
          })
        ),
        this
      );
    };
    Ke.prototype.contains = function (e) {
      let r = this._baseState;
      return mt(r.use === null), (r.contains = e), this;
    };
    Ke.prototype._decode = function (e, r) {
      let n = this._baseState;
      if (n.parent === null) return e.wrapResult(n.children[0]._decode(e, r));
      let i = n.default,
        a = !0,
        h = null;
      if ((n.key !== null && (h = e.enterKey(n.key)), n.optional)) {
        let g = null;
        if (
          (n.explicit !== null
            ? (g = n.explicit)
            : n.implicit !== null
              ? (g = n.implicit)
              : n.tag !== null && (g = n.tag),
          g === null && !n.any)
        ) {
          let M = e.save();
          try {
            n.choice === null
              ? this._decodeGeneric(n.tag, e, r)
              : this._decodeChoice(e, r),
              (a = !0);
          } catch {
            a = !1;
          }
          e.restore(M);
        } else if (((a = this._peekTag(e, g, n.any)), e.isError(a))) return a;
      }
      let v;
      if ((n.obj && a && (v = e.enterObject()), a)) {
        if (n.explicit !== null) {
          let M = this._decodeTag(e, n.explicit);
          if (e.isError(M)) return M;
          e = M;
        }
        let g = e.offset;
        if (n.use === null && n.choice === null) {
          let M;
          n.any && (M = e.save());
          let x = this._decodeTag(
            e,
            n.implicit !== null ? n.implicit : n.tag,
            n.any
          );
          if (e.isError(x)) return x;
          n.any ? (i = e.raw(M)) : (e = x);
        }
        if (
          (r &&
            r.track &&
            n.tag !== null &&
            r.track(e.path(), g, e.length, 'tagged'),
          r &&
            r.track &&
            n.tag !== null &&
            r.track(e.path(), e.offset, e.length, 'content'),
          n.any ||
            (n.choice === null
              ? (i = this._decodeGeneric(n.tag, e, r))
              : (i = this._decodeChoice(e, r))),
          e.isError(i))
        )
          return i;
        if (
          (!n.any &&
            n.choice === null &&
            n.children !== null &&
            n.children.forEach(function (x) {
              x._decode(e, r);
            }),
          n.contains && (n.tag === 'octstr' || n.tag === 'bitstr'))
        ) {
          let M = new Ux(i);
          i = this._getUse(n.contains, e._reporterState.obj)._decode(M, r);
        }
      }
      return (
        n.obj && a && (i = e.leaveObject(v)),
        n.key !== null && (i !== null || a === !0)
          ? e.leaveKey(h, n.key, i)
          : h !== null && e.exitKey(h),
        i
      );
    };
    Ke.prototype._decodeGeneric = function (e, r, n) {
      let i = this._baseState;
      return e === 'seq' || e === 'set'
        ? null
        : e === 'seqof' || e === 'setof'
          ? this._decodeList(r, e, i.args[0], n)
          : /str$/.test(e)
            ? this._decodeStr(r, e, n)
            : e === 'objid' && i.args
              ? this._decodeObjid(r, i.args[0], i.args[1], n)
              : e === 'objid'
                ? this._decodeObjid(r, null, null, n)
                : e === 'gentime' || e === 'utctime'
                  ? this._decodeTime(r, e, n)
                  : e === 'null_'
                    ? this._decodeNull(r, n)
                    : e === 'bool'
                      ? this._decodeBool(r, n)
                      : e === 'objDesc'
                        ? this._decodeStr(r, e, n)
                        : e === 'int' || e === 'enum'
                          ? this._decodeInt(r, i.args && i.args[0], n)
                          : i.use !== null
                            ? this._getUse(i.use, r._reporterState.obj)._decode(
                                r,
                                n
                              )
                            : r.error('unknown tag: ' + e);
    };
    Ke.prototype._getUse = function (e, r) {
      let n = this._baseState;
      return (
        (n.useDecoder = this._use(e, r)),
        mt(n.useDecoder._baseState.parent === null),
        (n.useDecoder = n.useDecoder._baseState.children[0]),
        n.implicit !== n.useDecoder._baseState.implicit &&
          ((n.useDecoder = n.useDecoder.clone()),
          (n.useDecoder._baseState.implicit = n.implicit)),
        n.useDecoder
      );
    };
    Ke.prototype._decodeChoice = function (e, r) {
      let n = this._baseState,
        i = null,
        a = !1;
      return (
        Object.keys(n.choice).some(function (h) {
          let v = e.save(),
            g = n.choice[h];
          try {
            let M = g._decode(e, r);
            if (e.isError(M)) return !1;
            (i = { type: h, value: M }), (a = !0);
          } catch {
            return e.restore(v), !1;
          }
          return !0;
        }, this),
        a ? i : e.error('Choice not matched')
      );
    };
    Ke.prototype._createEncoderBuffer = function (e) {
      return new jx(e, this.reporter);
    };
    Ke.prototype._encode = function (e, r, n) {
      let i = this._baseState;
      if (i.default !== null && i.default === e) return;
      let a = this._encodeValue(e, r, n);
      if (a !== void 0 && !this._skipDefault(a, r, n)) return a;
    };
    Ke.prototype._encodeValue = function (e, r, n) {
      let i = this._baseState;
      if (i.parent === null) return i.children[0]._encode(e, r || new Fx());
      let a = null;
      if (((this.reporter = r), i.optional && e === void 0))
        if (i.default !== null) e = i.default;
        else return;
      let h = null,
        v = !1;
      if (i.any) a = this._createEncoderBuffer(e);
      else if (i.choice) a = this._encodeChoice(e, r);
      else if (i.contains)
        (h = this._getUse(i.contains, n)._encode(e, r)), (v = !0);
      else if (i.children)
        (h = i.children
          .map(function (g) {
            if (g._baseState.tag === 'null_') return g._encode(null, r, e);
            if (g._baseState.key === null)
              return r.error('Child should have a key');
            let M = r.enterKey(g._baseState.key);
            if (typeof e != 'object')
              return r.error('Child expected, but input is not object');
            let x = g._encode(e[g._baseState.key], r, e);
            return r.leaveKey(M), x;
          }, this)
          .filter(function (g) {
            return g;
          })),
          (h = this._createEncoderBuffer(h));
      else if (i.tag === 'seqof' || i.tag === 'setof') {
        if (!(i.args && i.args.length === 1))
          return r.error('Too many args for : ' + i.tag);
        if (!Array.isArray(e))
          return r.error('seqof/setof, but data is not Array');
        let g = this.clone();
        (g._baseState.implicit = null),
          (h = this._createEncoderBuffer(
            e.map(function (M) {
              let x = this._baseState;
              return this._getUse(x.args[0], e)._encode(M, r);
            }, g)
          ));
      } else
        i.use !== null
          ? (a = this._getUse(i.use, n)._encode(e, r))
          : ((h = this._encodePrimitive(i.tag, e)), (v = !0));
      if (!i.any && i.choice === null) {
        let g = i.implicit !== null ? i.implicit : i.tag,
          M = i.implicit === null ? 'universal' : 'context';
        g === null
          ? i.use === null && r.error('Tag could be omitted only for .use()')
          : i.use === null && (a = this._encodeComposite(g, v, M, h));
      }
      return (
        i.explicit !== null &&
          (a = this._encodeComposite(i.explicit, !1, 'context', a)),
        a
      );
    };
    Ke.prototype._encodeChoice = function (e, r) {
      let n = this._baseState,
        i = n.choice[e.type];
      return (
        i ||
          mt(
            !1,
            e.type + ' not found in ' + JSON.stringify(Object.keys(n.choice))
          ),
        i._encode(e.value, r)
      );
    };
    Ke.prototype._encodePrimitive = function (e, r) {
      let n = this._baseState;
      if (/str$/.test(e)) return this._encodeStr(r, e);
      if (e === 'objid' && n.args)
        return this._encodeObjid(r, n.reverseArgs[0], n.args[1]);
      if (e === 'objid') return this._encodeObjid(r, null, null);
      if (e === 'gentime' || e === 'utctime') return this._encodeTime(r, e);
      if (e === 'null_') return this._encodeNull();
      if (e === 'int' || e === 'enum')
        return this._encodeInt(r, n.args && n.reverseArgs[0]);
      if (e === 'bool') return this._encodeBool(r);
      if (e === 'objDesc') return this._encodeStr(r, e);
      throw new Error('Unsupported tag: ' + e);
    };
    Ke.prototype._isNumstr = function (e) {
      return /^[0-9 ]*$/.test(e);
    };
    Ke.prototype._isPrintstr = function (e) {
      return /^[A-Za-z0-9 '()+,-./:=?]*$/.test(e);
    };
  });
  var Rs = R((bn) => {
    'use strict';
    S();
    function $2(t) {
      let e = {};
      return (
        Object.keys(t).forEach(function (r) {
          (r | 0) == r && (r = r | 0);
          let n = t[r];
          e[n] = r;
        }),
        e
      );
    }
    bn.tagClass = {
      0: 'universal',
      1: 'application',
      2: 'context',
      3: 'private',
    };
    bn.tagClassByName = $2(bn.tagClass);
    bn.tag = {
      0: 'end',
      1: 'bool',
      2: 'int',
      3: 'bitstr',
      4: 'octstr',
      5: 'null_',
      6: 'objid',
      7: 'objDesc',
      8: 'external',
      9: 'real',
      10: 'enum',
      11: 'embed',
      12: 'utf8str',
      13: 'relativeOid',
      16: 'seq',
      17: 'set',
      18: 'numstr',
      19: 'printstr',
      20: 't61str',
      21: 'videostr',
      22: 'ia5str',
      23: 'utctime',
      24: 'gentime',
      25: 'graphstr',
      26: 'iso646str',
      27: 'genstr',
      28: 'unistr',
      29: 'charstr',
      30: 'bmpstr',
    };
    bn.tagByName = $2(bn.tag);
  });
  var Hu = R((pk, X2) => {
    'use strict';
    S();
    var Vx = qe(),
      Yr = Es().Buffer,
      Z2 = Is(),
      zu = Rs();
    function J2(t) {
      (this.enc = 'der'),
        (this.name = t.name),
        (this.entity = t),
        (this.tree = new mr()),
        this.tree._init(t.body);
    }
    X2.exports = J2;
    J2.prototype.encode = function (e, r) {
      return this.tree._encode(e, r).join();
    };
    function mr(t) {
      Z2.call(this, 'der', t);
    }
    Vx(mr, Z2);
    mr.prototype._encodeComposite = function (e, r, n, i) {
      let a = Gx(e, r, n, this.reporter);
      if (i.length < 128) {
        let g = Yr.alloc(2);
        return (g[0] = a), (g[1] = i.length), this._createEncoderBuffer([g, i]);
      }
      let h = 1;
      for (let g = i.length; g >= 256; g >>= 8) h++;
      let v = Yr.alloc(1 + 1 + h);
      (v[0] = a), (v[1] = 128 | h);
      for (let g = 1 + h, M = i.length; M > 0; g--, M >>= 8) v[g] = M & 255;
      return this._createEncoderBuffer([v, i]);
    };
    mr.prototype._encodeStr = function (e, r) {
      if (r === 'bitstr')
        return this._createEncoderBuffer([e.unused | 0, e.data]);
      if (r === 'bmpstr') {
        let n = Yr.alloc(e.length * 2);
        for (let i = 0; i < e.length; i++)
          n.writeUInt16BE(e.charCodeAt(i), i * 2);
        return this._createEncoderBuffer(n);
      } else
        return r === 'numstr'
          ? this._isNumstr(e)
            ? this._createEncoderBuffer(e)
            : this.reporter.error(
                'Encoding of string type: numstr supports only digits and space'
              )
          : r === 'printstr'
            ? this._isPrintstr(e)
              ? this._createEncoderBuffer(e)
              : this.reporter.error(
                  'Encoding of string type: printstr supports only latin upper and lower case letters, digits, space, apostrophe, left and rigth parenthesis, plus sign, comma, hyphen, dot, slash, colon, equal sign, question mark'
                )
            : /str$/.test(r)
              ? this._createEncoderBuffer(e)
              : r === 'objDesc'
                ? this._createEncoderBuffer(e)
                : this.reporter.error(
                    'Encoding of string type: ' + r + ' unsupported'
                  );
    };
    mr.prototype._encodeObjid = function (e, r, n) {
      if (typeof e == 'string') {
        if (!r)
          return this.reporter.error(
            'string objid given, but no values map found'
          );
        if (!r.hasOwnProperty(e))
          return this.reporter.error('objid not found in values map');
        e = r[e].split(/[\s.]+/g);
        for (let v = 0; v < e.length; v++) e[v] |= 0;
      } else if (Array.isArray(e)) {
        e = e.slice();
        for (let v = 0; v < e.length; v++) e[v] |= 0;
      }
      if (!Array.isArray(e))
        return this.reporter.error(
          'objid() should be either array or string, got: ' + JSON.stringify(e)
        );
      if (!n) {
        if (e[1] >= 40)
          return this.reporter.error('Second objid identifier OOB');
        e.splice(0, 2, e[0] * 40 + e[1]);
      }
      let i = 0;
      for (let v = 0; v < e.length; v++) {
        let g = e[v];
        for (i++; g >= 128; g >>= 7) i++;
      }
      let a = Yr.alloc(i),
        h = a.length - 1;
      for (let v = e.length - 1; v >= 0; v--) {
        let g = e[v];
        for (a[h--] = g & 127; (g >>= 7) > 0; ) a[h--] = 128 | (g & 127);
      }
      return this._createEncoderBuffer(a);
    };
    function Gt(t) {
      return t < 10 ? '0' + t : t;
    }
    mr.prototype._encodeTime = function (e, r) {
      let n,
        i = new Date(e);
      return (
        r === 'gentime'
          ? (n = [
              Gt(i.getUTCFullYear()),
              Gt(i.getUTCMonth() + 1),
              Gt(i.getUTCDate()),
              Gt(i.getUTCHours()),
              Gt(i.getUTCMinutes()),
              Gt(i.getUTCSeconds()),
              'Z',
            ].join(''))
          : r === 'utctime'
            ? (n = [
                Gt(i.getUTCFullYear() % 100),
                Gt(i.getUTCMonth() + 1),
                Gt(i.getUTCDate()),
                Gt(i.getUTCHours()),
                Gt(i.getUTCMinutes()),
                Gt(i.getUTCSeconds()),
                'Z',
              ].join(''))
            : this.reporter.error(
                'Encoding ' + r + ' time is not supported yet'
              ),
        this._encodeStr(n, 'octstr')
      );
    };
    mr.prototype._encodeNull = function () {
      return this._createEncoderBuffer('');
    };
    mr.prototype._encodeInt = function (e, r) {
      if (typeof e == 'string') {
        if (!r)
          return this.reporter.error(
            'String int or enum given, but no values map'
          );
        if (!r.hasOwnProperty(e))
          return this.reporter.error(
            "Values map doesn't contain: " + JSON.stringify(e)
          );
        e = r[e];
      }
      if (typeof e != 'number' && !Yr.isBuffer(e)) {
        let a = e.toArray();
        !e.sign && a[0] & 128 && a.unshift(0), (e = Yr.from(a));
      }
      if (Yr.isBuffer(e)) {
        let a = e.length;
        e.length === 0 && a++;
        let h = Yr.alloc(a);
        return (
          e.copy(h), e.length === 0 && (h[0] = 0), this._createEncoderBuffer(h)
        );
      }
      if (e < 128) return this._createEncoderBuffer(e);
      if (e < 256) return this._createEncoderBuffer([0, e]);
      let n = 1;
      for (let a = e; a >= 256; a >>= 8) n++;
      let i = new Array(n);
      for (let a = i.length - 1; a >= 0; a--) (i[a] = e & 255), (e >>= 8);
      return i[0] & 128 && i.unshift(0), this._createEncoderBuffer(Yr.from(i));
    };
    mr.prototype._encodeBool = function (e) {
      return this._createEncoderBuffer(e ? 255 : 0);
    };
    mr.prototype._use = function (e, r) {
      return typeof e == 'function' && (e = e(r)), e._getEncoder('der').tree;
    };
    mr.prototype._skipDefault = function (e, r, n) {
      let i = this._baseState,
        a;
      if (i.default === null) return !1;
      let h = e.join();
      if (
        (i.defaultBuffer === void 0 &&
          (i.defaultBuffer = this._encodeValue(i.default, r, n).join()),
        h.length !== i.defaultBuffer.length)
      )
        return !1;
      for (a = 0; a < h.length; a++) if (h[a] !== i.defaultBuffer[a]) return !1;
      return !0;
    };
    function Gx(t, e, r, n) {
      let i;
      if (
        (t === 'seqof' ? (t = 'seq') : t === 'setof' && (t = 'set'),
        zu.tagByName.hasOwnProperty(t))
      )
        i = zu.tagByName[t];
      else if (typeof t == 'number' && (t | 0) === t) i = t;
      else return n.error('Unknown tag: ' + t);
      return i >= 31
        ? n.error('Multi-octet tag encoding unsupported')
        : (e || (i |= 32), (i |= zu.tagClassByName[r || 'universal'] << 6), i);
    }
  });
  var Q2 = R((bk, Y2) => {
    'use strict';
    S();
    var Wx = qe(),
      Ku = Hu();
    function Vu(t) {
      Ku.call(this, t), (this.enc = 'pem');
    }
    Wx(Vu, Ku);
    Y2.exports = Vu;
    Vu.prototype.encode = function (e, r) {
      let i = Ku.prototype.encode.call(this, e).toString('base64'),
        a = ['-----BEGIN ' + r.label + '-----'];
      for (let h = 0; h < i.length; h += 64) a.push(i.slice(h, h + 64));
      return (
        a.push('-----END ' + r.label + '-----'),
        a.join(`
`)
      );
    };
  });
  var Gu = R((ty) => {
    'use strict';
    S();
    var ey = ty;
    ey.der = Hu();
    ey.pem = Q2();
  });
  var $u = R((wk, oy) => {
    'use strict';
    S();
    var $x = qe(),
      Zx = it(),
      ry = lf().DecoderBuffer,
      ny = Is(),
      iy = Rs();
    function fy(t) {
      (this.enc = 'der'),
        (this.name = t.name),
        (this.entity = t),
        (this.tree = new It()),
        this.tree._init(t.body);
    }
    oy.exports = fy;
    fy.prototype.decode = function (e, r) {
      return (
        ry.isDecoderBuffer(e) || (e = new ry(e, r)), this.tree._decode(e, r)
      );
    };
    function It(t) {
      ny.call(this, 'der', t);
    }
    $x(It, ny);
    It.prototype._peekTag = function (e, r, n) {
      if (e.isEmpty()) return !1;
      let i = e.save(),
        a = Wu(e, 'Failed to peek tag: "' + r + '"');
      return e.isError(a)
        ? a
        : (e.restore(i),
          a.tag === r || a.tagStr === r || a.tagStr + 'of' === r || n);
    };
    It.prototype._decodeTag = function (e, r, n) {
      let i = Wu(e, 'Failed to decode tag of "' + r + '"');
      if (e.isError(i)) return i;
      let a = ay(e, i.primitive, 'Failed to get length of "' + r + '"');
      if (e.isError(a)) return a;
      if (!n && i.tag !== r && i.tagStr !== r && i.tagStr + 'of' !== r)
        return e.error('Failed to match tag: "' + r + '"');
      if (i.primitive || a !== null)
        return e.skip(a, 'Failed to match body of: "' + r + '"');
      let h = e.save(),
        v = this._skipUntilEnd(
          e,
          'Failed to skip indefinite length body: "' + this.tag + '"'
        );
      return e.isError(v)
        ? v
        : ((a = e.offset - h.offset),
          e.restore(h),
          e.skip(a, 'Failed to match body of: "' + r + '"'));
    };
    It.prototype._skipUntilEnd = function (e, r) {
      for (;;) {
        let n = Wu(e, r);
        if (e.isError(n)) return n;
        let i = ay(e, n.primitive, r);
        if (e.isError(i)) return i;
        let a;
        if (
          (n.primitive || i !== null
            ? (a = e.skip(i))
            : (a = this._skipUntilEnd(e, r)),
          e.isError(a))
        )
          return a;
        if (n.tagStr === 'end') break;
      }
    };
    It.prototype._decodeList = function (e, r, n, i) {
      let a = [];
      for (; !e.isEmpty(); ) {
        let h = this._peekTag(e, 'end');
        if (e.isError(h)) return h;
        let v = n.decode(e, 'der', i);
        if (e.isError(v) && h) break;
        a.push(v);
      }
      return a;
    };
    It.prototype._decodeStr = function (e, r) {
      if (r === 'bitstr') {
        let n = e.readUInt8();
        return e.isError(n) ? n : { unused: n, data: e.raw() };
      } else if (r === 'bmpstr') {
        let n = e.raw();
        if (n.length % 2 === 1)
          return e.error('Decoding of string type: bmpstr length mismatch');
        let i = '';
        for (let a = 0; a < n.length / 2; a++)
          i += String.fromCharCode(n.readUInt16BE(a * 2));
        return i;
      } else if (r === 'numstr') {
        let n = e.raw().toString('ascii');
        return this._isNumstr(n)
          ? n
          : e.error('Decoding of string type: numstr unsupported characters');
      } else {
        if (r === 'octstr') return e.raw();
        if (r === 'objDesc') return e.raw();
        if (r === 'printstr') {
          let n = e.raw().toString('ascii');
          return this._isPrintstr(n)
            ? n
            : e.error(
                'Decoding of string type: printstr unsupported characters'
              );
        } else
          return /str$/.test(r)
            ? e.raw().toString()
            : e.error('Decoding of string type: ' + r + ' unsupported');
      }
    };
    It.prototype._decodeObjid = function (e, r, n) {
      let i,
        a = [],
        h = 0,
        v = 0;
      for (; !e.isEmpty(); )
        (v = e.readUInt8()),
          (h <<= 7),
          (h |= v & 127),
          (v & 128) === 0 && (a.push(h), (h = 0));
      v & 128 && a.push(h);
      let g = (a[0] / 40) | 0,
        M = a[0] % 40;
      if ((n ? (i = a) : (i = [g, M].concat(a.slice(1))), r)) {
        let x = r[i.join(' ')];
        x === void 0 && (x = r[i.join('.')]), x !== void 0 && (i = x);
      }
      return i;
    };
    It.prototype._decodeTime = function (e, r) {
      let n = e.raw().toString(),
        i,
        a,
        h,
        v,
        g,
        M;
      if (r === 'gentime')
        (i = n.slice(0, 4) | 0),
          (a = n.slice(4, 6) | 0),
          (h = n.slice(6, 8) | 0),
          (v = n.slice(8, 10) | 0),
          (g = n.slice(10, 12) | 0),
          (M = n.slice(12, 14) | 0);
      else if (r === 'utctime')
        (i = n.slice(0, 2) | 0),
          (a = n.slice(2, 4) | 0),
          (h = n.slice(4, 6) | 0),
          (v = n.slice(6, 8) | 0),
          (g = n.slice(8, 10) | 0),
          (M = n.slice(10, 12) | 0),
          i < 70 ? (i = 2e3 + i) : (i = 1900 + i);
      else return e.error('Decoding ' + r + ' time is not supported yet');
      return Date.UTC(i, a - 1, h, v, g, M, 0);
    };
    It.prototype._decodeNull = function () {
      return null;
    };
    It.prototype._decodeBool = function (e) {
      let r = e.readUInt8();
      return e.isError(r) ? r : r !== 0;
    };
    It.prototype._decodeInt = function (e, r) {
      let n = e.raw(),
        i = new Zx(n);
      return r && (i = r[i.toString(10)] || i), i;
    };
    It.prototype._use = function (e, r) {
      return typeof e == 'function' && (e = e(r)), e._getDecoder('der').tree;
    };
    function Wu(t, e) {
      let r = t.readUInt8(e);
      if (t.isError(r)) return r;
      let n = iy.tagClass[r >> 6],
        i = (r & 32) === 0;
      if ((r & 31) === 31) {
        let h = r;
        for (r = 0; (h & 128) === 128; ) {
          if (((h = t.readUInt8(e)), t.isError(h))) return h;
          (r <<= 7), (r |= h & 127);
        }
      } else r &= 31;
      let a = iy.tag[r];
      return { cls: n, primitive: i, tag: r, tagStr: a };
    }
    function ay(t, e, r) {
      let n = t.readUInt8(r);
      if (t.isError(n)) return n;
      if (!e && n === 128) return null;
      if ((n & 128) === 0) return n;
      let i = n & 127;
      if (i > 4) return t.error('length octect is too long');
      n = 0;
      for (let a = 0; a < i; a++) {
        n <<= 8;
        let h = t.readUInt8(r);
        if (t.isError(h)) return h;
        n |= h;
      }
      return n;
    }
  });
  var hy = R((xk, sy) => {
    'use strict';
    S();
    var Jx = qe(),
      Xx = Es().Buffer,
      Zu = $u();
    function Ju(t) {
      Zu.call(this, t), (this.enc = 'pem');
    }
    Jx(Ju, Zu);
    sy.exports = Ju;
    Ju.prototype.decode = function (e, r) {
      let n = e.toString().split(/[\r\n]+/g),
        i = r.label.toUpperCase(),
        a = /^-----(BEGIN|END) ([^-]+)-----$/,
        h = -1,
        v = -1;
      for (let x = 0; x < n.length; x++) {
        let E = n[x].match(a);
        if (E !== null && E[2] === i)
          if (h === -1) {
            if (E[1] !== 'BEGIN') break;
            h = x;
          } else {
            if (E[1] !== 'END') break;
            v = x;
            break;
          }
      }
      if (h === -1 || v === -1)
        throw new Error('PEM section not found for: ' + i);
      let g = n.slice(h + 1, v).join('');
      g.replace(/[^a-z0-9+/=]+/gi, '');
      let M = Xx.from(g, 'base64');
      return Zu.prototype.decode.call(this, M, r);
    };
  });
  var Xu = R((cy) => {
    'use strict';
    S();
    var uy = cy;
    uy.der = $u();
    uy.pem = hy();
  });
  var ly = R((dy) => {
    'use strict';
    S();
    var Yx = Gu(),
      Qx = Xu(),
      eM = qe(),
      tM = dy;
    tM.define = function (e, r) {
      return new pf(e, r);
    };
    function pf(t, e) {
      (this.name = t),
        (this.body = e),
        (this.decoders = {}),
        (this.encoders = {});
    }
    pf.prototype._createNamed = function (e) {
      let r = this.name;
      function n(i) {
        this._initNamed(i, r);
      }
      return (
        eM(n, e),
        (n.prototype._initNamed = function (a, h) {
          e.call(this, a, h);
        }),
        new n(this)
      );
    };
    pf.prototype._getDecoder = function (e) {
      return (
        (e = e || 'der'),
        this.decoders.hasOwnProperty(e) ||
          (this.decoders[e] = this._createNamed(Qx[e])),
        this.decoders[e]
      );
    };
    pf.prototype.decode = function (e, r, n) {
      return this._getDecoder(r).decode(e, n);
    };
    pf.prototype._getEncoder = function (e) {
      return (
        (e = e || 'der'),
        this.encoders.hasOwnProperty(e) ||
          (this.encoders[e] = this._createNamed(Yx[e])),
        this.encoders[e]
      );
    };
    pf.prototype.encode = function (e, r, n) {
      return this._getEncoder(r).encode(e, n);
    };
  });
  var vy = R((py) => {
    'use strict';
    S();
    var qs = py;
    qs.Reporter = As().Reporter;
    qs.DecoderBuffer = lf().DecoderBuffer;
    qs.EncoderBuffer = lf().EncoderBuffer;
    qs.Node = Is();
  });
  var my = R((yy) => {
    'use strict';
    S();
    var by = yy;
    by._reverse = function (e) {
      let r = {};
      return (
        Object.keys(e).forEach(function (n) {
          (n | 0) == n && (n = n | 0);
          let i = e[n];
          r[i] = n;
        }),
        r
      );
    };
    by.der = Rs();
  });
  var Yu = R((gy) => {
    'use strict';
    S();
    var vf = gy;
    vf.bignum = it();
    vf.define = ly().define;
    vf.base = vy();
    vf.constants = my();
    vf.decoders = Xu();
    vf.encoders = Gu();
  });
  var My = R((Ok, xy) => {
    'use strict';
    S();
    var gr = Yu(),
      wy = gr.define('Time', function () {
        this.choice({ utcTime: this.utctime(), generalTime: this.gentime() });
      }),
      rM = gr.define('AttributeTypeValue', function () {
        this.seq().obj(this.key('type').objid(), this.key('value').any());
      }),
      Qu = gr.define('AlgorithmIdentifier', function () {
        this.seq().obj(
          this.key('algorithm').objid(),
          this.key('parameters').optional(),
          this.key('curve').objid().optional()
        );
      }),
      iM = gr.define('SubjectPublicKeyInfo', function () {
        this.seq().obj(
          this.key('algorithm').use(Qu),
          this.key('subjectPublicKey').bitstr()
        );
      }),
      nM = gr.define('RelativeDistinguishedName', function () {
        this.setof(rM);
      }),
      fM = gr.define('RDNSequence', function () {
        this.seqof(nM);
      }),
      _y = gr.define('Name', function () {
        this.choice({ rdnSequence: this.use(fM) });
      }),
      aM = gr.define('Validity', function () {
        this.seq().obj(
          this.key('notBefore').use(wy),
          this.key('notAfter').use(wy)
        );
      }),
      oM = gr.define('Extension', function () {
        this.seq().obj(
          this.key('extnID').objid(),
          this.key('critical').bool().def(!1),
          this.key('extnValue').octstr()
        );
      }),
      sM = gr.define('TBSCertificate', function () {
        this.seq().obj(
          this.key('version').explicit(0).int().optional(),
          this.key('serialNumber').int(),
          this.key('signature').use(Qu),
          this.key('issuer').use(_y),
          this.key('validity').use(aM),
          this.key('subject').use(_y),
          this.key('subjectPublicKeyInfo').use(iM),
          this.key('issuerUniqueID').implicit(1).bitstr().optional(),
          this.key('subjectUniqueID').implicit(2).bitstr().optional(),
          this.key('extensions').explicit(3).seqof(oM).optional()
        );
      }),
      hM = gr.define('X509Certificate', function () {
        this.seq().obj(
          this.key('tbsCertificate').use(sM),
          this.key('signatureAlgorithm').use(Qu),
          this.key('signatureValue').bitstr()
        );
      });
    xy.exports = hM;
  });
  var Ey = R((_r) => {
    'use strict';
    S();
    var wr = Yu();
    _r.certificate = My();
    var uM = wr.define('RSAPrivateKey', function () {
      this.seq().obj(
        this.key('version').int(),
        this.key('modulus').int(),
        this.key('publicExponent').int(),
        this.key('privateExponent').int(),
        this.key('prime1').int(),
        this.key('prime2').int(),
        this.key('exponent1').int(),
        this.key('exponent2').int(),
        this.key('coefficient').int()
      );
    });
    _r.RSAPrivateKey = uM;
    var cM = wr.define('RSAPublicKey', function () {
      this.seq().obj(
        this.key('modulus').int(),
        this.key('publicExponent').int()
      );
    });
    _r.RSAPublicKey = cM;
    var dM = wr.define('SubjectPublicKeyInfo', function () {
      this.seq().obj(
        this.key('algorithm').use(Sy),
        this.key('subjectPublicKey').bitstr()
      );
    });
    _r.PublicKey = dM;
    var Sy = wr.define('AlgorithmIdentifier', function () {
        this.seq().obj(
          this.key('algorithm').objid(),
          this.key('none').null_().optional(),
          this.key('curve').objid().optional(),
          this.key('params')
            .seq()
            .obj(this.key('p').int(), this.key('q').int(), this.key('g').int())
            .optional()
        );
      }),
      lM = wr.define('PrivateKeyInfo', function () {
        this.seq().obj(
          this.key('version').int(),
          this.key('algorithm').use(Sy),
          this.key('subjectPrivateKey').octstr()
        );
      });
    _r.PrivateKey = lM;
    var pM = wr.define('EncryptedPrivateKeyInfo', function () {
      this.seq().obj(
        this.key('algorithm')
          .seq()
          .obj(
            this.key('id').objid(),
            this.key('decrypt')
              .seq()
              .obj(
                this.key('kde')
                  .seq()
                  .obj(
                    this.key('id').objid(),
                    this.key('kdeparams')
                      .seq()
                      .obj(this.key('salt').octstr(), this.key('iters').int())
                  ),
                this.key('cipher')
                  .seq()
                  .obj(this.key('algo').objid(), this.key('iv').octstr())
              )
          ),
        this.key('subjectPrivateKey').octstr()
      );
    });
    _r.EncryptedPrivateKey = pM;
    var vM = wr.define('DSAPrivateKey', function () {
      this.seq().obj(
        this.key('version').int(),
        this.key('p').int(),
        this.key('q').int(),
        this.key('g').int(),
        this.key('pub_key').int(),
        this.key('priv_key').int()
      );
    });
    _r.DSAPrivateKey = vM;
    _r.DSAparam = wr.define('DSAparam', function () {
      this.int();
    });
    var bM = wr.define('ECPrivateKey', function () {
      this.seq().obj(
        this.key('version').int(),
        this.key('privateKey').octstr(),
        this.key('parameters').optional().explicit(0).use(yM),
        this.key('publicKey').optional().explicit(1).bitstr()
      );
    });
    _r.ECPrivateKey = bM;
    var yM = wr.define('ECParameters', function () {
      this.choice({ namedCurve: this.objid() });
    });
    _r.signature = wr.define('signature', function () {
      this.seq().obj(this.key('r').int(), this.key('s').int());
    });
  });
  var Ay = R((Lk, mM) => {
    mM.exports = {
      '2.16.840.1.101.3.4.1.1': 'aes-128-ecb',
      '2.16.840.1.101.3.4.1.2': 'aes-128-cbc',
      '2.16.840.1.101.3.4.1.3': 'aes-128-ofb',
      '2.16.840.1.101.3.4.1.4': 'aes-128-cfb',
      '2.16.840.1.101.3.4.1.21': 'aes-192-ecb',
      '2.16.840.1.101.3.4.1.22': 'aes-192-cbc',
      '2.16.840.1.101.3.4.1.23': 'aes-192-ofb',
      '2.16.840.1.101.3.4.1.24': 'aes-192-cfb',
      '2.16.840.1.101.3.4.1.41': 'aes-256-ecb',
      '2.16.840.1.101.3.4.1.42': 'aes-256-cbc',
      '2.16.840.1.101.3.4.1.43': 'aes-256-ofb',
      '2.16.840.1.101.3.4.1.44': 'aes-256-cfb',
    };
  });
  var Iy = R((Fk, By) => {
    S();
    var gM =
        /Proc-Type: 4,ENCRYPTED[\n\r]+DEK-Info: AES-((?:128)|(?:192)|(?:256))-CBC,([0-9A-H]+)[\n\r]+([0-9A-z\n\r+/=]+)[\n\r]+/m,
      wM = /^-----BEGIN ((?:.*? KEY)|CERTIFICATE)-----/m,
      _M =
        /^-----BEGIN ((?:.*? KEY)|CERTIFICATE)-----([0-9A-z\n\r+/=]+)-----END \1-----$/m,
      xM = sa(),
      MM = rs(),
      Ts = Ie().Buffer;
    By.exports = function (t, e) {
      var r = t.toString(),
        n = r.match(gM),
        i;
      if (n) {
        var h = 'aes' + n[1],
          v = Ts.from(n[2], 'hex'),
          g = Ts.from(n[3].replace(/[\r\n]/g, ''), 'base64'),
          M = xM(e, v.slice(0, 8), parseInt(n[1], 10)).key,
          x = [],
          E = MM.createDecipheriv(h, M, v);
        x.push(E.update(g)), x.push(E.final()), (i = Ts.concat(x));
      } else {
        var a = r.match(_M);
        i = Ts.from(a[2].replace(/[\r\n]/g, ''), 'base64');
      }
      var I = r.match(wM)[1];
      return { tag: I, data: i };
    };
  });
  var ba = R((Uk, qy) => {
    S();
    var St = Ey(),
      SM = Ay(),
      EM = Iy(),
      AM = rs(),
      BM = jh(),
      ec = Ie().Buffer;
    qy.exports = Ry;
    function Ry(t) {
      var e;
      typeof t == 'object' &&
        !ec.isBuffer(t) &&
        ((e = t.passphrase), (t = t.key)),
        typeof t == 'string' && (t = ec.from(t));
      var r = EM(t, e),
        n = r.tag,
        i = r.data,
        a,
        h;
      switch (n) {
        case 'CERTIFICATE':
          h = St.certificate.decode(i, 'der').tbsCertificate
            .subjectPublicKeyInfo;
        case 'PUBLIC KEY':
          switch (
            (h || (h = St.PublicKey.decode(i, 'der')),
            (a = h.algorithm.algorithm.join('.')),
            a)
          ) {
            case '1.2.840.113549.1.1.1':
              return St.RSAPublicKey.decode(h.subjectPublicKey.data, 'der');
            case '1.2.840.10045.2.1':
              return (
                (h.subjectPrivateKey = h.subjectPublicKey),
                { type: 'ec', data: h }
              );
            case '1.2.840.10040.4.1':
              return (
                (h.algorithm.params.pub_key = St.DSAparam.decode(
                  h.subjectPublicKey.data,
                  'der'
                )),
                { type: 'dsa', data: h.algorithm.params }
              );
            default:
              throw new Error('unknown key id ' + a);
          }
        case 'ENCRYPTED PRIVATE KEY':
          (i = St.EncryptedPrivateKey.decode(i, 'der')), (i = IM(i, e));
        case 'PRIVATE KEY':
          switch (
            ((h = St.PrivateKey.decode(i, 'der')),
            (a = h.algorithm.algorithm.join('.')),
            a)
          ) {
            case '1.2.840.113549.1.1.1':
              return St.RSAPrivateKey.decode(h.subjectPrivateKey, 'der');
            case '1.2.840.10045.2.1':
              return {
                curve: h.algorithm.curve,
                privateKey: St.ECPrivateKey.decode(h.subjectPrivateKey, 'der')
                  .privateKey,
              };
            case '1.2.840.10040.4.1':
              return (
                (h.algorithm.params.priv_key = St.DSAparam.decode(
                  h.subjectPrivateKey,
                  'der'
                )),
                { type: 'dsa', params: h.algorithm.params }
              );
            default:
              throw new Error('unknown key id ' + a);
          }
        case 'RSA PUBLIC KEY':
          return St.RSAPublicKey.decode(i, 'der');
        case 'RSA PRIVATE KEY':
          return St.RSAPrivateKey.decode(i, 'der');
        case 'DSA PRIVATE KEY':
          return { type: 'dsa', params: St.DSAPrivateKey.decode(i, 'der') };
        case 'EC PRIVATE KEY':
          return (
            (i = St.ECPrivateKey.decode(i, 'der')),
            { curve: i.parameters.value, privateKey: i.privateKey }
          );
        default:
          throw new Error('unknown key type ' + n);
      }
    }
    Ry.signature = St.signature;
    function IM(t, e) {
      var r = t.algorithm.decrypt.kde.kdeparams.salt,
        n = parseInt(t.algorithm.decrypt.kde.kdeparams.iters.toString(), 10),
        i = SM[t.algorithm.decrypt.cipher.algo.join('.')],
        a = t.algorithm.decrypt.cipher.iv,
        h = t.subjectPrivateKey,
        v = parseInt(i.split('-')[1], 10) / 8,
        g = BM.pbkdf2Sync(e, r, n, v, 'sha1'),
        M = AM.createDecipheriv(i, g, a),
        x = [];
      return x.push(M.update(h)), x.push(M.final()), ec.concat(x);
    }
  });
  var tc = R((Hk, RM) => {
    RM.exports = {
      '1.3.132.0.10': 'secp256k1',
      '1.3.132.0.33': 'p224',
      '1.2.840.10045.3.1.1': 'p192',
      '1.2.840.10045.3.1.7': 'p256',
      '1.3.132.0.34': 'p384',
      '1.3.132.0.35': 'p521',
    };
  });
  var ky = R((Kk, ks) => {
    S();
    var yt = Ie().Buffer,
      yn = qh(),
      qM = as(),
      TM = Ms().ec,
      Ps = ju(),
      PM = ba(),
      kM = tc();
    function OM(t, e, r, n, i) {
      var a = PM(e);
      if (a.curve) {
        if (n !== 'ecdsa' && n !== 'ecdsa/rsa')
          throw new Error('wrong private key type');
        return CM(t, a);
      } else if (a.type === 'dsa') {
        if (n !== 'dsa') throw new Error('wrong private key type');
        return NM(t, a, r);
      } else if (n !== 'rsa' && n !== 'ecdsa/rsa')
        throw new Error('wrong private key type');
      t = yt.concat([i, t]);
      for (
        var h = a.modulus.byteLength(), v = [0, 1];
        t.length + v.length + 1 < h;

      )
        v.push(255);
      v.push(0);
      for (var g = -1; ++g < t.length; ) v.push(t[g]);
      var M = qM(v, a);
      return M;
    }
    function CM(t, e) {
      var r = kM[e.curve.join('.')];
      if (!r) throw new Error('unknown curve ' + e.curve.join('.'));
      var n = new TM(r),
        i = n.keyFromPrivate(e.privateKey),
        a = i.sign(t);
      return yt.from(a.toDER());
    }
    function NM(t, e, r) {
      for (
        var n = e.params.priv_key,
          i = e.params.p,
          a = e.params.q,
          h = e.params.g,
          v = new Ps(0),
          g,
          M = rc(t, a).mod(a),
          x = !1,
          E = Ty(n, a, t, r);
        x === !1;

      )
        (g = Py(a, E, r)),
          (v = FM(h, g, i, a)),
          (x = g
            .invm(a)
            .imul(M.add(n.mul(v)))
            .mod(a)),
          x.cmpn(0) === 0 && ((x = !1), (v = new Ps(0)));
      return DM(v, x);
    }
    function DM(t, e) {
      (t = t.toArray()),
        (e = e.toArray()),
        t[0] & 128 && (t = [0].concat(t)),
        e[0] & 128 && (e = [0].concat(e));
      var r = t.length + e.length + 4,
        n = [48, r, 2, t.length];
      return (n = n.concat(t, [2, e.length], e)), yt.from(n);
    }
    function Ty(t, e, r, n) {
      if (((t = yt.from(t.toArray())), t.length < e.byteLength())) {
        var i = yt.alloc(e.byteLength() - t.length);
        t = yt.concat([i, t]);
      }
      var a = r.length,
        h = LM(r, e),
        v = yt.alloc(a);
      v.fill(1);
      var g = yt.alloc(a);
      return (
        (g = yn(n, g)
          .update(v)
          .update(yt.from([0]))
          .update(t)
          .update(h)
          .digest()),
        (v = yn(n, g).update(v).digest()),
        (g = yn(n, g)
          .update(v)
          .update(yt.from([1]))
          .update(t)
          .update(h)
          .digest()),
        (v = yn(n, g).update(v).digest()),
        { k: g, v }
      );
    }
    function rc(t, e) {
      var r = new Ps(t),
        n = (t.length << 3) - e.bitLength();
      return n > 0 && r.ishrn(n), r;
    }
    function LM(t, e) {
      (t = rc(t, e)), (t = t.mod(e));
      var r = yt.from(t.toArray());
      if (r.length < e.byteLength()) {
        var n = yt.alloc(e.byteLength() - r.length);
        r = yt.concat([n, r]);
      }
      return r;
    }
    function Py(t, e, r) {
      var n, i;
      do {
        for (n = yt.alloc(0); n.length * 8 < t.bitLength(); )
          (e.v = yn(r, e.k).update(e.v).digest()), (n = yt.concat([n, e.v]));
        (i = rc(n, t)),
          (e.k = yn(r, e.k)
            .update(e.v)
            .update(yt.from([0]))
            .digest()),
          (e.v = yn(r, e.k).update(e.v).digest());
      } while (i.cmp(t) !== -1);
      return i;
    }
    function FM(t, e, r, n) {
      return t.toRed(Ps.mont(r)).redPow(e).fromRed().mod(n);
    }
    ks.exports = OM;
    ks.exports.getKey = Ty;
    ks.exports.makeKey = Py;
  });
  var Dy = R((Gk, Ny) => {
    S();
    var ic = Ie().Buffer,
      ya = ju(),
      jM = Ms().ec,
      Cy = ba(),
      UM = tc();
    function zM(t, e, r, n, i) {
      var a = Cy(r);
      if (a.type === 'ec') {
        if (n !== 'ecdsa' && n !== 'ecdsa/rsa')
          throw new Error('wrong public key type');
        return HM(t, e, a);
      } else if (a.type === 'dsa') {
        if (n !== 'dsa') throw new Error('wrong public key type');
        return KM(t, e, a);
      } else if (n !== 'rsa' && n !== 'ecdsa/rsa')
        throw new Error('wrong public key type');
      e = ic.concat([i, e]);
      for (
        var h = a.modulus.byteLength(), v = [1], g = 0;
        e.length + v.length + 2 < h;

      )
        v.push(255), g++;
      v.push(0);
      for (var M = -1; ++M < e.length; ) v.push(e[M]);
      v = ic.from(v);
      var x = ya.mont(a.modulus);
      (t = new ya(t).toRed(x)),
        (t = t.redPow(new ya(a.publicExponent))),
        (t = ic.from(t.fromRed().toArray()));
      var E = g < 8 ? 1 : 0;
      for (
        h = Math.min(t.length, v.length),
          t.length !== v.length && (E = 1),
          M = -1;
        ++M < h;

      )
        E |= t[M] ^ v[M];
      return E === 0;
    }
    function HM(t, e, r) {
      var n = UM[r.data.algorithm.curve.join('.')];
      if (!n)
        throw new Error('unknown curve ' + r.data.algorithm.curve.join('.'));
      var i = new jM(n),
        a = r.data.subjectPrivateKey.data;
      return i.verify(e, t, a);
    }
    function KM(t, e, r) {
      var n = r.data.p,
        i = r.data.q,
        a = r.data.g,
        h = r.data.pub_key,
        v = Cy.signature.decode(t, 'der'),
        g = v.s,
        M = v.r;
      Oy(g, i), Oy(M, i);
      var x = ya.mont(n),
        E = g.invm(i),
        I = a
          .toRed(x)
          .redPow(new ya(e).mul(E).mod(i))
          .fromRed()
          .mul(h.toRed(x).redPow(M.mul(E).mod(i)).fromRed())
          .mod(n)
          .mod(i);
      return I.cmp(M) === 0;
    }
    function Oy(t, e) {
      if (t.cmpn(0) <= 0) throw new Error('invalid sig');
      if (t.cmp(e) >= e) throw new Error('invalid sig');
    }
    Ny.exports = zM;
  });
  var Hy = R(($k, zy) => {
    S();
    var Os = Ie().Buffer,
      jy = zn(),
      Cs = xh(),
      Uy = qe(),
      VM = ky(),
      GM = Dy(),
      mn = Th();
    Object.keys(mn).forEach(function (t) {
      (mn[t].id = Os.from(mn[t].id, 'hex')), (mn[t.toLowerCase()] = mn[t]);
    });
    function ma(t) {
      Cs.Writable.call(this);
      var e = mn[t];
      if (!e) throw new Error('Unknown message digest');
      (this._hashType = e.hash),
        (this._hash = jy(e.hash)),
        (this._tag = e.id),
        (this._signType = e.sign);
    }
    Uy(ma, Cs.Writable);
    ma.prototype._write = function (e, r, n) {
      this._hash.update(e), n();
    };
    ma.prototype.update = function (e, r) {
      return (
        typeof e == 'string' && (e = Os.from(e, r)), this._hash.update(e), this
      );
    };
    ma.prototype.sign = function (e, r) {
      this.end();
      var n = this._hash.digest(),
        i = VM(n, e, this._hashType, this._signType, this._tag);
      return r ? i.toString(r) : i;
    };
    function ga(t) {
      Cs.Writable.call(this);
      var e = mn[t];
      if (!e) throw new Error('Unknown message digest');
      (this._hash = jy(e.hash)), (this._tag = e.id), (this._signType = e.sign);
    }
    Uy(ga, Cs.Writable);
    ga.prototype._write = function (e, r, n) {
      this._hash.update(e), n();
    };
    ga.prototype.update = function (e, r) {
      return (
        typeof e == 'string' && (e = Os.from(e, r)), this._hash.update(e), this
      );
    };
    ga.prototype.verify = function (e, r, n) {
      typeof r == 'string' && (r = Os.from(r, n)), this.end();
      var i = this._hash.digest();
      return GM(r, i, e, this._signType, this._tag);
    };
    function Ly(t) {
      return new ma(t);
    }
    function Fy(t) {
      return new ga(t);
    }
    zy.exports = { Sign: Ly, Verify: Fy, createSign: Ly, createVerify: Fy };
  });
  var Vy = R((Jk, Ky) => {
    S();
    var WM = Ms(),
      $M = it();
    Ky.exports = function (e) {
      return new gn(e);
    };
    var Rt = {
      secp256k1: { name: 'secp256k1', byteLength: 32 },
      secp224r1: { name: 'p224', byteLength: 28 },
      prime256v1: { name: 'p256', byteLength: 32 },
      prime192v1: { name: 'p192', byteLength: 24 },
      ed25519: { name: 'ed25519', byteLength: 32 },
      secp384r1: { name: 'p384', byteLength: 48 },
      secp521r1: { name: 'p521', byteLength: 66 },
    };
    Rt.p224 = Rt.secp224r1;
    Rt.p256 = Rt.secp256r1 = Rt.prime256v1;
    Rt.p192 = Rt.secp192r1 = Rt.prime192v1;
    Rt.p384 = Rt.secp384r1;
    Rt.p521 = Rt.secp521r1;
    function gn(t) {
      (this.curveType = Rt[t]),
        this.curveType || (this.curveType = { name: t }),
        (this.curve = new WM.ec(this.curveType.name)),
        (this.keys = void 0);
    }
    gn.prototype.generateKeys = function (t, e) {
      return (this.keys = this.curve.genKeyPair()), this.getPublicKey(t, e);
    };
    gn.prototype.computeSecret = function (t, e, r) {
      (e = e || 'utf8'), Buffer.isBuffer(t) || (t = new Buffer(t, e));
      var n = this.curve.keyFromPublic(t).getPublic(),
        i = n.mul(this.keys.getPrivate()).getX();
      return nc(i, r, this.curveType.byteLength);
    };
    gn.prototype.getPublicKey = function (t, e) {
      var r = this.keys.getPublic(e === 'compressed', !0);
      return (
        e === 'hybrid' && (r[r.length - 1] % 2 ? (r[0] = 7) : (r[0] = 6)),
        nc(r, t)
      );
    };
    gn.prototype.getPrivateKey = function (t) {
      return nc(this.keys.getPrivate(), t);
    };
    gn.prototype.setPublicKey = function (t, e) {
      return (
        (e = e || 'utf8'),
        Buffer.isBuffer(t) || (t = new Buffer(t, e)),
        this.keys._importPublic(t),
        this
      );
    };
    gn.prototype.setPrivateKey = function (t, e) {
      (e = e || 'utf8'), Buffer.isBuffer(t) || (t = new Buffer(t, e));
      var r = new $M(t);
      return (
        (r = r.toString(16)),
        (this.keys = this.curve.genKeyPair()),
        this.keys._importPrivate(r),
        this
      );
    };
    function nc(t, e, r) {
      Array.isArray(t) || (t = t.toArray());
      var n = new Buffer(t);
      if (r && n.length < r) {
        var i = new Buffer(r - n.length);
        i.fill(0), (n = Buffer.concat([i, n]));
      }
      return e ? n.toString(e) : n;
    }
  });
  var ac = R((Yk, Gy) => {
    S();
    var ZM = zn(),
      fc = Ie().Buffer;
    Gy.exports = function (t, e) {
      for (var r = fc.alloc(0), n = 0, i; r.length < e; )
        (i = JM(n++)),
          (r = fc.concat([r, ZM('sha1').update(t).update(i).digest()]));
      return r.slice(0, e);
    };
    function JM(t) {
      var e = fc.allocUnsafe(4);
      return e.writeUInt32BE(t, 0), e;
    }
  });
  var oc = R((eO, Wy) => {
    S();
    Wy.exports = function (e, r) {
      for (var n = e.length, i = -1; ++i < n; ) e[i] ^= r[i];
      return e;
    };
  });
  var sc = R((rO, Zy) => {
    S();
    var $y = it(),
      XM = Ie().Buffer;
    function YM(t, e) {
      return XM.from(
        t
          .toRed($y.mont(e.modulus))
          .redPow(new $y(e.publicExponent))
          .fromRed()
          .toArray()
      );
    }
    Zy.exports = YM;
  });
  var Qy = R((nO, Yy) => {
    S();
    var QM = ba(),
      hc = Xi(),
      eS = zn(),
      Jy = ac(),
      Xy = oc(),
      uc = it(),
      tS = sc(),
      rS = as(),
      xr = Ie().Buffer;
    Yy.exports = function (e, r, n) {
      var i;
      e.padding ? (i = e.padding) : n ? (i = 1) : (i = 4);
      var a = QM(e),
        h;
      if (i === 4) h = iS(a, r);
      else if (i === 1) h = nS(a, r, n);
      else if (i === 3) {
        if (((h = new uc(r)), h.cmp(a.modulus) >= 0))
          throw new Error('data too long for modulus');
      } else throw new Error('unknown padding');
      return n ? rS(h, a) : tS(h, a);
    };
    function iS(t, e) {
      var r = t.modulus.byteLength(),
        n = e.length,
        i = eS('sha1').update(xr.alloc(0)).digest(),
        a = i.length,
        h = 2 * a;
      if (n > r - h - 2) throw new Error('message too long');
      var v = xr.alloc(r - n - h - 2),
        g = r - a - 1,
        M = hc(a),
        x = Xy(xr.concat([i, v, xr.alloc(1, 1), e], g), Jy(M, g)),
        E = Xy(M, Jy(x, a));
      return new uc(xr.concat([xr.alloc(1), E, x], r));
    }
    function nS(t, e, r) {
      var n = e.length,
        i = t.modulus.byteLength();
      if (n > i - 11) throw new Error('message too long');
      var a;
      return (
        r ? (a = xr.alloc(i - n - 3, 255)) : (a = fS(i - n - 3)),
        new uc(xr.concat([xr.from([0, r ? 1 : 2]), a, xr.alloc(1), e], i))
      );
    }
    function fS(t) {
      for (var e = xr.allocUnsafe(t), r = 0, n = hc(t * 2), i = 0, a; r < t; )
        i === n.length && ((n = hc(t * 2)), (i = 0)),
          (a = n[i++]),
          a && (e[r++] = a);
      return e;
    }
  });
  var n6 = R((aO, i6) => {
    S();
    var aS = ba(),
      e6 = ac(),
      t6 = oc(),
      r6 = it(),
      oS = as(),
      sS = zn(),
      hS = sc(),
      wa = Ie().Buffer;
    i6.exports = function (e, r, n) {
      var i;
      e.padding ? (i = e.padding) : n ? (i = 1) : (i = 4);
      var a = aS(e),
        h = a.modulus.byteLength();
      if (r.length > h || new r6(r).cmp(a.modulus) >= 0)
        throw new Error('decryption error');
      var v;
      n ? (v = hS(new r6(r), a)) : (v = oS(r, a));
      var g = wa.alloc(h - v.length);
      if (((v = wa.concat([g, v], h)), i === 4)) return uS(a, v);
      if (i === 1) return cS(a, v, n);
      if (i === 3) return v;
      throw new Error('unknown padding');
    };
    function uS(t, e) {
      var r = t.modulus.byteLength(),
        n = sS('sha1').update(wa.alloc(0)).digest(),
        i = n.length;
      if (e[0] !== 0) throw new Error('decryption error');
      var a = e.slice(1, i + 1),
        h = e.slice(i + 1),
        v = t6(a, e6(h, i)),
        g = t6(h, e6(v, r - i - 1));
      if (dS(n, g.slice(0, i))) throw new Error('decryption error');
      for (var M = i; g[M] === 0; ) M++;
      if (g[M++] !== 1) throw new Error('decryption error');
      return g.slice(M);
    }
    function cS(t, e, r) {
      for (var n = e.slice(0, 2), i = 2, a = 0; e[i++] !== 0; )
        if (i >= e.length) {
          a++;
          break;
        }
      var h = e.slice(2, i - 1);
      if (
        (((n.toString('hex') !== '0002' && !r) ||
          (n.toString('hex') !== '0001' && r)) &&
          a++,
        h.length < 8 && a++,
        a)
      )
        throw new Error('decryption error');
      return e.slice(i);
    }
    function dS(t, e) {
      (t = wa.from(t)), (e = wa.from(e));
      var r = 0,
        n = t.length;
      t.length !== e.length && (r++, (n = Math.min(t.length, e.length)));
      for (var i = -1; ++i < n; ) r += t[i] ^ e[i];
      return r;
    }
  });
  var f6 = R((wn) => {
    S();
    wn.publicEncrypt = Qy();
    wn.privateDecrypt = n6();
    wn.privateEncrypt = function (e, r) {
      return wn.publicEncrypt(e, r, !0);
    };
    wn.publicDecrypt = function (e, r) {
      return wn.privateDecrypt(e, r, !0);
    };
  });
  var v6 = R((_a) => {
    'use strict';
    S();
    function a6() {
      throw new Error(`secure random number generation not supported by this browser
use chrome, FireFox or Internet Explorer 11`);
    }
    var s6 = Ie(),
      o6 = Xi(),
      h6 = s6.Buffer,
      u6 = s6.kMaxLength,
      cc = globalThis.crypto || globalThis.msCrypto,
      c6 = Math.pow(2, 32) - 1;
    function d6(t, e) {
      if (typeof t != 'number' || t !== t)
        throw new TypeError('offset must be a number');
      if (t > c6 || t < 0) throw new TypeError('offset must be a uint32');
      if (t > u6 || t > e) throw new RangeError('offset out of range');
    }
    function l6(t, e, r) {
      if (typeof t != 'number' || t !== t)
        throw new TypeError('size must be a number');
      if (t > c6 || t < 0) throw new TypeError('size must be a uint32');
      if (t + e > r || t > u6) throw new RangeError('buffer too small');
    }
    (cc && cc.getRandomValues) || !A.browser
      ? ((_a.randomFill = lS), (_a.randomFillSync = pS))
      : ((_a.randomFill = a6), (_a.randomFillSync = a6));
    function lS(t, e, r, n) {
      if (!h6.isBuffer(t) && !(t instanceof globalThis.Uint8Array))
        throw new TypeError('"buf" argument must be a Buffer or Uint8Array');
      if (typeof e == 'function') (n = e), (e = 0), (r = t.length);
      else if (typeof r == 'function') (n = r), (r = t.length - e);
      else if (typeof n != 'function')
        throw new TypeError('"cb" argument must be a function');
      return d6(e, t.length), l6(r, e, t.length), p6(t, e, r, n);
    }
    function p6(t, e, r, n) {
      if (A.browser) {
        var i = t.buffer,
          a = new Uint8Array(i, e, r);
        if ((cc.getRandomValues(a), n)) {
          A.nextTick(function () {
            n(null, t);
          });
          return;
        }
        return t;
      }
      if (n) {
        o6(r, function (v, g) {
          if (v) return n(v);
          g.copy(t, e), n(null, t);
        });
        return;
      }
      var h = o6(r);
      return h.copy(t, e), t;
    }
    function pS(t, e, r) {
      if (
        (typeof e > 'u' && (e = 0),
        !h6.isBuffer(t) && !(t instanceof globalThis.Uint8Array))
      )
        throw new TypeError('"buf" argument must be a Buffer or Uint8Array');
      return (
        d6(e, t.length),
        r === void 0 && (r = t.length - e),
        l6(r, e, t.length),
        p6(t, e, r)
      );
    }
  });
  var hu = R((Pe) => {
    'use strict';
    S();
    Pe.randomBytes = Pe.rng = Pe.pseudoRandomBytes = Pe.prng = Xi();
    Pe.createHash = Pe.Hash = zn();
    Pe.createHmac = Pe.Hmac = qh();
    var vS = kp(),
      bS = Object.keys(vS),
      yS = [
        'sha1',
        'sha224',
        'sha256',
        'sha384',
        'sha512',
        'md5',
        'rmd160',
      ].concat(bS);
    Pe.getHashes = function () {
      return yS;
    };
    var b6 = jh();
    Pe.pbkdf2 = b6.pbkdf2;
    Pe.pbkdf2Sync = b6.pbkdf2Sync;
    var Nr = tb();
    Pe.Cipher = Nr.Cipher;
    Pe.createCipher = Nr.createCipher;
    Pe.Cipheriv = Nr.Cipheriv;
    Pe.createCipheriv = Nr.createCipheriv;
    Pe.Decipher = Nr.Decipher;
    Pe.createDecipher = Nr.createDecipher;
    Pe.Decipheriv = Nr.Decipheriv;
    Pe.createDecipheriv = Nr.createDecipheriv;
    Pe.getCiphers = Nr.getCiphers;
    Pe.listCiphers = Nr.listCiphers;
    var xa = pb();
    Pe.DiffieHellmanGroup = xa.DiffieHellmanGroup;
    Pe.createDiffieHellmanGroup = xa.createDiffieHellmanGroup;
    Pe.getDiffieHellman = xa.getDiffieHellman;
    Pe.createDiffieHellman = xa.createDiffieHellman;
    Pe.DiffieHellman = xa.DiffieHellman;
    var Ns = Hy();
    Pe.createSign = Ns.createSign;
    Pe.Sign = Ns.Sign;
    Pe.createVerify = Ns.createVerify;
    Pe.Verify = Ns.Verify;
    Pe.createECDH = Vy();
    var Ds = f6();
    Pe.publicEncrypt = Ds.publicEncrypt;
    Pe.privateEncrypt = Ds.privateEncrypt;
    Pe.publicDecrypt = Ds.publicDecrypt;
    Pe.privateDecrypt = Ds.privateDecrypt;
    var y6 = v6();
    Pe.randomFill = y6.randomFill;
    Pe.randomFillSync = y6.randomFillSync;
    Pe.createCredentials = function () {
      throw new Error(
        [
          'sorry, createCredentials is not implemented yet',
          'we accept pull requests',
          'https://github.com/crypto-browserify/crypto-browserify',
        ].join(`
`)
      );
    };
    Pe.constants = {
      DH_CHECK_P_NOT_SAFE_PRIME: 2,
      DH_CHECK_P_NOT_PRIME: 1,
      DH_UNABLE_TO_CHECK_GENERATOR: 4,
      DH_NOT_SUITABLE_GENERATOR: 8,
      NPN_ENABLED: 1,
      ALPN_ENABLED: 1,
      RSA_PKCS1_PADDING: 1,
      RSA_SSLV23_PADDING: 2,
      RSA_NO_PADDING: 3,
      RSA_PKCS1_OAEP_PADDING: 4,
      RSA_X931_PADDING: 5,
      RSA_PKCS1_PSS_PADDING: 6,
      POINT_CONVERSION_COMPRESSED: 2,
      POINT_CONVERSION_UNCOMPRESSED: 4,
      POINT_CONVERSION_HYBRID: 6,
    };
  });
  var g6 = R((pO, m6) => {
    'use strict';
    S();
    function dc(t) {
      var e = ((t / 8) | 0) + (t % 8 === 0 ? 0 : 1);
      return e;
    }
    var mS = { ES256: dc(256), ES384: dc(384), ES512: dc(521) };
    function gS(t) {
      var e = mS[t];
      if (e) return e;
      throw new Error('Unknown algorithm "' + t + '"');
    }
    m6.exports = gS;
  });
  var A6 = R((bO, E6) => {
    'use strict';
    S();
    var Ls = Ie().Buffer,
      _6 = g6(),
      Fs = 128,
      x6 = 0,
      wS = 32,
      _S = 16,
      xS = 2,
      M6 = _S | wS | (x6 << 6),
      js = xS | (x6 << 6);
    function MS(t) {
      return t.replace(/=/g, '').replace(/\+/g, '-').replace(/\//g, '_');
    }
    function S6(t) {
      if (Ls.isBuffer(t)) return t;
      if (typeof t == 'string') return Ls.from(t, 'base64');
      throw new TypeError(
        'ECDSA signature must be a Base64 string or a Buffer'
      );
    }
    function SS(t, e) {
      t = S6(t);
      var r = _6(e),
        n = r + 1,
        i = t.length,
        a = 0;
      if (t[a++] !== M6) throw new Error('Could not find expected "seq"');
      var h = t[a++];
      if ((h === (Fs | 1) && (h = t[a++]), i - a < h))
        throw new Error(
          '"seq" specified length of "' +
            h +
            '", only "' +
            (i - a) +
            '" remaining'
        );
      if (t[a++] !== js)
        throw new Error('Could not find expected "int" for "r"');
      var v = t[a++];
      if (i - a - 2 < v)
        throw new Error(
          '"r" specified length of "' +
            v +
            '", only "' +
            (i - a - 2) +
            '" available'
        );
      if (n < v)
        throw new Error(
          '"r" specified length of "' +
            v +
            '", max of "' +
            n +
            '" is acceptable'
        );
      var g = a;
      if (((a += v), t[a++] !== js))
        throw new Error('Could not find expected "int" for "s"');
      var M = t[a++];
      if (i - a !== M)
        throw new Error(
          '"s" specified length of "' + M + '", expected "' + (i - a) + '"'
        );
      if (n < M)
        throw new Error(
          '"s" specified length of "' +
            M +
            '", max of "' +
            n +
            '" is acceptable'
        );
      var x = a;
      if (((a += M), a !== i))
        throw new Error(
          'Expected to consume entire buffer, but "' +
            (i - a) +
            '" bytes remain'
        );
      var E = r - v,
        I = r - M,
        q = Ls.allocUnsafe(E + v + I + M);
      for (a = 0; a < E; ++a) q[a] = 0;
      t.copy(q, a, g + Math.max(-E, 0), g + v), (a = r);
      for (var k = a; a < k + I; ++a) q[a] = 0;
      return (
        t.copy(q, a, x + Math.max(-I, 0), x + M),
        (q = q.toString('base64')),
        (q = MS(q)),
        q
      );
    }
    function w6(t, e, r) {
      for (var n = 0; e + n < r && t[e + n] === 0; ) ++n;
      var i = t[e + n] >= Fs;
      return i && --n, n;
    }
    function ES(t, e) {
      t = S6(t);
      var r = _6(e),
        n = t.length;
      if (n !== r * 2)
        throw new TypeError(
          '"' +
            e +
            '" signatures must be "' +
            r * 2 +
            '" bytes, saw "' +
            n +
            '"'
        );
      var i = w6(t, 0, r),
        a = w6(t, r, t.length),
        h = r - i,
        v = r - a,
        g = 1 + 1 + h + 1 + 1 + v,
        M = g < Fs,
        x = Ls.allocUnsafe((M ? 2 : 3) + g),
        E = 0;
      return (
        (x[E++] = M6),
        M ? (x[E++] = g) : ((x[E++] = Fs | 1), (x[E++] = g & 255)),
        (x[E++] = js),
        (x[E++] = h),
        i < 0
          ? ((x[E++] = 0), (E += t.copy(x, E, 0, r)))
          : (E += t.copy(x, E, i, r)),
        (x[E++] = js),
        (x[E++] = v),
        a < 0 ? ((x[E++] = 0), t.copy(x, E, r)) : t.copy(x, E, r + a),
        x
      );
    }
    E6.exports = { derToJose: SS, joseToDer: ES };
  });
  var vc = R((mO, C6) => {
    S();
    var AS = j1(),
      yf = Ie().Buffer,
      Dr = hu(),
      I6 = A6(),
      B6 = Gi(),
      BS = `"%s" is not a valid algorithm.
  Supported algorithms are:
  "HS256", "HS384", "HS512", "RS256", "RS384", "RS512", "PS256", "PS384", "PS512", "ES256", "ES384", "ES512" and "none".`,
      Ma = 'secret must be a string or buffer',
      bf = 'key must be a string or a buffer',
      IS = 'key must be a string, a buffer or an object',
      lc = typeof Dr.createPublicKey == 'function';
    lc && ((bf += ' or a KeyObject'), (Ma += 'or a KeyObject'));
    function R6(t) {
      if (
        !yf.isBuffer(t) &&
        typeof t != 'string' &&
        (!lc ||
          typeof t != 'object' ||
          typeof t.type != 'string' ||
          typeof t.asymmetricKeyType != 'string' ||
          typeof t.export != 'function')
      )
        throw Mr(bf);
    }
    function q6(t) {
      if (!yf.isBuffer(t) && typeof t != 'string' && typeof t != 'object')
        throw Mr(IS);
    }
    function RS(t) {
      if (!yf.isBuffer(t)) {
        if (typeof t == 'string') return t;
        if (
          !lc ||
          typeof t != 'object' ||
          t.type !== 'secret' ||
          typeof t.export != 'function'
        )
          throw Mr(Ma);
      }
    }
    function pc(t) {
      return t.replace(/=/g, '').replace(/\+/g, '-').replace(/\//g, '_');
    }
    function T6(t) {
      t = t.toString();
      var e = 4 - (t.length % 4);
      if (e !== 4) for (var r = 0; r < e; ++r) t += '=';
      return t.replace(/\-/g, '+').replace(/_/g, '/');
    }
    function Mr(t) {
      var e = [].slice.call(arguments, 1),
        r = B6.format.bind(B6, t).apply(null, e);
      return new TypeError(r);
    }
    function qS(t) {
      return yf.isBuffer(t) || typeof t == 'string';
    }
    function Sa(t) {
      return qS(t) || (t = JSON.stringify(t)), t;
    }
    function P6(t) {
      return function (r, n) {
        RS(n), (r = Sa(r));
        var i = Dr.createHmac('sha' + t, n),
          a = (i.update(r), i.digest('base64'));
        return pc(a);
      };
    }
    function TS(t) {
      return function (r, n, i) {
        var a = P6(t)(r, i);
        return AS(yf.from(n), yf.from(a));
      };
    }
    function k6(t) {
      return function (r, n) {
        q6(n), (r = Sa(r));
        var i = Dr.createSign('RSA-SHA' + t),
          a = (i.update(r), i.sign(n, 'base64'));
        return pc(a);
      };
    }
    function O6(t) {
      return function (r, n, i) {
        R6(i), (r = Sa(r)), (n = T6(n));
        var a = Dr.createVerify('RSA-SHA' + t);
        return a.update(r), a.verify(i, n, 'base64');
      };
    }
    function PS(t) {
      return function (r, n) {
        q6(n), (r = Sa(r));
        var i = Dr.createSign('RSA-SHA' + t),
          a =
            (i.update(r),
            i.sign(
              {
                key: n,
                padding: Dr.constants.RSA_PKCS1_PSS_PADDING,
                saltLength: Dr.constants.RSA_PSS_SALTLEN_DIGEST,
              },
              'base64'
            ));
        return pc(a);
      };
    }
    function kS(t) {
      return function (r, n, i) {
        R6(i), (r = Sa(r)), (n = T6(n));
        var a = Dr.createVerify('RSA-SHA' + t);
        return (
          a.update(r),
          a.verify(
            {
              key: i,
              padding: Dr.constants.RSA_PKCS1_PSS_PADDING,
              saltLength: Dr.constants.RSA_PSS_SALTLEN_DIGEST,
            },
            n,
            'base64'
          )
        );
      };
    }
    function OS(t) {
      var e = k6(t);
      return function () {
        var n = e.apply(null, arguments);
        return (n = I6.derToJose(n, 'ES' + t)), n;
      };
    }
    function CS(t) {
      var e = O6(t);
      return function (n, i, a) {
        i = I6.joseToDer(i, 'ES' + t).toString('base64');
        var h = e(n, i, a);
        return h;
      };
    }
    function NS() {
      return function () {
        return '';
      };
    }
    function DS() {
      return function (e, r) {
        return r === '';
      };
    }
    C6.exports = function (e) {
      var r = { hs: P6, rs: k6, ps: PS, es: OS, none: NS },
        n = { hs: TS, rs: O6, ps: kS, es: CS, none: DS },
        i = e.match(/^(RS|PS|ES|HS)(256|384|512)$|^(none)$/i);
      if (!i) throw Mr(BS, e);
      var a = (i[1] || i[3]).toLowerCase(),
        h = i[2];
      return { sign: r[a](h), verify: n[a](h) };
    };
  });
  var bc = R((wO, N6) => {
    S();
    var LS = Et().Buffer;
    N6.exports = function (e) {
      return typeof e == 'string'
        ? e
        : typeof e == 'number' || LS.isBuffer(e)
          ? e.toString()
          : JSON.stringify(e);
    };
  });
  var z6 = R((xO, U6) => {
    S();
    var FS = Ie().Buffer,
      D6 = mh(),
      jS = vc(),
      US = Hf(),
      L6 = bc(),
      yc = Gi();
    function F6(t, e) {
      return FS.from(t, e)
        .toString('base64')
        .replace(/=/g, '')
        .replace(/\+/g, '-')
        .replace(/\//g, '_');
    }
    function zS(t, e, r) {
      r = r || 'utf8';
      var n = F6(L6(t), 'binary'),
        i = F6(L6(e), r);
      return yc.format('%s.%s', n, i);
    }
    function j6(t) {
      var e = t.header,
        r = t.payload,
        n = t.secret || t.privateKey,
        i = t.encoding,
        a = jS(e.alg),
        h = zS(e, r, i),
        v = a.sign(h, n);
      return yc.format('%s.%s', h, v);
    }
    function Us(t) {
      var e = t.secret || t.privateKey || t.key,
        r = new D6(e);
      (this.readable = !0),
        (this.header = t.header),
        (this.encoding = t.encoding),
        (this.secret = this.privateKey = this.key = r),
        (this.payload = new D6(t.payload)),
        this.secret.once(
          'close',
          function () {
            !this.payload.writable && this.readable && this.sign();
          }.bind(this)
        ),
        this.payload.once(
          'close',
          function () {
            !this.secret.writable && this.readable && this.sign();
          }.bind(this)
        );
    }
    yc.inherits(Us, US);
    Us.prototype.sign = function () {
      try {
        var e = j6({
          header: this.header,
          payload: this.payload.buffer,
          secret: this.secret.buffer,
          encoding: this.encoding,
        });
        return (
          this.emit('done', e),
          this.emit('data', e),
          this.emit('end'),
          (this.readable = !1),
          e
        );
      } catch (r) {
        (this.readable = !1), this.emit('error', r), this.emit('close');
      }
    };
    Us.sign = j6;
    U6.exports = Us;
  });
  var Y6 = R((SO, X6) => {
    S();
    var K6 = Ie().Buffer,
      H6 = mh(),
      HS = vc(),
      KS = Hf(),
      V6 = bc(),
      VS = Gi(),
      GS = /^[a-zA-Z0-9\-_]+?\.[a-zA-Z0-9\-_]+?\.([a-zA-Z0-9\-_]+)?$/;
    function WS(t) {
      return Object.prototype.toString.call(t) === '[object Object]';
    }
    function $S(t) {
      if (WS(t)) return t;
      try {
        return JSON.parse(t);
      } catch {
        return;
      }
    }
    function G6(t) {
      var e = t.split('.', 1)[0];
      return $S(K6.from(e, 'base64').toString('binary'));
    }
    function ZS(t) {
      return t.split('.', 2).join('.');
    }
    function W6(t) {
      return t.split('.')[2];
    }
    function JS(t, e) {
      e = e || 'utf8';
      var r = t.split('.')[1];
      return K6.from(r, 'base64').toString(e);
    }
    function $6(t) {
      return GS.test(t) && !!G6(t);
    }
    function Z6(t, e, r) {
      if (!e) {
        var n = new Error('Missing algorithm parameter for jws.verify');
        throw ((n.code = 'MISSING_ALGORITHM'), n);
      }
      t = V6(t);
      var i = W6(t),
        a = ZS(t),
        h = HS(e);
      return h.verify(a, i, r);
    }
    function J6(t, e) {
      if (((e = e || {}), (t = V6(t)), !$6(t))) return null;
      var r = G6(t);
      if (!r) return null;
      var n = JS(t);
      return (
        (r.typ === 'JWT' || e.json) && (n = JSON.parse(n, e.encoding)),
        { header: r, payload: n, signature: W6(t) }
      );
    }
    function mf(t) {
      t = t || {};
      var e = t.secret || t.publicKey || t.key,
        r = new H6(e);
      (this.readable = !0),
        (this.algorithm = t.algorithm),
        (this.encoding = t.encoding),
        (this.secret = this.publicKey = this.key = r),
        (this.signature = new H6(t.signature)),
        this.secret.once(
          'close',
          function () {
            !this.signature.writable && this.readable && this.verify();
          }.bind(this)
        ),
        this.signature.once(
          'close',
          function () {
            !this.secret.writable && this.readable && this.verify();
          }.bind(this)
        );
    }
    VS.inherits(mf, KS);
    mf.prototype.verify = function () {
      try {
        var e = Z6(this.signature.buffer, this.algorithm, this.key.buffer),
          r = J6(this.signature.buffer, this.encoding);
        return (
          this.emit('done', e, r),
          this.emit('data', e),
          this.emit('end'),
          (this.readable = !1),
          e
        );
      } catch (n) {
        (this.readable = !1), this.emit('error', n), this.emit('close');
      }
    };
    mf.decode = J6;
    mf.isValid = $6;
    mf.verify = Z6;
    X6.exports = mf;
  });
  var Hs = R((ki) => {
    S();
    var Q6 = z6(),
      zs = Y6(),
      XS = [
        'HS256',
        'HS384',
        'HS512',
        'RS256',
        'RS384',
        'RS512',
        'PS256',
        'PS384',
        'PS512',
        'ES256',
        'ES384',
        'ES512',
      ];
    ki.ALGORITHMS = XS;
    ki.sign = Q6.sign;
    ki.verify = zs.verify;
    ki.decode = zs.decode;
    ki.isValid = zs.isValid;
    ki.createSign = function (e) {
      return new Q6(e);
    };
    ki.createVerify = function (e) {
      return new zs(e);
    };
  });
  var mc = R((IO, e3) => {
    S();
    var YS = Hs();
    e3.exports = function (t, e) {
      e = e || {};
      var r = YS.decode(t, e);
      if (!r) return null;
      var n = r.payload;
      if (typeof n == 'string')
        try {
          var i = JSON.parse(n);
          i !== null && typeof i == 'object' && (n = i);
        } catch {}
      return e.complete === !0
        ? { header: r.header, payload: n, signature: r.signature }
        : n;
    };
  });
  var r3 = R((qO, t3) => {
    S();
    var gf = 1e3,
      wf = gf * 60,
      _f = wf * 60,
      _n = _f * 24,
      QS = _n * 7,
      eE = _n * 365.25;
    t3.exports = function (t, e) {
      e = e || {};
      var r = typeof t;
      if (r === 'string' && t.length > 0) return tE(t);
      if (r === 'number' && isFinite(t)) return e.long ? iE(t) : rE(t);
      throw new Error(
        'val is not a non-empty string or a valid number. val=' +
          JSON.stringify(t)
      );
    };
    function tE(t) {
      if (((t = String(t)), !(t.length > 100))) {
        var e =
          /^(-?(?:\d+)?\.?\d+) *(milliseconds?|msecs?|ms|seconds?|secs?|s|minutes?|mins?|m|hours?|hrs?|h|days?|d|weeks?|w|years?|yrs?|y)?$/i.exec(
            t
          );
        if (!!e) {
          var r = parseFloat(e[1]),
            n = (e[2] || 'ms').toLowerCase();
          switch (n) {
            case 'years':
            case 'year':
            case 'yrs':
            case 'yr':
            case 'y':
              return r * eE;
            case 'weeks':
            case 'week':
            case 'w':
              return r * QS;
            case 'days':
            case 'day':
            case 'd':
              return r * _n;
            case 'hours':
            case 'hour':
            case 'hrs':
            case 'hr':
            case 'h':
              return r * _f;
            case 'minutes':
            case 'minute':
            case 'mins':
            case 'min':
            case 'm':
              return r * wf;
            case 'seconds':
            case 'second':
            case 'secs':
            case 'sec':
            case 's':
              return r * gf;
            case 'milliseconds':
            case 'millisecond':
            case 'msecs':
            case 'msec':
            case 'ms':
              return r;
            default:
              return;
          }
        }
      }
    }
    function rE(t) {
      var e = Math.abs(t);
      return e >= _n
        ? Math.round(t / _n) + 'd'
        : e >= _f
          ? Math.round(t / _f) + 'h'
          : e >= wf
            ? Math.round(t / wf) + 'm'
            : e >= gf
              ? Math.round(t / gf) + 's'
              : t + 'ms';
    }
    function iE(t) {
      var e = Math.abs(t);
      return e >= _n
        ? Ks(t, e, _n, 'day')
        : e >= _f
          ? Ks(t, e, _f, 'hour')
          : e >= wf
            ? Ks(t, e, wf, 'minute')
            : e >= gf
              ? Ks(t, e, gf, 'second')
              : t + ' ms';
    }
    function Ks(t, e, r, n) {
      var i = e >= r * 1.5;
      return Math.round(t / r) + ' ' + n + (i ? 's' : '');
    }
  });
  var gc = R((PO, i3) => {
    S();
    var nE = r3();
    i3.exports = function (t, e) {
      var r = e || Math.floor(Date.now() / 1e3);
      if (typeof t == 'string') {
        var n = nE(t);
        return typeof n > 'u' ? void 0 : Math.floor(r + n / 1e3);
      } else return typeof t == 'number' ? r + t : void 0;
    };
  });
  var w3 = R((Re, g3) => {
    S();
    Re = g3.exports = Le;
    var ze;
    typeof A == 'object' &&
    A.env &&
    A.env.NODE_DEBUG &&
    /\bsemver\b/i.test(A.env.NODE_DEBUG)
      ? (ze = function () {
          var t = Array.prototype.slice.call(arguments, 0);
          t.unshift('SEMVER'), console.log.apply(console, t);
        })
      : (ze = function () {});
    Re.SEMVER_SPEC_VERSION = '2.0.0';
    var _c = 256,
      Vs = Number.MAX_SAFE_INTEGER || 9007199254740991,
      wc = 16,
      Ge = (Re.re = []),
      D = (Re.src = []),
      De = 0,
      xf = De++;
    D[xf] = '0|[1-9]\\d*';
    var Mf = De++;
    D[Mf] = '[0-9]+';
    var Ec = De++;
    D[Ec] = '\\d*[a-zA-Z-][a-zA-Z0-9-]*';
    var f3 = De++;
    D[f3] = '(' + D[xf] + ')\\.(' + D[xf] + ')\\.(' + D[xf] + ')';
    var a3 = De++;
    D[a3] = '(' + D[Mf] + ')\\.(' + D[Mf] + ')\\.(' + D[Mf] + ')';
    var xc = De++;
    D[xc] = '(?:' + D[xf] + '|' + D[Ec] + ')';
    var Mc = De++;
    D[Mc] = '(?:' + D[Mf] + '|' + D[Ec] + ')';
    var Ac = De++;
    D[Ac] = '(?:-(' + D[xc] + '(?:\\.' + D[xc] + ')*))';
    var Bc = De++;
    D[Bc] = '(?:-?(' + D[Mc] + '(?:\\.' + D[Mc] + ')*))';
    var Sc = De++;
    D[Sc] = '[0-9A-Za-z-]+';
    var Ba = De++;
    D[Ba] = '(?:\\+(' + D[Sc] + '(?:\\.' + D[Sc] + ')*))';
    var Ic = De++,
      o3 = 'v?' + D[f3] + D[Ac] + '?' + D[Ba] + '?';
    D[Ic] = '^' + o3 + '$';
    var Rc = '[v=\\s]*' + D[a3] + D[Bc] + '?' + D[Ba] + '?',
      qc = De++;
    D[qc] = '^' + Rc + '$';
    var Af = De++;
    D[Af] = '((?:<|>)?=?)';
    var Gs = De++;
    D[Gs] = D[Mf] + '|x|X|\\*';
    var Ws = De++;
    D[Ws] = D[xf] + '|x|X|\\*';
    var xn = De++;
    D[xn] =
      '[v=\\s]*(' +
      D[Ws] +
      ')(?:\\.(' +
      D[Ws] +
      ')(?:\\.(' +
      D[Ws] +
      ')(?:' +
      D[Ac] +
      ')?' +
      D[Ba] +
      '?)?)?';
    var Ef = De++;
    D[Ef] =
      '[v=\\s]*(' +
      D[Gs] +
      ')(?:\\.(' +
      D[Gs] +
      ')(?:\\.(' +
      D[Gs] +
      ')(?:' +
      D[Bc] +
      ')?' +
      D[Ba] +
      '?)?)?';
    var s3 = De++;
    D[s3] = '^' + D[Af] + '\\s*' + D[xn] + '$';
    var h3 = De++;
    D[h3] = '^' + D[Af] + '\\s*' + D[Ef] + '$';
    var u3 = De++;
    D[u3] =
      '(?:^|[^\\d])(\\d{1,' +
      wc +
      '})(?:\\.(\\d{1,' +
      wc +
      '}))?(?:\\.(\\d{1,' +
      wc +
      '}))?(?:$|[^\\d])';
    var Qs = De++;
    D[Qs] = '(?:~>?)';
    var $s = De++;
    D[$s] = '(\\s*)' + D[Qs] + '\\s+';
    Ge[$s] = new RegExp(D[$s], 'g');
    var fE = '$1~',
      c3 = De++;
    D[c3] = '^' + D[Qs] + D[xn] + '$';
    var d3 = De++;
    D[d3] = '^' + D[Qs] + D[Ef] + '$';
    var e0 = De++;
    D[e0] = '(?:\\^)';
    var Zs = De++;
    D[Zs] = '(\\s*)' + D[e0] + '\\s+';
    Ge[Zs] = new RegExp(D[Zs], 'g');
    var aE = '$1^',
      l3 = De++;
    D[l3] = '^' + D[e0] + D[xn] + '$';
    var p3 = De++;
    D[p3] = '^' + D[e0] + D[Ef] + '$';
    var Tc = De++;
    D[Tc] = '^' + D[Af] + '\\s*(' + Rc + ')$|^$';
    var Pc = De++;
    D[Pc] = '^' + D[Af] + '\\s*(' + o3 + ')$|^$';
    var Ea = De++;
    D[Ea] = '(\\s*)' + D[Af] + '\\s*(' + Rc + '|' + D[xn] + ')';
    Ge[Ea] = new RegExp(D[Ea], 'g');
    var oE = '$1$2$3',
      v3 = De++;
    D[v3] = '^\\s*(' + D[xn] + ')\\s+-\\s+(' + D[xn] + ')\\s*$';
    var b3 = De++;
    D[b3] = '^\\s*(' + D[Ef] + ')\\s+-\\s+(' + D[Ef] + ')\\s*$';
    var y3 = De++;
    D[y3] = '(<|>)?=?\\s*\\*';
    for (Oi = 0; Oi < De; Oi++)
      ze(Oi, D[Oi]), Ge[Oi] || (Ge[Oi] = new RegExp(D[Oi]));
    var Oi;
    Re.parse = Mn;
    function Mn(t, e) {
      if (
        ((!e || typeof e != 'object') &&
          (e = { loose: !!e, includePrerelease: !1 }),
        t instanceof Le)
      )
        return t;
      if (typeof t != 'string' || t.length > _c) return null;
      var r = e.loose ? Ge[qc] : Ge[Ic];
      if (!r.test(t)) return null;
      try {
        return new Le(t, e);
      } catch {
        return null;
      }
    }
    Re.valid = sE;
    function sE(t, e) {
      var r = Mn(t, e);
      return r ? r.version : null;
    }
    Re.clean = hE;
    function hE(t, e) {
      var r = Mn(t.trim().replace(/^[=v]+/, ''), e);
      return r ? r.version : null;
    }
    Re.SemVer = Le;
    function Le(t, e) {
      if (
        ((!e || typeof e != 'object') &&
          (e = { loose: !!e, includePrerelease: !1 }),
        t instanceof Le)
      ) {
        if (t.loose === e.loose) return t;
        t = t.version;
      } else if (typeof t != 'string')
        throw new TypeError('Invalid Version: ' + t);
      if (t.length > _c)
        throw new TypeError('version is longer than ' + _c + ' characters');
      if (!(this instanceof Le)) return new Le(t, e);
      ze('SemVer', t, e), (this.options = e), (this.loose = !!e.loose);
      var r = t.trim().match(e.loose ? Ge[qc] : Ge[Ic]);
      if (!r) throw new TypeError('Invalid Version: ' + t);
      if (
        ((this.raw = t),
        (this.major = +r[1]),
        (this.minor = +r[2]),
        (this.patch = +r[3]),
        this.major > Vs || this.major < 0)
      )
        throw new TypeError('Invalid major version');
      if (this.minor > Vs || this.minor < 0)
        throw new TypeError('Invalid minor version');
      if (this.patch > Vs || this.patch < 0)
        throw new TypeError('Invalid patch version');
      r[4]
        ? (this.prerelease = r[4].split('.').map(function (n) {
            if (/^[0-9]+$/.test(n)) {
              var i = +n;
              if (i >= 0 && i < Vs) return i;
            }
            return n;
          }))
        : (this.prerelease = []),
        (this.build = r[5] ? r[5].split('.') : []),
        this.format();
    }
    Le.prototype.format = function () {
      return (
        (this.version = this.major + '.' + this.minor + '.' + this.patch),
        this.prerelease.length &&
          (this.version += '-' + this.prerelease.join('.')),
        this.version
      );
    };
    Le.prototype.toString = function () {
      return this.version;
    };
    Le.prototype.compare = function (t) {
      return (
        ze('SemVer.compare', this.version, this.options, t),
        t instanceof Le || (t = new Le(t, this.options)),
        this.compareMain(t) || this.comparePre(t)
      );
    };
    Le.prototype.compareMain = function (t) {
      return (
        t instanceof Le || (t = new Le(t, this.options)),
        Sf(this.major, t.major) ||
          Sf(this.minor, t.minor) ||
          Sf(this.patch, t.patch)
      );
    };
    Le.prototype.comparePre = function (t) {
      if (
        (t instanceof Le || (t = new Le(t, this.options)),
        this.prerelease.length && !t.prerelease.length)
      )
        return -1;
      if (!this.prerelease.length && t.prerelease.length) return 1;
      if (!this.prerelease.length && !t.prerelease.length) return 0;
      var e = 0;
      do {
        var r = this.prerelease[e],
          n = t.prerelease[e];
        if ((ze('prerelease compare', e, r, n), r === void 0 && n === void 0))
          return 0;
        if (n === void 0) return 1;
        if (r === void 0) return -1;
        if (r === n) continue;
        return Sf(r, n);
      } while (++e);
    };
    Le.prototype.inc = function (t, e) {
      switch (t) {
        case 'premajor':
          (this.prerelease.length = 0),
            (this.patch = 0),
            (this.minor = 0),
            this.major++,
            this.inc('pre', e);
          break;
        case 'preminor':
          (this.prerelease.length = 0),
            (this.patch = 0),
            this.minor++,
            this.inc('pre', e);
          break;
        case 'prepatch':
          (this.prerelease.length = 0),
            this.inc('patch', e),
            this.inc('pre', e);
          break;
        case 'prerelease':
          this.prerelease.length === 0 && this.inc('patch', e),
            this.inc('pre', e);
          break;
        case 'major':
          (this.minor !== 0 ||
            this.patch !== 0 ||
            this.prerelease.length === 0) &&
            this.major++,
            (this.minor = 0),
            (this.patch = 0),
            (this.prerelease = []);
          break;
        case 'minor':
          (this.patch !== 0 || this.prerelease.length === 0) && this.minor++,
            (this.patch = 0),
            (this.prerelease = []);
          break;
        case 'patch':
          this.prerelease.length === 0 && this.patch++, (this.prerelease = []);
          break;
        case 'pre':
          if (this.prerelease.length === 0) this.prerelease = [0];
          else {
            for (var r = this.prerelease.length; --r >= 0; )
              typeof this.prerelease[r] == 'number' &&
                (this.prerelease[r]++, (r = -2));
            r === -1 && this.prerelease.push(0);
          }
          e &&
            (this.prerelease[0] === e
              ? isNaN(this.prerelease[1]) && (this.prerelease = [e, 0])
              : (this.prerelease = [e, 0]));
          break;
        default:
          throw new Error('invalid increment argument: ' + t);
      }
      return this.format(), (this.raw = this.version), this;
    };
    Re.inc = uE;
    function uE(t, e, r, n) {
      typeof r == 'string' && ((n = r), (r = void 0));
      try {
        return new Le(t, r).inc(e, n).version;
      } catch {
        return null;
      }
    }
    Re.diff = cE;
    function cE(t, e) {
      if (kc(t, e)) return null;
      var r = Mn(t),
        n = Mn(e),
        i = '';
      if (r.prerelease.length || n.prerelease.length) {
        i = 'pre';
        var a = 'prerelease';
      }
      for (var h in r)
        if ((h === 'major' || h === 'minor' || h === 'patch') && r[h] !== n[h])
          return i + h;
      return a;
    }
    Re.compareIdentifiers = Sf;
    var n3 = /^[0-9]+$/;
    function Sf(t, e) {
      var r = n3.test(t),
        n = n3.test(e);
      return (
        r && n && ((t = +t), (e = +e)),
        t === e ? 0 : r && !n ? -1 : n && !r ? 1 : t < e ? -1 : 1
      );
    }
    Re.rcompareIdentifiers = dE;
    function dE(t, e) {
      return Sf(e, t);
    }
    Re.major = lE;
    function lE(t, e) {
      return new Le(t, e).major;
    }
    Re.minor = pE;
    function pE(t, e) {
      return new Le(t, e).minor;
    }
    Re.patch = vE;
    function vE(t, e) {
      return new Le(t, e).patch;
    }
    Re.compare = Qr;
    function Qr(t, e, r) {
      return new Le(t, r).compare(new Le(e, r));
    }
    Re.compareLoose = bE;
    function bE(t, e) {
      return Qr(t, e, !0);
    }
    Re.rcompare = yE;
    function yE(t, e, r) {
      return Qr(e, t, r);
    }
    Re.sort = mE;
    function mE(t, e) {
      return t.sort(function (r, n) {
        return Re.compare(r, n, e);
      });
    }
    Re.rsort = gE;
    function gE(t, e) {
      return t.sort(function (r, n) {
        return Re.rcompare(r, n, e);
      });
    }
    Re.gt = Aa;
    function Aa(t, e, r) {
      return Qr(t, e, r) > 0;
    }
    Re.lt = Js;
    function Js(t, e, r) {
      return Qr(t, e, r) < 0;
    }
    Re.eq = kc;
    function kc(t, e, r) {
      return Qr(t, e, r) === 0;
    }
    Re.neq = m3;
    function m3(t, e, r) {
      return Qr(t, e, r) !== 0;
    }
    Re.gte = Oc;
    function Oc(t, e, r) {
      return Qr(t, e, r) >= 0;
    }
    Re.lte = Cc;
    function Cc(t, e, r) {
      return Qr(t, e, r) <= 0;
    }
    Re.cmp = Xs;
    function Xs(t, e, r, n) {
      switch (e) {
        case '===':
          return (
            typeof t == 'object' && (t = t.version),
            typeof r == 'object' && (r = r.version),
            t === r
          );
        case '!==':
          return (
            typeof t == 'object' && (t = t.version),
            typeof r == 'object' && (r = r.version),
            t !== r
          );
        case '':
        case '=':
        case '==':
          return kc(t, r, n);
        case '!=':
          return m3(t, r, n);
        case '>':
          return Aa(t, r, n);
        case '>=':
          return Oc(t, r, n);
        case '<':
          return Js(t, r, n);
        case '<=':
          return Cc(t, r, n);
        default:
          throw new TypeError('Invalid operator: ' + e);
      }
    }
    Re.Comparator = Wt;
    function Wt(t, e) {
      if (
        ((!e || typeof e != 'object') &&
          (e = { loose: !!e, includePrerelease: !1 }),
        t instanceof Wt)
      ) {
        if (t.loose === !!e.loose) return t;
        t = t.value;
      }
      if (!(this instanceof Wt)) return new Wt(t, e);
      ze('comparator', t, e),
        (this.options = e),
        (this.loose = !!e.loose),
        this.parse(t),
        this.semver === Ia
          ? (this.value = '')
          : (this.value = this.operator + this.semver.version),
        ze('comp', this);
    }
    var Ia = {};
    Wt.prototype.parse = function (t) {
      var e = this.options.loose ? Ge[Tc] : Ge[Pc],
        r = t.match(e);
      if (!r) throw new TypeError('Invalid comparator: ' + t);
      (this.operator = r[1]),
        this.operator === '=' && (this.operator = ''),
        r[2]
          ? (this.semver = new Le(r[2], this.options.loose))
          : (this.semver = Ia);
    };
    Wt.prototype.toString = function () {
      return this.value;
    };
    Wt.prototype.test = function (t) {
      return (
        ze('Comparator.test', t, this.options.loose),
        this.semver === Ia
          ? !0
          : (typeof t == 'string' && (t = new Le(t, this.options)),
            Xs(t, this.operator, this.semver, this.options))
      );
    };
    Wt.prototype.intersects = function (t, e) {
      if (!(t instanceof Wt)) throw new TypeError('a Comparator is required');
      (!e || typeof e != 'object') &&
        (e = { loose: !!e, includePrerelease: !1 });
      var r;
      if (this.operator === '')
        return (r = new Je(t.value, e)), Ys(this.value, r, e);
      if (t.operator === '')
        return (r = new Je(this.value, e)), Ys(t.semver, r, e);
      var n =
          (this.operator === '>=' || this.operator === '>') &&
          (t.operator === '>=' || t.operator === '>'),
        i =
          (this.operator === '<=' || this.operator === '<') &&
          (t.operator === '<=' || t.operator === '<'),
        a = this.semver.version === t.semver.version,
        h =
          (this.operator === '>=' || this.operator === '<=') &&
          (t.operator === '>=' || t.operator === '<='),
        v =
          Xs(this.semver, '<', t.semver, e) &&
          (this.operator === '>=' || this.operator === '>') &&
          (t.operator === '<=' || t.operator === '<'),
        g =
          Xs(this.semver, '>', t.semver, e) &&
          (this.operator === '<=' || this.operator === '<') &&
          (t.operator === '>=' || t.operator === '>');
      return n || i || (a && h) || v || g;
    };
    Re.Range = Je;
    function Je(t, e) {
      if (
        ((!e || typeof e != 'object') &&
          (e = { loose: !!e, includePrerelease: !1 }),
        t instanceof Je)
      )
        return t.loose === !!e.loose &&
          t.includePrerelease === !!e.includePrerelease
          ? t
          : new Je(t.raw, e);
      if (t instanceof Wt) return new Je(t.value, e);
      if (!(this instanceof Je)) return new Je(t, e);
      if (
        ((this.options = e),
        (this.loose = !!e.loose),
        (this.includePrerelease = !!e.includePrerelease),
        (this.raw = t),
        (this.set = t
          .split(/\s*\|\|\s*/)
          .map(function (r) {
            return this.parseRange(r.trim());
          }, this)
          .filter(function (r) {
            return r.length;
          })),
        !this.set.length)
      )
        throw new TypeError('Invalid SemVer Range: ' + t);
      this.format();
    }
    Je.prototype.format = function () {
      return (
        (this.range = this.set
          .map(function (t) {
            return t.join(' ').trim();
          })
          .join('||')
          .trim()),
        this.range
      );
    };
    Je.prototype.toString = function () {
      return this.range;
    };
    Je.prototype.parseRange = function (t) {
      var e = this.options.loose;
      t = t.trim();
      var r = e ? Ge[b3] : Ge[v3];
      (t = t.replace(r, RE)),
        ze('hyphen replace', t),
        (t = t.replace(Ge[Ea], oE)),
        ze('comparator trim', t, Ge[Ea]),
        (t = t.replace(Ge[$s], fE)),
        (t = t.replace(Ge[Zs], aE)),
        (t = t.split(/\s+/).join(' '));
      var n = e ? Ge[Tc] : Ge[Pc],
        i = t
          .split(' ')
          .map(function (a) {
            return _E(a, this.options);
          }, this)
          .join(' ')
          .split(/\s+/);
      return (
        this.options.loose &&
          (i = i.filter(function (a) {
            return !!a.match(n);
          })),
        (i = i.map(function (a) {
          return new Wt(a, this.options);
        }, this)),
        i
      );
    };
    Je.prototype.intersects = function (t, e) {
      if (!(t instanceof Je)) throw new TypeError('a Range is required');
      return this.set.some(function (r) {
        return r.every(function (n) {
          return t.set.some(function (i) {
            return i.every(function (a) {
              return n.intersects(a, e);
            });
          });
        });
      });
    };
    Re.toComparators = wE;
    function wE(t, e) {
      return new Je(t, e).set.map(function (r) {
        return r
          .map(function (n) {
            return n.value;
          })
          .join(' ')
          .trim()
          .split(' ');
      });
    }
    function _E(t, e) {
      return (
        ze('comp', t, e),
        (t = SE(t, e)),
        ze('caret', t),
        (t = xE(t, e)),
        ze('tildes', t),
        (t = AE(t, e)),
        ze('xrange', t),
        (t = IE(t, e)),
        ze('stars', t),
        t
      );
    }
    function gt(t) {
      return !t || t.toLowerCase() === 'x' || t === '*';
    }
    function xE(t, e) {
      return t
        .trim()
        .split(/\s+/)
        .map(function (r) {
          return ME(r, e);
        })
        .join(' ');
    }
    function ME(t, e) {
      var r = e.loose ? Ge[d3] : Ge[c3];
      return t.replace(r, function (n, i, a, h, v) {
        ze('tilde', t, n, i, a, h, v);
        var g;
        return (
          gt(i)
            ? (g = '')
            : gt(a)
              ? (g = '>=' + i + '.0.0 <' + (+i + 1) + '.0.0')
              : gt(h)
                ? (g = '>=' + i + '.' + a + '.0 <' + i + '.' + (+a + 1) + '.0')
                : v
                  ? (ze('replaceTilde pr', v),
                    (g =
                      '>=' +
                      i +
                      '.' +
                      a +
                      '.' +
                      h +
                      '-' +
                      v +
                      ' <' +
                      i +
                      '.' +
                      (+a + 1) +
                      '.0'))
                  : (g =
                      '>=' +
                      i +
                      '.' +
                      a +
                      '.' +
                      h +
                      ' <' +
                      i +
                      '.' +
                      (+a + 1) +
                      '.0'),
          ze('tilde return', g),
          g
        );
      });
    }
    function SE(t, e) {
      return t
        .trim()
        .split(/\s+/)
        .map(function (r) {
          return EE(r, e);
        })
        .join(' ');
    }
    function EE(t, e) {
      ze('caret', t, e);
      var r = e.loose ? Ge[p3] : Ge[l3];
      return t.replace(r, function (n, i, a, h, v) {
        ze('caret', t, n, i, a, h, v);
        var g;
        return (
          gt(i)
            ? (g = '')
            : gt(a)
              ? (g = '>=' + i + '.0.0 <' + (+i + 1) + '.0.0')
              : gt(h)
                ? i === '0'
                  ? (g =
                      '>=' + i + '.' + a + '.0 <' + i + '.' + (+a + 1) + '.0')
                  : (g = '>=' + i + '.' + a + '.0 <' + (+i + 1) + '.0.0')
                : v
                  ? (ze('replaceCaret pr', v),
                    i === '0'
                      ? a === '0'
                        ? (g =
                            '>=' +
                            i +
                            '.' +
                            a +
                            '.' +
                            h +
                            '-' +
                            v +
                            ' <' +
                            i +
                            '.' +
                            a +
                            '.' +
                            (+h + 1))
                        : (g =
                            '>=' +
                            i +
                            '.' +
                            a +
                            '.' +
                            h +
                            '-' +
                            v +
                            ' <' +
                            i +
                            '.' +
                            (+a + 1) +
                            '.0')
                      : (g =
                          '>=' +
                          i +
                          '.' +
                          a +
                          '.' +
                          h +
                          '-' +
                          v +
                          ' <' +
                          (+i + 1) +
                          '.0.0'))
                  : (ze('no pr'),
                    i === '0'
                      ? a === '0'
                        ? (g =
                            '>=' +
                            i +
                            '.' +
                            a +
                            '.' +
                            h +
                            ' <' +
                            i +
                            '.' +
                            a +
                            '.' +
                            (+h + 1))
                        : (g =
                            '>=' +
                            i +
                            '.' +
                            a +
                            '.' +
                            h +
                            ' <' +
                            i +
                            '.' +
                            (+a + 1) +
                            '.0')
                      : (g =
                          '>=' +
                          i +
                          '.' +
                          a +
                          '.' +
                          h +
                          ' <' +
                          (+i + 1) +
                          '.0.0')),
          ze('caret return', g),
          g
        );
      });
    }
    function AE(t, e) {
      return (
        ze('replaceXRanges', t, e),
        t
          .split(/\s+/)
          .map(function (r) {
            return BE(r, e);
          })
          .join(' ')
      );
    }
    function BE(t, e) {
      t = t.trim();
      var r = e.loose ? Ge[h3] : Ge[s3];
      return t.replace(r, function (n, i, a, h, v, g) {
        ze('xRange', t, n, i, a, h, v, g);
        var M = gt(a),
          x = M || gt(h),
          E = x || gt(v),
          I = E;
        return (
          i === '=' && I && (i = ''),
          M
            ? i === '>' || i === '<'
              ? (n = '<0.0.0')
              : (n = '*')
            : i && I
              ? (x && (h = 0),
                (v = 0),
                i === '>'
                  ? ((i = '>='),
                    x
                      ? ((a = +a + 1), (h = 0), (v = 0))
                      : ((h = +h + 1), (v = 0)))
                  : i === '<=' && ((i = '<'), x ? (a = +a + 1) : (h = +h + 1)),
                (n = i + a + '.' + h + '.' + v))
              : x
                ? (n = '>=' + a + '.0.0 <' + (+a + 1) + '.0.0')
                : E &&
                  (n = '>=' + a + '.' + h + '.0 <' + a + '.' + (+h + 1) + '.0'),
          ze('xRange return', n),
          n
        );
      });
    }
    function IE(t, e) {
      return ze('replaceStars', t, e), t.trim().replace(Ge[y3], '');
    }
    function RE(t, e, r, n, i, a, h, v, g, M, x, E, I) {
      return (
        gt(r)
          ? (e = '')
          : gt(n)
            ? (e = '>=' + r + '.0.0')
            : gt(i)
              ? (e = '>=' + r + '.' + n + '.0')
              : (e = '>=' + e),
        gt(g)
          ? (v = '')
          : gt(M)
            ? (v = '<' + (+g + 1) + '.0.0')
            : gt(x)
              ? (v = '<' + g + '.' + (+M + 1) + '.0')
              : E
                ? (v = '<=' + g + '.' + M + '.' + x + '-' + E)
                : (v = '<=' + v),
        (e + ' ' + v).trim()
      );
    }
    Je.prototype.test = function (t) {
      if (!t) return !1;
      typeof t == 'string' && (t = new Le(t, this.options));
      for (var e = 0; e < this.set.length; e++)
        if (qE(this.set[e], t, this.options)) return !0;
      return !1;
    };
    function qE(t, e, r) {
      for (var n = 0; n < t.length; n++) if (!t[n].test(e)) return !1;
      if (e.prerelease.length && !r.includePrerelease) {
        for (n = 0; n < t.length; n++)
          if (
            (ze(t[n].semver),
            t[n].semver !== Ia && t[n].semver.prerelease.length > 0)
          ) {
            var i = t[n].semver;
            if (
              i.major === e.major &&
              i.minor === e.minor &&
              i.patch === e.patch
            )
              return !0;
          }
        return !1;
      }
      return !0;
    }
    Re.satisfies = Ys;
    function Ys(t, e, r) {
      try {
        e = new Je(e, r);
      } catch {
        return !1;
      }
      return e.test(t);
    }
    Re.maxSatisfying = TE;
    function TE(t, e, r) {
      var n = null,
        i = null;
      try {
        var a = new Je(e, r);
      } catch {
        return null;
      }
      return (
        t.forEach(function (h) {
          a.test(h) &&
            (!n || i.compare(h) === -1) &&
            ((n = h), (i = new Le(n, r)));
        }),
        n
      );
    }
    Re.minSatisfying = PE;
    function PE(t, e, r) {
      var n = null,
        i = null;
      try {
        var a = new Je(e, r);
      } catch {
        return null;
      }
      return (
        t.forEach(function (h) {
          a.test(h) &&
            (!n || i.compare(h) === 1) &&
            ((n = h), (i = new Le(n, r)));
        }),
        n
      );
    }
    Re.minVersion = kE;
    function kE(t, e) {
      t = new Je(t, e);
      var r = new Le('0.0.0');
      if (t.test(r) || ((r = new Le('0.0.0-0')), t.test(r))) return r;
      r = null;
      for (var n = 0; n < t.set.length; ++n) {
        var i = t.set[n];
        i.forEach(function (a) {
          var h = new Le(a.semver.version);
          switch (a.operator) {
            case '>':
              h.prerelease.length === 0 ? h.patch++ : h.prerelease.push(0),
                (h.raw = h.format());
            case '':
            case '>=':
              (!r || Aa(r, h)) && (r = h);
              break;
            case '<':
            case '<=':
              break;
            default:
              throw new Error('Unexpected operation: ' + a.operator);
          }
        });
      }
      return r && t.test(r) ? r : null;
    }
    Re.validRange = OE;
    function OE(t, e) {
      try {
        return new Je(t, e).range || '*';
      } catch {
        return null;
      }
    }
    Re.ltr = CE;
    function CE(t, e, r) {
      return Nc(t, e, '<', r);
    }
    Re.gtr = NE;
    function NE(t, e, r) {
      return Nc(t, e, '>', r);
    }
    Re.outside = Nc;
    function Nc(t, e, r, n) {
      (t = new Le(t, n)), (e = new Je(e, n));
      var i, a, h, v, g;
      switch (r) {
        case '>':
          (i = Aa), (a = Cc), (h = Js), (v = '>'), (g = '>=');
          break;
        case '<':
          (i = Js), (a = Oc), (h = Aa), (v = '<'), (g = '<=');
          break;
        default:
          throw new TypeError('Must provide a hilo val of "<" or ">"');
      }
      if (Ys(t, e, n)) return !1;
      for (var M = 0; M < e.set.length; ++M) {
        var x = e.set[M],
          E = null,
          I = null;
        if (
          (x.forEach(function (q) {
            q.semver === Ia && (q = new Wt('>=0.0.0')),
              (E = E || q),
              (I = I || q),
              i(q.semver, E.semver, n)
                ? (E = q)
                : h(q.semver, I.semver, n) && (I = q);
          }),
          E.operator === v ||
            E.operator === g ||
            ((!I.operator || I.operator === v) && a(t, I.semver)))
        )
          return !1;
        if (I.operator === g && h(t, I.semver)) return !1;
      }
      return !0;
    }
    Re.prerelease = DE;
    function DE(t, e) {
      var r = Mn(t, e);
      return r && r.prerelease.length ? r.prerelease : null;
    }
    Re.intersects = LE;
    function LE(t, e, r) {
      return (t = new Je(t, r)), (e = new Je(e, r)), t.intersects(e);
    }
    Re.coerce = FE;
    function FE(t) {
      if (t instanceof Le) return t;
      if (typeof t != 'string') return null;
      var e = t.match(Ge[u3]);
      return e == null
        ? null
        : Mn(e[1] + '.' + (e[2] || '0') + '.' + (e[3] || '0'));
    }
  });
  var Dc = R((CO, _3) => {
    S();
    var jE = w3();
    _3.exports = jE.satisfies(A.version, '^6.12.0 || >=8.0.0');
  });
  var A3 = R((DO, E3) => {
    S();
    var Qe = Bf(),
      UE = i0(),
      x3 = n0(),
      zE = mc(),
      HE = gc(),
      KE = Dc(),
      VE = Hs(),
      M3 = ['RS256', 'RS384', 'RS512', 'ES256', 'ES384', 'ES512'],
      S3 = ['RS256', 'RS384', 'RS512'],
      GE = ['HS256', 'HS384', 'HS512'];
    KE &&
      (M3.splice(3, 0, 'PS256', 'PS384', 'PS512'),
      S3.splice(3, 0, 'PS256', 'PS384', 'PS512'));
    E3.exports = function (t, e, r, n) {
      typeof r == 'function' && !n && ((n = r), (r = {})),
        r || (r = {}),
        (r = Object.assign({}, r));
      var i;
      if (
        (n
          ? (i = n)
          : (i = function (x, E) {
              if (x) throw x;
              return E;
            }),
        r.clockTimestamp && typeof r.clockTimestamp != 'number')
      )
        return i(new Qe('clockTimestamp must be a number'));
      if (
        r.nonce !== void 0 &&
        (typeof r.nonce != 'string' || r.nonce.trim() === '')
      )
        return i(new Qe('nonce must be a non-empty string'));
      var a = r.clockTimestamp || Math.floor(Date.now() / 1e3);
      if (!t) return i(new Qe('jwt must be provided'));
      if (typeof t != 'string') return i(new Qe('jwt must be a string'));
      var h = t.split('.');
      if (h.length !== 3) return i(new Qe('jwt malformed'));
      var v;
      try {
        v = zE(t, { complete: !0 });
      } catch (x) {
        return i(x);
      }
      if (!v) return i(new Qe('invalid token'));
      var g = v.header,
        M;
      if (typeof e == 'function') {
        if (!n)
          return i(
            new Qe(
              'verify must be called asynchronous if secret or public key is provided as a callback'
            )
          );
        M = e;
      } else
        M = function (x, E) {
          return E(null, e);
        };
      return M(g, function (x, E) {
        if (x)
          return i(
            new Qe('error in secret or public key callback: ' + x.message)
          );
        var I = h[2].trim() !== '';
        if (!I && E) return i(new Qe('jwt signature is required'));
        if (I && !E) return i(new Qe('secret or public key must be provided'));
        if (
          (!I && !r.algorithms && (r.algorithms = ['none']),
          r.algorithms ||
            (r.algorithms =
              E.toString().includes('BEGIN CERTIFICATE') ||
              E.toString().includes('BEGIN PUBLIC KEY')
                ? M3
                : E.toString().includes('BEGIN RSA PUBLIC KEY')
                  ? S3
                  : GE),
          !~r.algorithms.indexOf(v.header.alg))
        )
          return i(new Qe('invalid algorithm'));
        var q;
        try {
          q = VE.verify(t, v.header.alg, E);
        } catch (Fe) {
          return i(Fe);
        }
        if (!q) return i(new Qe('invalid signature'));
        var k = v.payload;
        if (typeof k.nbf < 'u' && !r.ignoreNotBefore) {
          if (typeof k.nbf != 'number') return i(new Qe('invalid nbf value'));
          if (k.nbf > a + (r.clockTolerance || 0))
            return i(new UE('jwt not active', new Date(k.nbf * 1e3)));
        }
        if (typeof k.exp < 'u' && !r.ignoreExpiration) {
          if (typeof k.exp != 'number') return i(new Qe('invalid exp value'));
          if (a >= k.exp + (r.clockTolerance || 0))
            return i(new x3('jwt expired', new Date(k.exp * 1e3)));
        }
        if (r.audience) {
          var L = Array.isArray(r.audience) ? r.audience : [r.audience],
            xe = Array.isArray(k.aud) ? k.aud : [k.aud],
            U = xe.some(function (Fe) {
              return L.some(function (Se) {
                return Se instanceof RegExp ? Se.test(Fe) : Se === Fe;
              });
            });
          if (!U)
            return i(
              new Qe('jwt audience invalid. expected: ' + L.join(' or '))
            );
        }
        if (r.issuer) {
          var Me =
            (typeof r.issuer == 'string' && k.iss !== r.issuer) ||
            (Array.isArray(r.issuer) && r.issuer.indexOf(k.iss) === -1);
          if (Me) return i(new Qe('jwt issuer invalid. expected: ' + r.issuer));
        }
        if (r.subject && k.sub !== r.subject)
          return i(new Qe('jwt subject invalid. expected: ' + r.subject));
        if (r.jwtid && k.jti !== r.jwtid)
          return i(new Qe('jwt jwtid invalid. expected: ' + r.jwtid));
        if (r.nonce && k.nonce !== r.nonce)
          return i(new Qe('jwt nonce invalid. expected: ' + r.nonce));
        if (r.maxAge) {
          if (typeof k.iat != 'number')
            return i(new Qe('iat required when maxAge is specified'));
          var Te = HE(r.maxAge, k.iat);
          if (typeof Te > 'u')
            return i(
              new Qe(
                '"maxAge" should be a number of seconds or string representing a timespan eg: "1d", "20h", 60'
              )
            );
          if (a >= Te + (r.clockTolerance || 0))
            return i(new x3('maxAge exceeded', new Date(Te * 1e3)));
        }
        if (r.complete === !0) {
          var Ee = v.signature;
          return i(null, { header: g, payload: k, signature: Ee });
        }
        return i(null, k);
      });
    };
  });
  var P3 = R((FO, T3) => {
    S();
    var B3 = 1 / 0,
      R3 = 9007199254740991,
      WE = 17976931348623157e292,
      I3 = 0 / 0,
      $E = '[object Arguments]',
      ZE = '[object Function]',
      JE = '[object GeneratorFunction]',
      XE = '[object String]',
      YE = '[object Symbol]',
      QE = /^\s+|\s+$/g,
      eA = /^[-+]0x[0-9a-f]+$/i,
      tA = /^0b[01]+$/i,
      rA = /^0o[0-7]+$/i,
      iA = /^(?:0|[1-9]\d*)$/,
      nA = parseInt;
    function fA(t, e) {
      for (var r = -1, n = t ? t.length : 0, i = Array(n); ++r < n; )
        i[r] = e(t[r], r, t);
      return i;
    }
    function aA(t, e, r, n) {
      for (var i = t.length, a = r + (n ? 1 : -1); n ? a-- : ++a < i; )
        if (e(t[a], a, t)) return a;
      return -1;
    }
    function oA(t, e, r) {
      if (e !== e) return aA(t, sA, r);
      for (var n = r - 1, i = t.length; ++n < i; ) if (t[n] === e) return n;
      return -1;
    }
    function sA(t) {
      return t !== t;
    }
    function hA(t, e) {
      for (var r = -1, n = Array(t); ++r < t; ) n[r] = e(r);
      return n;
    }
    function uA(t, e) {
      return fA(e, function (r) {
        return t[r];
      });
    }
    function cA(t, e) {
      return function (r) {
        return t(e(r));
      };
    }
    var t0 = Object.prototype,
      Fc = t0.hasOwnProperty,
      r0 = t0.toString,
      dA = t0.propertyIsEnumerable,
      lA = cA(Object.keys, Object),
      pA = Math.max;
    function vA(t, e) {
      var r = q3(t) || wA(t) ? hA(t.length, String) : [],
        n = r.length,
        i = !!n;
      for (var a in t)
        (e || Fc.call(t, a)) &&
          !(i && (a == 'length' || yA(a, n))) &&
          r.push(a);
      return r;
    }
    function bA(t) {
      if (!mA(t)) return lA(t);
      var e = [];
      for (var r in Object(t)) Fc.call(t, r) && r != 'constructor' && e.push(r);
      return e;
    }
    function yA(t, e) {
      return (
        (e = e ?? R3),
        !!e &&
          (typeof t == 'number' || iA.test(t)) &&
          t > -1 &&
          t % 1 == 0 &&
          t < e
      );
    }
    function mA(t) {
      var e = t && t.constructor,
        r = (typeof e == 'function' && e.prototype) || t0;
      return t === r;
    }
    function gA(t, e, r, n) {
      (t = jc(t) ? t : qA(t)), (r = r && !n ? BA(r) : 0);
      var i = t.length;
      return (
        r < 0 && (r = pA(i + r, 0)),
        SA(t) ? r <= i && t.indexOf(e, r) > -1 : !!i && oA(t, e, r) > -1
      );
    }
    function wA(t) {
      return (
        _A(t) &&
        Fc.call(t, 'callee') &&
        (!dA.call(t, 'callee') || r0.call(t) == $E)
      );
    }
    var q3 = Array.isArray;
    function jc(t) {
      return t != null && MA(t.length) && !xA(t);
    }
    function _A(t) {
      return Uc(t) && jc(t);
    }
    function xA(t) {
      var e = Lc(t) ? r0.call(t) : '';
      return e == ZE || e == JE;
    }
    function MA(t) {
      return typeof t == 'number' && t > -1 && t % 1 == 0 && t <= R3;
    }
    function Lc(t) {
      var e = typeof t;
      return !!t && (e == 'object' || e == 'function');
    }
    function Uc(t) {
      return !!t && typeof t == 'object';
    }
    function SA(t) {
      return typeof t == 'string' || (!q3(t) && Uc(t) && r0.call(t) == XE);
    }
    function EA(t) {
      return typeof t == 'symbol' || (Uc(t) && r0.call(t) == YE);
    }
    function AA(t) {
      if (!t) return t === 0 ? t : 0;
      if (((t = IA(t)), t === B3 || t === -B3)) {
        var e = t < 0 ? -1 : 1;
        return e * WE;
      }
      return t === t ? t : 0;
    }
    function BA(t) {
      var e = AA(t),
        r = e % 1;
      return e === e ? (r ? e - r : e) : 0;
    }
    function IA(t) {
      if (typeof t == 'number') return t;
      if (EA(t)) return I3;
      if (Lc(t)) {
        var e = typeof t.valueOf == 'function' ? t.valueOf() : t;
        t = Lc(e) ? e + '' : e;
      }
      if (typeof t != 'string') return t === 0 ? t : +t;
      t = t.replace(QE, '');
      var r = tA.test(t);
      return r || rA.test(t) ? nA(t.slice(2), r ? 2 : 8) : eA.test(t) ? I3 : +t;
    }
    function RA(t) {
      return jc(t) ? vA(t) : bA(t);
    }
    function qA(t) {
      return t ? uA(t, RA(t)) : [];
    }
    T3.exports = gA;
  });
  var O3 = R((UO, k3) => {
    S();
    var TA = '[object Boolean]',
      PA = Object.prototype,
      kA = PA.toString;
    function OA(t) {
      return t === !0 || t === !1 || (CA(t) && kA.call(t) == TA);
    }
    function CA(t) {
      return !!t && typeof t == 'object';
    }
    k3.exports = OA;
  });
  var F3 = R((HO, L3) => {
    S();
    var C3 = 1 / 0,
      NA = 17976931348623157e292,
      N3 = 0 / 0,
      DA = '[object Symbol]',
      LA = /^\s+|\s+$/g,
      FA = /^[-+]0x[0-9a-f]+$/i,
      jA = /^0b[01]+$/i,
      UA = /^0o[0-7]+$/i,
      zA = parseInt,
      HA = Object.prototype,
      KA = HA.toString;
    function VA(t) {
      return typeof t == 'number' && t == ZA(t);
    }
    function D3(t) {
      var e = typeof t;
      return !!t && (e == 'object' || e == 'function');
    }
    function GA(t) {
      return !!t && typeof t == 'object';
    }
    function WA(t) {
      return typeof t == 'symbol' || (GA(t) && KA.call(t) == DA);
    }
    function $A(t) {
      if (!t) return t === 0 ? t : 0;
      if (((t = JA(t)), t === C3 || t === -C3)) {
        var e = t < 0 ? -1 : 1;
        return e * NA;
      }
      return t === t ? t : 0;
    }
    function ZA(t) {
      var e = $A(t),
        r = e % 1;
      return e === e ? (r ? e - r : e) : 0;
    }
    function JA(t) {
      if (typeof t == 'number') return t;
      if (WA(t)) return N3;
      if (D3(t)) {
        var e = typeof t.valueOf == 'function' ? t.valueOf() : t;
        t = D3(e) ? e + '' : e;
      }
      if (typeof t != 'string') return t === 0 ? t : +t;
      t = t.replace(LA, '');
      var r = jA.test(t);
      return r || UA.test(t) ? zA(t.slice(2), r ? 2 : 8) : FA.test(t) ? N3 : +t;
    }
    L3.exports = VA;
  });
  var U3 = R((VO, j3) => {
    S();
    var XA = '[object Number]',
      YA = Object.prototype,
      QA = YA.toString;
    function eB(t) {
      return !!t && typeof t == 'object';
    }
    function tB(t) {
      return typeof t == 'number' || (eB(t) && QA.call(t) == XA);
    }
    j3.exports = tB;
  });
  var V3 = R((WO, K3) => {
    S();
    var rB = '[object Object]';
    function iB(t) {
      var e = !1;
      if (t != null && typeof t.toString != 'function')
        try {
          e = !!(t + '');
        } catch {}
      return e;
    }
    function nB(t, e) {
      return function (r) {
        return t(e(r));
      };
    }
    var fB = Function.prototype,
      z3 = Object.prototype,
      H3 = fB.toString,
      aB = z3.hasOwnProperty,
      oB = H3.call(Object),
      sB = z3.toString,
      hB = nB(Object.getPrototypeOf, Object);
    function uB(t) {
      return !!t && typeof t == 'object';
    }
    function cB(t) {
      if (!uB(t) || sB.call(t) != rB || iB(t)) return !1;
      var e = hB(t);
      if (e === null) return !0;
      var r = aB.call(e, 'constructor') && e.constructor;
      return typeof r == 'function' && r instanceof r && H3.call(r) == oB;
    }
    K3.exports = cB;
  });
  var W3 = R((ZO, G3) => {
    S();
    var dB = '[object String]',
      lB = Object.prototype,
      pB = lB.toString,
      vB = Array.isArray;
    function bB(t) {
      return !!t && typeof t == 'object';
    }
    function yB(t) {
      return typeof t == 'string' || (!vB(t) && bB(t) && pB.call(t) == dB);
    }
    G3.exports = yB;
  });
  var Y3 = R((XO, X3) => {
    S();
    var mB = 'Expected a function',
      $3 = 1 / 0,
      gB = 17976931348623157e292,
      Z3 = 0 / 0,
      wB = '[object Symbol]',
      _B = /^\s+|\s+$/g,
      xB = /^[-+]0x[0-9a-f]+$/i,
      MB = /^0b[01]+$/i,
      SB = /^0o[0-7]+$/i,
      EB = parseInt,
      AB = Object.prototype,
      BB = AB.toString;
    function IB(t, e) {
      var r;
      if (typeof e != 'function') throw new TypeError(mB);
      return (
        (t = kB(t)),
        function () {
          return (
            --t > 0 && (r = e.apply(this, arguments)), t <= 1 && (e = void 0), r
          );
        }
      );
    }
    function RB(t) {
      return IB(2, t);
    }
    function J3(t) {
      var e = typeof t;
      return !!t && (e == 'object' || e == 'function');
    }
    function qB(t) {
      return !!t && typeof t == 'object';
    }
    function TB(t) {
      return typeof t == 'symbol' || (qB(t) && BB.call(t) == wB);
    }
    function PB(t) {
      if (!t) return t === 0 ? t : 0;
      if (((t = OB(t)), t === $3 || t === -$3)) {
        var e = t < 0 ? -1 : 1;
        return e * gB;
      }
      return t === t ? t : 0;
    }
    function kB(t) {
      var e = PB(t),
        r = e % 1;
      return e === e ? (r ? e - r : e) : 0;
    }
    function OB(t) {
      if (typeof t == 'number') return t;
      if (TB(t)) return Z3;
      if (J3(t)) {
        var e = typeof t.valueOf == 'function' ? t.valueOf() : t;
        t = J3(e) ? e + '' : e;
      }
      if (typeof t != 'string') return t === 0 ? t : +t;
      t = t.replace(_B, '');
      var r = MB.test(t);
      return r || SB.test(t) ? EB(t.slice(2), r ? 2 : 8) : xB.test(t) ? Z3 : +t;
    }
    X3.exports = RB;
  });
  var sm = R((QO, om) => {
    S();
    var Q3 = gc(),
      CB = Dc(),
      em = Hs(),
      NB = P3(),
      tm = O3(),
      rm = F3(),
      zc = U3(),
      nm = V3(),
      Ci = W3(),
      DB = Y3(),
      fm = [
        'RS256',
        'RS384',
        'RS512',
        'ES256',
        'ES384',
        'ES512',
        'HS256',
        'HS384',
        'HS512',
        'none',
      ];
    CB && fm.splice(3, 0, 'PS256', 'PS384', 'PS512');
    var LB = {
        expiresIn: {
          isValid: function (t) {
            return rm(t) || (Ci(t) && t);
          },
          message:
            '"expiresIn" should be a number of seconds or string representing a timespan',
        },
        notBefore: {
          isValid: function (t) {
            return rm(t) || (Ci(t) && t);
          },
          message:
            '"notBefore" should be a number of seconds or string representing a timespan',
        },
        audience: {
          isValid: function (t) {
            return Ci(t) || Array.isArray(t);
          },
          message: '"audience" must be a string or array',
        },
        algorithm: {
          isValid: NB.bind(null, fm),
          message: '"algorithm" must be a valid string enum value',
        },
        header: { isValid: nm, message: '"header" must be an object' },
        encoding: { isValid: Ci, message: '"encoding" must be a string' },
        issuer: { isValid: Ci, message: '"issuer" must be a string' },
        subject: { isValid: Ci, message: '"subject" must be a string' },
        jwtid: { isValid: Ci, message: '"jwtid" must be a string' },
        noTimestamp: {
          isValid: tm,
          message: '"noTimestamp" must be a boolean',
        },
        keyid: { isValid: Ci, message: '"keyid" must be a string' },
        mutatePayload: {
          isValid: tm,
          message: '"mutatePayload" must be a boolean',
        },
      },
      FB = {
        iat: { isValid: zc, message: '"iat" should be a number of seconds' },
        exp: { isValid: zc, message: '"exp" should be a number of seconds' },
        nbf: { isValid: zc, message: '"nbf" should be a number of seconds' },
      };
    function am(t, e, r, n) {
      if (!nm(r)) throw new Error('Expected "' + n + '" to be a plain object.');
      Object.keys(r).forEach(function (i) {
        var a = t[i];
        if (!a) {
          if (!e) throw new Error('"' + i + '" is not allowed in "' + n + '"');
          return;
        }
        if (!a.isValid(r[i])) throw new Error(a.message);
      });
    }
    function jB(t) {
      return am(LB, !1, t, 'options');
    }
    function UB(t) {
      return am(FB, !0, t, 'payload');
    }
    var im = { audience: 'aud', issuer: 'iss', subject: 'sub', jwtid: 'jti' },
      zB = [
        'expiresIn',
        'notBefore',
        'noTimestamp',
        'audience',
        'issuer',
        'subject',
        'jwtid',
      ];
    om.exports = function (t, e, r, n) {
      typeof r == 'function' ? ((n = r), (r = {})) : (r = r || {});
      var i = typeof t == 'object' && !Buffer.isBuffer(t),
        a = Object.assign(
          {
            alg: r.algorithm || 'HS256',
            typ: i ? 'JWT' : void 0,
            kid: r.keyid,
          },
          r.header
        );
      function h(x) {
        if (n) return n(x);
        throw x;
      }
      if (!e && r.algorithm !== 'none')
        return h(new Error('secretOrPrivateKey must have a value'));
      if (typeof t > 'u') return h(new Error('payload is required'));
      if (i) {
        try {
          UB(t);
        } catch (x) {
          return h(x);
        }
        r.mutatePayload || (t = Object.assign({}, t));
      } else {
        var v = zB.filter(function (x) {
          return typeof r[x] < 'u';
        });
        if (v.length > 0)
          return h(
            new Error(
              'invalid ' + v.join(',') + ' option for ' + typeof t + ' payload'
            )
          );
      }
      if (typeof t.exp < 'u' && typeof r.expiresIn < 'u')
        return h(
          new Error(
            'Bad "options.expiresIn" option the payload already has an "exp" property.'
          )
        );
      if (typeof t.nbf < 'u' && typeof r.notBefore < 'u')
        return h(
          new Error(
            'Bad "options.notBefore" option the payload already has an "nbf" property.'
          )
        );
      try {
        jB(r);
      } catch (x) {
        return h(x);
      }
      var g = t.iat || Math.floor(Date.now() / 1e3);
      if (
        (r.noTimestamp ? delete t.iat : i && (t.iat = g),
        typeof r.notBefore < 'u')
      ) {
        try {
          t.nbf = Q3(r.notBefore, g);
        } catch (x) {
          return h(x);
        }
        if (typeof t.nbf > 'u')
          return h(
            new Error(
              '"notBefore" should be a number of seconds or string representing a timespan eg: "1d", "20h", 60'
            )
          );
      }
      if (typeof r.expiresIn < 'u' && typeof t == 'object') {
        try {
          t.exp = Q3(r.expiresIn, g);
        } catch (x) {
          return h(x);
        }
        if (typeof t.exp > 'u')
          return h(
            new Error(
              '"expiresIn" should be a number of seconds or string representing a timespan eg: "1d", "20h", 60'
            )
          );
      }
      Object.keys(im).forEach(function (x) {
        var E = im[x];
        if (typeof r[x] < 'u') {
          if (typeof t[E] < 'u')
            return h(
              new Error(
                'Bad "options.' +
                  x +
                  '" option. The payload already has an "' +
                  E +
                  '" property.'
              )
            );
          t[E] = r[x];
        }
      });
      var M = r.encoding || 'utf8';
      if (typeof n == 'function')
        (n = n && DB(n)),
          em
            .createSign({ header: a, privateKey: e, payload: t, encoding: M })
            .once('error', n)
            .once('done', function (x) {
              n(null, x);
            });
      else return em.sign({ header: a, payload: t, secret: e, encoding: M });
    };
  });
  var HB = R((tC, Hc) => {
    S();
    Hc.exports = {
      verify: A3(),
      sign: sm(),
      JsonWebTokenError: Bf(),
      NotBeforeError: i0(),
      TokenExpiredError: n0(),
    };
    Object.defineProperty(Hc.exports, 'decode', {
      enumerable: !1,
      value: mc(),
    });
  });
  return HB();
})();
/*!
 * The buffer module from node.js, for the browser.
 *
 * @author   Feross Aboukhadijeh <https://feross.org>
 * @license  MIT
 */
/*! ieee754. BSD-3-Clause License. Feross Aboukhadijeh <https://feross.org/opensource> */
/*! safe-buffer. MIT License. Feross Aboukhadijeh <https://feross.org/opensource> */
//# sourceMappingURL=index.js.map

globalThis.jwt = jwt;
