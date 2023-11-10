/**
 * #plotparty 2023-11-07
 * Collab @greweb x @swizzlevixn
 */
use byteorder::*;
use clap::*;
use noise::*;
use rand::prelude::*;
use std::f64::consts::PI;
use std::ops::RangeInclusive;
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "210.0")]
  pub height: f64,
  #[clap(short, long, default_value = "297.0")]
  pub width: f64,
  #[clap(short, long, default_value = "8.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
  #[clap(short, long, default_value = "36.0")]
  pub scale: f64,
  #[clap(short, long, default_value = "4.0")]
  pub density: f64,
  #[clap(short, long, default_value = "0.8")]
  pub strokew: f64,
  #[clap(short, long, default_value = "500000")]
  pub iterations: usize,
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let bound = (pad, pad, width - pad, height - pad);
  let iterations = opts.iterations;
  let density = opts.density;
  let scale = opts.scale;
  let strokew = opts.strokew;

  let crow_svg = r#"
<svg width="100%" height="100%" viewBox="0 0 2933 1591" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" xml:space="preserve" xmlns:serif="http://www.serif.com/" style="fill-rule:evenodd;clip-rule:evenodd;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:1.5;">
  <g transform="matrix(1,0,0,1,-299.065,-1761.09)">
      <path d="M299.565,1857.83C352.831,1790.05 444.742,1801.83 465.049,1800.71C485.355,1799.59 539.493,1810.49 569.234,1803.77C598.975,1797.05 604.963,1771.98 703.371,1763.42C801.779,1754.85 849.717,1777.33 905.548,1811.85C961.379,1846.37 1030.56,1908.41 1075.57,1932C1120.58,1955.58 1195.09,1974.48 1252.09,1967.34C1309.09,1960.2 1386.87,1997.42 1386.87,1997.42C1386.87,1997.42 1433.54,2004.56 1456.49,2011.91C1479.44,2019.27 1511.05,2037.63 1511.05,2037.63C1511.05,2037.63 1548.72,2035.09 1580.37,2047.79C1612.02,2060.48 1667.03,2085.86 1667.03,2085.86C1667.03,2085.86 1688.05,2093.26 1721.11,2107.12C1754.17,2120.98 1789.5,2141.12 1789.5,2141.12C1789.5,2141.12 1824.35,2149.18 1830.28,2163.06C1836.22,2176.93 1853.39,2170.93 1855.5,2181.79C1857.62,2192.65 1902.78,2186.85 1925.65,2192.98C1965.28,2203.59 2041.3,2231.63 2093.29,2245.47C2145.29,2259.32 2206.31,2292.77 2263.21,2311.78C2320.12,2330.78 2440.84,2383.39 2457.74,2402.86C2474.64,2422.34 2488.66,2442.24 2491.54,2450.64C2493.35,2455.91 2475.05,2453.29 2475.05,2453.29C2475.05,2453.29 2488.38,2485.52 2475.68,2496.68C2462.98,2507.85 2447.15,2492.47 2447.15,2492.47C2447.15,2492.47 2458.68,2513.2 2449.75,2524.74C2440.83,2536.28 2424.01,2537.32 2424.01,2537.32C2424.01,2537.32 2467.45,2542.87 2483.47,2551.65C2499.48,2560.43 2498.3,2581.32 2498.3,2581.32C2498.3,2581.32 2523.13,2587.79 2543.03,2596.89C2562.92,2605.99 2607.23,2639.76 2607.23,2639.76C2607.23,2639.76 2659.64,2649.65 2699.01,2671.9C2738.37,2694.15 2743.35,2704.81 2743.35,2704.81C2743.35,2704.81 2837.27,2727.26 2887.34,2757.88C2937.4,2788.5 2992.87,2813.39 2992.87,2813.39C2992.87,2813.39 3100.59,2876.68 3071.79,2897.89C3043,2919.1 3001.89,2912.94 3001.89,2912.94C3001.89,2912.94 3229.45,3034.41 3216.96,3044.6C3204.47,3054.8 3177.18,3039.99 3177.18,3039.99C3177.18,3039.99 3243.55,3080.23 3229.64,3094.05C3215.72,3107.88 3184.02,3088.2 3184.02,3088.2C3184.02,3088.2 3184.51,3095.32 3178.93,3106.07C3173.34,3116.83 3090.2,3087.12 3090.2,3087.12C3090.2,3087.12 3100.22,3099.75 3093.78,3108.43C3087.34,3117.11 2810.77,2961.13 2810.77,2961.13C2810.77,2961.13 2524.07,2909.91 2444.98,2899.97C2365.89,2890.03 2124.14,2853.24 2124.14,2853.24L1986.11,2846.8C1986.11,2846.8 1990.96,2865.57 1975.1,2879.66C1959.23,2893.76 1883.5,2964.72 1846.64,3017.94C1809.79,3071.16 1768.25,3136.58 1768.25,3136.58C1768.25,3136.58 1814.58,3123.19 1835.5,3125.15C1856.42,3127.11 1876.91,3153.32 1876.91,3153.32C1876.91,3153.32 1909.43,3164.33 1908.87,3181.52C1908.31,3198.72 1886.85,3229.31 1886.85,3229.31C1886.85,3229.31 1892.5,3201.61 1891.38,3187.28C1890.26,3172.96 1869.93,3177.55 1869.93,3177.55C1869.93,3177.55 1858.59,3160.45 1826.82,3171.2C1795.05,3181.95 1799.75,3203.37 1769.14,3199.59C1738.53,3195.81 1732.8,3185.22 1732.8,3185.22C1732.8,3185.22 1732.83,3217.92 1710.32,3230.92C1687.8,3243.93 1659.16,3291.72 1659.16,3291.72C1659.16,3291.72 1645.58,3311.21 1642.78,3305.05C1639.98,3298.89 1638.84,3282.86 1638.84,3282.86C1638.84,3282.86 1627.64,3287.84 1629.12,3302.56C1630.59,3317.27 1634.72,3334.6 1634.72,3334.6C1634.72,3334.6 1621.22,3329 1613.7,3302.69C1607.87,3282.28 1610.82,3278.31 1614.46,3268.16C1618.1,3258.01 1626.15,3253.31 1635.55,3241.79C1644.94,3230.26 1649.68,3223.71 1649.68,3223.71C1649.68,3223.71 1596.11,3250.05 1579.88,3257.19C1563.65,3264.32 1498.49,3312.78 1485.03,3309.72C1471.58,3306.66 1465.24,3294.2 1465.24,3294.2C1465.24,3294.2 1438.57,3288.57 1432.6,3306.88C1426.64,3325.19 1426.77,3340.79 1426.77,3340.79C1426.77,3340.79 1402.97,3317.78 1422.19,3294.43C1441.4,3271.08 1452.59,3265.54 1452.59,3265.54C1452.59,3265.54 1470.58,3255.5 1482.64,3248.69C1494.71,3241.88 1560.96,3217.25 1560.96,3217.25C1560.96,3217.25 1548.12,3220.54 1528.01,3216.16C1507.9,3211.79 1503.94,3204.31 1503.94,3204.31C1503.94,3204.31 1487.79,3202.44 1481.49,3218.24C1475.2,3234.03 1469.29,3248.12 1469.29,3248.12C1469.29,3248.12 1453.99,3213.83 1466.85,3199.71C1479.71,3185.6 1497.1,3176.61 1497.1,3176.61C1497.1,3176.61 1552.16,3155.08 1567.91,3160.79C1583.65,3166.51 1601.21,3168.19 1601.21,3168.19C1601.21,3168.19 1601.38,3156.25 1617.46,3154.29C1633.53,3152.33 1667.27,3162.54 1674.65,3127.96C1682.02,3093.39 1718.77,3085.39 1738.7,3060.16C1758.62,3034.93 1869.22,2884.29 1869.22,2884.29C1869.22,2884.29 1838.6,2892.62 1781.22,2887.51C1723.83,2882.4 1644.79,2878.1 1644.79,2878.1C1644.79,2878.1 1664.2,2940.54 1611.3,2956.67C1558.4,2972.8 1506.88,2982.66 1506.88,2982.66C1506.88,2982.66 1497.24,3050.74 1471.02,3052.53C1444.81,3054.32 1428.24,3051.37 1428.24,3051.37C1428.24,3051.37 1469.84,3090.57 1428.68,3099.85C1387.52,3109.14 1289.28,3148.68 1244.61,3169.53C1199.94,3190.38 1108.18,3227.84 1104.99,3244.63C1101.8,3261.42 1075.08,3299.46 1054.13,3303.84C1033.18,3308.21 1031.12,3296.3 1031.12,3296.3C1031.12,3296.3 1011.28,3301.55 1008.91,3317.18C1006.54,3332.81 1005.98,3351.04 1005.98,3351.04C1005.98,3351.04 993.914,3343.7 992.888,3323.57C991.861,3303.43 1011.35,3282.98 1011.35,3282.98C1011.35,3282.98 1005.02,3273.47 1012.02,3263.67C1019.02,3253.87 1024.74,3252.19 1024.74,3252.19L1012.02,3263.67C1012.02,3263.67 972.667,3278.8 965.386,3266.65C958.106,3254.49 961.87,3240.91 961.87,3240.91C961.87,3240.91 938.777,3242.89 940.249,3265.1C941.674,3286.6 945.087,3295.78 945.087,3295.78C945.087,3295.78 917.277,3250.08 930.522,3235.64C943.768,3221.2 948.668,3216.67 948.668,3216.67C948.668,3216.67 926.494,3225.14 924.26,3234.69C922.345,3242.88 916.913,3255.35 901.615,3254.1C886.316,3252.85 875.697,3263.93 856.282,3258.86C836.866,3253.8 824.806,3221.43 824.806,3221.43C824.806,3221.43 815.356,3210.44 805.743,3218.18C796.131,3225.93 773.434,3233.47 773.434,3233.47C773.434,3233.47 773.554,3212.48 796.482,3204.71C819.411,3196.94 857.656,3184.79 867.992,3195C878.327,3205.22 908.487,3216.71 908.487,3216.71C908.487,3216.71 923.852,3206.9 939.408,3198.44C954.964,3189.98 1003.72,3206.93 1003.72,3206.93C1003.72,3206.93 1046.34,3180.99 1068.01,3171.75C1089.69,3162.51 1105.3,3163.65 1122.21,3155.81C1139.13,3147.97 1323.52,3065.52 1323.52,3065.52C1323.52,3065.52 1329.3,3043.97 1329.3,3029.18C1329.3,3014.39 1346.46,2984.47 1346.46,2984.47C1346.46,2984.47 1344.69,2949.42 1310.37,2933.82C1276.06,2918.22 1251.08,2892.2 1251.08,2892.2C1251.08,2892.2 1124.99,2863.51 1044.74,2793.36C964.496,2723.19 802.722,2559.9 744.806,2472.73C686.89,2385.56 615.972,2336.2 603.176,2284.94C590.381,2233.68 560.95,2114.07 553.196,2068.96C545.441,2023.86 533.517,2000.61 507.399,1983.51C481.282,1966.4 443.415,1935.62 443.415,1935.62C443.415,1935.62 399.668,1932.02 363.857,1907.05C328.045,1882.08 329.062,1885.5 299.565,1857.83Z" style="fill:none;stroke:black;stroke-width:1px;"/>
  </g>
