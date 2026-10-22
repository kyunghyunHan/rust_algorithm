/*
strategy_pattern


같은 작업을 수행하는 여러 방법을 만들고 필요한 방법을 선택해 설계하는방법
자주 바뀌는 것이 Method이라면 메서드를 클래스로 바꾸고 interface타입의 상속 구조를 만들어라
*/
trait CoolingStrategy {
    fn fan_power(&self, temperature: f32) -> u8;
}

struct OnOff;

impl CoolingStrategy for OnOff {
    fn fan_power(&self, temperature: f32) -> u8 {
        if temperature >= 40.0 {
            100
        } else {
            0
        }
    }
}
struct Proportional;

impl CoolingStrategy for Proportional {
    fn fan_power(&self, temperature: f32) -> u8 {
        // 40℃ 이하면 0%, 초과 1℃마다 10%, 최대 100%
        ((temperature - 40.0) * 10.0).clamp(0.0, 100.0) as u8
    }
}
fn run(strategy: &dyn CoolingStrategy, temperature: f32) {
    println!("팬 출력: {}%", strategy.fan_power(temperature));
}

fn main() {
    run(&OnOff, 43.0); // 팬 출력: 100%
    run(&Proportional, 43.0); // 팬 출력: 30%
}
