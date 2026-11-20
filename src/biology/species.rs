use crate::environment::substrate::Substrate;
use std::collections::HashMap;

#[derive(Debug, Clone)]

pub struct Species {
  pub name: String,

  // Celcius
  pub optimalTemp: f32,
  pub tempResistance: f32,

  // Percentage
  pub optimalMoisture: f32,
  pub moistureResistance: f32,

  // Grams
  pub baseGrowthRate: f32,
  pub substrateGrowth: HashMap<Substrate, f32>,

  pub baseSporeProduction: f32,
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
fn kingSpot() -> Species {
  Species {
    name: String::from("King Spot Fungus"),

    optimalTemp: 22.5,
    tempResistance: 0.5,

    optimalMoisture: 80,
    moistureResistance: 5,

    baseGrowthRate: 0.5,
    substratePreferences: HashMap::from([
      (Substrate::Hardwood, 1.5),
      (Substrate::Softwood, 1.25),
      (Substrate::Soil, 0.75),
      (Substrate::LeafLitter, 0.25),
    ]),

    baseSporeProduction: 0.05,
  }
}