</svg>
"#;

  let mut rng = rng_from_seed(opts.seed);
  let paint = PaintMask::new(0.2, width, height);

  let polylines = svg2polylines::parse(crow_svg, 1., true).unwrap();

  let mut minx = 100000.;
  let mut miny = 100000.;
  let mut maxx = -100000.;
  let mut maxy = -100000.;
  let curves: Vec<Vec<(f64, f64)>> = polylines
    .iter()
    .map(|l| {
      l.iter()
        .map(|p| {
          if p.x < minx {
            minx = p.x;
          }
          if p.y < miny {
            miny = p.y;
          }
          if p.x > maxx {
            maxx = p.x;
          }
          if p.y > maxy {
            maxy = p.y;
          }

          (p.x, p.y)
        })
        .collect()
    })
    .collect();

  let ratio = (maxy - miny) / (maxx - minx);

  let curves: Vec<Vec<(f64, f64)>> = curves
    .iter()
    .map(|rt| {
      rt.iter()
        .map(|p| {
          (
            (p.0 - minx) / (maxx - minx),
            ratio * (p.1 - miny) / (maxy - miny),
          )
        })
        .collect()
    })
    .collect();

  let mut routes: Vec<(usize, Vec<(f64, f64)>)> = Vec::new();

  let clr = 0;

  let my = 1.2 * ratio * scale;
  let mut y = pad + strokew;
  let maxy = height - pad - my - strokew;
  let county = ((maxy - y) / my).floor();
  y += (height - 2.0 * pad - county * my) / 2.;

  let mx = scale;
  let maxx = width - mx - pad - strokew;
  let countx = ((maxx - pad - strokew) / mx).floor();
  let offx = (width - 2.0 * pad - countx * mx) / 2. - mx / 4.0;

  for yi in 0..county as usize {
    let mut x = offx + pad + strokew + if yi % 2 == 0 { 0. } else { mx / 2. };
    for _xi in 0..countx as usize {
      routes.extend(
        curves
          .iter()
          .map(|rt| {
            (
              clr,
              rt.iter()
                .map(|p| (x + p.0 * scale, y + p.1 * scale))
                .collect(),
            )
          })
          .collect::<Vec<_>>(),
      );
      x += scale;
    }
    y += my;
  }

  // strokes -> fill -> strokes. will create nice textures!
  let mut drawings = paint.clone_empty();
  for (_clr, route) in routes.iter() {
    drawings.paint_polyline(route, strokew);
  }
  let filling = WormsFilling::rand(&mut rng);
  let routes =
    filling.fill_in_paint(&mut rng, &drawings, clr, density, bound, iterations);

  vec!["black"]
    .iter()
    .enumerate()
    .map(|(ci, color)| {
      let mut data = Data::new();
      for (i, route) in routes.clone() {
        if i == ci {
          data = render_route(data, route);
        }
      }
      let mut l = layer(format!("{} {}", ci, String::from(*color)).as_str());
      l = l.add(base_path(color, 0.35, data));
      l
    })
    .collect()
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document = base_document("white", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
}

