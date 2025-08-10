use crate::fixed::*;
use crate::types::*;

pub struct InputFrame { pub id: EntityId, pub thrust: V3, pub fire: bool }

pub fn step(world: &crate::ecs::World, inputs: &[InputFrame]) {
    // apply inputs
    {
        let mut bodies = world.bodies.write();
        for inp in inputs {
            if let Some(b) = bodies.get_mut(&inp.id) {
                // very simple thrust model: dv = thrust * a * dt
               let a = to_fix(20.0); // TEMP: higher accel so movement is obvious
                let dv = inp.thrust.scale(mul(a, DT));
                b.vel = b.vel.add(dv);
            }
        }
    }
    // integrate positions
    {
        let mut bodies = world.bodies.write();
        for (_, b) in bodies.iter_mut() {
            let dp = b.vel.scale(DT);
            b.pos = b.pos.add(dp);
        }
    }
}
