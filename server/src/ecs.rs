use std::collections::HashMap;
use parking_lot::RwLock;
use crate::types::*;

pub struct World { pub bodies: RwLock<HashMap<EntityId, Body>>, pub next_id: RwLock<EntityId> }
impl World {
    pub fn new() -> Self { Self { bodies: RwLock::new(HashMap::new()), next_id: RwLock::new(1) } }
    pub fn spawn_ship(&self) -> EntityId {
        let mut idg = self.next_id.write(); let id = *idg; *idg += 1;
        self.bodies.write().insert(id, Body { id, kind:Kind::Ship, ..Default::default() }); id
    }
}