#[derive(Clone)]
pub struct PaintMask {
  mask: Vec<bool>,
  precision: f64,
  width: f64,
  height: f64,
}

impl PaintMask {
  fn clone_empty(&self) -> Self {
    Self {
      mask: vec![false; self.mask.len()],
      precision: self.precision,
      width: self.width,
      height: self.height,
    }
  }

  fn new(precision: f64, width: f64, height: f64) -> Self {
    let wi = (width / precision) as usize;
    let hi = (height / precision) as usize;
    Self {
      mask: vec![false; wi * hi],
      width,
      height,
      precision,
    }
  }

  fn is_painted(&self, point: (f64, f64)) -> bool {
    let precision = self.precision;
    let wi = (self.width / precision) as usize;
    let hi = (self.height / precision) as usize;
    let x = ((point.0.max(0.) / precision) as usize).min(wi - 1);
    let y = ((point.1.max(0.) / precision) as usize).min(hi - 1);
    self.mask[x + y * wi]
  }

  fn paint_polyline(&mut self, polyline: &Vec<(f64, f64)>, strokew: f64) {
    if polyline.len() < 1 {
      return;
    }
    let first = polyline[0];
    let mut minx = first.0;
    let mut miny = first.1;
    let mut maxx = first.0;
    let mut maxy = first.1;
    for p in polyline.iter().skip(1) {
      minx = minx.min(p.0);
      miny = miny.min(p.1);
      maxx = maxx.max(p.0);
      maxy = maxy.max(p.1);
    }
    minx = (minx - strokew).max(0.0);
    miny = (miny - strokew).max(0.0);
    maxx = (maxx + strokew).min(self.width);
    maxy = (maxy + strokew).min(self.height);

    let precision = self.precision;
    let width = self.width;
    let minx = (minx / precision) as usize;
    let miny = (miny / precision) as usize;
    let maxx = (maxx / precision) as usize;
    let maxy = (maxy / precision) as usize;
    let wi = (width / precision) as usize;
    for x in minx..maxx {
      for y in miny..maxy {
        let point = (x as f64 * precision, y as f64 * precision);
        for i in 0..polyline.len() - 1 {
          let a = polyline[i];
          let b = polyline[i + 1];
          if sd_segment(point, a, b) < strokew {
            self.mask[x + y * wi] = true;
            break;
          }
        }
      }
    }
  }
}

