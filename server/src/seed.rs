#[derive(Clone, Copy)]
pub struct SplitMix64 { x: u64 }
impl SplitMix64 {
    pub fn new(seed: u64) -> Self { Self { x: seed } }
    pub fn next(&mut self) -> u64 { self.x = self.x.wrapping_add(0x9E3779B97F4A7C15); let mut z = self.x; z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9); z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB); z ^ (z >> 31) }
    pub fn next_f01(&mut self) -> f64 { (self.next() >> 11) as f64 * (1.0 / (1u64 << 53) as f64) }
}
