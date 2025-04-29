use rand::Rng;

fn generate_pension_lottery_ticket() -> String {
    let mut rng = rand::thread_rng();
    
    // 000000부터 999999까지의 6자리 번호 생성
    format!("{:06}", rng.gen_range(0..=999999))
}

pub fn example() {
    let mut lottery_tickets = Vec::new();

    // 1~5회차 (1조~5조) 번호 생성
    for i in 1..=5 {
        let number = generate_pension_lottery_ticket();
        lottery_tickets.push((format!("{}조", i), number));
    }
    
    // 보너스 번호 생성 (6조로 표시)
    let bonus_number = generate_pension_lottery_ticket();
    lottery_tickets.push((String::from("보너스조"), bonus_number));

    // 결과 출력
    for (group, number) in lottery_tickets {
        println!("{}: {}", group, number);
    }
}
