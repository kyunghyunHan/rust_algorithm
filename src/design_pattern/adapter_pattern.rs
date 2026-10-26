/*
Adapter Pattern — 어댑터 패턴

기존 객체의 인터페이스가 사용하는 쪽과 맞지 않을 때,
중간 객체가 호출 방식과 데이터를 변환하여 연결하는 구조 패턴이다.

이 예제:
- 클라이언트는 Target의 request()를 사용한다.
- SpecificTarget은 specific_request()만 제공한다.
- TargetAdapter가 기존 객체를 감싸서 Target을 구현한다.
- 어댑터는 기존 메서드를 호출하고, 결과 문자열을 뒤집어 반환한다.

1. 기존 객체가 원하는 인터페이스와 안 맞는다.
2. 새 구조체인 어댑터를 만든다.
3. 어댑터 안에 기존 객체를 보관한다.
4. 어댑터가 사용하는 쪽에서 원하는 메서드를 제공한다.
5. 그 메서드 안에서 기존 객체를 호출하고 필요한 변환을 한다.
*/

// ============================================================
// 1. 클라이언트가 기대하는 인터페이스
// ============================================================

trait Target {
    fn request(&self) -> String;
}

// ============================================================
// 2. 이미 인터페이스가 맞는 객체
// ============================================================

struct OrdinaryTarget;

impl Target for OrdinaryTarget {
    fn request(&self) -> String {
        "Ordinary request.".to_string()
    }
}

// ============================================================
// 3. 인터페이스가 맞지 않는 기존 객체
// ============================================================

struct SpecificTarget;

impl SpecificTarget {
    fn specific_request(&self) -> String {
        ".tseuqer cificepS".to_string()
    }
}

// ============================================================
// 4. 어댑터
// ============================================================

struct TargetAdapter {
    // 기존 객체를 내부에 보관한다.
    adaptee: SpecificTarget,
}

impl TargetAdapter {
    fn new(adaptee: SpecificTarget) -> Self {
        Self { adaptee }
    }
}

impl Target for TargetAdapter {
    fn request(&self) -> String {
        // 기존 객체의 기능을 호출한다.
        let result = self.adaptee.specific_request();

        // 결과를 클라이언트가 원하는 형식으로 변환한다.
        result.chars().rev().collect()
    }
}

// ============================================================
// 5. 클라이언트
// ============================================================

// Target을 구현한 타입을 받는다.
// 구체적인 타입마다 사용법을 따로 알 필요가 없다.
fn call(target: impl Target) {
    println!("{}", target.request());
}

fn main() {
    // OrdinaryTarget은 Target을 구현했으므로 바로 사용 가능
    println!("=== 호환되는 객체 ===");
    call(OrdinaryTarget);

    let adaptee = SpecificTarget;

    println!("\n=== 기존 객체의 원래 결과 ===");
    println!("{}", adaptee.specific_request());

    // SpecificTarget은 Target을 구현하지 않았으므로 불가능
    // call(adaptee);

    // 기존 객체를 어댑터에 넣는다.
    let adapter = TargetAdapter::new(adaptee);

    println!("\n=== 어댑터를 통한 결과 ===");
    call(adapter);
}