fn div_euclid(a: i32, m: i32) -> i32 {
    let r = modular_remainder(a, m);
    (a - r) / m
}

fn modular_remainder(a: i32, m: i32) -> i32 {
    ((a % m) + m) % m
}

fn basic_modular_operations() {
    println!("=== Basic Modular Arithmetic ===");

    let examples: Vec<(i32, i32)> = vec![(17, 5), (25, 7), (100, 13), (42, 8), (-7, 3), (-15, 4)];

    println!("a   | m  | a mod m | Quotient | Remainder");
    println!("----|----|---------|---------|---------");

    for (a, m) in examples {
        let quotient = div_euclid(a, m);
        let remainder = modular_remainder(a, m);
        println!(
            "{:3} | {:2} |   {:3}   |   {:2}     |   {:3}",
            a, m, remainder, quotient, remainder
        );
    }
    println!();
}

fn modular_arithmetic_properties() {
    println!("=== Modular Arithmetic Properties ===");

    let a = 23;
    let b = 17;
    let m = 7;

    println!("a = {}, b = {}, m = {}", a, b, m);
    println!();

    // Addition property
    let add_direct = modular_remainder(a + b, m);
    let add_modular = modular_remainder(modular_remainder(a, m) + modular_remainder(b, m), m);
    println!("Addition Property:");
    println!("  ({} + {}) mod {} = {}", a, b, m, add_direct);
    println!(
        "  (({} mod {}) + ({} mod {})) mod {} = ({} + {}) mod {} = {}",
        a, m, b, m, m, modular_remainder(a, m), modular_remainder(b, m), m, add_modular
    );
    println!("  Results equal? {}", add_direct == add_modular);
    println!();

    // Multiplication property
    let mul_direct = modular_remainder(a * b, m);
    let mul_modular = modular_remainder(modular_remainder(a, m) * modular_remainder(b, m), m);
    println!("Multiplication Property:");
    println!(
        "  ({} × {}) mod {} = {} mod {} = {}",
        a, b, m, a * b, m, mul_direct
    );
    println!(
        "  (({} mod {}) × ({} mod {})) mod {} = ({} × {}) mod {} = {}",
        a, m, b, m, m, modular_remainder(a, m), modular_remainder(b, m), m, mul_modular
    );
    println!("  Results equal? {}", mul_direct == mul_modular);
    println!();

    // Exponentiation property
    let exp = 3;
    let pow_direct = modular_remainder(a.pow(exp), m);
    let pow_modular = modular_exponentiation(a as i64, exp as i64, m as i64) as i32;
    println!("Exponentiation Property:");
    println!(
        "  {}^{} mod {} = {} mod {} = {}",
        a, exp, m, a.pow(exp), m, pow_direct
    );
    println!("  Using modular exponentiation: {}", pow_modular);
    println!();
}

fn modular_exponentiation(base: i64, exp: i64, modulus: i64) -> i64 {
    if modulus == 1 {
        return 0;
    }

    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exp;

    println!("  Step-by-step calculation:");
    println!("  Initial: result = 1, base = {}, exp = {}", base, exp);

    while exp > 0 {
        if exp % 2 == 1 {
            let old_result = result;
            result = (result * base) % modulus;
            println!("    exp is odd -> result = {} × {} = {} ≡ {} (mod {})", 
                     old_result, base, old_result * base, result, modulus);
        }
        exp >>= 1;
        if exp > 0 {
            let old_base = base;
            base = (base * base) % modulus;
            println!("    base = {}² = {} ≡ {} (mod {}), remaining exp: {}", 
                     old_base, old_base * old_base, base, modulus, exp);
        }
    }

    result
}

fn clock_arithmetic() {
    println!("=== Clock Arithmetic ===");
    println!("24-hour clock time calculations (mod 24)");
    println!();

    let scenarios = vec![
        ("9 AM + 7 hours", 9, 7),
        ("8 PM + 9 hours", 20, 9),
        ("3 AM - 5 hours", 3, -5),
        ("Midnight + 15 hours", 0, 15),
    ];

    for (desc, start_time, hours) in scenarios {
        let result_24h = modular_remainder(start_time + hours, 24);
        let result_12h = if result_24h == 0 {
            12
        } else if result_24h > 12 {
            result_24h - 12
        } else {
            result_24h
        };
        let ampm = if result_24h < 12 || result_24h == 24 {
            "AM"
        } else {
            "PM"
        };

        println!("{}", desc);
        println!(
            "  Calculation: ({} + {}) mod 24 = {}",
            start_time, hours, result_24h
        );
        println!("  Result: {} {}:00", result_12h, ampm);
        println!();
    }
}

fn modular_inverse_example() {
    println!("=== Modular Inverse ===");

    let a = 3;
    let m = 7;

    println!("Finding modular inverse of {} mod {}:", a, m);
    println!("Condition: {} × x ≡ 1 (mod {})", a, m);
    println!();

    println!("x | {} × x | ({} × x) mod {}", a, a, m);
    println!("--|-------|----------");
    for x in 1..m {
        let product = a * x;
        let result = modular_remainder(product, m);
        let is_inverse = if result == 1 { " ← Inverse!" } else { "" };
        println!("{} |   {:2}   |    {}    {}", x, product, result, is_inverse);
    }

    match extended_gcd(a, m) {
        Some(inverse) => {
            println!(
                "Extended Euclidean Algorithm result: {} mod {} inverse is {}",
                a, m, inverse
            );
            println!(
                "Verification: {} × {} mod {} = {}",
                a,
                inverse,
                m,
                modular_remainder(a * inverse, m)
            );
        }
        None => println!("{} and {} are not coprime, so inverse doesn't exist.", a, m),
    }
    println!();
}

