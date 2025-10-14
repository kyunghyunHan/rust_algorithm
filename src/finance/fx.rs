fn annuity_value(cashflow: f64, rate: f64, years: u32) -> f64 {
    if rate == 0.0 {
        return cashflow * years as f64;
    }
    cashflow * (1.0 - (1.0 + rate).powf(-(years as f64))) / rate
}

fn accrual_factor(days: u32, convention: &str) -> f64 {
    match convention {
        "act/365" => days as f64 / 365.0,
        "act/360" => days as f64 / 360.0,
        "30/360" => 1.0 / 4.0,
        _ => panic!("Unsupported daycount convention"),
    }
}

fn main() {
    // ✅ Annuity 계산 예시
    let v = annuity_value(1.0, 0.04, 10);
    println!("Annuity present value = {:.4}", v);

    // ✅ Daycount 계산 예시
    let act365 = accrual_factor(91, "act/365");
    let act360 = accrual_factor(91, "act/360");
    let thirty360 = accrual_factor(91, "30/360");

    println!("Accrual (act/365): {:.6}", act365);
    println!("Accrual (act/360): {:.6}", act360);
    println!("Accrual (30/360): {:.6}", thirty360);

    // ✅ 이자 지급 계산 예시
    let notional = 1_000_000.0;
    let rate = 0.05;
    let interest_payment = notional * rate * act365;
    println!("Interest Payment: ${:.2}", interest_payment);
}
