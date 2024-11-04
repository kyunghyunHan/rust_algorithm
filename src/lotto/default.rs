use rand::seq::SliceRandom;
use rand::thread_rng;

fn generate_lotto_numbers() -> Vec<u32> {
    let mut rng = thread_rng();

    // 1부터 45까지의 숫자를 생성
    let mut numbers: Vec<u32> = (1..=45).collect();

    // 무작위로 6개의 숫자를 선택하고 정렬
    numbers.as_mut_slice().shuffle(&mut rng);
    let selected_numbers = &numbers[0..6];

    // 선택된 번호를 정렬해서 반환
    let mut result = selected_numbers.to_vec();
    result.sort();
    result
}

pub fn example() {
    let lotto_numbers = generate_lotto_numbers();
    println!("Generated lotto numbers: {:?}", lotto_numbers);
}
