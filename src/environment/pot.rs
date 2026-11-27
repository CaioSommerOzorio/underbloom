use crate::biology::colony::Colony;
use crate::biology::species::all_species;
use crate::environment::substrate::Substrate;

pub struct Pot {
  pub substrate: Substrate,
  pub moisture: f32,
  pub temp: f32,
  pub colonies: Vec<Colony>,
}

impl Pot {
  pub fn add_colony(&mut self, species_name: &str) {
    for species in all_species() {
      if &species.name == species_name {
        self.colonies.push(Colony::new(species));
        break;
      }
    }
  }
}
