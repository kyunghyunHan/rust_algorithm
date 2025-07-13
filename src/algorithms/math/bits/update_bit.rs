pub fn update_bit(number: u32, bit_position: u32, bit_value: bool) -> u32 {
    let bit_value_normalized = if bit_value { 1 } else { 0 };
    let clear_mask = !(1 << bit_position);
    (number & clear_mask) | (bit_value_normalized << bit_position)
}
