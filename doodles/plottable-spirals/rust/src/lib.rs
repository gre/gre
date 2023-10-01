// @flow
/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Spirals
 */
mod utils;
use noise::*;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::f64::consts::PI;
use std::f64::INFINITY;
use svg::node::element::path::Data;
use svg::node::element::{Group, Path};
use svg::parser::Event;
use svg::Document;
use wasm_bindgen::prelude::*;

// Input to the art function
#[derive(Deserialize)]
pub struct Opts {
  pub hash: String,
  pub width: f64,
  pub height: f64,
  pub pad: f64,
}

// Feature tells caracteristics of a given art variant
// It is returned in the .SVG file

#[derive(Clone, Serialize)]
pub struct Feature {
  // which inks are used
  pub inks: String,
  // how much inks are used
  pub inks_count: usize,
  // which paper is used
  pub paper: String,
  // anything special
  pub special: String,
  // bold
  pub bold_each: String,
  // effect area
  pub effect_area: String,
}

#[derive(Clone, Copy, Serialize)]
pub struct Ink(&'static str, &'static str, &'static str, f64);

#[derive(Clone, Copy, Serialize)]
pub struct Paper(&'static str, &'static str, bool);

// This is also returned in the SVG to have more metadata for the JS side to render a digital version
#[derive(Clone, Serialize)]
pub struct Palette {
  pub primary: Ink,
  pub secondary: Ink,
  pub third: Ink,
  pub paper: Paper,
}

pub fn art(opts: &Opts, mask_mode: bool) -> (svg::Document, Feature) {
  let height = opts.height;
  let width = opts.width;
  let pad: f64 = opts.pad;
  let mut rng = rng_from_fxhash(&opts.hash);

  let mut has_amour = false;
  let dictionary = if rng.gen_bool(0.02) {
    has_amour = true;
    "amour amour amour amour"
  } else {
    "independence liberty autonomy determination free will choice flexibility openness diversity tolerance acceptance equality justice fairness creativity progress courage peace joy bliss mindfulness awareness truthfulness authenticity honesty integrity honor respect dignity worthiness gratitude compassion empathy kindness generosity altruism forgiveness harmony unity cooperation collaboration solidarity fraternity"
  };

  let gold_gel = Ink("Gold Gel", "#B92", "#DB4", 0.6);
  let silver_gel = Ink("Silver Gel", "#CCC", "#FFF", 0.6);

  let black = Ink("Black", "#111", "#000", 0.35);
  let brillant_red = Ink("Brillant Red", "#F32", "#931", 0.35);
  let seibokublue = Ink("Sailor Sei-boku", "#5a82bd", "#348", 0.35);
  let soft_mint = Ink("Soft Mint", "#00d6b1", "#0a7", 0.35);
  let turquoise = Ink("Turquoise", "#0AD", "#058", 0.35);
  let amber = Ink("Amber", "#FB2", "#F80", 0.35);
  let pink = Ink("Pink", "#ff87a2", "#ee8174", 0.35);
  let spring_green = Ink("Spring Green", "#783", "#350", 0.35);

  let white_paper = Paper("White", "#fff", false);
  let black_paper = Paper("Black", "#111", true);
  let red_paper = Paper("Red", "#aa0000", true);

  let perlin: Perlin = Perlin::new();

  // PAPER AND INKS

  let black_paper_chance = 0.1;
  let red_paper_chance = 0.05;
  let monochrome_chance = 0.33;

  let paper = if rng.gen_bool(red_paper_chance) {
    red_paper
  } else if rng.gen_bool(black_paper_chance) {
    black_paper
  } else {
    white_paper
  };

  let count = if paper.2 {
    rng.gen_range(1, 3)
  } else {
    rng.gen_range(1, 4)
  };

  let mut words = dictionary.split(" ").collect::<Vec<_>>();
  rng.shuffle(&mut words);
  words = words[0..count].to_vec();

  let mut colors = if paper.2 {
    vec![silver_gel, gold_gel]
  } else {
    vec![
      black,
      seibokublue,
      amber,
      soft_mint,
      pink,
      turquoise,
      brillant_red,
      spring_green,
    ]
  };

  rng.shuffle(&mut colors);
  let monochrome = rng.gen_bool(monochrome_chance);
  let ink_count = if monochrome { 2 } else { count };
  colors = colors[0..ink_count].to_vec();

  let third_to_first = ink_count == 3 && rng.gen_bool(0.5);
  if third_to_first {
    colors[2] = colors[0];
  }

  // TEXT STYLES

  let font_size = rng.gen_range(22.0, 32.0) * colors[0].3;
  let word_dist =
    (0.04 + rng.gen_range(-0.1, 0.6) * rng.gen_range(0.0, 1.0)) * font_size;
  let line_dist = (0.3 + 0.3 * rng.gen_range(0f64, 1.0).powi(3)
    - 0.15 * rng.gen_range(0f64, 1.0).powi(5))
    * font_size;

  // VALUE FUNCTIONS

  let color_anomaly = 0.0008;
  let repeat_divisor = (1.0
    + rng.gen_range(0.0, 4.0)
      * rng.gen_range(0.0, 1.0)
      * rng.gen_range(0.0, 1.0)) as usize;
  let color_word_attached = rng.gen_bool(0.2);
  let color_seed = rng.gen_range(0.0, 100000.0);
  let color_freq = 0.5 + rng.gen_range(0.0, 16.0) * rng.gen_range(0.0, 1.0);
  let color_field = rng.gen_bool(0.7);

  let vsplit = rng.gen_bool(0.3);
  let hsplit = rng.gen_bool(0.3);

  let mut has_concentric = false;

  let mut concentric_color_add = vec![];
  if rng.gen_bool(0.4) {
    concentric_color_add.push((0.0, rng.gen_range(0.0, 0.2)));
    has_concentric = true;
  }

  if rng.gen_bool(0.3) {
    concentric_color_add
      .push((rng.gen_range(0.2, 0.3), rng.gen_range(0.3, 0.4)));
    has_concentric = true;
  }
  if rng.gen_bool(0.4) {
    concentric_color_add.push((rng.gen_range(0.35, 0.45), 0.5));
    has_concentric = true;
  }

  let clr_mod = ink_count + if rng.gen_bool(0.1) { 1 } else { 0 };

  let color_fn = |rng: &mut StdRng, pos: (f64, f64), i: usize| -> usize {
    if rng.gen_bool(color_anomaly) {
      return rng.gen_range(0, clr_mod);
    }
    if monochrome {
      return 0;
    }

    let mut color = 0;
    if color_word_attached {
      color = (i / repeat_divisor) % clr_mod;
    } else if color_field {
      let v = perlin.get([
        color_seed + pos.0 * color_freq / width,
        color_seed + pos.1 * color_freq / width,
      ]);
      let v = (v + 0.5) * (clr_mod as f64);
      color = v.floor() as usize % clr_mod;
    }

    if concentric_color_add.len() > 0 {
      let dist_center =
        ((pos.0 - width / 2.0).powi(2) + (pos.1 - height / 2.0).powi(2)).sqrt()
          / width;
      for &(from, to) in concentric_color_add.iter() {
        if dist_center > from && dist_center < to {
          color += 1;
        }
      }
    }

    if vsplit {
      if pos.0 < width / 2.0 {
        color += 1;
      }
    }
    if hsplit {
      if pos.1 < height / 2.0 {
        color += 1;
      }
    }

    color % clr_mod
  };

  let mut concentric_rays_bold = vec![];
  if rng.gen_bool(0.1) {
    concentric_rays_bold.push((0.43, 0.5));
  }

  let bold_activate = !paper.2 && rng.gen_bool(0.2);
  let bold_mod = rng.gen_range(3, 7);

  let bold_fn = |_rng: &mut StdRng, _pos: (f64, f64), i: usize| -> bool {
    return bold_activate && (i % bold_mod == 0);
  };

  let mut routes = Vec::new();

  let svg_content = r###"
<svg width="210mm" height="20mm" viewBox="0 0 210 20" version="1.1" xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape" xmlns="http://www.w3.org/2000/svg" xmlns:svg="http://www.w3.org/2000/svg">
<path inkscape:label="a&gt;" d="m 21.76293,6.7007898 c -0.587829,0.00242 -1.320241,0.3596935 -1.807709,1.1427253 -0.686102,1.1021155 -1.125714,3.5069959 -0.128259,4.5641509 1.190651,1.261964 1.761388,-0.925892 1.916858,-1.671895 0.08948,-0.429251 0.111347,-4.2847041 0.183267,-4.4265355 -0.09348,1.7557402 0.05813,3.1990452 0.293299,4.8591275 0.09373,0.426436 0.345945,1.247769 1.088507,1.143109 0.298652,-0.04208 0.440622,-0.231233 0.605042,-0.43484" />
<path inkscape:label="b&lt;&gt;" d="m 25.737839,10.796537 c 0.620919,-0.572669 1.336395,-1.2246554 1.885216,-2.0137173 0.562711,-1.1302695 0.870691,-3.1925042 0.870751,-4.5065002 0,-0.8259429 0.01579,-1.7372536 -7.57e-4,-2.5542369 -0.0048,-0.2352586 -0.016,-0.5298614 -0.274789,-0.5227914 -0.2387,0.00684 -0.374712,0.2499894 -0.467642,0.4561901 -0.740994,1.7849729 -0.748637,4.3869162 -0.97882,6.6776543 0.05775,1.5635038 0.09351,2.8566544 0.7752,4.3451474 0.237882,0.339524 0.800708,0.923791 1.262034,0.654379 0.425772,-0.248647 0.475812,-1.021993 0.403372,-1.460799 -0.188614,-1.141504 -0.915579,-1.715965 0.222898,-2.6215334" />
<path inkscape:label="c&gt;" d="m 34.340922,8.3846091 c -0.749161,-0.2562348 -1.293221,0.185729 -1.697813,0.8066315 -0.895015,1.3736754 -1.480694,4.1425484 0.958409,3.9565574 0.870982,-0.07425 1.600192,-0.647494 2.28543,-1.267816" />
<path inkscape:label="d&gt;" d="m 41.500549,7.8902989 c -0.692421,-0.036313 -1.467352,0.218311 -2.000632,0.6996245 -0.874318,0.8202022 -1.59255,2.9115796 -0.827598,4.0043516 0.428585,0.612304 1.27199,0.598034 1.798639,0.06669 0.585302,-0.590519 1.12499,-1.789695 1.096807,-2.8211667 -0.0457,-1.8265972 0.166227,-3.584172 0.05385,-5.329285 0.06391,0.2336029 -0.02152,6.9577107 0.05207,7.3457657 0.05599,0.294949 0.167043,0.734834 0.373274,1.00243 0.467772,0.606989 1.077718,0.563089 1.604575,0.07295" />
<path inkscape:label="e&lt;&gt;" d="m 46.367086,11.744509 c 1.20163,-0.58801 2.396155,-2.0180797 2.550359,-3.2273876 0.04007,-0.3871168 -0.06184,-0.7948669 -0.397792,-1.0225988 -0.470739,-0.3191536 -0.954918,0.1433123 -1.25256,0.4850464 -0.832559,1.3042379 -1.154092,2.90265 -0.738194,4.369291 0.30902,0.808728 1.176577,1.204 1.930257,1.040407 0.585267,-0.137188 1.109914,-0.646914 1.546026,-1.04068" />
<path inkscape:label="f&lt;&gt;" d="m 52.200266,13.147766 c 1.989953,-0.305777 3.290677,-3.2993706 3.587091,-4.6379135 0.187901,-1.488931 0.310969,-2.7429633 -0.173041,-4.0258319 -0.0642,-0.1266545 -0.135644,-0.2660288 -0.324935,-0.2690588 -0.213222,-0.00383 -0.309454,0.2266321 -0.345174,0.3310289 -0.316017,1.9567767 -0.39508,3.6647153 -0.509786,5.5768513 -0.06479,1.352898 -0.18292,2.724132 -0.209538,4.100382 -0.01685,1.539305 -0.134908,3.557685 0.687645,5.041398 0.326822,0.589442 1.109712,1.814912 1.922589,1.482713 0.765153,-0.312696 0.579626,-1.595102 0.382546,-2.222118 -0.352902,-1.122755 -0.67716,-2.28335 -1.296318,-3.29371 -0.419375,-0.684496 -1.03819,-0.981998 -1.637247,-1.075507 0.07276,-0.739984 0.0063,-1.012713 0.604999,-1.242198 1.124849,-0.526148 2.358077,-1.231732 3.495312,-1.379134" />
<path inkscape:label="g&gt;" d="m 63.25505,8.2864844 c -0.671862,0.01919 -1.510513,0.7629609 -2.079442,1.3665839 -0.60564,0.7257627 -0.897366,1.8182727 -0.6984,2.7559067 0.122337,0.578345 0.60363,1.127792 1.228759,0.870952 1.314699,-0.844342 1.548423,-2.704506 1.697268,-3.9630134 0.05579,-0.213224 -0.04418,-1.1545177 -0.01122,-1.3906098 0.111662,0.302267 0.63361,3.6562842 0.6652,3.9441812 0.144991,1.957607 -0.05163,4.121998 -0.49939,6.184859 -0.126373,0.440623 -0.409355,1.824013 -1.041891,1.762073 -0.743439,-0.0728 -0.315948,-1.807042 -0.231898,-2.2326 0.4446,-2.218923 1.06269,-4.990804 3.18615,-6.318567" />
<path inkscape:label="h&lt;&gt;" d="m 67.250952,8.5091546 c 1.576333,-1.011771 2.239778,-3.6870181 2.347429,-5.1758413 -0.05307,-0.9575887 0.01918,-2.54990572 -0.78841,-2.95339064 -0.396342,0.27187898 -0.3289,0.83011094 -0.37844,1.27292114 -0.207191,2.3502239 0.11622,4.4961003 0.328517,6.9234977 -0.0014,1.5611005 0.05614,2.5607055 0.153996,3.8333645 0.502499,4.10638 -1.234049,-6.2474579 0.97168,-4.2686792 0.79792,1.4427074 0.24373,3.4396712 0.955785,4.7326052 0.490091,0.83345 1.332067,-0.158366 1.517594,-0.667485" />
<path inkscape:label="i&lt;&gt;" d="m 75.469826,4.1742661 0.01821,-0.5388624 m -1.881019,8.7053783 c 0.770345,-0.933682 1.796261,-2.3446883 1.742143,-3.4208573 0.02462,-0.4646149 0.07282,-1.3827291 0.07282,-1.9566339 0,0 -0.04011,3.5691462 -0.0038,3.9659382 0.05992,0.655177 0.29637,3.584232 1.430882,1.949074 0.321774,-0.445143 0.466582,-0.99564 0.589041,-1.478617" />
<path inkscape:label="j&lt;&gt;" d="m 80.6917,3.6705161 0.0019,0.4686376 m -2.292374,9.2152393 c 0.943214,-0.02871 1.752327,-1.216319 2.118668,-2.016564 0.600365,-1.3463082 0.442069,-2.7109403 0.167288,-4.1675184 0.626026,1.3975616 1.262391,3.3527724 1.586495,5.0414074 0.03925,0.253682 0.22819,3.010932 0.22832,3.360939 4e-4,1.204353 0.0756,2.434323 -0.0895,3.629806 -0.07091,0.51355 -0.438129,2.133118 -1.180419,1.97563 -0.39242,-0.08322 -0.40433,-0.713334 -0.44701,-1.034565 -0.146732,-1.104072 0.04108,-2.219796 0.288775,-3.293722 0.372553,-1.6153 1.545889,-3.99887 1.779342,-4.643906" />
<path inkscape:label="k&lt;&gt;" d="m 84.474727,11.244409 c 2.226502,-1.6739484 3.013011,-4.7031476 2.957621,-7.326836 0.0089,-0.5928014 -0.02037,-1.6872641 -0.651122,-1.6895841 -0.605167,-0.00223 -0.611917,2.9208065 -0.692887,4.2131435 -0.07629,1.2180071 0.275477,2.50013 -0.06759,3.6605626 0.157236,1.317218 0.205593,2.64587 0.335534,3.965869 -0.03316,-1.620262 -1.252139,-4.8592148 1.344936,-5.799659 0.330925,-0.1198336 1.058095,-0.1236175 1.032275,0.5568333 -0.07188,0.9534414 -1.235395,1.9675757 -1.911087,2.8108847 -0.790722,0.767243 -0.789809,0.517579 0.07169,1.123394 0.452865,0.318459 1.717034,0.479336 2.352586,0.501926" />
<path inkscape:label="l&lt;&gt;" d="m 90.864037,13.26787 c 1.298586,-1.171605 1.881342,-2.405165 2.568025,-3.8986824 0.500917,-1.086798 0.757092,-2.2350216 0.857583,-3.4281644 0.07336,-0.8708819 0.168542,-1.8198772 0.02866,-2.6887411 -0.0455,-0.2829319 -0.171543,-0.7751518 -0.559528,-0.7037178 -0.715875,0.1318203 -0.907184,1.6869875 -1.027573,2.2497437 -0.290993,1.3601051 -0.580189,2.7748552 -0.652019,4.1675586 -0.197899,1.8678464 -0.1399,4.5137154 1.87642,4.5681564 0.329849,-0.0021 0.569748,-0.106001 0.873848,-0.198967" />
<path inkscape:label="m&lt;&gt;" d="m 96.793364,10.130718 c 0.124732,0.402788 0.526091,3.379858 0.519191,3.322628 -0.03342,-0.277109 -0.07264,-0.590868 -0.09992,-0.886664 -0.08946,-0.956998 -0.462495,-3.20444 0.533185,-3.891569 0.343894,-0.080918 0.527,0.5150326 0.639519,0.7989623 0.38229,0.9666587 0.571952,2.6749487 0.722302,3.6757527 -0.04032,-0.650536 -0.06733,-1.033111 -0.0918,-1.658644 -0.04786,-0.866569 -0.141116,-2.036794 0.443911,-2.943583 0.551198,-0.6250692 0.964398,0.2893081 1.121608,0.6579406 0.52979,1.2199574 0.56269,2.3758584 0.58259,3.9371564 l -0.0674,-2.256526 c -0.0402,-0.618307 0.0633,-3.0067409 1.06971,-2.4339728 0.29982,0.1706262 0.25752,1.3269453 0.28042,1.7783128 0.12193,1.38517 -0.34603,3.807754 1.12986,3.24603" />
<path inkscape:label="n&lt;&gt;" d="m 106.42757,9.6448948 c 0,-0.428726 0.46668,4.4817782 0.42398,4.0774592 -0.0336,-0.325672 -0.10621,-1.91031 -0.12561,-2.22107 -0.0222,-0.357998 -0.0626,-0.530419 -0.0707,-0.941054 -0.0619,-0.603646 0.13223,-2.1914015 0.73298,-2.1438732 1.16033,0.6085121 1.19816,2.1366082 1.21472,3.0848872 l 0.0672,1.478819 c 0.27087,-1.220449 -0.5021,-4.0505406 0.73667,-4.7893595 0.54233,0.1557653 0.43628,0.955706 0.46919,1.4284618 0.0747,1.0533817 0.14587,2.2875887 0.44498,3.2937217 0.24265,0.816903 1.07752,1.098351 1.64283,0.470528" />
<path inkscape:label="o&gt;" d="m 114.66718,10.270122 c -0.15881,0.237144 -0.25052,0.512386 -0.31062,0.789516 -0.16834,0.867855 -0.0928,1.807257 0.31194,2.601649 0.16969,0.248437 0.36485,0.684544 0.72465,0.615954 0.40668,-0.115391 0.60753,-0.531711 0.8033,-0.870501 0.39724,-0.778042 0.68244,-1.6298 0.74184,-2.505263 -0.0234,-0.377261 -0.0882,-0.752505 -0.12796,-1.128968 -0.0608,-0.298722 -0.21646,-0.5738657 -0.35951,-0.8415977 -0.22915,-0.3933303 -0.58768,-0.8255262 -1.08767,-0.8067922 -0.53024,0.03917 -0.85444,0.5525521 -1.02649,1.0035333 -0.19131,0.4318242 -0.11543,0.9122926 7.6e-4,1.3519906 0.0326,0.275085 0.16947,0.554487 0.48099,0.575407 0.64077,0.145032 1.27612,-0.118073 1.83418,-0.410267 0.84683,-0.424603 1.53516,-1.0850998 2.30382,-1.6264084" />
<path inkscape:label="p" d="m 120.86921,12.455715 c 1.11981,-0.160397 3.00341,-1.351772 2.68384,-2.331769 -0.48261,-1.196932 -1.91802,-2.1018379 -3.1145,-2.6253328 -0.0964,1.3623849 0.41869,5.5154138 0.59594,7.4257738 0.0757,0.757082 0.11837,1.945987 0.18755,2.688752" />
<path inkscape:label="q" d="m 128.59015,7.6595504 c -0.98131,-0.00179 -1.75084,0.5410548 -2.23106,1.4115922 -0.4339,0.8119987 -1.07093,2.5719234 -0.41164,3.4155234 1.44745,1.042634 2.82537,-1.365963 2.77982,-2.272799 0.0467,-0.820181 -0.10363,-1.7452662 -0.13824,-2.2854395 0.49788,3.1832255 0.003,6.7812745 0.34037,9.7466785 0.0698,0.577881 0.056,1.199116 0.39942,1.613254" />
<path inkscape:label="r" d="m 131.91381,13.11131 c -0.10211,-0.408426 -0.11821,-1.033283 -0.14881,-1.411592 -0.0793,-0.947778 -0.19694,-2.1487115 -0.10514,-3.2264753 0.0747,-0.7324515 0.43918,-1.4651278 1.32629,-1.4248298 0.72206,0.1896841 0.68604,1.4128636 0.49997,1.9625837 -0.28672,0.7846262 -1.05268,1.3927854 -1.6394,1.9493474 0.89678,0.630302 2.02195,1.359967 3.02486,2.016553" />
<path inkscape:label="s&lt;" d="m 136.41883,11.762292 c 0.56827,-0.641132 1.10669,-1.40111 1.52257,-2.1510067 0.46013,-0.8368244 0.97255,-2.0213397 0.67833,-3.1451932 -0.27895,-0.4742303 -0.7352,-0.088465 -0.6539,0.3216355 0.39445,1.539324 2.64405,2.4340218 3.60088,3.7647234 1.36705,1.692153 -1.26672,3.5048 -2.56883,2.21217 -0.60799,-0.603567 0.17068,-1.505436 0.58043,-2.077798" />
<path inkscape:label="t&lt;&gt;" d="m 144.57835,6.1913118 c 1.44269,-0.4205189 2.52583,-0.8455782 4.03313,-1.3443753 M 144.34156,12.3211 c 0.79881,-1.241109 1.69369,-2.5614163 1.94706,-3.764249 0.13577,-1.9020856 0.0522,-3.9753835 0.002,-5.5791525 0.36022,2.0351336 -0.44653,9.1507855 0.5424,10.1989525 1.06333,1.035676 1.65183,-0.275907 2.14614,-1.124479" />
<path inkscape:label="uR" d="m 152.41954,8.2671834 c 0,1.3426999 0.0168,2.6386046 0.24064,3.9659086 0.0881,0.521622 0.20635,1.208864 0.83486,1.31782 0.71117,0.123346 1.02642,-0.916527 1.14204,-1.452253 0.3119,-1.451991 0.40734,-3.5108994 -0.33542,-4.8397573" />
<path inkscape:label="v" d="m 157.97614,7.8963148 c 0.90985,1.4124802 1.43255,3.0252012 1.9163,4.6215292 0.0632,0.203173 0.22519,0.463864 0.47403,0.409168 0.19193,-0.09876 0.27725,-0.33661 0.35814,-0.528557 0.21914,-0.671866 0.0328,-1.383956 -0.12139,-2.048507 -0.11917,-0.5386498 -0.30468,-1.0917967 -0.20864,-1.6487975 0.055,-0.2619347 0.34532,-0.3611408 0.55544,-0.4724326 0.13046,-0.062774 0.2652,-0.1187459 0.38692,-0.1979695" />
<path inkscape:label="w" d="m 163.39899,8.4949212 c 1.02732,1.1100855 1.1482,2.7085548 1.97844,3.8825598 1.32704,0.858652 0.0569,-3.5824626 -0.0291,-4.0169931 0.38237,1.1187655 1.45362,3.6713281 2.4871,4.0331161 0.0788,-1.083167 -0.84489,-3.2475756 -0.1957,-4.2220032 0.29651,-0.4073566 0.81269,-0.4429627 1.27125,-0.5505276" />
<path inkscape:label="x" d="m 171.27855,7.3527661 c 1.31381,1.9804071 1.88706,3.1413729 2.89027,5.3774689 m -3.09206,2.419873 c 1.10416,-2.525856 2.35294,-5.2893724 3.56272,-7.3940313" />
<path inkscape:label="y&gt;" d="m 178.20595,7.9352949 c 0.20789,1.3257216 -0.019,4.5506611 0.87651,4.9909801 1.14117,0.474023 0.86934,-4.6498432 0.80394,-5.2598674 -0.16558,3.1179724 2.38685,7.5106894 1.75373,11.2927474 -0.11365,0.678884 -0.65337,1.648246 -1.4852,1.587706 -0.96822,-0.07058 -1.0773,-1.386055 -1.0753,-2.12545 0.0301,-2.057913 1.3534,-4.176914 3.02373,-5.723483" />
<path inkscape:label="z&lt;&gt;" d="m 183.52385,8.4559897 c 1.02886,0.4318463 2.79657,-0.7739886 3.65481,0.2198017 0.80183,0.9284727 -2.2037,2.1409126 -2.49274,3.2097046 -0.33917,1.434158 3.65852,1.707942 4.00104,3.359586 0.24377,0.974655 -2.79878,4.826582 -4.60757,4.737582 -0.88919,-0.122837 0.98408,-4.02731 1.47823,-4.872025 1.19962,-2.05068 2.21313,-3.920801 3.94857,-5.5119137" />
</svg>
"###;

  // add text
  let non_attached_pad = 0.0;
  let extra_pad = 1.0;
  let letters_ref: LetterSvgReferential =
    LetterSvgReferential::new(svg_content, 0.1, non_attached_pad, extra_pad);

  let mut unsafe_curves = Vec::new();

  let mut spiral = spiral_optimized(
    width / 2.0,
    height / 2.0,
    width / 2.0 - pad,
    line_dist,
    0.1,
  );

  let has_thread = if rng.gen_bool(0.02) {
    spiral = vec![vec![(width - pad, pad)], spiral].concat();
    true
  } else {
    false
  };

  unsafe_curves.push(spiral);

  let curves = unsafe_curves.clone();

  // offset text exactly on the curve line
  let yoffset = -font_size * 0.5;

  let mut queue = VecDeque::new();
  for word in words {
    queue.push_back(word.to_string());
  }

  let mut total_words = 0;

  let mut offsets = vec![];

  let b = 0.01 * font_size;
  for i in 0..3 {
    let angle = i as f64 * 2.0 * PI / 3.0;
    let offset = (angle.cos() * b, angle.sin() * b);
    offsets.push(offset);
  }

  for c in curves.clone() {
    let length = curve_length(&c);
    let extrapad = word_dist;
    let mut subset = vec![];
    let mut sum = extrapad;
    let mut sum_words = 0.0;
    // we try to pull as much word as we can to fill the curve
    while let Some(word) = queue.pop_front() {
      let text = word.clone();
      let measure = measure_text(&letters_ref, text.clone(), font_size);
      if sum + measure + word_dist < length {
        sum += measure + word_dist;
        sum_words += measure;
        subset.push(text);
        queue.push_back(word.clone());
      } else {
        queue.push_front(word.clone());
        break;
      }
    }
    if subset.len() == 0 {
      continue;
    }
    // we will equilibrate the padding for all the words to smoothly occupy the curve
    let pad = (length - sum_words) / (subset.len() as f64);
    let mut xstart = 0.0;
    for text in subset {
      xstart += pad / 2.0;
      let res =
        draw_text(&letters_ref, text.clone(), font_size, xstart, yoffset, &c);

      if res.0.len() == 0 {
        continue;
      }

      let pos = calc_text_center(&res.0);

      let clr_index = color_fn(&mut rng, pos, total_words);
      let bold = bold_fn(&mut rng, pos, total_words);

      let rts = res.0;

      if bold {
        for offset in offsets.iter() {
          routes.extend(
            rts
              .iter()
              .map(|r| {
                (
                  clr_index,
                  r.iter().map(|p| (p.0 + offset.0, p.1 + offset.1)).collect(),
                )
              })
              .collect::<Vec<_>>(),
          );
        }
      } else {
        routes.extend(
          rts
            .iter()
            .map(|r| (clr_index, r.clone()))
            .collect::<Vec<_>>(),
        );
      }

      total_words += 1;

      xstart += res.1 + pad / 2.0;
    }
  }

  let mut has_empty_word = false;

  let colors_count = colors.len();
  let mut color_presence = vec![false; colors_count];
  for (i, _) in routes.iter() {
    if *i < colors_count {
      color_presence[*i] = true;
    } else {
      has_empty_word = true;
    }
  }
  let mut inks = vec![];
  for (i, &present) in color_presence.iter().enumerate() {
    if present && !inks.contains(&colors[i].0) {
      inks.push(colors[i].0);
    }
  }

  inks.sort();
  let inks_length = inks.len();

  let mut specials = vec![];
  if has_thread {
    specials.push("Thread");
  }
  if has_amour {
    specials.push("Amour");
  }
  if has_empty_word {
    specials.push("Empty Colors");
  }

  let mut area_effects = vec![];
  if inks_length > 1 {
    if vsplit {
      area_effects.push("V-Split");
    }
    if hsplit {
      area_effects.push("H-Split");
    }
    if has_concentric {
      area_effects.push("Concentric");
    }
    if color_field {
      area_effects.push("Color Field");
    }
  }

  let feature = Feature {
    inks: inks.join(", "),
    inks_count: inks_length,
    paper: paper.0.to_string(),
    special: specials.join(", "),
    bold_each: if bold_activate {
      format!("{}", bold_mod)
    } else {
      "".to_string()
    },
    effect_area: area_effects.join(", "),
  };

  let feature_json = serde_json::to_string(&feature).unwrap();

  let palette_json = serde_json::to_string(&Palette {
    paper,
    primary: colors[0 % colors.len()],
    secondary: colors[1 % colors.len()],
    third: colors[2 % colors.len()],
  })
  .unwrap();

  let mask_colors = vec!["#0FF", "#F0F", "#FF0"];

  let layers = make_layers(
    colors
      .iter()
      .enumerate()
      .map(|(i, c)| {
        (
          if mask_mode { mask_colors[i] } else { c.1 },
          c.0.to_string(),
          c.3,
          routes
            .iter()
            .filter_map(
              |(ci, routes)| {
                if *ci == i {
                  Some(routes.clone())
                } else {
                  None
                }
              },
            )
            .collect(),
        )
      })
      .collect(),
  );

  let mut document = svg::Document::new()
    .set(
      "data-credits",
      "@greweb - 2023 - GREWEBARTHURCOLLAB".to_string(),
    )
    .set("data-hash", opts.hash.to_string())
    .set("data-traits", feature_json)
    .set("data-palette", palette_json)
    .set("viewBox", (0, 0, width, height))
    .set("width", format!("{}mm", width))
    .set("height", format!("{}mm", height))
    .set(
      "style",
      if mask_mode {
        "background:white".to_string()
      } else {
        format!("background:{}", paper.1)
      },
    )
    .set(
      "xmlns:inkscape",
      "http://www.inkscape.org/namespaces/inkscape",
    )
    .set("xmlns", "http://www.w3.org/2000/svg");
  for l in layers {
    document = document.add(l);
  }

  (document, feature)
}

#[wasm_bindgen]
pub fn render(val: &JsValue) -> String {
  let opts = val.into_serde().unwrap();
  let (doc, _) = art(&opts, true);
  let str = doc.to_string();
  return str;
}

fn render_route(data: Data, route: Vec<(f64, f64)>) -> Data {
  if route.len() == 0 {
    return data;
  }
  let first_p = route[0];
  let mut d =
    data.move_to((significant_mm(first_p.0), significant_mm(first_p.1)));
  for p in route {
    d = d.line_to((significant_mm(p.0), significant_mm(p.1)));
  }
  return d;
}

#[inline]
fn significant_mm(f: f64) -> f64 {
  (f * 100.0).floor() / 100.0
}

fn make_layers(
  data: Vec<(&str, String, f64, Vec<Vec<(f64, f64)>>)>,
) -> Vec<Group> {
  let layers: Vec<Group> = data
    .iter()
    .filter(|(_color, _label, _stroke_width, routes)| routes.len() > 0)
    .enumerate()
    .map(|(ci, (color, label, stroke_width, routes))| {
      let mut l = Group::new()
        .set("inkscape:groupmode", "layer")
        .set("inkscape:label", format!("{} {}", ci, label.clone()))
        .set("fill", "none")
        .set("stroke", color.clone())
        .set("stroke-linecap", "round")
        .set("stroke-width", *stroke_width);
      let opacity: f64 = 0.7;
      let opdiff = 0.15 / (routes.len() as f64);
      let mut trace = 0f64;
      for route in routes.clone() {
        trace += 1f64;
        let data = render_route(Data::new(), route);
        l = l.add(
          Path::new()
            .set(
              "opacity",
              (1000. * (opacity - trace * opdiff)).floor() / 1000.0,
            )
            .set("d", data),
        );
      }
      l
    })
    .collect();
  layers
}

#[inline]
fn euclidian_dist((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> f64 {
  let dx = x1 - x2;
  let dy = y1 - y2;
  return (dx * dx + dy * dy).sqrt();
}

fn draw_text(
  letter_ref: &LetterSvgReferential,
  text: String,           // text to draw
  size: f64,              // font size
  xstart: f64,            // x move on the path
  yoffset: f64,           // make diff baseline
  path: &Vec<(f64, f64)>, // curve to follow
) -> (Vec<Vec<(f64, f64)>>, f64) {
  let mut routes = Vec::new();
  let mut x = 0.;
  let mut y = 0.;
  let mut prev_can_attach = false;
  let mut last: Vec<(f64, f64)> = vec![];
  for c in text.chars() {
    if let Some(letter) = letter_ref.get_letter(&c.to_string()) {
      let (rts, (dx, dy)) = letter.render((x, y), size, false);
      if prev_can_attach && letter.can_attach_left {
        let mut rts = rts.clone();

        let mut add = rts.pop().unwrap();
        // interpolate curve to attach more smoothly
        if last.len() > 0 {
          let lastp = last[last.len() - 1];
          let firstp = add[0];
          // ygap between last and first
          let ygap = firstp.1 - lastp.1;
          let mut i = 1;
          let mut maxlen = 0.5 * size;
          while i < add.len() {
            if maxlen < 0. {
              break;
            }
            let l = euclidian_dist(add[i - 1], add[i]);
            if ygap > 0.0 {
              if add[i].1 < lastp.1 {
                break;
              }
            } else {
              if add[i].1 > lastp.1 {
                break;
              }
            }
            i += 1;
            maxlen -= l;
          }
          if i == add.len() {
            i -= 1;
          }
          let stopi = i;
          add = add
            .iter()
            .enumerate()
            .map(|(i, &p)| {
              if i <= stopi {
                let y = p.1 - ygap * (1.0 - i as f64 / stopi as f64);
                (p.0, y)
              } else {
                p
              }
            })
            .collect();
        }

        last.extend(add);

        routes.extend(rts); // ° on i and j
      } else {
        if last.len() > 0 {
          routes.push(last);
          last = vec![];
        }
        routes.extend(rts);
      }
      prev_can_attach = letter.can_attach_right;
      x += dx;
      y += dy;
    } else {
      prev_can_attach = false;
      // println!("letter not found: {}", c);
    }
  }
  if last.len() > 0 {
    routes.push(last);
  }

  // rotate with angle and translate to origin all routes
  let mut proj_routes = Vec::new();
  for route in routes {
    let mut proj_route = Vec::new();
    for (x, y) in route {
      // use x to find position in path and project x,y
      let (origin, a) = lookup_curve_point_and_angle(&path, x + xstart);

      let y = y + yoffset;
      let disp = (-y * a.sin(), y * a.cos());

      let p = (origin.0 + disp.0, origin.1 + disp.1);

      proj_route.push(p);
    }
    proj_routes.push(proj_route);
  }

  (proj_routes, x)
}

fn angle2(p1: (f64, f64), p2: (f64, f64)) -> f64 {
  let (x1, y1) = p1;
  let (x2, y2) = p2;
  let dx = x2 - x1;
  let dy = y2 - y1;
  dy.atan2(dx)
}

fn curve_length(path: &Vec<(f64, f64)>) -> f64 {
  let mut len = 0.0;
  for i in 0..path.len() - 1 {
    len += euclidian_dist(path[i], path[i + 1]);
  }
  len
}

fn measure_text(
  letter_ref: &LetterSvgReferential,
  text: String,
  size: f64,
) -> f64 {
  let mut x = 0.;
  for c in text.chars() {
    if let Some(letter) = letter_ref.get_letter(&c.to_string()) {
      let (dx, _dy) = letter.render((x, 0.0), size, false).1;
      x += dx;
    }
  }
  x
}

fn lookup_curve_point_and_angle(
  path: &Vec<(f64, f64)>,
  l: f64,
) -> ((f64, f64), f64) {
  let mut i = 0;
  if l < 0.0 {
    return (path[0], angle2(path[0], path[1]));
  }
  let mut len = 0.0;
  while i < path.len() - 1 {
    let l1 = euclidian_dist(path[i], path[i + 1]);
    if len + l1 > l {
      let r = (l - len) / l1;
      let x = path[i].0 + r * (path[i + 1].0 - path[i].0);
      let y = path[i].1 + r * (path[i + 1].1 - path[i].1);
      let angle = angle2(path[i], path[i + 1]);
      return ((x, y), angle);
    }
    len += l1;
    i += 1;
  }
  return (
    path[path.len() - 1],
    angle2(path[path.len() - 2], path[path.len() - 1]),
  );
}

#[derive(Clone)]
struct Letter {
  pub routes: Vec<Vec<(f64, f64)>>,
  pub width: f64,
  pub height: f64,
  pub can_attach_left: bool,
  pub can_attach_right: bool,
}
impl Letter {
  fn new(
    routes: Vec<Vec<(f64, f64)>>,
    width: f64,
    height: f64,
    can_attach_left: bool,
    can_attach_right: bool,
  ) -> Letter {
    Letter {
      routes,
      width,
      height,
      can_attach_left,
      can_attach_right,
    }
  }

  fn render(
    &self,
    (x, y): (f64, f64),
    size: f64,
    vertical: bool,
  ) -> (Vec<Vec<(f64, f64)>>, (f64, f64)) {
    let mut routes = self.routes.clone();
    let w = self.width;
    let h = self.height;
    let ratio = w / h;
    let scale = size / h;

    for route in routes.iter_mut() {
      for p in route.iter_mut() {
        p.0 *= scale;
        p.1 *= scale;
        if vertical {
          *p = (h * scale - p.1, p.0);
        }
        p.0 += x;
        p.1 += y;
      }
    }
    let delta = if vertical {
      (0.0, ratio * size)
    } else {
      (ratio * size, 0.0)
    };
    (routes, delta)
  }
}

#[derive(Clone)]
struct LetterSvgReferential {
  letters: HashMap<String, Letter>,
}

impl LetterSvgReferential {
  fn new(
    content: &str,
    letter_precision: f64,
    non_attached_pad: f64,
    extra_pad: f64,
  ) -> LetterSvgReferential {
    let mut height = 0.0;
    let mut documents_per_layer: HashMap<String, String> = HashMap::new();

    for event in svg::read(content).unwrap() {
      match event {
        Event::Tag(_, _, attributes) => {
          if let Some(c) = attributes.get("inkscape:label") {
            if let Some(d) = attributes.get("d") {
              let data: String = d.to_string();
              let document =
                Document::new().add(Path::new().set("d", data)).to_string();
              documents_per_layer.insert(c.to_string(), document);
            }
          }

          if let Some(h) = attributes.get("height") {
            let mut hv = h.to_string();
            hv = hv.replace("mm", "");
            if let Some(h) = hv.parse::<f64>().ok() {
              height = h;
            }
          }
        }
        _ => {}
      }
    }

    let mut letters = HashMap::new();
    for (c, svg) in documents_per_layer.iter() {
      let polylines =
        svg2polylines::parse(svg.as_str(), letter_precision, true).unwrap();

      let mut minx = std::f64::INFINITY;
      let mut maxx = -std::f64::INFINITY;
      for poly in polylines.iter() {
        for p in poly.iter() {
          if p.x < minx {
            minx = p.x;
          }
          if p.x > maxx {
            maxx = p.x;
          }
        }
      }

      let mut width = maxx - minx;

      let mut dx = minx;

      let letter_name = c[0..1].to_string();
      // < : can attach left
      let can_attach_left = c.contains("&lt;");
      // > : can attach right
      let can_attach_right = c.contains("&gt;");
      // R : add extra pad on the right
      let add_extra_pad_right = c.contains("R");

      if !can_attach_left {
        dx -= non_attached_pad;
        width += non_attached_pad;
      }

      if !can_attach_right {
        width += non_attached_pad;
      }
      if add_extra_pad_right {
        width += extra_pad;
      }

      /*
      if !can_attach {
        dx -= non_attached_pad;
        width += 2.0 * non_attached_pad;
      }
      */

      let routes: Vec<Vec<(f64, f64)>> = polylines
        .iter()
        .map(|l| l.iter().map(|p| (p.x - dx, p.y)).collect())
        .collect();

      letters.insert(
        letter_name.clone(),
        Letter::new(routes, width, height, can_attach_left, can_attach_right),
      );
    }

    letters.insert(
      " ".to_string(),
      Letter::new(vec![], 0.5 * height, height, false, false),
    );

    LetterSvgReferential { letters }
  }

  fn get_letter(&self, c: &String) -> Option<&Letter> {
    self.letters.get(c)
  }
}

fn spiral_optimized(
  x: f64,
  y: f64,
  radius: f64,
  dr: f64,
  approx: f64,
) -> Vec<(f64, f64)> {
  let two_pi = 2.0 * PI;
  let mut route = Vec::new();
  let mut r = radius;
  let mut a = 0f64;
  loop {
    let p = (
      significant_mm(x + r * a.cos()),
      significant_mm(y + r * a.sin()),
    );
    let l = route.len();
    if l == 0 || euclidian_dist(route[l - 1], p) > approx {
      route.push(p);
    }
    let da = 1.0 / (r + 8.0); // bigger radius is more we have to do angle iterations
    a = (a + da) % two_pi;
    r -= dr * da / two_pi;
    if r < 0.05 {
      break;
    }
  }
  route
}

fn calc_text_center(routes: &Vec<Vec<(f64, f64)>>) -> (f64, f64) {
  let mut min_x = INFINITY;
  let mut max_x = -INFINITY;
  let mut min_y = INFINITY;
  let mut max_y = -INFINITY;
  for route in routes.iter() {
    for p in route.iter() {
      if p.0 < min_x {
        min_x = p.0;
      }
      if p.0 > max_x {
        max_x = p.0;
      }
      if p.1 < min_y {
        min_y = p.1;
      }
      if p.1 > max_y {
        max_y = p.1;
      }
    }
  }
  ((min_x + max_x) / 2.0, (min_y + max_y) / 2.0)
}

fn rng_from_fxhash(hash: &String) -> StdRng {
  let mut bs = [0; 32];
  bs58::decode(hash.chars().skip(2).take(43).collect::<String>())
    .into(&mut bs)
    .unwrap();
  let rng = StdRng::from_seed(bs);
  return rng;
}
