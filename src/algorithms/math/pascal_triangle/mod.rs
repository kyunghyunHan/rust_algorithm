pub fn pascal_triangle(line_number: u32) -> Vec<u32> {
    let mut current_line = vec![1];

    let mut current_line_size = line_number + 1;

    for i in 1..current_line_size {
        current_line[i as usize] = current_line[i as usize - 1] * (line_number - i)
    }
    current_line
}
