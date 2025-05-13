pub fn example() {
    prime_number_sieve()
}

fn prime_number_sieve() {
    let number = 1000000;

    let mut a = vec![0; number + 1];

    for i in 2..=number {
        a[i] = i;
    }

    for i in 2..=number {
        if a[i] == 0 {
            continue;
        }

        let mut j = 2 * i;

        while j <= number {
            a[j] = 0;
            j += i;
        }
    }

    for i in 2..=number {
        if a[i] != 0 {
            print!("{}", a[i]);
        }
    }
    println!();
}