// TODO we can optim something as we just need a "point_in_segment"

fn sd_segment(
  (px, py): (f64, f64),
  (ax, ay): (f64, f64),
  (bx, by): (f64, f64),
) -> f64 {
  let pa_x = px - ax;
  let pa_y = py - ay;
  let ba_x = bx - ax;
  let ba_y = by - ay;

  let dot_pa_ba = pa_x * ba_x + pa_y * ba_y;
  let dot_ba_ba = ba_x * ba_x + ba_y * ba_y;
  let h = (dot_pa_ba / dot_ba_ba).max(0.0).min(1.0);

  let h_x = ba_x * h;
  let h_y = ba_y * h;

  ((pa_x - h_x) * (pa_x - h_x) + (pa_y - h_y) * (pa_y - h_y)).sqrt()
}

// homemade implementation of a filling technique that will spawn random worms that eat the space to colorize it!
struct WormsFilling {
  rot: f64,
  step: f64,
  straight: f64,
  min_l: usize,
  max_l: usize,
  decrease_value: f64,
  search_max: usize,
  min_weight: f64,
  freq: f64,
  seed: f64,
}
impl WormsFilling {
  // new
  fn rand<R: Rng>(rng: &mut R) -> Self {
    let seed = rng.gen_range(-999., 999.);
    let rot = PI / rng.gen_range(1.0, 2.0);
    let step = 0.4;
    let straight = rng.gen_range(0.0, 0.1);
    let min_l = 5;
    let max_l = 20;
    let decrease_value = 1.;
    let search_max = 500;
    let min_weight = 1.;
    let freq = 0.05;
    Self {
      rot,
      step,
      straight,
      min_l,
      max_l,
      decrease_value,
      search_max,
      min_weight,
      freq,
      seed,
    }
  }

