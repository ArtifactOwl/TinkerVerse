pub fn fnv1a64(bytes: &[u8]) -> u64 {
    const OFF: u64 = 0xcbf29ce484222325;
    const PR: u64 = 0x100000001b3;
    let mut h = OFF;
    for &b in bytes { h ^= b as u64; h = h.wrapping_mul(PR); }
    h
}
