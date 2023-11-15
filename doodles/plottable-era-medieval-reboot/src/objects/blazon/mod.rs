use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */
pub mod flagpattern;
pub mod traits;

use self::traits::Blazon;

pub fn get_duel_houses<R: Rng>(rng: &mut R) -> (Blazon, Blazon) {
  (Blazon::Lys, Blazon::Lys)
}
