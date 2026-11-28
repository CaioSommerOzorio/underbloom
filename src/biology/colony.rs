use crate::biology::species::Species;
use crate::environment::pot::Pot;
use crate::environment::substrate::Substrate;
use std::f32::consts::PI;

#[derive(Clone)]
pub struct Colony {
  pub species: Species,
  pub biomass: f32,
  pub age: u64,
  pub growth_rate: f32,
  pub active: bool,
}

fn modifier(env: f32, optimal: f32, res: f32) -> f32 {
  let diff = (env - optimal).abs() / res;
  let modifier = (((diff * PI).cos() + 1.0) / 2.0).powf(0.25);
  return modifier;
}

impl Colony {
  pub fn new(species: Species) -> Self {
    Self {
      species,
      biomass: 1.0,
      age: 0,
      growth_rate: 0.0,
      active: true,
    }
  }
  pub fn grow(&self, temp: f32, moisture: f32, substrate: Substrate) -> f32 {
    if !self.active {
      return 0.0;
    }

    let species = &self.species;

    let mut temp_modifier = modifier(temp, species.optimal_temp, species.temp_resistance);
    let mut moisture_modifier = modifier(
      moisture,
      species.optimal_moisture,
      species.moisture_resistance,
    );

    if temp - species.optimal_temp > species.temp_resistance {
      temp_modifier = -(temp - species.optimal_temp).sqrt();
      moisture_modifier = 1.0;
    }

    println!("{} * {}", temp_modifier, moisture_modifier);
    let growth_modifier = temp_modifier * moisture_modifier;
    return species.base_growth_rate * growth_modifier;
  }
}
