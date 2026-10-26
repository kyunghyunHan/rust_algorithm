/*
싱글턴 패턴 (Singleton)

■ 목적
- 프로그램에서 특정 타입의 인스턴스를 하나만 사용하도록 제한한다.
- 프로그램의 여러 곳에서 그 인스턴스에 접근할 수 있는 전역 접근 지점을 제공한다.

■ 동작 방식
1. 처음 요청하면 객체를 생성해서 보관한다.
2. 이후 요청에는 새 객체를 만들지 않고 기존 객체의 참조를 반환한다.
3. 외부에서는 객체를 마음대로 추가 생성하지 못하도록 제한한다.

예:
let a = Database::get_instance(); // 처음 요청: 객체 생성
let b = Database::get_instance(); // 이후 요청: 기존 객체 참조 반환

a와 b는 서로 다른 변수지만 같은 Database 객체를 가리킨다.
즉, 참조는 여러 개여도 실제 객체는 하나다.

■ Rust 구현 요소
- 비공개 생성 함수와 비공개 필드:
  모듈 밖에서 객체를 직접 생성하지 못하도록 제한한다.
- static:
  프로그램 전체에서 공유하는 인스턴스 저장 공간을 둔다.
- OnceLock:
  여러 스레드가 동시에 요청해도 초기화를 한 번만 수행한다.
- get_instance():
  저장된 객체의 참조를 반환하는 공개 연관 함수다.
- get_or_init():
  저장된 객체가 없으면 생성하고, 있으면 기존 객체를 반환한다.

※ Rust에서 new()는 일반적인 연관 함수다.
   이름이 new라고 해서 반드시 새 객체를 만들어야 하는 것은 아니다.
   이 예제에서는 new()가 생성하고 get_instance()가 공유를 담당한다.

■ 지연 초기화
- 프로그램 시작 시 바로 생성하지 않고 처음 요청할 때 생성한다.
- 위 구현은 지연 초기화를 사용하지만,
  지연 초기화 자체가 싱글턴의 필수 조건은 아니다.

■ 사용 예
- 프로그램 전체에서 공유하는 설정 객체
- 공통 로그 관리자
- 공유 데이터베이스 연결 풀 관리자

※ 공유 관리자 하나가 내부적으로 여러 DB 연결을 관리할 수도 있다.

■ 장점
- 모든 사용자가 같은 인스턴스를 사용한다.
- 여러 곳에서 공통 접근 함수를 통해 접근할 수 있다.
- 지연 초기화를 사용하면 실제로 필요할 때 생성할 수 있다.

■ 단점
- 본래 기능 외에 인스턴스 생성과 관리까지 책임져
  단일 책임 원칙(SRP)을 위반할 수 있다.
- 전역 접근 때문에 코드의 의존관계가 드러나지 않을 수 있다.
- 공유 상태가 테스트 사이에 남거나 대체하기 어려워
  테스트가 복잡해질 수 있다.
- 내부 값을 변경한다면 별도의 동기화가 필요할 수 있다.

※ OnceLock은 한 번만 초기화하는 것을 보장한다.
   객체 내부 상태의 변경까지 자동으로 동기화하지는 않는다.
   공유 상태를 변경하려면 Mutex 등의 수단을 사용한다.
*/

use std::sync::OnceLock;

mod database {
    use super::OnceLock;

    // 프로그램 전체에서 공유하는 인스턴스 저장소
    static INSTANCE: OnceLock<Database> = OnceLock::new();

    pub struct Database {
        // 비공개 필드: 모듈 밖에서 직접 생성하지 못하게 함
        connection_url: String,
    }

    impl Database {
        // 비공개 생성 함수
        fn new() -> Self {
            println!("데이터베이스 객체 생성");

            Self {
                connection_url: String::from("postgres://localhost/mydb"),
            }
        }

        // 객체를 처음 요청할 때 생성하고,
        // 이후에는 이미 생성된 객체의 참조를 반환
        pub fn get_instance() -> &'static Self {
            INSTANCE.get_or_init(Self::new)
        }

        // 공유 참조로 호출할 수 있는 기능
        pub fn query(&self, sql: &str) {
            println!("[{}] 쿼리 실행: {}", self.connection_url, sql);
        }
    }
}

use database::Database;

fn main() {
    let foo = Database::get_instance();
    foo.query("SELECT * FROM users");

    let bar = Database::get_instance();
    bar.query("SELECT * FROM products");

    // 두 참조가 같은 객체를 가리키는지 확인
    println!("같은 객체인가? {}", std::ptr::eq(foo, bar));
}