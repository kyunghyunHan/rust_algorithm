use std::f64::consts::E;

fn df_continuous(r: f64, tau: f64) -> f64 {
    (-r * tau).exp()
}

// m times per year compounding
fn df_discrete(r: f64, m: f64, tau: f64) -> f64 {
    1.0 / (1.0 + r / m).powf(m * tau)
}

// From discount factor -> continuous zero rate
fn zero_rate_cont_from_df(z: f64, tau: f64) -> f64 {
    -(z.ln()) / tau
}

// From discount factor -> discrete zero rate
fn zero_rate_disc_from_df(z: f64, m: f64, tau: f64) -> f64 {
    m * (z.powf(-1.0 / (m * tau)) - 1.0)
}

// Present value with continuous compounding
fn pv_continuous(r: f64, t_now: f64, cashflows: &[(f64, f64)]) -> f64 {
    cashflows
        .iter()
        .map(|(amt, t)| amt * df_continuous(r, t - t_now))
        .sum()
}

pub fn example() {
    let r = 0.05;       // 5% annual rate
    let tau = 2.0;      // 2 years
    let ms = [1.0, 2.0, 12.0, 365.0];

    println!("=== Discount Factors for r = 5%, tau = 2y ===");
    println!("Continuous: Z = {:.6}", df_continuous(r, tau));
    for &m in &ms {
        println!("m = {:>3.0}:   Z = {:.6}", m, df_discrete(r, m, tau));
    }

    println!("\n=== Recover Zero Rates from a given Z ===");
    let z = df_continuous(r, tau);
    let r_from_z_cont = zero_rate_cont_from_df(z, tau);
    println!("From Z (continuous): Z = {:.6} -> r = {:.6}%", z, r_from_z_cont * 100.0);
    for &m in &ms {
        let r_from_z_disc = zero_rate_disc_from_df(z, m, tau);
        println!("From Z (m={:>3.0}): r = {:.6}%", m, r_from_z_disc * 100.0);
    }

    println!("\n=== PV with continuous compounding (flat r=5%) ===");
    let cfs = vec![(50.0, 0.5), (50.0, 1.0), (1_050.0, 2.0)];
    let pv = pv_continuous(r, 0.0, &cfs);
    println!("Cashflows: {:?}", cfs);
    println!("PV (continuous 5%): {:.4}", pv);
}
