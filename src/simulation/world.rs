use crate::environment::area::Area;

pub struct World {
  pub areas: Vec<Area>,
  pub time: u64,
}
