use std::collections::HashMap;

fn solution(movie: Vec<String>) -> Vec<String> {
    let mut answer: Vec<String>;
    let mut frequency: HashMap<String, i32> = HashMap::new();

    for word in &movie {
        *frequency.entry(word.clone()).or_insert(0) += 1;
    }

    answer = frequency.keys().cloned().collect();

    answer.sort_by(|a, b| {
        let freq_a = frequency.get(a).unwrap();
        let freq_b = frequency.get(b).unwrap();
        
        if freq_a != freq_b {
            freq_b.cmp(freq_a)  // 빈도수 내림차순
        } else {
            a.cmp(b)  // 같은 빈도수일 때 문자열 사전순
        }
    });

    answer
}

fn main() {
    let movie = vec![
        "spy".to_string(),
        "ray".to_string(),
        "spy".to_string(),
        "room".to_string(),
        "once".to_string(),
        "ray".to_string(),
        "spy".to_string(),
        "once".to_string(),
    ];

    let result = solution(movie);

    for s in result {
        print!("{} ", s);
    }
    println!();
}
