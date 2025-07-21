pub fn is_power_of_two(number: u32) -> bool {
    (number & (number - 1)) == 0
}
