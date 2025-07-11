pub fn get_bits(number: i32, bit_position: i32) -> i32 {
    number >> bit_position & 1
}
