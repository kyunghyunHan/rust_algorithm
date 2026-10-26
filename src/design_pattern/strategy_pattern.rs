/*
Strategy Pattern — 전략 패턴

같은 작업을 수행하는 여러 알고리즘을 공통 인터페이스로 분리하고,
필요한 알고리즘을 선택해서 사용하는 설계 패턴이다.

핵심:
- 호출 방법은 같게 하고, 실제 처리 방법은 전략별로 분리한다.
- 작업을 사용하는 쪽은 직접 알고리즘을 구현하지 않고 전략에 위임한다.
- 기존 실행 코드를 크게 수정하지 않고 새로운 전략을 추가할 수 있다.

구성:
1. 전략 인터페이스: 모든 전략의 공통 동작을 정의한다. Rust에서는 trait.
2. 구체 전략: 각 알고리즘을 구현한다. Rust에서는 struct + impl.
3. 콘텍스트: 전략을 사용해 작업을 수행한다.
4. 클라이언트: 사용할 전략을 선택하고 전달한다.

예시:
- 냉각 제어: 팬 출력 계산 → OnOff / Proportional
- 포켓몬: 공격 수행 → Sing / Flame / MillionVolt
- 내비게이션: 경로 계산 → WalkingStrategy / PublicTransportStrategy

이 코드의 전략 전달 방식:
- &dyn CoolingStrategy: 전략을 빌려서 호출한다.
- Box<dyn Attack>: 전략을 소유하며 실행 중 다른 타입의 전략으로 교체할 수 있다.
- Navigator<T>: 제네릭으로 전략 타입을 정한다.
  다른 타입의 전략을 사용하려면 별도의 Navigator를 생성한다.

주의:
- 여러 알고리즘을 하나의 구현으로 합치는 것이 아니다.
- 모든 메서드를 별도 구조체로 분리할 필요는 없다.
- 선택지가 단순하고 변화가 적다면 함수나 클로저로도 충분하다.
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
fn pokemon_ex() {
    let pikache = Pokemon::pikachu();
    pikache.introduce();
}
pub fn example() {
    // pokemon_ex();
}

//conceptural
trait RouteStrategy {
    fn build_route(&self, from: &str, to: &str);
}

struct WalkingStrategy;

impl RouteStrategy for WalkingStrategy {
    fn build_route(&self, from: &str, to: &str) {
        println!("Walking route from {} to {}: 4 km, 30 min", from, to);
    }
}

struct PublicTransportStrategy;

impl RouteStrategy for PublicTransportStrategy {
    fn build_route(&self, from: &str, to: &str) {
        println!(
            "Public transport route from {} to {}: 3 km, 5 min",
            from, to
        );
    }
}

struct Navigator<T: RouteStrategy> {
    route_strategy: T,
}
impl<T: RouteStrategy> Navigator<T> {
    pub fn new(route_strategy: T) -> Self {
        Self { route_strategy }
    }

    pub fn route(&self, from: &str, to: &str) {
        self.route_strategy.build_route(from, to);
    }
}

fn navigaor_ex() {
    let navigator = Navigator::new(WalkingStrategy);
    navigator.route("Home", "Club");
    navigator.route("Club", "Work");

    let navigator = Navigator::new(PublicTransportStrategy);
    navigator.route("Home", "Club");
    navigator.route("Club", "Work");
}
