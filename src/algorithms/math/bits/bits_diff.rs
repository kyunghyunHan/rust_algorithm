use crate::algorithms::math::bits::count_set_bits::count_set_bits;
pub fn bits_diff(number_a: u32, number_b: u32) -> u32 {
    count_set_bits(number_a ^ number_b)
}
