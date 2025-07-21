pub fn get_bits(number: u32, bit_position: u32) -> u32 {
    number >> bit_position & 1
}
