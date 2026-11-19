use crate::simulation::world::World;

pub fn tick(world: &mut World) {
  world.time += 1;

  for area in &mut world.areas {
    for colony in &mut area.colonies {
      colony.grow();
    }
  }
}
