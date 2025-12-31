pub fn bits_to_target(bits: u32) -> [u8; 32] {
    let exponent = (bits >> 24) as usize;
    let mantissa = bits & 0x00ff_ffff;

    let mut target = [0u8; 32];

    if exponent <= 3 {
        let value = mantissa >> (8 * (3 - exponent));
        target[28..32].copy_from_slice(&value.to_be_bytes());
    } else {
        let offset = exponent - 3;
        let value = mantissa.to_be_bytes();
        let start = 32 - offset - 4;
        if start < 32 {
            target[start..start + 4].copy_from_slice(&value);
        }
    }

    target
}
