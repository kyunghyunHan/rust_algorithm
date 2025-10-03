use std::f64::consts::E;

/// 복리 주기 정의
#[derive(Clone, Copy, Debug)]
enum Compounding {
    PerYear(u32),   // 1년에 m번
    Continuous,     // 연속복리
}

/// 미래가치 계산
fn future_value(N: f64, r: f64, T: f64, c: Compounding) -> f64 {
    match c {
        Compounding::Continuous => N * E.powf(r * T),
        Compounding::PerYear(m_u32) => {
            let m = m_u32 as f64;
            N * (1.0 + r / m).powf(m * T)
        }
    }
}

/// 연속복리율 r_cont → m회 복리 등가이율 r_m
fn discrete_rate_from_continuous(r_cont: f64, m: u32) -> f64 {
    let m_f = m as f64;
    m_f * (E.powf(r_cont / m_f) - 1.0)
}

/// m회 복리율 r_m → 연속복리율 r_cont
fn continuous_rate_from_discrete(r_m: f64, m: u32) -> f64 {
    let m_f = m as f64;
    m_f * (1.0 + r_m / m_f).ln()
}

pub fn example() {
    let N = 100.0;   // 원금
    let r = 0.05;    // 연 5%
    let T = 2.0;     // 2년 투자

    let cases = [
        (Compounding::PerYear(1), "Annual (m=1)"),
        (Compounding::PerYear(2), "Semi-annual (m=2)"),
        (Compounding::PerYear(12), "Monthly (m=12)"),
        (Compounding::PerYear(365), "Daily (m=365)"),
        (Compounding::Continuous, "Continuous"),
    ];

    println!("=== Future Value Comparison (N={N}, r={:.2}%, T={T}y) ===", r * 100.0);
    for (c, label) in cases {
        let fv = future_value(N, r, T, c);
        println!("{:<15} -> {:.4}", label, fv);
    }

    // 변환 예시: 연속복리율 5% ↔ 월복리(12회)
    let r_cont = 0.05;
    let m = 12;
    let r_m = discrete_rate_from_continuous(r_cont, m);
    let r_cont_back = continuous_rate_from_discrete(r_m, m);

    println!("\n=== Rate Conversion Example ===");
    println!("Continuous r = {:.4}% → Discrete (m=12) r_m = {:.4}%", r_cont * 100.0, r_m * 100.0);
    println!("Back to continuous: {:.4}%", r_cont_back * 100.0);

    // 실제 금액 비교
    let fv_cont = future_value(N, r_cont, T, Compounding::Continuous);
    let fv_m = future_value(N, r_m, T, Compounding::PerYear(m));
    println!("\nCheck equality at T={T} years:");
    println!(" FV_cont = {:.4}\n FV_m    = {:.4}\n diff    = {:.8}", fv_cont, fv_m, (fv_cont - fv_m).abs());
}