  fn fill_in_paint<R: Rng>(
    &self,
    rng: &mut R,
    drawings: &PaintMask,
    clr: usize,
    density: f64,
    bound: (f64, f64, f64, f64),
    iterations: usize,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    let f = |x, y| {
      if drawings.is_painted((x, y)) {
        density
      } else {
        0.0
      }
    };
    let coloring = |_: &Vec<(f64, f64)>| clr;
    self.fill(rng, &f, bound, &coloring, iterations)
  }

  fn fill<R: Rng>(
    &self,
    rng: &mut R,
    f: &dyn Fn(f64, f64) -> f64,
    bound: (f64, f64, f64, f64),
    clr: &dyn Fn(&Vec<(f64, f64)>) -> usize,
    iterations: usize,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    let mut routes = vec![];
    let perlin = Perlin::new();
    let w = bound.2 - bound.0;
    let h = bound.3 - bound.1;
    let precision = 0.4;
    if w <= 2. * precision || h <= 2. * precision {
      return routes;
    }
    let mut map = WeightMap::new(w, h, 0.4);

    map.fill_fn(&|p| f(p.0 + bound.0, p.1 + bound.1));

    let seed = self.seed;
    let rot = self.rot;
    let step = self.step;
    let straight = self.straight;
    let min_l = self.min_l;
    let max_l = self.max_l;
    let decrease_value = self.decrease_value;
    let search_max = self.search_max;
    let min_weight = self.min_weight;
    let freq = self.freq;

    let mut bail_out = 0;

    for _i in 0..iterations {
      let top = map.search_weight_top(rng, search_max, min_weight);
      if top.is_none() {
        bail_out += 1;
        if bail_out > 10 {
          break;
        }
      }
      if let Some(o) = top {
        let angle = perlin.get([seed, freq * o.0, freq * o.1]);

        if let Some(a) = map.best_direction(o, step, angle, PI, PI / 4.0, 0.0) {
          let route = map.dig_random_route(
            o,
            a,
            step,
            rot,
            straight,
            max_l,
            decrease_value,
          );
          if route.len() >= min_l {
            let points: Vec<(f64, f64)> = rdp(&route, 0.05);
            // remap
            let rt = points
              .iter()
              .map(|&p| (p.0 + bound.0, p.1 + bound.1))
              .collect::<Vec<_>>();
            let c = clr(&rt);
            routes.push((c, rt));
          }
        }
      }
    }

    routes
  }
}

