/*
Observer Pattern — 옵저버 패턴

특정 객체에서 이벤트가 발생했을 때,
등록된 여러 구독자에게 알림을 전달하는 설계 패턴이다.

구성:
- 발행자(Subject / Publisher): 구독자를 관리하고 알림을 전달한다.
- 구독자(Observer / Subscriber): 알림을 받아 처리한다.
- 클라이언트: 구독자를 등록하거나 해제한다.

아래 두 가지 구현을 비교한다.
1. trait 객체를 이용한 알림
2. 함수 포인터를 이용한 이벤트별 알림

주의:
- 이 예제의 알림은 동기적으로 순서대로 실행된다.
- Editor의 load/save는 실제 파일 작업 없이 이벤트만 발생시킨다.
*/
/*
옵저버 패턴 적용 예시 — 온도 감시 시스템

온도 센서에서 새로운 측정값을 얻으면,
온도 변경 이벤트를 구독한 모든 구독자에게 전달한다.

발행자:
- 온도 측정값을 구독자들에게 전달한다.

구독자:
- OLED: 현재 온도를 화면에 표시한다.
- 팬 제어: 온도에 따라 팬 출력을 조절한다.
- 경보: 과열 여부를 판단하여 부저를 작동시킨다.
- 로그: 측정값을 기록한다.

동작 흐름:
1. 각 구독자가 온도 변경 이벤트를 구독한다.
2. 센서에서 온도를 측정한다.
3. 발행자가 구독자들에게 측정값을 전달한다.
4. 각 구독자가 자신의 역할에 맞게 처리한다.

장점:
- 온도 측정 코드는 각 구독자의 처리 방식을 몰라도 된다.
- UART 출력 같은 기능은 새 구독자로 추가할 수 있다.

주의:
- 인터럽트에서는 이벤트만 전달하고,
  화면 갱신이나 로그 기록은 메인 루프 또는 RTOS 태스크에서 처리한다.
*/
use std::collections::HashMap;

// ============================================================
// 1. trait 객체를 이용한 옵저버
// ============================================================

trait Observer {
    fn update(&self, message: &str);
}

struct NamedObserver {
    name: String,
}

impl NamedObserver {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Observer for NamedObserver {
    fn update(&self, message: &str) {
        println!("[{}] 알림을 받았습니다: {}", self.name, message);
    }
}

struct Subject {
    observers: Vec<Box<dyn Observer>>,
}

impl Subject {
    fn new() -> Self {
        Self {
            observers: Vec::new(),
        }
    }

    fn attach(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    fn notify(&self, message: &str) {
        for observer in &self.observers {
            observer.update(message);
        }
    }
}

fn subject_ex() {
    let mut subject = Subject::new();

    subject.attach(Box::new(NamedObserver::new("옵저버1")));
    subject.attach(Box::new(NamedObserver::new("옵저버2")));

    subject.notify("주제가 변경되었습니다!");
}

// ============================================================
// 2. 함수 포인터를 이용한 이벤트별 옵저버
// ============================================================

#[derive(PartialEq, Eq, Hash)]
enum Event {
    Load,
    Save,
}

// 파일 경로를 빌려 받아 처리하는 함수
type Subscriber = fn(&str);

// 구독 하나를 식별하는 ID
#[derive(Clone, Copy, PartialEq, Eq)]
struct SubscriptionId(usize);

#[derive(Default)]
struct Publisher {
    events: HashMap<Event, Vec<(SubscriptionId, Subscriber)>>,
    next_id: usize,
}

impl Publisher {
    // 구독을 등록하고, 해제에 사용할 ID를 반환
    fn subscribe(&mut self, event: Event, listener: Subscriber) -> SubscriptionId {
        let id = SubscriptionId(self.next_id);
        self.next_id += 1;

        self.events.entry(event).or_default().push((id, listener));

        id
    }

    fn unsubscribe(&mut self, event: Event, id: SubscriptionId) {
        if let Some(listeners) = self.events.get_mut(&event) {
            listeners.retain(|(listener_id, _)| *listener_id != id);
        }
    }

    fn notify(&self, event: Event, file_path: &str) {
        // 구독자가 없으면 아무 작업도 하지 않는다.
        if let Some(listeners) = self.events.get(&event) {
            for (_, listener) in listeners {
                listener(file_path);
            }
        }
    }
}

// 이벤트를 발생시키는 편집기
#[derive(Default)]
struct Editor {
    publisher: Publisher,
    file_path: String,
}

impl Editor {
    fn events(&mut self) -> &mut Publisher {
        &mut self.publisher
    }

    fn load(&mut self, path: &str) {
        self.file_path = path.to_string();
        self.publisher.notify(Event::Load, &self.file_path);
    }

    fn save(&self) {
        self.publisher.notify(Event::Save, &self.file_path);
    }
}

fn save_listener(file_path: &str) {
    println!("이메일 알림: admin@example.com — 파일 저장: {}", file_path);
}

fn editor_ex() {
    let mut editor = Editor::default();

    // 주변 변수를 캡처하지 않는 클로저는 함수 포인터로 사용 가능
    editor.events().subscribe(Event::Load, |file_path| {
        println!("로그 기록: 파일 불러오기: {}", file_path);
    });

    let save_id = editor.events().subscribe(Event::Save, save_listener);

    editor.load("test1.txt");
    editor.load("test2.txt");
    editor.save(); // 이메일 알림 출력

    editor.events().unsubscribe(Event::Save, save_id);

    editor.save(); // 구독 해제했으므로 이메일 알림 없음
}

fn main() {
    println!("=== 기본 옵저버 ===");
    subject_ex();

    println!("\n=== 편집기 이벤트 ===");
    editor_ex();
}
