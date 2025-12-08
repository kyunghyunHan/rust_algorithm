use std::collections::HashSet;
use rand::Rng;

// 사건(Event)을 HashSet으로 설정
type Event = HashSet<i32>;

// 합사건: A ∪ B
fn union(a: &Event, b: &Event) -> Event {
    a.union(b).cloned().collect()
}

// 교사건: A ∩ B
fn intersection(a: &Event, b: &Event) -> Event {
    a.intersection(b).cloned().collect()
}

// 여사건: Aᶜ = 전체집합 - A
fn complement(universal: &Event, a: &Event) -> Event {
    universal.difference(a).cloned().collect()
}

// 공사건 여부: A == ∅
fn is_empty(a: &Event) -> bool {
    a.is_empty()
}

// 상호배타(Mutually Exclusive): A ∩ B == ∅
fn is_mutually_exclusive(a: &Event, b: &Event) -> bool {
    intersection(a, b).is_empty()
}

fn sample_space_dice(n: usize) -> f64 {
    let mut rng = rand::rng();
    let mut count_even = 0;

    for _ in 0..n {
        let x: u32 = rng.random_range(1..=6);
        if x % 2 == 0 {
            count_even += 1;
        }
    }

    count_even as f64 / n as f64
}

fn main() {
    // 전체 표본공간 (예: 주사위 1~6)
    let universal: Event = (1..=6).collect();

    // 사건 정의
    let event_a: Event = [1, 2, 3].into_iter().collect(); // 1,2,3 나오기
    let event_b: Event = [3, 4].into_iter().collect();    // 3,4 나오기
    let event_c: Event = [7].into_iter().collect();       // 공사건 예시

    // 합사건
    let a_union_b = union(&event_a, &event_b);
    println!("A ∪ B  = {:?}", a_union_b);

    // 교사건
    let a_intersect_b = intersection(&event_a, &event_b);
    println!("A ∩ B  = {:?}", a_intersect_b);

    // 여사건
    let a_complement = complement(&universal, &event_a);
    println!("Aᶜ (여사건) = {:?}", a_complement);

    // 공사건 여부
    println!("C는 공사건인가? {}", is_empty(&event_c));

    // 상호배타 여부
    println!("A와 C는 상호배타인가? {}", is_mutually_exclusive(&event_a, &event_c));
    println!("A와 B는 상호배타인가? {}", is_mutually_exclusive(&event_a, &event_b));

    // 시뮬레이션: 주사위에서 짝수가 나올 확률 추정
    let p = sample_space_dice(1_000_000);
    println!("Estimated Probability(Even) = {}", p);
}
