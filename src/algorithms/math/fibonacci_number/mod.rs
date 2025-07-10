fn fibonacci(n: i32) -> Vec<i32> {
    let mut fib_sequence = vec![1];
    let mut current_value = 1;
    let mut previos_value = 0;

    if n == 1 {
        return fib_sequence;
    }

    let mut iteration_counter = n - 1;

    while iteration_counter != 0 {
        current_value += previos_value;
        previos_value = current_value - previos_value;

        fib_sequence.push(current_value);
        iteration_counter -= 1;
    }

    fib_sequence
}

pub fn example() {}