fn extended_gcd(a: i32, m: i32) -> Option<i32> {
    fn gcd_extended(a: i32, b: i32) -> (i32, i32, i32) {
        if a == 0 {
            (b, 0, 1)
        } else {
            let (gcd, x1, y1) = gcd_extended(b % a, a);
            (gcd, y1 - (b / a) * x1, x1)
        }
    }

    let (gcd, x, _) = gcd_extended(a, m);

    if gcd != 1 {
        None
    } else {
        Some(modular_remainder(x, m))
    }
}

fn chinese_remainder_theorem() {
    println!("=== Chinese Remainder Theorem ===");

    let remainders = vec![2, 3, 2];
    let moduli = vec![3, 5, 7];

    println!("System of congruences:");
    for i in 0..remainders.len() {
        println!("  x ≡ {} (mod {})", remainders[i], moduli[i]);
    }
    println!();

    println!("Step-by-step solution:");
    let product: i32 = moduli.iter().product();
    println!("1. M = {} × {} × {} = {}", moduli[0], moduli[1], moduli[2], product);

    let mut solution = 0;
    for i in 0..remainders.len() {
        let mi = product / moduli[i];
        println!("2-{}. M{} = {} / {} = {}", i+1, i+1, product, moduli[i], mi);
        
        match extended_gcd(mi, moduli[i]) {
            Some(yi) => {
                println!("     y{} = {} mod {} inverse = {}", i+1, mi, moduli[i], yi);
                let term = remainders[i] * mi * yi;
                println!("     term{} = {} × {} × {} = {}", i+1, remainders[i], mi, yi, term);
                solution += term;
            }
            None => {
                println!("     Cannot find inverse!");
                return;
            }
        }
    }

    solution = modular_remainder(solution, product);
    println!();
    println!("3. Solution: x ≡ {} (mod {})", solution, product);
    
    // Verification
    println!();
    println!("Verification:");
    for i in 0..remainders.len() {
        let check = solution % moduli[i];
        println!("  {} mod {} = {} (original: {})", solution, moduli[i], check, remainders[i]);
    }
}

fn real_world_applications() {
    println!("=== Real World Applications ===");

    // ISBN-10 checksum
    println!("1. ISBN-10 Checksum (using mod 11):");
    let isbn = "020161622"; // excluding last digit
    println!("   ISBN: {}-?", isbn);
    
    let mut sum = 0;
    for (i, digit) in isbn.chars().enumerate() {
        let d = digit.to_digit(10).unwrap() as i32;
        let weight = 10 - i as i32;
        sum += d * weight;
        println!("   Position {}: {} × {} = {}", i+1, d, weight, d * weight);
    }
    
    let checksum = modular_remainder(11 - (sum % 11), 11);
    let check_digit = if checksum == 10 { 'X' } else { char::from_digit(checksum as u32, 10).unwrap() };
    println!("   Sum: {}, Checksum: {} mod 11 = {}", sum, 11 - (sum % 11), checksum);
    println!("   Complete ISBN: {}-{}", isbn, check_digit);
    println!();

    // Hash table example
    println!("2. Hash Table (using mod 7):");
    let keys = vec![15, 22, 8, 35, 41, 28];
    println!("   Table size: 7");
    println!("   Key | Hash (key mod 7) | Position | Collision?");
    println!("   ----|------------------|----------|----------");
    
    let mut used_positions = vec![false; 7];
    for key in keys {
        let hash = modular_remainder(key, 7);
        let collision = if used_positions[hash as usize] { "Collision!" } else { "" };
        used_positions[hash as usize] = true;
        println!("   {:2}  |        {:2}        |    {}     | {}", key, hash, hash, collision);
    }
    
    println!();
    println!("   Collision resolution methods:");
    println!("   • Chaining: Store in linked list at same position");
    println!("   • Open addressing: Find next empty slot");
    println!();

    // RSA cryptography example
    println!("3. RSA Encryption Basics:");
    println!("   Public key: (n=33, e=3)");
    println!("   Private key: (n=33, d=7)");
    
    let message = 4;
    let n = 33;
    let e = 3;
    let d = 7;
    let encrypted = modular_exponentiation(message, e, n);
    let decrypted = modular_exponentiation(encrypted, d, n);
    println!("   Message: {}", message);
    println!("   Encryption: {}^{} mod {} = {}", message, e, n, encrypted);
    println!("   Decryption: {}^{} mod {} = {}", encrypted, d, n, decrypted);
    println!();
}

pub fn example() {
    println!("🔢 Master Modular Arithmetic! 🔢");
    println!("==================================");
    println!();

    basic_modular_operations();
    modular_arithmetic_properties();
    clock_arithmetic();
    modular_inverse_example();
    chinese_remainder_theorem();
    real_world_applications();

    println!("=== Practice Problems ===");
    println!("1. 47 mod 13 = ?");
    println!("2. (-23) mod 8 = ?");
    println!("3. What is the modular inverse of 5 mod 12?");
    println!("4. Find x where x ≡ 1 (mod 4) and x ≡ 2 (mod 9)");
    println!();
    println!("Answers: 1) {}, 2) {}, 3) {}, 4) {}", 
             modular_remainder(47, 13), 
             modular_remainder(-23, 8),
             extended_gcd(5, 12).unwrap_or(0),
             29);
}