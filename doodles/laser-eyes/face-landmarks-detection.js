/**
 * @license
 * Copyright 2021 Google LLC. All Rights Reserved.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * =============================================================================
 */
import {
  tensor2d as t,
  tensor1d as e,
  div as n,
  slice as r,
  tidy as o,
  image as i,
  mul as s,
  sub as a,
  squeeze as u,
  concat2d as c,
  sigmoid as h,
  add as l,
  Tensor as f,
  browser as d,
  expandDims as p,
  cast as m,
  concat as v,
  reshape as b,
  env as y,
  getBackend as g,
} from "@tensorflow/tfjs-core";
import { loadGraphModel as P } from "@tensorflow/tfjs-converter";
var w = function () {
  return (w =
    Object.assign ||
    function (t) {
      for (var e, n = 1, r = arguments.length; n < r; n++)
        for (var o in (e = arguments[n]))
          Object.prototype.hasOwnProperty.call(e, o) && (t[o] = e[o]);
      return t;
    }).apply(this, arguments);
};
function x(t, e, n, r) {
  return new (n || (n = Promise))(function (o, i) {
    function s(t) {
      try {
        u(r.next(t));
      } catch (t) {
        i(t);
      }
    }
    function a(t) {
      try {
        u(r.throw(t));
      } catch (t) {
        i(t);
      }
    }
    function u(t) {
      var e;
      t.done
        ? o(t.value)
        : ((e = t.value),
          e instanceof n
            ? e
            : new n(function (t) {
                t(e);
              })).then(s, a);
    }
    u((r = r.apply(t, e || [])).next());
  });
}
function E(t, e) {
  var n,
    r,
    o,
    i,
    s = {
      label: 0,
      sent: function () {
        if (1 & o[0]) throw o[1];
        return o[1];
      },
      trys: [],
      ops: [],
    };
  return (
    (i = { next: a(0), throw: a(1), return: a(2) }),
    "function" == typeof Symbol &&
      (i[Symbol.iterator] = function () {
        return this;
      }),
    i
  );
  function a(i) {
    return function (a) {
      return (function (i) {
        if (n) throw new TypeError("Generator is already executing.");
        for (; s; )
          try {
            if (
              ((n = 1),
              r &&
                (o =
                  2 & i[0]
                    ? r.return
                    : i[0]
                    ? r.throw || ((o = r.return) && o.call(r), 0)
                    : r.next) &&
                !(o = o.call(r, i[1])).done)
            )
              return o;
            switch (((r = 0), o && (i = [2 & i[0], o.value]), i[0])) {
              case 0:
              case 1:
                o = i;
                break;
              case 4:
                return s.label++, { value: i[1], done: !1 };
              case 5:
                s.label++, (r = i[1]), (i = [0]);
                continue;
              case 7:
                (i = s.ops.pop()), s.trys.pop();
                continue;
              default:
                if (
                  !((o = s.trys),
                  (o = o.length > 0 && o[o.length - 1]) ||
                    (6 !== i[0] && 2 !== i[0]))
                ) {
                  s = 0;
                  continue;
                }
                if (3 === i[0] && (!o || (i[1] > o[0] && i[1] < o[3]))) {
                  s.label = i[1];
                  break;
                }
                if (6 === i[0] && s.label < o[1]) {
                  (s.label = o[1]), (o = i);
                  break;
                }
                if (o && s.label < o[2]) {
                  (s.label = o[2]), s.ops.push(i);
                  break;
                }
                o[2] && s.ops.pop(), s.trys.pop();
                continue;
            }
            i = e.call(t, s);
          } catch (t) {
            (i = [6, t]), (r = 0);
          } finally {
            n = o = 0;
          }
        if (5 & i[0]) throw i[1];
        return { value: i[0] ? i[1] : void 0, done: !0 };
      })([i, a]);
    };
  }
}
function k(t, e, n, r) {
  return new (n || (n = Promise))(function (o, i) {
    function s(t) {
      try {
        u(r.next(t));
      } catch (t) {
        i(t);
      }
    }
    function a(t) {
      try {
        u(r.throw(t));
      } catch (t) {
        i(t);
      }
    }
    function u(t) {
      var e;
      t.done
        ? o(t.value)
        : ((e = t.value),
          e instanceof n
            ? e
            : new n(function (t) {
                t(e);
              })).then(s, a);
    }
    u((r = r.apply(t, e || [])).next());
  });
}
function L(t, e) {
  var n,
    r,
    o,
    i,
    s = {
      label: 0,
      sent: function () {
        if (1 & o[0]) throw o[1];
        return o[1];
      },
      trys: [],
      ops: [],
    };
  return (
    (i = { next: a(0), throw: a(1), return: a(2) }),
    "function" == typeof Symbol &&
      (i[Symbol.iterator] = function () {
        return this;
      }),
    i
  );
  function a(i) {
    return function (a) {
      return (function (i) {
        if (n) throw new TypeError("Generator is already executing.");
        for (; s; )
          try {
            if (
              ((n = 1),
              r &&
                (o =
                  2 & i[0]
                    ? r.return
                    : i[0]
                    ? r.throw || ((o = r.return) && o.call(r), 0)
                    : r.next) &&
                !(o = o.call(r, i[1])).done)
            )
              return o;
            switch (((r = 0), o && (i = [2 & i[0], o.value]), i[0])) {
              case 0:
              case 1:
                o = i;
                break;
              case 4:
                return s.label++, { value: i[1], done: !1 };
              case 5:
                s.label++, (r = i[1]), (i = [0]);
                continue;
              case 7:
                (i = s.ops.pop()), s.trys.pop();
                continue;
              default:
                if (
                  !(o = (o = s.trys).length > 0 && o[o.length - 1]) &&
                  (6 === i[0] || 2 === i[0])
                ) {
                  s = 0;
                  continue;
                }
                if (3 === i[0] && (!o || (i[1] > o[0] && i[1] < o[3]))) {
                  s.label = i[1];
                  break;
                }
                if (6 === i[0] && s.label < o[1]) {
                  (s.label = o[1]), (o = i);
                  break;
                }
                if (o && s.label < o[2]) {
                  (s.label = o[2]), s.ops.push(i);
                  break;
                }
                o[2] && s.ops.pop(), s.trys.pop();
                continue;
            }
            i = e.call(t, s);
          } catch (t) {
            (i = [6, t]), (r = 0);
          } finally {
            n = o = 0;
          }
        if (5 & i[0]) throw i[1];
        return { value: i[0] ? i[1] : void 0, done: !0 };
      })([i, a]);
    };
  }
}
var M = function (t) {
    return {
      startEndTensor: t,
      startPoint: r(t, [0, 0], [-1, 2]),
      endPoint: r(t, [0, 2], [-1, 2]),
    };
  },
  I = { strides: [8, 16], anchors: [2, 6] };
