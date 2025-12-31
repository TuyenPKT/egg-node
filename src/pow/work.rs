pub fn work_from_bits(bits: u32) -> u128 {
    // Work ~ 1 / target
    // Đơn giản hóa: dùng mantissa làm proxy
    let mantissa = (bits & 0x00ff_ffff) as u128;
    if mantissa == 0 {
        0
    } else {
        (1u128 << 64) / mantissa
    }
}