// data model that stores values information in 2D
struct WeightMap {
  weights: Vec<f64>,
  w: usize,
  h: usize,
  width: f64,
  height: f64,
  precision: f64,
}
impl WeightMap {
  fn new(width: f64, height: f64, precision: f64) -> WeightMap {
    let w = ((width / precision) + 1.0) as usize;
    let h = ((height / precision) + 1.0) as usize;
    let weights = vec![0.0; w * h];
    WeightMap {
      weights,
      w,
      h,
      width,
      height,
      precision,
    }
  }
  fn fill_fn(&mut self, f: &impl Fn((f64, f64)) -> f64) {
    for y in 0..self.h {
      for x in 0..self.w {
        let p = (x as f64 * self.precision, y as f64 * self.precision);
        let v = f(p);
        self.weights[y * self.w + x] = v;
      }
    }
  }

  // do a simple bilinear interpolation
  fn get_weight(&self, p: (f64, f64)) -> f64 {
    let x = p.0 / self.precision;
    let y = p.1 / self.precision;
    let x0 = x.floor() as usize;
    let y0 = y.floor() as usize;
    let x1 = (x0 + 1).min(self.w - 1);
    let y1 = (y0 + 1).min(self.h - 1);
    let dx = x - x0 as f64;
    let dy = y - y0 as f64;
    let w00 = self.weights[y0 * self.w + x0];
    let w01 = self.weights[y0 * self.w + x1];
    let w10 = self.weights[y1 * self.w + x0];
    let w11 = self.weights[y1 * self.w + x1];
    let w0 = w00 * (1.0 - dx) + w01 * dx;
    let w1 = w10 * (1.0 - dx) + w11 * dx;
    w0 * (1.0 - dy) + w1 * dy
  }

  // apply a gaussian filter to the weights around the point p with a given radius
  fn decrease_weight_gaussian(
    &mut self,
    p: (f64, f64),
    radius: f64,
    value: f64,
  ) {
    let x = p.0 / self.precision;
    let y = p.1 / self.precision;
    let x0 = ((x - radius).floor().max(0.) as usize).min(self.w);
    let y0 = ((y - radius).floor().max(0.) as usize).min(self.h);
    let x1 = ((x + radius).ceil().max(0.) as usize).min(self.w);
    let y1 = ((y + radius).ceil().max(0.) as usize).min(self.h);
    if x0 >= self.w || y0 >= self.h {
      return;
    }
    for y in y0..y1 {
      for x in x0..x1 {
        let p = (x as f64 * self.precision, y as f64 * self.precision);
        let d = (p.0 - p.0).hypot(p.1 - p.1);
        if d < radius {
          let w = self.weights[y * self.w + x];
          let v = value * (1.0 - d / radius);
          self.weights[y * self.w + x] = w - v;
        }
      }
    }
  }

