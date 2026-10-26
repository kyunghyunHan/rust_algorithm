/*
Decorator Pattern — 데코레이터 패턴

구조 패턴:
- 기존 객체를 감싸서 추가 기능을 붙인다.
- 기본 객체와 데코레이터는 같은 인터페이스를 구현한다.
- 데코레이터는 안쪽 객체에 작업을 위임하고,
  그 결과에 자신의 기능을 추가한다.
- 여러 데코레이터를 중첩하여 기능을 조합할 수 있다.

예시:
- 기본 객체: 빵
- 데코레이터: 치즈, 에그
- 공통 인터페이스: Toast
*/

// 기본 토스트와 토핑의 공통 인터페이스
trait Toast {
    fn description(&self) -> String;
    fn get_calories(&self) -> u32;
    fn price(&self) -> u32;
}

// ============================================================
// 기본 컴포넌트: 빵
// 칼로리와 가격은 설명을 위한 예시 값
// ============================================================

struct Bread;

impl Toast for Bread {
    fn description(&self) -> String {
        "빵".to_string()
    }

    fn get_calories(&self) -> u32 {
        200
    }

    fn price(&self) -> u32 {
        2_000
    }
}

// ============================================================
// 데코레이터: 치즈
// ============================================================

struct Cheese {
    // 빵뿐 아니라 다른 토핑으로 감싼 토스트도 받을 수 있다.
    inner: Box<dyn Toast>,
}

impl Cheese {
    fn new(inner: Box<dyn Toast>) -> Self {
        Self { inner }
    }
}

impl Toast for Cheese {
    fn description(&self) -> String {
        format!("{} + 치즈", self.inner.description())
    }

    fn get_calories(&self) -> u32 {
        // 안쪽 토스트의 칼로리 + 치즈 칼로리
        self.inner.get_calories() + 70
    }

    fn price(&self) -> u32 {
        self.inner.price() + 500
    }
}

// ============================================================
// 데코레이터: 에그
// ============================================================

struct Egg {
    inner: Box<dyn Toast>,
}

impl Egg {
    fn new(inner: Box<dyn Toast>) -> Self {
        Self { inner }
    }
}

impl Toast for Egg {
    fn description(&self) -> String {
        format!("{} + 에그", self.inner.description())
    }

    fn get_calories(&self) -> u32 {
        self.inner.get_calories() + 90
    }

    fn price(&self) -> u32 {
        self.inner.price() + 700
    }
}

// ============================================================
// 클라이언트: 구체적인 토핑 종류를 몰라도 사용 가능
// 원문의 SalaryManager와 비슷한 역할
// ============================================================

struct Order {
    toast: Box<dyn Toast>,
}

impl Order {
    fn new(toast: Box<dyn Toast>) -> Self {
        Self { toast }
    }

    fn print_receipt(&self) {
        println!("메뉴: {}", self.toast.description());
        println!("칼로리: {} kcal", self.toast.get_calories());
        println!("가격: {}원", self.toast.price());
    }
}

fn main() {
    // 1. 기본 빵
    let mut toast: Box<dyn Toast> = Box::new(Bread);

    // 2. 기존 빵을 치즈로 감싸기
    toast = Box::new(Cheese::new(toast));

    // 3. 치즈 토스트를 에그로 감싸기
    toast = Box::new(Egg::new(toast));

    // 구조: Egg → Cheese → Bread
    let order = Order::new(toast);
    order.print_receipt();

    println!();

    // 설정에 따라 토핑 조합
    let add_cheese = false;
    let add_egg = true;

    let mut toast: Box<dyn Toast> = Box::new(Bread);

    if add_cheese {
        toast = Box::new(Cheese::new(toast));
    }

    if add_egg {
        toast = Box::new(Egg::new(toast));
    }

    let order = Order::new(toast);
    order.print_receipt();
}