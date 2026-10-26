/*
객체 생성·제공 관련 예제 3가지

1. 정렬
   - SortPolicy: 정렬 전략 인터페이스
   - SortFactory: 미리 만든 전략 객체를 보관하고 제공
   - User: 제공받은 전략으로 정렬
   - 전략 패턴 + 정책 레지스트리 구조
   - 구현체별 생성 메서드를 재정의하는 Factory Method와는 다르다.

2. 다이얼로그
   - Button: 제품 인터페이스
   - Dialog: 크리에이터
   - create_button(): 팩토리 메서드
   - 공통 render() 흐름에서 구현체별 버튼을 생성
   - Box<dyn Button>을 이용한 동적 디스패치

3. 미로
   - Room: 제품 인터페이스
   - MazeGame: 크리에이터
   - rooms(): 방들을 제공하는 팩토리 메서드 역할
   - 공통 play() 흐름에서 구현체별 방을 사용
   - 연관 타입과 제네릭을 이용한 정적 디스패치
*/

fn main() {
    println!("=== 1. 정렬 ===");
    sorting::example();

    println!("\n=== 2. 다이얼로그 ===");
    dialog::example();

    println!("\n=== 3. 미로 ===");
    maze::example();
}

// ============================================================
// 1. 정렬 전략 + 정책 제공 팩토리
// ============================================================

mod sorting {
    trait SortPolicy {
        fn sort(&self, numbers: &mut [i32]);
    }

    struct BubbleSort;

    impl SortPolicy for BubbleSort {
        fn sort(&self, numbers: &mut [i32]) {
            for end in (1..numbers.len()).rev() {
                let mut swapped = false;

                for i in 0..end {
                    if numbers[i] > numbers[i + 1] {
                        numbers.swap(i, i + 1);
                        swapped = true;
                    }
                }

                if !swapped {
                    break;
                }
            }
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum SortType {
        Bubble,
    }

    // 정책 객체를 생성·보관하고 참조로 제공한다.
    struct SortFactory {
        policies: Vec<(SortType, Box<dyn SortPolicy>)>,
    }

    impl SortFactory {
        fn new() -> Self {
            Self {
                policies: vec![
                    (SortType::Bubble, Box::new(BubbleSort)),
                ],
            }
        }

        fn get_sort_policy(
            &self,
            sort_type: SortType,
        ) -> Option<&dyn SortPolicy> {
            self.policies
                .iter()
                .find(|(kind, _)| *kind == sort_type)
                .map(|(_, policy)| policy.as_ref())
        }
    }

    struct User<'a> {
        factory: &'a SortFactory,
    }

    impl<'a> User<'a> {
        fn new(factory: &'a SortFactory) -> Self {
            Self { factory }
        }

        fn sort_numbers(
            &self,
            numbers: &mut [i32],
            sort_type: SortType,
        ) -> Result<(), &'static str> {
            let policy = self
                .factory
                .get_sort_policy(sort_type)
                .ok_or("지원하지 않는 정렬 종류입니다")?;

            policy.sort(numbers);

            Ok(())
        }
    }

    pub fn example() {
        let factory = SortFactory::new();
        let user = User::new(&factory);

        let mut numbers = vec![5, 2, 9, 1, 3];

        println!("정렬 전: {:?}", numbers);

        match user.sort_numbers(&mut numbers, SortType::Bubble) {
            Ok(()) => println!("정렬 후: {:?}", numbers),
            Err(message) => eprintln!("오류: {message}"),
        }
    }
}

// ============================================================
// 2. 팩토리 메서드 — 다이얼로그
// 실제 GUI가 아닌 콘솔 출력 예제
// ============================================================

mod dialog {
    // 제품 인터페이스
    trait Button {
        fn render(&self);
        fn on_click(&self);
    }

    // 크리에이터
    trait Dialog {
        // 생성할 버튼 종류는 각 구현체가 결정한다.
        fn create_button(&self) -> Box<dyn Button>;

        // 공통 작업 흐름
        fn render(&self) {
            let button = self.create_button();
            button.render();
        }

        fn refresh(&self) {
            println!("Dialog - Refresh");
        }
    }

    // HTML 제품
    struct HtmlButton;

