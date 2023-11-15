/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */
use rand::prelude::*;

// medieval name generator
pub fn epic_title<R: Rng>(rng: &mut R) -> String {
  let city_start = vec![
    "An", "Cul", "Dun", "Nor", "Ship", "Tre", "Win", "Mere", "Pol", "Tarn",
    "Lin", "Man", "Baa", "Bra", "Bri", "Istan", "Bor", "Ast", "Ach", "Axe",
    "Car", "Wolf", "Chet", "Holm", "Pen", "Port", "Beck", "Buck", "Bull",
    "Bul", "Lis",
  ];
  let city_suffixes = vec![
    "bourg", "burg", "castle", "bul", "des", "ster", "chester", "llon", "bury",
    "borough", "by", "cott", "field", "gate", "ing", "tun", "wick", "worth",
    "caster", "burgh", "ver", "bon",
  ];
  let mut city = city_start[rng.gen_range(0..city_start.len())].to_string();
  city += city_suffixes[rng.gen_range(0..city_suffixes.len())];

  let events = vec![
    "Battle",
    "War",
    "Fall",
    "Conquest",
    "Crusade",
    "Attack",
    "Defense",
    "Siege",
    "Raid",
    "Rise",
    "Destruction",
    "Era",
    "Age",
    "Reign",
    "Empire",
    "Kingdom",
    "Doom",
    "Dawn",
    "History",
  ];
  let i = (rng.gen_range(0.0..events.len() as f32) * rng.gen_range(0.5..1.0))
    as usize;
  let event = events[i].to_string();
  let going_prefix = rng.gen_bool(0.5);
  let year = rng.gen_range(500..1400);
  if going_prefix {
    return format!("{} {} - circa {}", city, event, year);
  } else {
    return format!("{} of {} - circa {}", event, city, year);
  }
}
