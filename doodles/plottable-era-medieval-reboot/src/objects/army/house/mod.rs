use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */
pub mod flagpattern;
pub mod traits;

use self::traits::House;

pub fn get_duel_houses<R: Rng>(rng: &mut R) -> (House, House) {
  (House::Lys, House::Lys)
}
