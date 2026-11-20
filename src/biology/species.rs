use crate::environment::substrate::Substrate;

#[derive(Debug, Clone)]

pub struct Species {
  pub name: String,

  pub optimalTemp: f32,
  pub tempResistance: f32,

  pub optimalMoisture: f32,
  pub moistureResistance: f32,

  pub baseGrowthRate: f32,
  pub substratePreferences: Vec<Substrate>,
  // to introduce:
  // - nutrients
  // - enzyme production
  // - enzyme production rate
  // - health
  // - reproductive state
  // - genetic variation
}

// https://cfaes.osu.edu/fact-sheet/shiitake-mushroom-production-logs-and-laying-yards
pub fn lentinulaEdodes() -> Species {
  Species {
    name: String::from("Lentinula edodes"),

    optimalTemp: 22.0,
    tempResistance: 4.0,

    optimalMoisture: 87.5,
    moistureResistance: 2.5,

    baseGrowthRate: 0.5,

    substratePreferences: vec![Substrate::Hardwood],
  }
}
