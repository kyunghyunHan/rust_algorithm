fn solution(input: String) -> bool {
    let mut c = input.chars().collect::<Vec<char>>();
    let mut result = false;

    let mut rev = c.iter().rev().map(|x| *x).collect::<Vec<char>>();

    if c == rev {
        result = true;
    }

    result
}
pub fn example() {
    let input = "Hello World";

    println!("{}", solution(input.to_string()));
}
