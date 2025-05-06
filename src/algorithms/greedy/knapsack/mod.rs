fn fractional_knapsack(capacity: f64, weights: &[f64], profits: &[f64]) -> (f64, Vec<f64>) {
    // 물건의 개수
    let n = weights.len();
    
    // 각 물건의 단위 무게당 이익 계산 및 인덱스와 함께 저장
    let mut items: Vec<(usize, f64)> = (0..n)
        .map(|i| (i, profits[i] / weights[i]))
        .collect();
    
    // 단위 무게당 이익이 높은 순으로 정렬
    items.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    let mut total_profit = 0.0;
    let mut remaining_capacity = capacity;
    
    // 결과를 저장할 배열 (각 물건을 얼마나 선택했는지)
    let mut fractions = vec![0.0; n];
    
    // 탐욕적 선택: 단위 무게당 이익이 높은 순으로 물건 선택
    for (i, ratio) in items {
        if weights[i] <= remaining_capacity {
            // 물건 전체를 선택
            fractions[i] = 1.0;
            total_profit += profits[i];
            remaining_capacity -= weights[i];
        } else {
            // 물건의 일부만 선택
            fractions[i] = remaining_capacity / weights[i];
            total_profit += profits[i] * fractions[i];
            remaining_capacity = 0.0;
            break;
        }
    }
    
    (total_profit, fractions)
}

pub fn example() {
    let capacity = 10.0;
    let weights = [5.0, 4.0, 3.0, 4.0];  // w1, w2, w3, w4
    let profits = [18.0, 20.0, 9.0, 25.0];  // p1, p2, p3, p4
    
    let (total_profit, fractions) = fractional_knapsack(capacity, &weights, &profits);
    
    println!("최대 이익: {:.1}", total_profit);
    println!("선택한 물건의 비율:");
    for (i, frac) in fractions.iter().enumerate() {
        if *frac > 0.0 {
            println!("물건 {}: {:.1}%", i + 1, frac * 100.0);
        }
    }
}