  // find the best direction to continue the route by step
  // returns None if we reach an edge or if there is no direction that can be found in the given angle += max_angle_rotation and when the weight is lower than 0.0
  fn best_direction(
    &self,
    p: (f64, f64),
    step: f64,
    angle: f64,
    max_ang_rotation: f64,
    angle_precision: f64,
    straight_factor: f64,
  ) -> Option<f64> {
    let mut best_ang = None;
    let mut best_weight = 0.0;
    let mut a = -max_ang_rotation;
    while a < max_ang_rotation {
      let ang = a + angle;
      let dx = step * ang.cos();
      let dy = step * ang.sin();
      let np = (p.0 + dx, p.1 + dy);
      if np.0 < 0.0 || np.0 > self.width || np.1 < 0.0 || np.1 > self.height {
        a += angle_precision;
        continue;
      }
      // more important when a is near 0.0 depending on straight factor
      let wmul = (1.0 - straight_factor)
        + (1.0 - a.abs() / max_ang_rotation) * straight_factor;
      let weight = self.get_weight(np) * wmul;
      if weight > best_weight {
        best_weight = weight;
        best_ang = Some(ang);
      }
      a += angle_precision;
    }
    return best_ang;
  }

  // FIXME we could optim this by keeping track of tops and not searching too random
  fn search_weight_top<R: Rng>(
    &mut self,
    rng: &mut R,
    search_max: usize,
    min_weight: f64,
  ) -> Option<(f64, f64)> {
    let mut best_w = min_weight;
    let mut best_p = None;
    for _i in 0..search_max {
      let x = rng.gen_range(0.0, self.width);
      let y = rng.gen_range(0.0, self.height);
      let p = (x, y);
      let w = self.get_weight(p);
      if w > best_w {
        best_w = w;
        best_p = Some(p);
      }
    }
    return best_p;
  }

  fn dig_random_route(
    &mut self,
    origin: (f64, f64),
    initial_angle: f64,
    step: f64,
    max_ang_rotation: f64,
    straight_factor: f64,
    max_length: usize,
    decrease_value: f64,
  ) -> Vec<(f64, f64)> {
    let mut route = Vec::new();
    let mut p = origin;
    let mut angle = initial_angle;
    for _i in 0..max_length {
      if let Some(ang) = self.best_direction(
        p,
        step,
        angle,
        max_ang_rotation,
        0.2 * max_ang_rotation,
        straight_factor,
      ) {
        angle = ang;
        let prev = p;
        p = (p.0 + step * angle.cos(), p.1 + step * angle.sin());
        route.push(p);
        self.decrease_weight_gaussian(prev, step, decrease_value);
      } else {
        break;
      }
    }

    route
  }
}

