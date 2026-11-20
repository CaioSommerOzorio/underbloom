use crate::biology::species::Species;
use crate::environment::pot::Pot;

pub struct Colony {
  pub species: Species,
  pub biomass: f32,
  pub age: f32,
  pub active: bool,
}

impl Colony {
  pub fn grow(&mut self, pot: &Pot) {
    if !self.active {
      return;
    }
    let species = &self.species;
    let temp_diff = (pot.temp - species.temp).abs();
    let temp_modifier = (1.0 - temp_diff / species.temp_resistance).clamp(0.0, 1.0);
    let growth_modifier = temp_modifier;
    self.biomass += self.biomass * species.base_growth_rate * growth_modifier;
  }
}
