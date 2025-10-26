/// 선도계약 유틸리티 (무배당 / 연속복리 금리 r 버전 포함)
/// 필요에 따라 f64로 단순 계산

/// 만기(T)에서의 페이오프: Long = S_T - K, Short = K - S_T
pub fn payoff_long_forward(s_t: f64, k: f64) -> f64 {
    s_t - k
}

pub fn payoff_short_forward(s_t: f64, k: f64) -> f64 {
    k - s_t
}

/// 무배당, 연속복리 금리 r일 때 선도가격 F(t,T) = S_t * exp(r * tau)
pub fn forward_price_no_div(s_t: f64, r: f64, tau: f64) -> f64 {
    s_t * (r * tau).exp()
}

/// 배당수익률 q가 있을 때: F(t,T) = S_t * exp((r - q) * tau)
pub fn forward_price_with_yield(s_t: f64, r: f64, q: f64, tau: f64) -> f64 {
    s_t * ((r - q) * tau).exp()
}

/// 장포지션의 현재가치: V_long = S_t - K * exp(-r * tau)
/// (무배당 가정; 배당 있으면 S_t 대신 배당 현재가치 차감/수익률 형태로 보정)
pub fn value_long_forward_no_div(s_t: f64, k: f64, r: f64, tau: f64) -> f64 {
    s_t - k * (-r * tau).exp()
}

/// 같은 값을 F로 표현하면: V_long = exp(-r * tau) * (F - K)
pub fn value_long_from_forward_price(f_t: f64, k: f64, r: f64, tau: f64) -> f64 {
    (-r * tau).exp() * (f_t - k)
}

/// 단포지션 현재가치 (무배당): V_short = K * exp(-r * tau) - S_t
pub fn value_short_forward_no_div(s_t: f64, k: f64, r: f64, tau: f64) -> f64 {
    k * (-r * tau).exp() - s_t
}

/// === 본문 예시 검증 ===
/// 금리 0, 주가 항상 100, 배당 없음 → F = 100, V = 100 - K (모든 t)
pub fn example_value_with_zero_rate_and_constant_price(k: f64) -> (f64, f64) {
    let f = 100.0;              // F(t,T)
    let v = 100.0 - k;          // V_K(t,T)
    (f, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payoff() {
        assert!((payoff_long_forward(120.0, 100.0) - 20.0).abs() < 1e-9);
        assert!((payoff_short_forward(120.0, 100.0) - (-20.0)).abs() < 1e-9);
    }

    #[test]
    fn test_forward_price_no_div() {
        let s = 100.0;
        let r = 0.05;
        let tau = 1.0;
        let f = forward_price_no_div(s, r, tau);
        // F ≈ 100 * e^0.05
        assert!((f - (100.0 * (0.05f64).exp())).abs() < 1e-9);
    }

    #[test]
    fn test_value_long() {
        let s = 100.0;
        let r = 0.03;
        let tau = 0.5;
        let k = 102.0;

        let v1 = value_long_forward_no_div(s, k, r, tau);
        let f = forward_price_no_div(s, r, tau);
        let v2 = value_long_from_forward_price(f, k, r, tau);
        assert!((v1 - v2).abs() < 1e-9);
    }

    #[test]
    fn test_example_from_text() {
        let k = 95.0;
        let (f, v) = example_value_with_zero_rate_and_constant_price(k);
        assert!((f - 100.0).abs() < 1e-9);
        assert!((v - (100.0 - k)).abs() < 1e-9);
    }
}
