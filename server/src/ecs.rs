use std::collections::HashMap;
use parking_lot::RwLock;
use crate::types::*;
use crate::feeds::*;
use crate::fixed::*;

#[derive(Clone)]
pub struct Inventory { pub feeds: [f64; FEED_DIM], pub food_kg: f64 }
impl Default for Inventory { fn default() -> Self { Self { feeds: [0.0; FEED_DIM], food_kg: 0.0 } } }

#[derive(Clone)]
pub struct ResourceNode { pub id: EntityId, pub pos: V3, pub feeds_frac: [f64; FEED_DIM], pub remaining_kg: f64, pub radius: Fix }

pub struct World {
  pub bodies: RwLock<HashMap<EntityId, Body>>,
  pub next_id: RwLock<EntityId>,
  pub inventories: RwLock<HashMap<EntityId, Inventory>>,
  pub resource_nodes: RwLock<HashMap<EntityId, ResourceNode>>,
  pub mining: RwLock<HashMap<EntityId, EntityId>>,
}
impl World {
  pub fn new() -> Self {
    Self { bodies: RwLock::new(HashMap::new()), next_id: RwLock::new(1),
           inventories: RwLock::new(HashMap::new()),
           resource_nodes: RwLock::new(HashMap::new()),
           mining: RwLock::new(HashMap::new()) }
  }
  pub fn spawn_ship(&self) -> EntityId {
    let mut idg = self.next_id.write(); let id = *idg; *idg += 1;
    self.bodies.write().insert(id, Body { id, kind:Kind::Ship, ..Default::default() });
    self.inventories.write().entry(id).or_default();
    id
  }
  pub fn spawn_asteroid_node(&self, pos: V3, vel: V3, comp: [f64; FEED_DIM], total_kg: f64, radius: Fix) -> EntityId {
    let mut idg = self.next_id.write(); let id = *idg; *idg += 1;
    self.bodies.write().insert(id, Body { id, kind:Kind::Asteroid, pos, vel, ..Default::default() });
    self.resource_nodes.write().insert(id, ResourceNode { id, pos, feeds_frac: comp, remaining_kg: total_kg, radius });
    id
  }
}
