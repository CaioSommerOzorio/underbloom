use crate::biology::colony::Colony;
use crate::environment::substrate::Substrate;

pub struct Area {
    pub name: String,
    pub substrate: Substrate,
    pub moisture: f32,
    pub temperature: f32,
    pub colonies: Vec<Colony>,
    // to be implemented
    // pub nutrients: 
}