function B(t, n) {
  var i, u, c;
  if (t.topLeft instanceof f && t.bottomRight instanceof f) {
    var h = o(function () {
      return [
        v([r(a(n - 1, t.topLeft), 0, 1), r(t.topLeft, 1, 1)]),
        v([a(n - 1, r(t.bottomRight, 0, 1)), r(t.bottomRight, 1, 1)]),
      ];
    });
    (i = h[0]),
      (u = h[1]),
      null != t.landmarks &&
        (c = o(function () {
          var r = a(e([n - 1, 0]), t.landmarks),
            o = e([1, -1]);
          return s(r, o);
        }));
  } else {
    var l = t.topLeft,
      d = l[0],
      p = l[1],
      m = t.bottomRight,
      b = m[0],
      y = m[1];
    (i = [n - 1 - d, p]),
      (u = [n - 1 - b, y]),
      null != t.landmarks &&
        (c = t.landmarks.map(function (t) {
          return [n - 1 - t[0], t[1]];
        }));
  }
  var g = { topLeft: i, bottomRight: u };
  return (
    null != c && (g.landmarks = c),
    null != t.probability &&
      (g.probability =
        t.probability instanceof f ? t.probability.clone() : t.probability),
    g
  );
}
function C(t, e) {
  return o(function () {
    var n;
    return (
      (n = t.hasOwnProperty("box") ? t.box : t),
      u(
        (function (t, e) {
          var n = s(t.startPoint, e),
            r = s(t.endPoint, e),
            o = c([n, r], 1);
          return M(o);
        })(n, e).startEndTensor
      )
    );
  });
}
var O = (function () {
  function v(n, r, o, i, s, a) {
    (this.blazeFaceModel = n),
      (this.width = r),
      (this.height = o),
      (this.maxFaces = i),
      (this.anchorsData = (function (t, e, n) {
        for (var r = [], o = 0; o < n.strides.length; o++)
          for (
            var i = n.strides[o],
              s = Math.floor((e + i - 1) / i),
              a = Math.floor((t + i - 1) / i),
              u = n.anchors[o],
              c = 0;
            c < s;
            c++
          )
            for (var h = i * (c + 0.5), l = 0; l < a; l++)
              for (var f = i * (l + 0.5), d = 0; d < u; d++) r.push([f, h]);
        return r;
      })(r, o, I)),
      (this.anchors = t(this.anchorsData)),
      (this.inputSizeData = [r, o]),
      (this.inputSize = e([r, o])),
      (this.iouThreshold = s),
      (this.scoreThreshold = a);
  }
  return (
    (v.prototype.getBoundingBoxes = function (e, d, p) {
      return (
        void 0 === p && (p = !0),
        k(this, void 0, void 0, function () {
          var m,
            v,
            y,
            g,
            P,
            w,
            x,
            E,
            I,
            B,
            C,
            O,
            R,
            T,
            F = this;
          return L(this, function (U) {
            switch (U.label) {
              case 0:
                return (
                  (m = o(function () {
                    var t = i.resizeBilinear(e, [F.width, F.height]),
                      o = s(a(n(t, 255), 0.5), 2),
                      f = F.blazeFaceModel.predict(o),
                      d = u(f),
                      p = (function (t, e, o) {
                        var i = r(t, [0, 1], [-1, 2]),
                          u = l(i, e),
                          h = r(t, [0, 3], [-1, 2]),
                          f = n(h, o),
                          d = n(u, o),
                          p = n(f, 2),
                          m = a(d, p),
                          v = l(d, p),
                          b = s(m, o),
                          y = s(v, o);
                        return c([b, y], 1);
                      })(d, F.anchors, F.inputSize),
                      m = r(d, [0, 0], [-1, 1]);
                    return [d, p, u(h(m))];
                  })),
                  (v = m[0]),
                  (y = m[1]),
                  (g = m[2]),
                  (P = console.warn),
                  (console.warn = function () {}),
                  (w = i.nonMaxSuppression(
                    y,
                    g,
                    this.maxFaces,
                    this.iouThreshold,
                    this.scoreThreshold
                  )),
                  (console.warn = P),
                  [4, w.array()]
                );
              case 1:
                return (
                  (x = U.sent()),
                  w.dispose(),
                  (E = x.map(function (t) {
                    return r(y, [t, 0], [1, -1]);
                  })),
                  d
                    ? [3, 3]
                    : [
                        4,
                        Promise.all(
                          E.map(function (t) {
                            return k(F, void 0, void 0, function () {
                              var e;
                              return L(this, function (n) {
                                switch (n.label) {
                                  case 0:
                                    return [4, t.array()];
                                  case 1:
                                    return (e = n.sent()), t.dispose(), [2, e];
                                }
                              });
                            });
                          })
                        ),
                      ]
                );
              case 2:
                (E = U.sent()), (U.label = 3);
              case 3:
                for (
                  I = e.shape[1],
                    B = e.shape[2],
                    C = d
                      ? n([B, I], this.inputSize)
                      : [B / this.inputSizeData[0], I / this.inputSizeData[1]],
                    O = [],
                    R = function (e) {
                      var n = E[e],
                        i = o(function () {
                          var o = M(n instanceof f ? n : t(n));
                          if (!p) return o;
                          var i,
                            s = x[e];
                          return (
                            (i = d
                              ? r(F.anchors, [s, 0], [1, 2])
                              : F.anchorsData[s]),
                            {
                              box: o,
                              landmarks: b(u(r(v, [s, 5], [1, -1])), [6, -1]),
                              probability: r(g, [s], [1]),
                              anchor: i,
                            }
                          );
                        });
                      O.push(i);
                    },
                    T = 0;
                  T < E.length;
                  T++
                )
                  R(T);
                return (
                  y.dispose(),
                  g.dispose(),
                  v.dispose(),
                  [2, { boxes: O, scaleFactor: C }]
                );
            }
          });
        })
      );
    }),
    (v.prototype.estimateFaces = function (t, e, n, i) {
      return (
        void 0 === e && (e = !1),
        void 0 === n && (n = !1),
        void 0 === i && (i = !0),
        k(this, void 0, void 0, function () {
          var a,
            u,
            c,
            h,
            v,
            b,
            y = this;
          return L(this, function (g) {
            switch (g.label) {
              case 0:
                return (
                  (a = (function (t) {
                    return t instanceof f
                      ? [t.shape[0], t.shape[1]]
                      : [t.height, t.width];
                  })(t)),
                  (u = a[1]),
                  (c = o(function () {
                    return (
                      t instanceof f || (t = d.fromPixels(t)),
                      p(m(t, "float32"), 0)
                    );
                  })),
                  [4, this.getBoundingBoxes(c, e, i)]
                );
              case 1:
                return (
                  (h = g.sent()),
                  (v = h.boxes),
                  (b = h.scaleFactor),
                  c.dispose(),
                  e
                    ? [
                        2,
                        v.map(function (t) {
                          var e = C(t, b),
                            o = {
                              topLeft: r(e, [0], [2]),
                              bottomRight: r(e, [2], [2]),
                            };
                          if (i) {
                            var a = t,
                              c = a.landmarks,
                              h = a.probability,
                              f = a.anchor,
                              d = s(l(c, f), b);
                            (o.landmarks = d), (o.probability = h);
                          }
                          return n && (o = B(o, u)), o;
                        }),
                      ]
                    : [
                        2,
                        Promise.all(
                          v.map(function (t) {
                            return k(y, void 0, void 0, function () {
                              var e,
                                r,
                                o,
                                s,
                                a,
                                c,
                                h,
                                l,
                                f,
                                d,
                                p,
                                m = this;
                              return L(this, function (v) {
                                switch (v.label) {
                                  case 0:
                                    return (
                                      (e = C(t, b)), i ? [3, 2] : [4, e.array()]
                                    );
                                  case 1:
                                    return (
                                      (a = v.sent()),
                                      (r = {
                                        topLeft: a.slice(0, 2),
                                        bottomRight: a.slice(2),
                                      }),
                                      [3, 4]
                                    );
                                  case 2:
                                    return [
                                      4,
                                      Promise.all(
                                        [t.landmarks, e, t.probability].map(
                                          function (t) {
                                            return k(
                                              m,
                                              void 0,
                                              void 0,
                                              function () {
                                                return L(this, function (e) {
                                                  return [2, t.array()];
                                                });
                                              }
                                            );
                                          }
                                        )
                                      ),
                                    ];
                                  case 3:
                                    (o = v.sent()),
                                      (s = o[0]),
                                      (a = o[1]),
                                      (c = o[2]),
                                      (h = t.anchor),
                                      (f = (l = b)[0]),
                                      (d = l[1]),
                                      (p = s.map(function (t) {
                                        return [
                                          (t[0] + h[0]) * f,
                                          (t[1] + h[1]) * d,
                                        ];
                                      })),
                                      (r = {
                                        topLeft: a.slice(0, 2),
                                        bottomRight: a.slice(2),
                                        landmarks: p,
                                        probability: c,
                                      }),
                                      (function (t) {
                                        t.startEndTensor.dispose(),
                                          t.startPoint.dispose(),
                                          t.endPoint.dispose();
                                      })(t.box),
                                      t.landmarks.dispose(),
                                      t.probability.dispose(),
                                      (v.label = 4);
                                  case 4:
                                    return (
                                      e.dispose(), n && (r = B(r, u)), [2, r]
                                    );
                                }
                              });
                            });
                          })
                        ),
                      ]
                );
            }
          });
        })
      );
    }),
    v
  );
})();
function R(t) {
  var e = void 0 === t ? {} : t,
    n = e.maxFaces,
    r = void 0 === n ? 10 : n,
    o = e.inputWidth,
    i = void 0 === o ? 128 : o,
    s = e.inputHeight,
    a = void 0 === s ? 128 : s,
    u = e.iouThreshold,
    c = void 0 === u ? 0.3 : u,
    h = e.scoreThreshold,
    l = void 0 === h ? 0.75 : h,
    f = e.modelUrl;
  return k(this, void 0, void 0, function () {
    var t;
    return L(this, function (e) {
      switch (e.label) {
        case 0:
          return null == f ? [3, 2] : [4, P(f)];
        case 1:
          return (t = e.sent()), [3, 4];
        case 2:
          return [4, P(HACKURL.blazerface)];
        case 3:
          (t = e.sent()), (e.label = 4);
        case 4:
          return [2, new O(t, i, a, r, c, l)];
      }
    });
  });
}
var T = {
  silhouette: [
    10,
    338,
    297,
    332,
    284,
    251,
    389,
    356,
    454,
    323,
    361,
    288,
    397,
    365,
    379,
    378,
    400,
    377,
    152,
    148,
    176,
    149,
    150,
    136,
    172,
    58,
    132,
    93,
    234,
    127,
    162,
    21,
    54,
    103,
    67,
    109,
  ],
  lipsUpperOuter: [61, 185, 40, 39, 37, 0, 267, 269, 270, 409, 291],
  lipsLowerOuter: [146, 91, 181, 84, 17, 314, 405, 321, 375, 291],
  lipsUpperInner: [78, 191, 80, 81, 82, 13, 312, 311, 310, 415, 308],
  lipsLowerInner: [78, 95, 88, 178, 87, 14, 317, 402, 318, 324, 308],
  rightEyeUpper0: [246, 161, 160, 159, 158, 157, 173],
  rightEyeLower0: [33, 7, 163, 144, 145, 153, 154, 155, 133],
  rightEyeUpper1: [247, 30, 29, 27, 28, 56, 190],
  rightEyeLower1: [130, 25, 110, 24, 23, 22, 26, 112, 243],
  rightEyeUpper2: [113, 225, 224, 223, 222, 221, 189],
  rightEyeLower2: [226, 31, 228, 229, 230, 231, 232, 233, 244],
  rightEyeLower3: [143, 111, 117, 118, 119, 120, 121, 128, 245],
  rightEyebrowUpper: [156, 70, 63, 105, 66, 107, 55, 193],
  rightEyebrowLower: [35, 124, 46, 53, 52, 65],
  rightEyeIris: [473, 474, 475, 476, 477],
  leftEyeUpper0: [466, 388, 387, 386, 385, 384, 398],
  leftEyeLower0: [263, 249, 390, 373, 374, 380, 381, 382, 362],
  leftEyeUpper1: [467, 260, 259, 257, 258, 286, 414],
  leftEyeLower1: [359, 255, 339, 254, 253, 252, 256, 341, 463],
  leftEyeUpper2: [342, 445, 444, 443, 442, 441, 413],
  leftEyeLower2: [446, 261, 448, 449, 450, 451, 452, 453, 464],
  leftEyeLower3: [372, 340, 346, 347, 348, 349, 350, 357, 465],
  leftEyebrowUpper: [383, 300, 293, 334, 296, 336, 285, 417],
  leftEyebrowLower: [265, 353, 276, 283, 282, 295],
  leftEyeIris: [468, 469, 470, 471, 472],
  midwayBetweenEyes: [168],
  noseTip: [1],
  noseBottom: [2],
  noseRightCorner: [98],
  noseLeftCorner: [327],
  rightCheek: [205],
  leftCheek: [425],
};
function F(t) {
  return [
    Math.abs(t.endPoint[0] - t.startPoint[0]),
    Math.abs(t.endPoint[1] - t.startPoint[1]),
  ];
}
function U(t) {
  return [
    t.startPoint[0] + (t.endPoint[0] - t.startPoint[0]) / 2,
    t.startPoint[1] + (t.endPoint[1] - t.startPoint[1]) / 2,
  ];
}
function S(t, e) {
  void 0 === e && (e = 1.5);
  var n = U(t),
    r = F(t),
    o = [(e * r[0]) / 2, (e * r[1]) / 2];
  return {
    startPoint: [n[0] - o[0], n[1] - o[1]],
    endPoint: [n[0] + o[0], n[1] + o[1]],
    landmarks: t.landmarks,
  };
}
function D(t) {
  var e = U(t),
    n = F(t),
    r = Math.max.apply(Math, n) / 2;
  return {
    startPoint: [e[0] - r, e[1] - r],
    endPoint: [e[0] + r, e[1] + r],
    landmarks: t.landmarks,
  };
}
var W = [
  [1, 0, 0],
  [0, 1, 0],
  [0, 0, 1],
];
function z(t, e) {
  var n,
    r = Math.PI / 2 - Math.atan2(-(e[1] - t[1]), e[0] - t[0]);
  return (n = r) - 2 * Math.PI * Math.floor((n + Math.PI) / (2 * Math.PI));
}
function H(t, e) {
  return [
    [1, 0, t],
    [0, 1, e],
    [0, 0, 1],
  ];
}
function j(t, e) {
  for (var n = 0, r = 0; r < t.length; r++) n += t[r] * e[r];
  return n;
}
function A(t, e) {
  for (var n = [], r = 0; r < t.length; r++) n.push(t[r][e]);
  return n;
}
function V(t, e) {
  for (var n = [], r = t.length, o = 0; o < r; o++) {
    n.push([]);
    for (var i = 0; i < r; i++) n[o].push(j(t[o], A(e, i)));
  }
  return n;
}
function _(t, e) {
  var n = Math.cos(t),
    r = Math.sin(t),
    o = [
      [n, -r, 0],
      [r, n, 0],
      [0, 0, 1],
    ],
    i = V(H(e[0], e[1]), o);
  return V(i, H(-e[0], -e[1]));
}
var G = [13, T.midwayBetweenEyes[0]],
  K = [3, 2],
  N = T.leftEyeLower0,
  q = [N[0], N[N.length - 1]],
  J = T.rightEyeLower0,
  Q = [J[0], J[J.length - 1]],
  X = [
    { key: "EyeUpper0", indices: [9, 10, 11, 12, 13, 14, 15] },
    { key: "EyeUpper1", indices: [25, 26, 27, 28, 29, 30, 31] },
    { key: "EyeUpper2", indices: [41, 42, 43, 44, 45, 46, 47] },
    { key: "EyeLower0", indices: [0, 1, 2, 3, 4, 5, 6, 7, 8] },
    { key: "EyeLower1", indices: [16, 17, 18, 19, 20, 21, 22, 23, 24] },
    { key: "EyeLower2", indices: [32, 33, 34, 35, 36, 37, 38, 39, 40] },
    { key: "EyeLower3", indices: [54, 55, 56, 57, 58, 59, 60, 61, 62] },
    { key: "EyebrowUpper", indices: [63, 64, 65, 66, 67, 68, 69, 70] },
    { key: "EyebrowLower", indices: [48, 49, 50, 51, 52, 53] },
  ];
