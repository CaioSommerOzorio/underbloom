use crate::simulation::world::World;

pub fn tick(world: &mut World) {
  world.time += 1;
}
