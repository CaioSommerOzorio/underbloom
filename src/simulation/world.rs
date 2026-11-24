use crate::environment::pot::Pot;
use crate::environment::substrate::Substrate;

pub struct World {
  pub pots: Vec<Pot>,
  pub time: u64,
}

impl World {
  pub fn add_pot(&mut self) {
    self.pots.push(Pot {
      substrate: Substrate::Soil,
      moisture: 0.0,
      temp: 0.0,
      colonies: Vec::new(),
    });
  }
}
