use crate::pow::target::bits_to_target;

/// Ước lượng work từ bits (most-work rule)
/// - target nhỏ => work lớn
/// - dùng 16 byte đầu (MSB) của target để so sánh
pub fn work_from_bits(bits: u32) -> u128 {
    let target = bits_to_target(bits);

    // Lấy 16 byte ĐẦU (most significant bytes)
    let mut t: u128 = 0;
    for b in &target[0..16] {
        t = (t << 8) | (*b as u128);
    }

    if t == 0 {
        return u128::MAX;
    }

    // Scale work: tránh saturate quá sớm
    (u128::MAX >> 1) / t
}
