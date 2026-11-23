use crate::simulation::world::World;

pub fn tick(world: &mut World) {
  world.time += 1;

  for pot in &mut world.pots {
    for colony in &mut pot.colonies {
      colony.biomass += colony.grow(pot.temp, pot.substrate);
    }
  }
}
