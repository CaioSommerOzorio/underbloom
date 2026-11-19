use crate::biology::species::Species;

pub struct Colony {
  pub species: Species,
  pub biomass: f32,
  pub age: f32,
  pub active: bool,
}

impl Colony {
  pub fn grow(&mut self) {
    if self.active {
      self.biomass += self.species.base_growth_rate;
    }
  }
}
