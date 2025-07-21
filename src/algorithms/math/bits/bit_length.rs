use std::iter::SkipWhile;

pub fn bit_length(number: u32) -> u32 {
    let mut bits_counter = 0;
    while (1 << bits_counter) <= number {
        bits_counter += 1;
    }

    bits_counter
}
