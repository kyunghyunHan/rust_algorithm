fn square_root(number: f64, tolerance: u32) -> Result<f64, String> {
    // Check for negative input
    if number < 0.0 {
        return Err("The method supports only positive numbers".to_string());
    }

    // Handle square root of 0
    if number == 0.0 {
        return Ok(0.0);
    }

    // Start approximation from 1.0
    let mut root = 1.0;

    // Compute required delta based on tolerance
    let required_delta = 1.0 / 10f64.powi(tolerance as i32);

    // Approximate using Newton-Raphson method (Babylonian)
    while (number - root * root).abs() > required_delta {
        root = root - (root * root - number) / (2.0 * root);
    }

    // Round to specified tolerance
    let multiplier = 10f64.powi(tolerance as i32);
    Ok((root * multiplier).round() / multiplier)
}
pub fn example() {
    match square_root(2.0, 5) {
        Ok(result) => println!("Square root: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}