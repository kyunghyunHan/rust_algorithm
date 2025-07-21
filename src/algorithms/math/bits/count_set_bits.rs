pub fn count_set_bits(original_number: u32) -> u32 {
    let mut set_bits_count = 0;
    let mut number = original_number;

    while number != 0 {
        // Add the last bit of the number to the sum of set bits
        set_bits_count += number & 1;
        // Unsigned right shift by one bit
        number >>= 1;
    }

    set_bits_count
}
