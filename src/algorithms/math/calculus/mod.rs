//natural logarithm
// 'e'
fn ln() {
    let x = 10.0_f64;
    println!("ln(10) = {}", x.ln());
}
pub type real = f64;

fn f_example(x: real) -> real {
    x * x * x
}

fn diff_forward<F>(f: F, x: real, h: real) -> real
where
    F: Fn(real) -> real,
{
    (f(x + h) - f(x)) / h
}
fn diff_backward<F>(f: F, x: real, h: real) -> real
where
    F: Fn(real) -> real,
{
    (f(x) - f(x - h)) / h
}

pub fn diff_second<F>(f: F, x: real, h: real) -> real
where
    F: Fn(real) -> real,
{
    (f(x + h) - 2.0 * f(x) + f(x - h)) / (h * h)
}

fn grad<F>(f: F, x: &[real], h: real) -> Vec<real>
where
    F: Fn(&[real]) -> real,
{
    let n = x.len();
    let mut g = vec![0.0; n];
    let mut x_plus = x.to_vec();
    let mut x_minus = x.to_vec();

    for i in 0..n {
        x_plus[i] += h;
        x_minus[i] -= h;
        g[i] = (f(&x_plus) - f(&x_minus)) / (2.0 * h);
        x_plus[i] = x[i];
        x_minus[i] = x[i];
    }

    g
}
fn example() {
    let f = |x: real| x * x * x;
    let h = 1e-5;

    let d1 = diff_forward(f, 2., h);

    println!("forward = {}", d1);
}
fn test() {}