function Y(t, e, n, r) {
  for (var o = 0; o < X.length; o++) {
    var i = X[o],
      s = i.key,
      a = i.indices,
      u = T["" + n + s];
    if (null == r || r.includes(s))
      for (var c = 0; c < a.length; c++) {
        var h = a[c];
        t[u[c]] = [e[h][0], e[h][1], (e[h][2] + t[u[c]][2]) / 2];
      }
  }
}
var Z = (function () {
    function e(t, e, n, r, o, i, s) {
      (this.regionsOfInterest = []),
        (this.runsWithoutFaceDetector = 0),
        (this.boundingBoxDetector = t),
        (this.meshDetector = e),
        (this.irisModel = s),
        (this.meshWidth = n),
        (this.meshHeight = r),
        (this.maxContinuousChecks = o),
        (this.maxFaces = i);
    }
    return (
      (e.prototype.transformRawCoords = function (t, e, n, r) {
        var o,
          i,
          s,
          a,
          u = this,
          c = F({ startPoint: e.startPoint, endPoint: e.endPoint }),
          h = [c[0] / this.meshWidth, c[1] / this.meshHeight],
          l = t.map(function (t) {
            return [
              h[0] * (t[0] - u.meshWidth / 2),
              h[1] * (t[1] - u.meshHeight / 2),
              t[2],
            ];
          }),
          f = _(n, [0, 0]),
          d = l.map(function (t) {
            return (function (t, e) {
              return [j(t, e[0]), j(t, e[1])];
            })(t, f).concat([t[2]]);
          }),
          p =
            ((i = [
              [(o = r)[0][0], o[1][0]],
              [o[0][1], o[1][1]],
            ]),
            (s = [o[0][2], o[1][2]]),
            (a = [-j(i[0], s), -j(i[1], s)]),
            [i[0].concat(a[0]), i[1].concat(a[1]), [0, 0, 1]]),
          m = U({ startPoint: e.startPoint, endPoint: e.endPoint }).concat([1]),
          v = [j(m, p[0]), j(m, p[1])];
        return d.map(function (t) {
          return [t[0] + v[0], t[1] + v[1], t[2]];
        });
      }),
      (e.prototype.getLeftToRightEyeDepthDifference = function (t) {
        return t[q[0]][2] - t[Q[0]][2];
      }),
      (e.prototype.getEyeBox = function (t, e, n, r, o) {
        void 0 === o && (o = !1);
        var s = D(S(this.calculateLandmarksBoundingBox([t[n], t[r]]), 2.3)),
          a = F(s),
          u = i.cropAndResize(
            e,
            [
              [
                s.startPoint[1] / this.meshHeight,
                s.startPoint[0] / this.meshWidth,
                s.endPoint[1] / this.meshHeight,
                s.endPoint[0] / this.meshWidth,
              ],
            ],
            [0],
            [64, 64]
          );
        return o && (u = i.flipLeftRight(u)), { box: s, boxSize: a, crop: u };
      }),
      (e.prototype.getEyeCoords = function (t, e, n, r) {
        void 0 === r && (r = !1);
        for (var o = [], i = 0; i < 76; i++) {
          var s = t[3 * i],
            a = t[3 * i + 1],
            u = t[3 * i + 2];
          o.push([
            (r ? 1 - s / 64 : s / 64) * n[0] + e.startPoint[0],
            (a / 64) * n[1] + e.startPoint[1],
            u,
          ]);
        }
        return { rawCoords: o, iris: o.slice(71) };
      }),
      (e.prototype.getAdjustedIrisCoords = function (t, e, n) {
        var r = t[T[n + "EyeUpper0"][3]][2],
          o = t[T[n + "EyeLower0"][4]][2],
          i = (r + o) / 2;
        return e.map(function (t, e) {
          var n = i;
          return 2 === e ? (n = r) : 4 === e && (n = o), [t[0], t[1], n];
        });
      }),
      (e.prototype.predict = function (e, r) {
        return x(this, void 0, void 0, function () {
          var s,
            a,
            c,
            h,
            l = this;
          return E(this, function (f) {
            switch (f.label) {
              case 0:
                return this.shouldUpdateRegionsOfInterest()
                  ? (!1,
                    !0,
                    [4, this.boundingBoxDetector.getBoundingBoxes(e, !1, !0)])
                  : [3, 2];
              case 1:
                return (
                  (s = f.sent()),
                  (a = s.boxes),
                  (c = s.scaleFactor),
                  0 === a.length
                    ? ((this.regionsOfInterest = []), [2, null])
                    : ((h = a.map(function (t) {
                        var e,
                          n,
                          r = {
                            startPoint: u(t.box.startPoint).arraySync(),
                            endPoint: u(t.box.endPoint).arraySync(),
                          },
                          o = D(
                            S(
                              ((n = c),
                              {
                                startPoint: [
                                  (e = r).startPoint[0] * n[0],
                                  e.startPoint[1] * n[1],
                                ],
                                endPoint: [
                                  e.endPoint[0] * n[0],
                                  e.endPoint[1] * n[1],
                                ],
                              })
                            )
                          );
                        return w({}, o, { landmarks: t.landmarks.arraySync() });
                      })),
                      a.forEach(function (t) {
                        null != t &&
                          null != t.startPoint &&
                          (t.startEndTensor.dispose(),
                          t.startPoint.dispose(),
                          t.endPoint.dispose());
                      }),
                      this.updateRegionsOfInterest(h),
                      (this.runsWithoutFaceDetector = 0),
                      [3, 3])
                );
              case 2:
                this.runsWithoutFaceDetector++, (f.label = 3);
              case 3:
                return [
                  2,
                  o(function () {
                    return l.regionsOfInterest.map(function (o, s) {
                      var a,
                        c = o.landmarks.length >= 468,
                        h = G[0],
                        f = G[1];
                      !1 === c && ((h = K[0]), (f = K[1])),
                        (a = z(o.landmarks[h], o.landmarks[f]));
                      var d = U({
                          startPoint: o.startPoint,
                          endPoint: o.endPoint,
                        }),
                        p = [d[0] / e.shape[2], d[1] / e.shape[1]],
                        m = e,
                        y = W;
                      0 !== a &&
                        ((m = i.rotateWithOffset(e, a, 0, p)), (y = _(-a, d)));
                      var g = {
                          startPoint: o.startPoint,
                          endPoint: o.endPoint,
                        },
                        P = n(
                          (function (t, e, n) {
                            var r = e.shape[1],
                              o = e.shape[2],
                              s = [
                                [
                                  t.startPoint[1] / r,
                                  t.startPoint[0] / o,
                                  t.endPoint[1] / r,
                                  t.endPoint[0] / o,
                                ],
                              ];
                            return i.cropAndResize(e, s, [0], n, "bilinear", 0);
                          })(g, m, [l.meshHeight, l.meshWidth]),
                          255
                        ),
                        x = l.meshDetector.predict(P),
                        E = x[1],
                        k = x[2],
                        L = b(k, [-1, 3]).arraySync();
                      if (r) {
                        var M = l.getEyeBox(L, P, q[0], q[1], !0),
                          I = M.box,
                          B = M.boxSize,
                          C = M.crop,
                          O = l.getEyeBox(L, P, Q[0], Q[1]),
                          R = O.box,
                          T = O.boxSize,
                          F = O.crop,
                          H = l.irisModel.predict(v([C, F])).dataSync(),
                          j = H.slice(0, 228),
                          A = l.getEyeCoords(j, I, B, !0),
                          V = A.rawCoords,
                          N = A.iris,
                          J = H.slice(228),
                          X = l.getEyeCoords(J, R, T),
                          Z = X.rawCoords,
                          $ = X.iris,
                          tt = l.getLeftToRightEyeDepthDifference(L);
                        Math.abs(tt) < 30
                          ? (Y(L, V, "left"), Y(L, Z, "right"))
                          : tt < 1
                          ? Y(L, V, "left", ["EyeUpper0", "EyeLower0"])
                          : Y(L, Z, "right", ["EyeUpper0", "EyeLower0"]);
                        var et = l.getAdjustedIrisCoords(L, N, "left"),
                          nt = l.getAdjustedIrisCoords(L, $, "right");
                        L = L.concat(et).concat(nt);
                      }
                      var rt = l.transformRawCoords(L, o, a, y),
                        ot = t(rt),
                        it = S(l.calculateLandmarksBoundingBox(rt)),
                        st = D(it);
                      return (
                        (l.regionsOfInterest[s] = w({}, st, {
                          landmarks: ot.arraySync(),
                        })),
                        {
                          coords: t(L, [L.length, 3]),
                          scaledCoords: ot,
                          box: it,
                          flag: u(E),
                        }
                      );
                    });
                  }),
                ];
            }
          });
        });
      }),
      (e.prototype.updateRegionsOfInterest = function (t) {
        for (var e = 0; e < t.length; e++) {
          var n = t[e],
            r = this.regionsOfInterest[e],
            o = 0;
          if (r && r.startPoint) {
            var i = n.startPoint,
              s = i[0],
              a = i[1],
              u = n.endPoint,
              c = u[0],
              h = u[1],
              l = r.startPoint,
              f = l[0],
              d = l[1],
              p = r.endPoint,
              m = p[0],
              v = p[1],
              b = Math.max(s, f),
              y = Math.max(a, d),
              g = (Math.min(c, m) - b) * (Math.min(h, v) - y);
            o = g / ((c - s) * (h - a) + (m - f) * (v - a) - g);
          }
          o < 0.25 && (this.regionsOfInterest[e] = n);
        }
        this.regionsOfInterest = this.regionsOfInterest.slice(0, t.length);
      }),
      (e.prototype.clearRegionOfInterest = function (t) {
        null != this.regionsOfInterest[t] &&
          (this.regionsOfInterest = this.regionsOfInterest
            .slice(0, t)
            .concat(this.regionsOfInterest.slice(t + 1)));
      }),
      (e.prototype.shouldUpdateRegionsOfInterest = function () {
        var t = this.regionsOfInterest.length,
          e = 0 === t;
        return 1 === this.maxFaces || e
          ? e
          : t !== this.maxFaces &&
              this.runsWithoutFaceDetector >= this.maxContinuousChecks;
      }),
      (e.prototype.calculateLandmarksBoundingBox = function (t) {
        var e = t.map(function (t) {
            return t[0];
          }),
          n = t.map(function (t) {
            return t[1];
          });
        return {
          startPoint: [Math.min.apply(Math, e), Math.min.apply(Math, n)],
          endPoint: [Math.max.apply(Math, e), Math.max.apply(Math, n)],
        };
      }),
      e
    );
  })(),
  $ = [
    [0.499976992607117, 0.652534008026123],
    [0.500025987625122, 0.547487020492554],
    [0.499974012374878, 0.602371990680695],
    [0.482113003730774, 0.471979022026062],
    [0.500150978565216, 0.527155995368958],
    [0.499909996986389, 0.498252987861633],
    [0.499523013830185, 0.40106201171875],
    [0.289712011814117, 0.380764007568359],
    [0.499954998493195, 0.312398016452789],
    [0.499987006187439, 0.269918978214264],
    [0.500023007392883, 0.107050001621246],
    [0.500023007392883, 0.666234016418457],
    [0.5000159740448, 0.679224014282227],
    [0.500023007392883, 0.692348003387451],
    [0.499976992607117, 0.695277988910675],
    [0.499976992607117, 0.70593398809433],
    [0.499976992607117, 0.719385027885437],
    [0.499976992607117, 0.737019002437592],
    [0.499967992305756, 0.781370997428894],
    [0.499816000461578, 0.562981009483337],
    [0.473773002624512, 0.573909997940063],
    [0.104906998574734, 0.254140973091125],
    [0.365929991006851, 0.409575998783112],
    [0.338757991790771, 0.41302502155304],
    [0.311120003461838, 0.409460008144379],
    [0.274657994508743, 0.389131009578705],
    [0.393361985683441, 0.403706014156342],
    [0.345234006643295, 0.344011008739471],
    [0.370094001293182, 0.346076011657715],
    [0.319321990013123, 0.347265005111694],
    [0.297903001308441, 0.353591024875641],
    [0.24779200553894, 0.410809993743896],
    [0.396889001131058, 0.842755019664764],
    [0.280097991228104, 0.375599980354309],
    [0.106310002505779, 0.399955987930298],
    [0.2099249958992, 0.391353011131287],
    [0.355807989835739, 0.534406006336212],
    [0.471751004457474, 0.65040397644043],
    [0.474155008792877, 0.680191993713379],
    [0.439785003662109, 0.657229006290436],
    [0.414617002010345, 0.66654098033905],
    [0.450374007225037, 0.680860996246338],
    [0.428770989179611, 0.682690978050232],
    [0.374971002340317, 0.727805018424988],
    [0.486716985702515, 0.547628998756409],
    [0.485300987958908, 0.527395009994507],
    [0.257764995098114, 0.314490020275116],
    [0.401223003864288, 0.455172002315521],
    [0.429818987846375, 0.548614978790283],
    [0.421351999044418, 0.533740997314453],
    [0.276895999908447, 0.532056987285614],
    [0.483370006084442, 0.499586999416351],
    [0.33721199631691, 0.282882988452911],
    [0.296391993761063, 0.293242990970612],
    [0.169294998049736, 0.193813979625702],
    [0.447580009698868, 0.302609980106354],
    [0.392390012741089, 0.353887975215912],
    [0.354490011930466, 0.696784019470215],
    [0.067304998636246, 0.730105042457581],
    [0.442739009857178, 0.572826027870178],
    [0.457098007202148, 0.584792017936707],
    [0.381974011659622, 0.694710969924927],
    [0.392388999462128, 0.694203019142151],
    [0.277076005935669, 0.271932005882263],
    [0.422551989555359, 0.563233017921448],
    [0.385919004678726, 0.281364023685455],
    [0.383103013038635, 0.255840003490448],
    [0.331431001424789, 0.119714021682739],
    [0.229923993349075, 0.232002973556519],
    [0.364500999450684, 0.189113974571228],
    [0.229622006416321, 0.299540996551514],
    [0.173287004232407, 0.278747975826263],
    [0.472878992557526, 0.666198015213013],
    [0.446828007698059, 0.668527007102966],
    [0.422762006521225, 0.673889994621277],
    [0.445307999849319, 0.580065965652466],
    [0.388103008270264, 0.693961024284363],
    [0.403039008378983, 0.706539988517761],
    [0.403629004955292, 0.693953037261963],
    [0.460041999816895, 0.557139039039612],
    [0.431158006191254, 0.692366003990173],
    [0.452181994915009, 0.692366003990173],
    [0.475387006998062, 0.692366003990173],
    [0.465828001499176, 0.779190003871918],
    [0.472328990697861, 0.736225962638855],
    [0.473087012767792, 0.717857003211975],
    [0.473122000694275, 0.704625964164734],
    [0.473033010959625, 0.695277988910675],
    [0.427942007780075, 0.695277988910675],
    [0.426479011774063, 0.703539967536926],
    [0.423162013292313, 0.711845993995667],
    [0.4183090031147, 0.720062971115112],
    [0.390094995498657, 0.639572978019714],
    [0.013953999616206, 0.560034036636353],
    [0.499913990497589, 0.58014702796936],
    [0.413199990987778, 0.69539999961853],
    [0.409626007080078, 0.701822996139526],
    [0.468080013990402, 0.601534962654114],
    [0.422728985548019, 0.585985004901886],
    [0.463079988956451, 0.593783974647522],
    [0.37211999297142, 0.47341400384903],
    [0.334562003612518, 0.496073007583618],
    [0.411671012639999, 0.546965003013611],
    [0.242175996303558, 0.14767599105835],
    [0.290776997804642, 0.201445996761322],
    [0.327338010072708, 0.256527006626129],
    [0.399509996175766, 0.748921036720276],
    [0.441727995872498, 0.261676013469696],
    [0.429764986038208, 0.187834024429321],
    [0.412198007106781, 0.108901023864746],
    [0.288955003023148, 0.398952007293701],
    [0.218936994671822, 0.435410976409912],
    [0.41278201341629, 0.398970007896423],
    [0.257135003805161, 0.355440020561218],
    [0.427684992551804, 0.437960982322693],
    [0.448339998722076, 0.536936044692993],
    [0.178560003638268, 0.45755398273468],
    [0.247308000922203, 0.457193970680237],
    [0.286267012357712, 0.467674970626831],
    [0.332827985286713, 0.460712015628815],
    [0.368755996227264, 0.447206974029541],
    [0.398963987827301, 0.432654976844788],
    [0.476410001516342, 0.405806005001068],
    [0.189241006970406, 0.523923993110657],
    [0.228962004184723, 0.348950982093811],
    [0.490725994110107, 0.562400996685028],
    [0.404670000076294, 0.485132992267609],
    [0.019469000399113, 0.401564002037048],
    [0.426243007183075, 0.420431017875671],
    [0.396993011236191, 0.548797011375427],
    [0.266469985246658, 0.376977026462555],
    [0.439121007919312, 0.51895797252655],
    [0.032313998788595, 0.644356966018677],
    [0.419054001569748, 0.387154996395111],
    [0.462783008813858, 0.505746960639954],
    [0.238978996872902, 0.779744982719421],
    [0.198220998048782, 0.831938028335571],
    [0.107550002634525, 0.540755033493042],
    [0.183610007166862, 0.740257024765015],
    [0.134409993886948, 0.333683013916016],
    [0.385764002799988, 0.883153975009918],
    [0.490967005491257, 0.579378008842468],
    [0.382384985685349, 0.508572995662689],
    [0.174399003386497, 0.397670984268188],
    [0.318785011768341, 0.39623498916626],
    [0.343364000320435, 0.400596976280212],
    [0.396100014448166, 0.710216999053955],
    [0.187885001301765, 0.588537991046906],
    [0.430987000465393, 0.944064974784851],
    [0.318993002176285, 0.898285031318665],
    [0.266247987747192, 0.869701027870178],
    [0.500023007392883, 0.190576016902924],
    [0.499976992607117, 0.954452991485596],
    [0.366169989109039, 0.398822009563446],
    [0.393207013607025, 0.39553701877594],
    [0.410373002290726, 0.391080021858215],
    [0.194993004202843, 0.342101991176605],
    [0.388664990663528, 0.362284004688263],
    [0.365961998701096, 0.355970978736877],
    [0.343364000320435, 0.355356991291046],
    [0.318785011768341, 0.35834002494812],
    [0.301414996385574, 0.363156020641327],
    [0.058132998645306, 0.319076001644135],
    [0.301414996385574, 0.387449026107788],
    [0.499987989664078, 0.618434011936188],
    [0.415838003158569, 0.624195992946625],
    [0.445681989192963, 0.566076993942261],
    [0.465844005346298, 0.620640993118286],
    [0.49992299079895, 0.351523995399475],
    [0.288718998432159, 0.819945991039276],
    [0.335278987884521, 0.852819979190826],
    [0.440512001514435, 0.902418971061707],
    [0.128294005990028, 0.791940987110138],
    [0.408771991729736, 0.373893976211548],
    [0.455606997013092, 0.451801002025604],
    [0.499877005815506, 0.908990025520325],
    [0.375436991453171, 0.924192011356354],
    [0.11421000212431, 0.615022003650665],
    [0.448662012815475, 0.695277988910675],
    [0.4480200111866, 0.704632043838501],
    [0.447111994028091, 0.715808033943176],
    [0.444831997156143, 0.730794012546539],
    [0.430011987686157, 0.766808986663818],
    [0.406787008047104, 0.685672998428345],
    [0.400738000869751, 0.681069016456604],
    [0.392399996519089, 0.677703022956848],
    [0.367855995893478, 0.663918972015381],
    [0.247923001646996, 0.601333022117615],
    [0.452769994735718, 0.420849978923798],
    [0.43639200925827, 0.359887003898621],
    [0.416164010763168, 0.368713974952698],
    [0.413385987281799, 0.692366003990173],
    [0.228018000721931, 0.683571994304657],
    [0.468268007040024, 0.352671027183533],
    [0.411361992359161, 0.804327011108398],
    [0.499989002943039, 0.469825029373169],
    [0.479153990745544, 0.442654013633728],
    [0.499974012374878, 0.439637005329132],
    [0.432112008333206, 0.493588984012604],
    [0.499886006116867, 0.866917014122009],
    [0.49991300702095, 0.821729004383087],
    [0.456548988819122, 0.819200992584229],
    [0.344549000263214, 0.745438992977142],
    [0.37890899181366, 0.574010014533997],
    [0.374292999505997, 0.780184984207153],
    [0.319687992334366, 0.570737957954407],
    [0.357154995203018, 0.604269981384277],
    [0.295284003019333, 0.621580958366394],
    [0.447750002145767, 0.862477004528046],
    [0.410986006259918, 0.508723020553589],
    [0.31395098567009, 0.775308012962341],
    [0.354128003120422, 0.812552988529205],
    [0.324548006057739, 0.703992962837219],
    [0.189096003770828, 0.646299958229065],
    [0.279776990413666, 0.71465802192688],
    [0.1338230073452, 0.682700991630554],
    [0.336768001317978, 0.644733011722565],
    [0.429883986711502, 0.466521978378296],
    [0.455527991056442, 0.548622965812683],
    [0.437114000320435, 0.558896005153656],
    [0.467287987470627, 0.529924988746643],
    [0.414712011814117, 0.335219979286194],
    [0.37704598903656, 0.322777986526489],
    [0.344107985496521, 0.320150971412659],
    [0.312875986099243, 0.32233202457428],
    [0.283526003360748, 0.333190023899078],
    [0.241245999932289, 0.382785975933075],
    [0.102986000478268, 0.468762993812561],
    [0.267612010240555, 0.424560010433197],
    [0.297879010438919, 0.433175981044769],
    [0.333433985710144, 0.433878004550934],
    [0.366427004337311, 0.426115989685059],
    [0.396012008190155, 0.416696012020111],
    [0.420121014118195, 0.41022801399231],
    [0.007561000064015, 0.480777025222778],
    [0.432949006557465, 0.569517970085144],
    [0.458638995885849, 0.479089021682739],
    [0.473466008901596, 0.545744001865387],
    [0.476087987422943, 0.563830018043518],
    [0.468472003936768, 0.555056989192963],
    [0.433990985155106, 0.582361996173859],
    [0.483518004417419, 0.562983989715576],
    [0.482482999563217, 0.57784903049469],
    [0.42645001411438, 0.389798998832703],
    [0.438998997211456, 0.39649498462677],
    [0.450067013502121, 0.400434017181396],
    [0.289712011814117, 0.368252992630005],
    [0.276670008897781, 0.363372981548309],
    [0.517862021923065, 0.471948027610779],
    [0.710287988185883, 0.380764007568359],
    [0.526226997375488, 0.573909997940063],
    [0.895093023777008, 0.254140973091125],
    [0.634069979190826, 0.409575998783112],
    [0.661242008209229, 0.41302502155304],
    [0.688880026340485, 0.409460008144379],
    [0.725341975688934, 0.389131009578705],
    [0.606630027294159, 0.40370500087738],
    [0.654766023159027, 0.344011008739471],
    [0.629905998706818, 0.346076011657715],
    [0.680678009986877, 0.347265005111694],
    [0.702096998691559, 0.353591024875641],
    [0.75221198797226, 0.410804986953735],
    [0.602918028831482, 0.842862963676453],
    [0.719901978969574, 0.375599980354309],
    [0.893692970275879, 0.399959981441498],
    [0.790081977844238, 0.391354024410248],
    [0.643998026847839, 0.534487962722778],
    [0.528249025344849, 0.65040397644043],
    [0.525849997997284, 0.680191040039062],
    [0.560214996337891, 0.657229006290436],
    [0.585384011268616, 0.66654098033905],
    [0.549625992774963, 0.680860996246338],
    [0.57122802734375, 0.682691991329193],
    [0.624852001667023, 0.72809898853302],
    [0.513050019741058, 0.547281980514526],
    [0.51509702205658, 0.527251958847046],
    [0.742246985435486, 0.314507007598877],
    [0.598631024360657, 0.454979002475739],
    [0.570338010787964, 0.548575043678284],
    [0.578631997108459, 0.533622980117798],
    [0.723087012767792, 0.532054007053375],
    [0.516445994377136, 0.499638974666595],
    [0.662801027297974, 0.282917976379395],
    [0.70362401008606, 0.293271005153656],
    [0.830704987049103, 0.193813979625702],
    [0.552385985851288, 0.302568018436432],
    [0.607609987258911, 0.353887975215912],
    [0.645429015159607, 0.696707010269165],
    [0.932694971561432, 0.730105042457581],
    [0.557260990142822, 0.572826027870178],
    [0.542901992797852, 0.584792017936707],
    [0.6180260181427, 0.694710969924927],
    [0.607590973377228, 0.694203019142151],
    [0.722943007946014, 0.271963000297546],
    [0.577413976192474, 0.563166975975037],
    [0.614082992076874, 0.281386971473694],
    [0.616907000541687, 0.255886018276215],
    [0.668509006500244, 0.119913995265961],
    [0.770092010498047, 0.232020974159241],
    [0.635536015033722, 0.189248979091644],
    [0.77039098739624, 0.299556016921997],
    [0.826722025871277, 0.278755009174347],
    [0.527121007442474, 0.666198015213013],
    [0.553171992301941, 0.668527007102966],
    [0.577238023281097, 0.673889994621277],
    [0.554691970348358, 0.580065965652466],
    [0.611896991729736, 0.693961024284363],
    [0.59696102142334, 0.706539988517761],
    [0.596370995044708, 0.693953037261963],
    [0.539958000183105, 0.557139039039612],
    [0.568841993808746, 0.692366003990173],
    [0.547818005084991, 0.692366003990173],
    [0.52461302280426, 0.692366003990173],
    [0.534089982509613, 0.779141008853912],
    [0.527670979499817, 0.736225962638855],
    [0.526912987232208, 0.717857003211975],
    [0.526877999305725, 0.704625964164734],
    [0.526966989040375, 0.695277988910675],
    [0.572058022022247, 0.695277988910675],
    [0.573521018028259, 0.703539967536926],
    [0.57683801651001, 0.711845993995667],
    [0.581691026687622, 0.720062971115112],
    [0.609944999217987, 0.639909982681274],
    [0.986046016216278, 0.560034036636353],
    [0.5867999792099, 0.69539999961853],
    [0.590372025966644, 0.701822996139526],
    [0.531915009021759, 0.601536989212036],
    [0.577268004417419, 0.585934996604919],
    [0.536915004253387, 0.593786001205444],
    [0.627542972564697, 0.473352015018463],
    [0.665585994720459, 0.495950996875763],
    [0.588353991508484, 0.546862006187439],
    [0.757824003696442, 0.14767599105835],
    [0.709249973297119, 0.201507985591888],
    [0.672684013843536, 0.256581008434296],
    [0.600408971309662, 0.74900496006012],
    [0.55826598405838, 0.261672019958496],
    [0.570303976535797, 0.187870979309082],
    [0.588165998458862, 0.109044015407562],
    [0.711045026779175, 0.398952007293701],
    [0.781069993972778, 0.435405015945435],
    [0.587247014045715, 0.398931980133057],
    [0.742869973182678, 0.355445981025696],
    [0.572156012058258, 0.437651991844177],
    [0.55186802148819, 0.536570012569427],
    [0.821442008018494, 0.457556009292603],
    [0.752701997756958, 0.457181990146637],
    [0.71375697851181, 0.467626988887787],
    [0.66711300611496, 0.460672974586487],
    [0.631101012229919, 0.447153985500336],
    [0.6008620262146, 0.432473003864288],
    [0.523481011390686, 0.405627012252808],
    [0.810747981071472, 0.523926019668579],
    [0.771045982837677, 0.348959028720856],
    [0.509127020835876, 0.562718033790588],
    [0.595292985439301, 0.485023975372314],
    [0.980530977249146, 0.401564002037048],
    [0.573499977588654, 0.420000016689301],
    [0.602994978427887, 0.548687994480133],
    [0.733529984951019, 0.376977026462555],
    [0.560611009597778, 0.519016981124878],
    [0.967685997486115, 0.644356966018677],
    [0.580985009670258, 0.387160003185272],
    [0.537728011608124, 0.505385041236877],
    [0.760966002941132, 0.779752969741821],
    [0.801778972148895, 0.831938028335571],
    [0.892440974712372, 0.54076099395752],
    [0.816350996494293, 0.740260004997253],
    [0.865594983100891, 0.333687007427216],
    [0.614073991775513, 0.883246004581451],
    [0.508952975273132, 0.579437971115112],
    [0.617941975593567, 0.508316040039062],
    [0.825608015060425, 0.397674977779388],
    [0.681214988231659, 0.39623498916626],
    [0.656635999679565, 0.400596976280212],
    [0.603900015354156, 0.710216999053955],
    [0.81208598613739, 0.588539004325867],
    [0.56801301240921, 0.944564998149872],
    [0.681007981300354, 0.898285031318665],
    [0.733752012252808, 0.869701027870178],
    [0.633830010890961, 0.398822009563446],
    [0.606792986392975, 0.39553701877594],
    [0.589659988880157, 0.391062021255493],
    [0.805015981197357, 0.342108011245728],
    [0.611334979534149, 0.362284004688263],
    [0.634037971496582, 0.355970978736877],
    [0.656635999679565, 0.355356991291046],
    [0.681214988231659, 0.35834002494812],
    [0.698584973812103, 0.363156020641327],
    [0.941866993904114, 0.319076001644135],
    [0.698584973812103, 0.387449026107788],
    [0.584177017211914, 0.624107003211975],
    [0.554318010807037, 0.566076993942261],
    [0.534153997898102, 0.62064003944397],
    [0.711217999458313, 0.819975018501282],
    [0.664629995822906, 0.852871000766754],
    [0.559099972248077, 0.902631998062134],
    [0.871706008911133, 0.791940987110138],
    [0.591234028339386, 0.373893976211548],
    [0.544341027736664, 0.451583981513977],
    [0.624562978744507, 0.924192011356354],
    [0.88577002286911, 0.615028977394104],
    [0.551338016986847, 0.695277988910675],
    [0.551980018615723, 0.704632043838501],
    [0.552887976169586, 0.715808033943176],
    [0.555167973041534, 0.730794012546539],
    [0.569944024085999, 0.767035007476807],
    [0.593203008174896, 0.685675978660583],
    [0.599261999130249, 0.681069016456604],
    [0.607599973678589, 0.677703022956848],
    [0.631937980651855, 0.663500010967255],
    [0.752032995223999, 0.601315021514893],
    [0.547226011753082, 0.420395016670227],
    [0.563543975353241, 0.359827995300293],
    [0.583841025829315, 0.368713974952698],
    [0.586614012718201, 0.692366003990173],
    [0.771915018558502, 0.683578014373779],
    [0.531597018241882, 0.352482974529266],
    [0.588370978832245, 0.804440975189209],
    [0.52079701423645, 0.442565023899078],
    [0.567984998226166, 0.493479013442993],
    [0.543282985687256, 0.819254994392395],
    [0.655317008495331, 0.745514988899231],
    [0.621008992195129, 0.574018001556396],
    [0.625559985637665, 0.78031200170517],
    [0.680198013782501, 0.570719003677368],
    [0.64276397228241, 0.604337990283966],
    [0.704662978649139, 0.621529996395111],
    [0.552012026309967, 0.862591981887817],
    [0.589071989059448, 0.508637011051178],
    [0.685944974422455, 0.775357007980347],
    [0.645735025405884, 0.812640011310577],
    [0.675342977046967, 0.703978002071381],
    [0.810858011245728, 0.646304965019226],
    [0.72012197971344, 0.714666962623596],
    [0.866151988506317, 0.682704985141754],
    [0.663187026977539, 0.644596993923187],
    [0.570082008838654, 0.466325998306274],
    [0.544561982154846, 0.548375964164734],
    [0.562758982181549, 0.558784961700439],
    [0.531987011432648, 0.530140042304993],
    [0.585271000862122, 0.335177004337311],
    [0.622952997684479, 0.32277899980545],
    [0.655896008014679, 0.320163011550903],
    [0.687132000923157, 0.322345972061157],
    [0.716481983661652, 0.333200991153717],
    [0.758756995201111, 0.382786989212036],
    [0.897013008594513, 0.468769013881683],
    [0.732392013072968, 0.424547016620636],
    [0.70211398601532, 0.433162987232208],
    [0.66652500629425, 0.433866024017334],
    [0.633504986763, 0.426087975502014],
    [0.603875994682312, 0.416586995124817],
    [0.579657971858978, 0.409945011138916],
    [0.992439985275269, 0.480777025222778],
    [0.567192018032074, 0.569419980049133],
    [0.54136598110199, 0.478899002075195],
    [0.526564002037048, 0.546118021011353],
    [0.523913025856018, 0.563830018043518],
    [0.531529009342194, 0.555056989192963],
    [0.566035985946655, 0.582329034805298],
    [0.51631098985672, 0.563053965568542],
    [0.5174720287323, 0.577877044677734],
    [0.573594987392426, 0.389806985855103],
    [0.560697972774506, 0.395331978797913],
    [0.549755990505219, 0.399751007556915],
    [0.710287988185883, 0.368252992630005],
    [0.723330020904541, 0.363372981548309],
  ];
