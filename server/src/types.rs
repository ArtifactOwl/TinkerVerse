use crate::fixed::*;
pub type EntityId = u64;
#[derive(Copy, Clone, PartialEq)] pub enum Kind { Unknown, Ship, Projectile, Asteroid }
#[derive(Copy, Clone)]
pub struct Body { pub id: EntityId, pub kind: Kind, pub pos: V3, pub vel: V3, pub hp: Fix }
impl Default for Body {
  fn default() -> Self { Self { id:0, kind:Kind::Unknown, pos:V3::ZERO, vel:V3::ZERO, hp: crate::fixed::to_fix(100.0) } }
}
