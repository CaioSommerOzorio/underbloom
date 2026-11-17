use crate::environment::substrate::Substrate;

#[derive(Debug, Clone)]

pub struct Species {
    pub name: String,

    pub optimal_temperature: f32,
    pub temperature_resistance: f32,

    pub optimal_moisture: f32,
    pub moisture_resistance: f32,

    pub base_growth_rate: f32,
    pub substrate_preferences: Vec<Substrate>,

    // to introduce:
    // - nutrients
    // - enzyme production
    // - enzyme production rate
    // - health
    // - reproductive state
    // - genetic variation
}

// https://cfaes.osu.edu/fact-sheet/shiitake-mushroom-production-logs-and-laying-yards
pub fn lentinula_edodes() -> Species {
  Species {
    name: String::from("Lentinula edodes"),

    optimal_temperature: 22.0,
    temperature_resistance: 4.0,

    optimal_moisture: 87.5,
    moisture_resistance: 2.5,

    base_growth_rate: 0.5,

    substrate_preferences: vec![
      Substrate::Hardwood,
    ]
  }
}