function tt(t) {
  return x(this, void 0, void 0, function () {
    var e, n, r, o, i, s, a, u, c, h, l, f, d, p, m, v;
    return E(this, function (b) {
      switch (b.label) {
        case 0:
          return (
            (e = t.maxContinuousChecks),
            (n = void 0 === e ? 5 : e),
            (r = t.detectionConfidence),
            (o = void 0 === r ? 0.9 : r),
            (i = t.maxFaces),
            (s = void 0 === i ? 10 : i),
            (a = t.iouThreshold),
            (u = void 0 === a ? 0.3 : a),
            (c = t.scoreThreshold),
            (h = void 0 === c ? 0.75 : c),
            (l = t.shouldLoadIrisModel),
            (f = void 0 === l || l),
            (d = t.modelUrl),
            (p = t.detectorModelUrl),
            (m = t.irisModelUrl),
            f ? [4, Promise.all([et(p, s, u, h), nt(d), rt(m)])] : [3, 2]
          );
        case 1:
          return (v = b.sent()), [3, 4];
        case 2:
          return [4, Promise.all([et(p, s, u, h), nt(d)])];
        case 3:
          (v = b.sent()), (b.label = 4);
        case 4:
          return [2, new st(v[0], v[1], n, o, s, f ? v[2] : null)];
      }
    });
  });
}
function et(t, e, n, r) {
  return x(this, void 0, void 0, function () {
    return E(this, function (o) {
      return [
        2,
        R({ modelUrl: t, maxFaces: e, iouThreshold: n, scoreThreshold: r }),
      ];
    });
  });
}
function nt(t) {
  return x(this, void 0, void 0, function () {
    return E(this, function (e) {
      return null != t ? [2, P(t)] : [2, P(HACKURL.facemesh)];
    });
  });
}
function rt(t) {
  return x(this, void 0, void 0, function () {
    return E(this, function (e) {
      return null != t ? [2, P(t)] : [2, P(HACKURL.iris)];
    });
  });
}
function ot(t, n) {
  if (t.mesh instanceof f) {
    var i = o(function () {
        var i = e([n - 1, 0, 0]),
          u = e([1, -1, 1]);
        return o(function () {
          return [
            v([
              a(n - 1, r(t.boundingBox.topLeft, 0, 1)),
              r(t.boundingBox.topLeft, 1, 1),
            ]),
            v([
              a(n - 1, r(t.boundingBox.bottomRight, 0, 1)),
              r(t.boundingBox.bottomRight, 1, 1),
            ]),
            s(a(i, t.mesh), u),
            s(a(i, t.scaledMesh), u),
          ];
        });
      }),
      u = i[0],
      c = i[1],
      h = i[2],
      l = i[3];
    return Object.assign({}, t, {
      boundingBox: { topLeft: u, bottomRight: c },
      mesh: h,
      scaledMesh: l,
    });
  }
  return Object.assign({}, t, {
    boundingBox: {
      topLeft: [n - 1 - t.boundingBox.topLeft[0], t.boundingBox.topLeft[1]],
      bottomRight: [
        n - 1 - t.boundingBox.bottomRight[0],
        t.boundingBox.bottomRight[1],
      ],
    },
    mesh: t.mesh.map(function (t) {
      var e = t.slice(0);
      return (e[0] = n - 1 - t[0]), e;
    }),
    scaledMesh: t.scaledMesh.map(function (t) {
      var e = t.slice(0);
      return (e[0] = n - 1 - t[0]), e;
    }),
  });
}
var it,
  st = (function () {
    function t(t, e, n, r, o, i) {
      (this.kind = "MediaPipeFaceMesh"),
        (this.pipeline = new Z(t, e, 192, 192, n, o, i)),
        (this.detectionConfidence = r);
    }
    return (
      (t.getAnnotations = function () {
        return T;
      }),
      (t.getUVCoords = function () {
        return $;
      }),
      (t.prototype.estimateFaces = function (t) {
        return x(this, void 0, void 0, function () {
          var n,
            r,
            i,
            s,
            a,
            u,
            c,
            h,
            l,
            v,
            b,
            P,
            w = this;
          return E(this, function (k) {
            switch (k.label) {
              case 0:
                if (
                  ((n = t.returnTensors),
                  (r = void 0 !== n && n),
                  (i = t.flipHorizontal),
                  (s = void 0 !== i && i),
                  (a = t.predictIrises),
                  (u = void 0 === a || a),
                  (c = t.input),
                  u && null == this.pipeline.irisModel)
                )
                  throw new Error(
                    "The iris model was not loaded as part of facemesh. Please initialize the model with facemesh.load({shouldLoadIrisModel: true})."
                  );
                return (
                  (h = (function (t) {
                    return t instanceof f
                      ? [t.shape[0], t.shape[1]]
                      : [t.height, t.width];
                  })(c)),
                  (l = h[1]),
                  (v = o(function () {
                    return (
                      c instanceof f || (c = d.fromPixels(c)),
                      p(m(c, "float32"), 0)
                    );
                  })),
                  "webgl" !== g()
                    ? [3, 2]
                    : ((P = y().get("WEBGL_PACK_DEPTHWISECONV")),
                      y().set("WEBGL_PACK_DEPTHWISECONV", !0),
                      [4, this.pipeline.predict(v, u)])
                );
              case 1:
                return (
                  (b = k.sent()), y().set("WEBGL_PACK_DEPTHWISECONV", P), [3, 4]
                );
              case 2:
                return [4, this.pipeline.predict(v, u)];
              case 3:
                (b = k.sent()), (k.label = 4);
              case 4:
                return (
                  v.dispose(),
                  null != b && b.length > 0
                    ? [
                        2,
                        Promise.all(
                          b.map(function (t, n) {
                            return x(w, void 0, void 0, function () {
                              var o,
                                i,
                                a,
                                c,
                                h,
                                f,
                                d,
                                p,
                                m,
                                v,
                                b,
                                y,
                                g,
                                P,
                                w = this;
                              return E(this, function (k) {
                                switch (k.label) {
                                  case 0:
                                    return (
                                      (o = t.coords),
                                      (i = t.scaledCoords),
                                      (a = t.box),
                                      (c = t.flag),
                                      (h = [c]),
                                      r || (h = h.concat([o, i])),
                                      [
                                        4,
                                        Promise.all(
                                          h.map(function (t) {
                                            return x(
                                              w,
                                              void 0,
                                              void 0,
                                              function () {
                                                return E(this, function (e) {
                                                  return [2, t.array()];
                                                });
                                              }
                                            );
                                          })
                                        ),
                                      ]
                                    );
                                  case 1:
                                    if (
                                      ((f = k.sent()),
                                      (d = f[0]),
                                      c.dispose(),
                                      d < this.detectionConfidence &&
                                        this.pipeline.clearRegionOfInterest(n),
                                      r)
                                    )
                                      return (
                                        (p = {
                                          kind: "MediaPipePredictionTensors",
                                          faceInViewConfidence: d,
                                          mesh: o,
                                          scaledMesh: i,
                                          boundingBox: {
                                            topLeft: e(a.startPoint),
                                            bottomRight: e(a.endPoint),
                                          },
                                        }),
                                        s ? [2, ot(p, l)] : [2, p]
                                      );
                                    for (P in ((m = f.slice(1)),
                                    (v = m[0]),
                                    (b = m[1]),
                                    i.dispose(),
                                    o.dispose(),
                                    (y = {
                                      kind: "MediaPipePredictionValues",
                                      faceInViewConfidence: d,
                                      boundingBox: {
                                        topLeft: a.startPoint,
                                        bottomRight: a.endPoint,
                                      },
                                      mesh: v,
                                      scaledMesh: b,
                                    }),
                                    s && (y = ot(y, l)),
                                    (g = {}),
                                    T))
                                      (u || !1 === P.includes("Iris")) &&
                                        (g[P] = T[P].map(function (t) {
                                          return y.scaledMesh[t];
                                        }));
                                    return (y.annotations = g), [2, y];
                                }
                              });
                            });
                          })
                        ),
                      ]
                    : [2, []]
                );
            }
          });
        });
      }),
      t
    );
  })();
function at(t, e) {
  return (
    void 0 === t && (t = it.mediapipeFacemesh),
    void 0 === e && (e = {}),
    x(this, void 0, void 0, function () {
      return E(this, function (n) {
        if (t === it.mediapipeFacemesh) return [2, tt(e)];
        throw new Error(t + " is not a valid package name.");
      });
    })
  );
}
!(function (t) {
  t.mediapipeFacemesh = "mediapipe-facemesh";
})(it || (it = {}));
export { it as SupportedPackages, at as load };
