use rand::Rng;

/// 파생상품의 지급(payoff) 함수
fn weather_derivative_payoff(snowfall: f64, threshold: f64, payout: f64) -> f64 {
    if snowfall > threshold {
        payout
    } else {
        0.0
    }
}

/// 파생상품의 현재가치 계산 (단순 할인)
fn present_value(payoff: f64, rate: f64, t: f64) -> f64 {
    payoff / (1.0 + rate).powf(t)
}

pub fn example() {
    let threshold = 50.0;  // 50인치 기준
    let payout = 1.0;      // 지급금액 $1
    let rate = 0.05;       // 할인율 5%
    let t = 1.0;           // 1년

    let mut rng = rand::thread_rng();
    let mut total_value = 0.0;
    let simulations = 100_000;

    for _ in 0..simulations {
        // 무작위로 눈의 양을 정규분포 근사로 샘플링 (단순화)
        let snowfall = rng.gen_range(0.0..100.0);
        let payoff = weather_derivative_payoff(snowfall, threshold, payout);
        let pv = present_value(payoff, rate, t);
        total_value += pv;
    }

    let expected_value = total_value / simulations as f64;
    println!("Expected present value of derivative ≈ ${:.4}", expected_value);
}
