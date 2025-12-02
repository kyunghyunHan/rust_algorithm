//natural logarithm
// 'e'
fn ln() {
    let x = 10.0_f64;
    println!("ln(10) = {}", x.ln());
}
pub type real = f64;

pub fn diff_second<F>(f: F, x: real, h: real) -> real
where
    F: Fn(real) -> real,
{
    (f(x + h) - 2.0 * f(x) + f(x - h)) / (h * h)
}

fn example() {
    let f = |x: real| x * x * x;
}
