pub type Fix = i64;
const SCALE: i64 = 1_000_000;
pub const TICK_HZ: u32 = 60;
pub const DT: Fix = SCALE / TICK_HZ as i64;

#[inline] pub fn to_fix(f: f64) -> Fix { (f * SCALE as f64).round() as Fix }
#[inline] pub fn to_f64(a: Fix) -> f64 { (a as f64) / SCALE as f64 }
#[inline] pub fn add(a: Fix, b: Fix) -> Fix { a.wrapping_add(b) }
#[inline] pub fn sub(a: Fix, b: Fix) -> Fix { a.wrapping_sub(b) }
#[inline] pub fn mul(a: Fix, b: Fix) -> Fix { (a.wrapping_mul(b)) / SCALE }
#[inline] pub fn div(a: Fix, b: Fix) -> Fix { (a.wrapping_mul(SCALE)) / b }

#[derive(Copy, Clone, Default)]
pub struct V3 { pub x: Fix, pub y: Fix, pub z: Fix }
impl V3 {
  pub const ZERO: V3 = V3 { x:0, y:0, z:0 };
  pub fn add(self, o: V3) -> V3 { V3 { x: add(self.x,o.x), y: add(self.y,o.y), z: add(self.z,o.z) } }
  pub fn scale(self, s: Fix) -> V3 { V3 { x: mul(self.x,s), y: mul(self.y,s), z: mul(self.z,s) } }
}
