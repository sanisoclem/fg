use std::collections::HashMap;

use bevy::prelude::*;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct BlueprintId(u32);

#[derive(Resource, Default)]
pub struct BlueprintRegistry {
  blueprints: HashMap<BlueprintId, BlueprintDescriptor>,
  last_id: u32,
}

impl BlueprintRegistry {
  pub fn clear(&mut self) {
    self.blueprints.clear()
  }
  pub fn register(&mut self, blueprint: BlueprintDescriptor) {
    self.last_id += 1;
    let id = BlueprintId(self.last_id);
    self.blueprints.insert(id, blueprint);
  }
  pub fn get(&self, id: &BlueprintId) -> Option<&BlueprintDescriptor> {
    self.blueprints.get(id)
  }
  pub fn get_all(&self) -> impl Iterator<Item = &BlueprintDescriptor> {
    self.blueprints.values()
  }
}

pub enum Footprint {
  Rectangle(u8),
}

// TODO: move this to separate module, impl Semiring
pub struct ResourceAmount;

pub struct BlueprintDescriptor {
  pub scene: Handle<DynamicScene>,
  pub footprint: Footprint,
  pub cost: ResourceAmount,
}