#[derive(Clone, Copy)]
pub struct Ink(&'static str, &'static str, &'static str, f64);
#[derive(Clone, Copy)]
pub struct Paper(&'static str, &'static str, bool);

pub fn rng_from_seed(s: f64) -> impl Rng {
  let mut bs = [0; 16];
  bs.as_mut().write_f64::<BigEndian>(s).unwrap();
  let mut rng = SmallRng::from_seed(bs);
  // run it a while to have better randomness
  for _i in 0..50 {
    rng.gen::<f64>();
  }
  return rng;
}

// adapted from library "ramer_douglas_peucker"
/// Given a set of points and an epsilon, returns a list of indexes to keep.
/// If the first and last points are the same, then the points are treated as a closed polygon
pub fn rdp(points: &Vec<(f64, f64)>, epsilon: f64) -> Vec<(f64, f64)> {
  if points.len() < 3 {
    return points.clone();
  }
  let mut ranges = Vec::<RangeInclusive<usize>>::new();

  let mut results = Vec::new();
  results.push(0); // We always keep the starting point

  // Set of ranges to work through
  ranges.push(0..=points.len() - 1);

  while let Some(range) = ranges.pop() {
    let range_start = *range.start();
    let range_end = *range.end();

    let start = points[range_start];
    let end = points[range_end];

    // Caches a bit of the calculation to make the loop quicker
    let line = LineDistance::new(start, end);

    let (max_distance, max_index) =
      points[range_start + 1..range_end].iter().enumerate().fold(
        (0.0_f64, 0),
        move |(max_distance, max_index), (index, &point)| {
          let distance = match line.to(point) {
            Some(distance) => distance,
            None => {
              let base = point.0 - start.0;
              let height = point.1 - start.1;
              base.hypot(height)
            }
          };

          if distance > max_distance {
            // new max distance!
            // +1 to the index because we start enumerate()ing on the 1st element
            return (distance, index + 1);
          }

          // no new max, pass the previous through
          (max_distance, max_index)
        },
      );

    // If there is a point outside of epsilon, subdivide the range and try again
    if max_distance > epsilon {
      // We add range_start to max_index because the range needs to be in
      // the space of the whole vector and not the range
      let division_point = range_start + max_index;

      let first_section = range_start..=division_point;
      let second_section = division_point..=range_end;

      // Process the second one first to maintain the stack
      // The order of ranges and results are opposite, hence the awkwardness
      let should_keep_second_half = division_point - range_start > 2;
      if should_keep_second_half {
        ranges.push(second_section);
      }

      if division_point - range_start > 2 {
        ranges.push(first_section);
      } else {
        results.push(division_point);
      }

      if !should_keep_second_half {
        results.push(range_end);
      }
    } else {
      // Keep the end point for the results
      results.push(range_end);
    }
  }

  results.iter().map(|&i| points[i]).collect()
}

pub fn euclidian_dist((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> f64 {
  let dx = x1 - x2;
  let dy = y1 - y2;
  return (dx * dx + dy * dy).sqrt();
}

// adapted from "legasea_line"
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct LineDistance {
  a: f64,
  b: f64,
  c: f64,
  pub length: f64,
}

impl LineDistance {
  pub fn new(p1: (f64, f64), p2: (f64, f64)) -> Self {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let a = y2 - y1;
    let b = x2 - x1;
    let c = (x2 * y1) - (y2 * x1);
    let length = euclidian_dist(p1, p2);
    Self { a, b, c, length }
  }
  pub fn to(&self, point: (f64, f64)) -> Option<f64> {
    let Self { a, b, c, length } = self;
    if 0.0 == *length {
      None
    } else {
      // https://en.wikipedia.org/wiki/Distance_from_a_point_to_a_line#Line_defined_by_two_points
      Some(((a * point.0) - (b * point.1) + c).abs() / length)
    }
  }
}

pub fn layer(id: &str) -> Group {
  return Group::new()
    .set("inkscape:groupmode", "layer")
    .set("inkscape:label", id);
}

pub fn base_document(bg: &str, width: f64, height: f64) -> Document {
  Document::new()
    .set(
      "xmlns:inkscape",
      "http://www.inkscape.org/namespaces/inkscape",
    )
    .set("viewBox", (0, 0, width, height))
    .set("width", format!("{}mm", width))
    .set("height", format!("{}mm", height))
    .set("style", format!("background:{}", bg))
}

pub fn base_path(color: &str, stroke_width: f64, data: Data) -> Path {
  Path::new()
    .set("fill", "none")
    .set("stroke", color)
    .set("stroke-width", stroke_width)
    .set("d", data)
    .set("style", "mix-blend-mode: multiply;")
}

pub fn render_route(data: Data, route: Vec<(f64, f64)>) -> Data {
  let mut first = true;
  let mut d = data;
  for p in route {
    if first {
      first = false;
      d = d.move_to(p);
    } else {
      d = d.line_to(p);
    }
  }
  return d;
}
