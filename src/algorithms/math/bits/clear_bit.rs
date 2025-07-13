pub fn clear_bit(number: u32, bit_position: u32) -> u32 {
    let mask = !(1 << bit_position);
    number & mask
}