    impl Button for HtmlButton {
        fn render(&self) {
            println!("<button>Test Button</button>");

            // 실제 클릭 이벤트 대신 즉시 호출하여 시연
            self.on_click();
        }

        fn on_click(&self) {
            println!("Click! Hello, HTML!");
        }
    }

    // HTML 크리에이터
    struct HtmlDialog;

    impl Dialog for HtmlDialog {
        fn create_button(&self) -> Box<dyn Button> {
            Box::new(HtmlButton)
        }
    }

    // Windows 제품
    struct WindowsButton;

    impl Button for WindowsButton {
        fn render(&self) {
            println!("Drawing a Windows button");
            self.on_click();
        }

        fn on_click(&self) {
            println!("Click! Hello, Windows!");
        }
    }

    // Windows 크리에이터
    struct WindowsDialog;

    impl Dialog for WindowsDialog {
        fn create_button(&self) -> Box<dyn Button> {
            Box::new(WindowsButton)
        }
    }

    fn initialize() -> Box<dyn Dialog> {
        // 빌드 대상 OS에 따라 크리에이터 선택
        if cfg!(target_os = "windows") {
            Box::new(WindowsDialog)
        } else {
            Box::new(HtmlDialog)
        }
    }

    pub fn example() {
        let dialog = initialize();

        // 구체적인 다이얼로그 타입을 몰라도 사용 가능
        dialog.render();
        dialog.refresh();
    }
}

// ============================================================
// 3. 팩토리 메서드 — 미로
// ============================================================

mod maze {
    // 제품 인터페이스
    trait Room {
        fn render(&self);
    }

    // 크리에이터
    trait MazeGame {
        // 각 미로 구현체가 사용하는 구체적인 방 타입
        type RoomImpl: Room;

        // 원본 예제처럼 보관된 방을 복제해서 제공한다.
        fn rooms(&self) -> Vec<Self::RoomImpl>;

        // 공통 작업 흐름
        fn play(&self) {
            for room in self.rooms() {
                room.render();
            }
        }
    }

    // ── 마법 미로 ──

    #[derive(Clone)]
    struct MagicRoom {
        title: String,
    }

    impl MagicRoom {
        fn new(title: &str) -> Self {
            Self {
                title: title.to_string(),
            }
        }
    }

    impl Room for MagicRoom {
        fn render(&self) {
            println!("Magic Room: {}", self.title);
        }
    }

    struct MagicMaze {
        rooms: Vec<MagicRoom>,
    }

    impl MagicMaze {
        fn new() -> Self {
            Self {
                rooms: vec![
                    MagicRoom::new("Infinite Room"),
                    MagicRoom::new("Red Room"),
                ],
            }
        }
    }

    impl MazeGame for MagicMaze {
        type RoomImpl = MagicRoom;

        fn rooms(&self) -> Vec<Self::RoomImpl> {
            self.rooms.clone()
        }
    }

    // ── 일반 미로 ──

    #[derive(Clone)]
    struct OrdinaryRoom {
        id: u32,
    }

    impl OrdinaryRoom {
        fn new(id: u32) -> Self {
            Self { id }
        }
    }

    impl Room for OrdinaryRoom {
        fn render(&self) {
            println!("Ordinary Room: #{}", self.id);
        }
    }

    struct OrdinaryMaze {
        rooms: Vec<OrdinaryRoom>,
    }

    impl OrdinaryMaze {
        fn new() -> Self {
            Self {
                rooms: vec![
                    OrdinaryRoom::new(1),
                    OrdinaryRoom::new(2),
                ],
            }
        }
    }

    impl MazeGame for OrdinaryMaze {
        type RoomImpl = OrdinaryRoom;

        fn rooms(&self) -> Vec<Self::RoomImpl> {
            let mut rooms = self.rooms.clone();
            rooms.reverse();
            rooms
        }
    }

    // 제네릭으로 크리에이터를 받아 실행
    fn run(maze_game: impl MazeGame) {
        println!("Loading resources...");
        println!("Starting the game...");

        maze_game.play();
    }

    pub fn example() {
        run(OrdinaryMaze::new());
        run(MagicMaze::new());
    }
}