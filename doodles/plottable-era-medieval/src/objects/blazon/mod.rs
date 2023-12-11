use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub fn get_duel_houses<R: Rng>(rng: &mut R) -> (Blazon, Blazon) {
  let mut choices = vec![Blazon::Lys, Blazon::Dragon, Blazon::Falcon];
  choices.shuffle(rng);
  (choices[0], choices[1])
}

// Blazon are "teams" attacker or defender can be.
// we would only color the attackers, as the defender have a more neutral color consistent with the castle.

#[derive(Debug, Clone, Copy)]
pub enum Blazon {
  // Lys:
  // army: knights, cavalry
  // engine: trebuchet, belfry
  // blazon: lys
  // color: white/blue/gold
  Lys,

  // Dragon:
  // army: warrior with axe, fire archer
  // engine: trebuchet, catapult
  // blazon: fire
  // color: red/black
  Dragon,

  // Falcon
  // army: spear, lances, cavalry
  // color: gold/green
  // blazon: falcon head
  Falcon,
}

impl Blazon {
  pub fn rand<R: Rng>(rand: &mut R) -> Self {
    match rand.gen_range(0..3) {
      0 => Blazon::Lys,
      1 => Blazon::Dragon,
      2 => Blazon::Falcon,
      _ => unreachable!(),
    }
  }
}
