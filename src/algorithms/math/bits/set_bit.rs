pub fn set_bit(number: i32, bit_position: i32) -> i32 {
    number | (1 << bit_position)
}
