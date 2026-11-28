use crate::environment::substrate::Substrate;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Species {
  pub name: String,

  // Celcius
  pub optimal_temp: f32,
  pub temp_resistance: f32,

  // Percentage
  pub optimal_moisture: f32,
  pub moisture_resistance: f32,

  // Grams
  pub base_growth_rate: f32,
  pub substrate_growth: HashMap<Substrate, f32>,

  pub base_spore_production: f32,
  // to introduce:
  // - nutrients
  // - enzyme production
  // - enzyme production rate
  // - health
  // - reproductive state
  // - genetic variation
}

// I've decided to make up my own mushrooms
// since I can't find a uniform and cosistent source

// There's a bunch of cool mushrooms I wanna add but there's just
// not enough information about them

// King spot fungus
// basically it's the base fungus
fn king_spot() -> Species {
  Species {
    name: String::from("King Spot Fungus"),

    optimal_temp: 24.0,
    temp_resistance: 4.0,

    optimal_moisture: 77.5,
    moisture_resistance: 7.5,

    base_growth_rate: 1.0,
    substrate_growth: HashMap::from([
      (Substrate::Hardwood, 1.5),
      (Substrate::Softwood, 1.25),
      (Substrate::Soil, 0.75),
      (Substrate::LeafLitter, 0.25),
    ]),

    base_spore_production: 0.05,
  }
}

pub fn all_species() -> Vec<Species> {
  vec![king_spot()]
}

pub fn all_species_names() -> Vec<String> {
  vec![king_spot().name]
}
