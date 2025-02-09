fn solution(p: String, n: i32) -> String {
    let mut answer: String;

    let mut hour = p[3..5].parse::<i32>().unwrap();
    let mut min = p[6..8].parse::<i32>().unwrap();
    let mut sec = p[9..11].parse::<i32>().unwrap();
    println!("{}", hour);
    println!("{}", min);
    println!("{}", sec);

    if &p[0..2] == "PM" && hour != 12 {
        hour += 12;
    }

    if &p[0..2] == "AM" && hour == 12 {
        hour = 0;
    }

    sec += n;

    // 초가 60 이상이면 분으로 변환
    min += sec / 60;
    sec = sec % 60;

    // 분이 60 이상이면 시로 변환
    hour += min / 60;
    min = min % 60;

    // 시가 24 이상이면 24로 나눈 나머지
    hour = hour % 24;

    // let mut answer = format!("{:0}:{:02}:{:02}", hour, min, sec);

    answer = if hour < 10 { "0" } else { "" }.to_string() + &hour.to_string() + ":";
    answer += &(if min < 10 { "0" } else { "" }.to_string() + &min.to_string() + ":");
    answer += &(if sec < 10 { "0" } else { "" }.to_string() + &sec.to_string());
    answer
}
fn main() {
    let x = "PM 01:00:00";
    let n = 10;
    println!("{}", solution(x.to_string(), n));
}
