use crate::biology::colony::Colony;
use crate::environment::substrate::Substrate;

pub struct Pot {
  pub name: String,
  pub substrate: Substrate,
  pub moisture: f32,
  pub temp: f32,
  pub colonies: Vec<Colony>,
  // to be implemented
  // pub nutrients:
}
