use crate::simulation::world::World;

pub fn tick(world: &mut World) {
  world.time += 1;

  for pot in &mut world.pots {
    for colony in &mut pot.colonies {
      colony.growth_rate = colony.grow(pot.temp, pot.moisture, pot.substrate);
      colony.biomass += colony.growth_rate;
      colony.age += 1;
      if colony.biomass < 0.0 {
        colony.biomass = 0.0;
      }
    }
  }
}
