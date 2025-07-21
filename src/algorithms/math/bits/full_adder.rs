use crate::algorithms::math::bits::get_bit::get_bits;

pub fn full_adder(a: u32, b: u32) -> u32 {
    let mut result = 0;
    let mut carry: u32 = 0;

    for i in 0..32 {
        let ai = get_bits(a, i);
        let bi = get_bits(b, i);

        let carr_in = carry;

        let ai_plus_bi = ai ^ bi;
        let bit_sum = ai_plus_bi ^ carr_in;

        let carry_out = (ai_plus_bi & carr_in) | (ai & bi);
        carry = carry_out;

        result |= bit_sum << i;
    }

    result
}
