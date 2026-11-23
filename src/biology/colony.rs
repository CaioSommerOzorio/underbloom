use crate::biology::species::Species;
use crate::environment::substrate::Substrate;

pub struct Colony {
  pub species: Species,
  pub biomass: f32,
  pub age: f32,
  pub active: bool,
}

impl Colony {
  pub fn grow(&self, temp: f32, substrate: Substrate) -> f32 {
    if !self.active {
      return 0.0;
    }
    let species = &self.species;
    let temp_diff = (temp - species.optimalTemp).abs();
    let temp_modifier = (1.0 - temp_diff / species.tempResistance).clamp(0.0, 1.0);
    let growth_modifier = temp_modifier;
    return self.biomass * species.baseGrowthRate * growth_modifier;
  }
}
