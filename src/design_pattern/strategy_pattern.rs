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

trait Attack {
    fn motion(&self);
}
struct Sing;
impl Attack for Sing {
    fn motion(&self) {
        println!("공격 -노래하기");
    }
}
struct Flame;
impl Attack for Flame {
    fn motion(&self) {
        println!("공격 - 불꽃");
    }
}

struct MillionVolt;
impl Attack for MillionVolt {
    fn motion(&self) {
        println!("공격 - 백만");
    }
}
trait Passive {
    fn detail(&self);
}
struct Defensibiliy;
impl Passive for Defensibiliy {
    fn detail(&self) {
        println!("패시브 - 방어");
    }
}
struct Speedability;
impl Passive for Speedability {
    fn detail(&self) {
        println!("패시브 - 스피드");
    }
}
struct Avoidability;
impl Passive for Avoidability {
    fn detail(&self) {
        println!("패시브 - 스피드");
    }
}
struct Pokemon {
    name: String,
    element: String,
    attack: Box<dyn Attack>,
    passive: Box<dyn Passive>,
}
impl Pokemon {
    fn new(name: &str, element: &str, attack: Box<dyn Attack>, passive: Box<dyn Passive>) -> Self {
        Self {
            name: name.to_string(),
            element: element.to_string(),
            attack,
            passive,
        }
    }
    fn name(&self) {
        println!("이름: {}, 속성: {}", self.name, self.element);
    }

    fn introduce(&self) {
        self.name();
        self.attack.motion();
        self.passive.detail();
    }
    fn set_attack(&mut self, attack: Box<dyn Attack>) {
        self.attack = attack;
    }

    fn set_passive(&mut self, passive: Box<dyn Passive>) {
        self.passive = passive;
    }
    fn pairi() -> Self {
        Self::new("파이리", "불", Box::new(Flame), Box::new(Defensibiliy))
    }

    // 코드 7-8: 푸린 생성
    fn purin() -> Self {
        Self::new("푸린", "노멀", Box::new(Sing), Box::new(Avoidability))
    }

    // 코드 7-9: 피카츄 생성
    fn pikachu() -> Self {
        Self::new(
            "피카츄",
            "번개",
            Box::new(MillionVolt),
            Box::new(Speedability),
        )
    }
}

fn example() {
    let pikache = Pokemon::pikachu();
    pikache.introduce();
}
