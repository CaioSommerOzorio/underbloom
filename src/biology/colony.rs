use crate::biology::species::Species;

pub struct Colony {
    pub species: Species,
    pub biomass: f32,
    pub age: f32,
    pub active: bool,
}
