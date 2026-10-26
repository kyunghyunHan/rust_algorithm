/*
State Pattern — 상태 패턴

객체가 같은 명령을 받아도 현재 상태에 따라 다르게 행동하도록,
각 상태의 동작과 전환 규칙을 별도 구조체로 분리하는 설계 패턴이다.

핵심:
- 상태별 동작을 공통 인터페이스(trait)로 정의한다.
- 각 상태는 해당 인터페이스를 구현한다.
- 객체는 현재 상태를 보관하고, 상태별 작업을 그 상태 객체에 위임한다.
- 현재 상태 객체를 교체하면 같은 명령에 대한 행동도 달라진다.
- 상태 전환은 상태 객체나 콘텍스트가 결정할 수 있다.

예시 — 선풍기:
- 정지 상태에서 ON  → 송풍으로 전환
- 송풍 상태에서 ON  → 송풍 유지
- 송풍 상태에서 OFF → 정지로 전환

사용 목적:
- 여러 메서드에 반복되는 상태별 조건문을 줄인다.
- 특정 상태의 동작과 전환 규칙을 한곳에 모아 유지보수를 쉽게 한다.

주의:
- 실제 객체의 타입이 바뀌는 것이 아니라, 내부 상태 객체가 교체된다.
- 상태가 적고 단순하면 enum + match로도 충분하다.
*/

use std::io::{self, Write};

// ============================================================
// 1. 선풍기 상태 패턴
// ============================================================

trait FanState {
    // Some: 다음 상태로 변경
    // None: 현재 상태 유지
    fn on_button(&self) -> Option<Box<dyn FanState>>;
    fn off_button(&self) -> Option<Box<dyn FanState>>;
}

struct Stop;

impl FanState for Stop {
    fn on_button(&self) -> Option<Box<dyn FanState>> {
        println!("정지 → 송풍");
        Some(Box::new(Wind))
    }

    fn off_button(&self) -> Option<Box<dyn FanState>> {
        println!("이미 정지 상태입니다.");
        None
    }
}

struct Wind;

impl FanState for Wind {
    fn on_button(&self) -> Option<Box<dyn FanState>> {
        println!("이미 송풍 상태입니다.");
        None
    }

    fn off_button(&self) -> Option<Box<dyn FanState>> {
        println!("송풍 → 정지");
        Some(Box::new(Stop))
    }
}

struct ElectricFan {
    state: Box<dyn FanState>,
}

impl ElectricFan {
    fn new() -> Self {
        Self {
            state: Box::new(Stop),
        }
    }

    fn on_push(&mut self) {
        if let Some(next_state) = self.state.on_button() {
            self.state = next_state;
        }
    }

    fn off_push(&mut self) {
        if let Some(next_state) = self.state.off_button() {
            self.state = next_state;
        }
    }
}

fn fan_ex() {
    println!("=== 선풍기 예제 ===");

    let mut fan = ElectricFan::new();

    fan.off_push();
    fan.on_push();
    fan.on_push();
    fan.off_push();
}

// ============================================================
// 2. 음악 플레이어 상태 패턴
// 실제 오디오 재생이 아닌 상태 전환 예제
// ============================================================

struct Track {
    title: String,
    duration: u32,
    cursor: u32,
}

impl Track {
    fn new(title: &str, duration: u32) -> Self {
        Self {
            title: title.to_string(),
            duration,
            cursor: 0,
        }
    }
}

// 재생 데이터와 기본 조작을 담당
struct Player {
    playlist: Vec<Track>,
    current_track: usize,
    volume: u8,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            playlist: vec![
                Track::new("Track 1", 180),
                Track::new("Track 2", 165),
                Track::new("Track 3", 197),
                Track::new("Track 4", 205),
            ],
            current_track: 0,
            volume: 25,
        }
    }
}

impl Player {
    fn next_track(&mut self) {
        if self.playlist.is_empty() {
            return;
        }

        self.current_track =
            (self.current_track + 1) % self.playlist.len();
    }

    fn prev_track(&mut self) {
        if self.playlist.is_empty() {
            return;
        }

        self.current_track =
            (self.current_track + self.playlist.len() - 1)
                % self.playlist.len();
    }

    fn play(&mut self) {
        if let Some(track) = self.track() {
            println!("재생: {}", track.title);
        }
    }

    fn pause(&mut self) {
        if let Some(track) = self.track() {
            println!("일시정지: {}", track.title);
        }
    }

    fn rewind(&mut self) {
        if let Some(track) = self.playlist.get_mut(self.current_track) {
            track.cursor = 0;
        }
    }

    fn track(&self) -> Option<&Track> {
        self.playlist.get(self.current_track)
    }
}

// 상태가 자신을 소비하고 다음 상태를 반환한다.
trait PlayerState {
    fn play(
        self: Box<Self>,
        player: &mut Player,
    ) -> Box<dyn PlayerState>;

    fn stop(
        self: Box<Self>,
        player: &mut Player,
    ) -> Box<dyn PlayerState>;

    fn render(&self, player: &Player);
}

// 정지
struct StoppedState;

impl PlayerState for StoppedState {
    fn play(
        self: Box<Self>,
        player: &mut Player,
    ) -> Box<dyn PlayerState> {
        player.play();
        println!("정지 → 재생");

        Box::new(PlayingState)
    }

    fn stop(
        self: Box<Self>,
        _: &mut Player,
    ) -> Box<dyn PlayerState> {
        println!("이미 정지 상태입니다.");

        self
    }

    fn render(&self, player: &Player) {
        render_status("정지", player);
    }
}

// 일시정지
struct PausedState;

impl PlayerState for PausedState {
    fn play(
        self: Box<Self>,
        player: &mut Player,
    ) -> Box<dyn PlayerState> {
        player.play();
        println!("일시정지 → 재생");

        Box::new(PlayingState)
    }

    fn stop(
        self: Box<Self>,
        player: &mut Player,
    ) -> Box<dyn PlayerState> {
        player.rewind();
        println!("일시정지 → 정지");

        Box::new(StoppedState)
    }

    fn render(&self, player: &Player) {
        render_status("일시정지", player);
    }
}

// 재생
struct PlayingState;

impl PlayerState for PlayingState {
    fn play(
        self: Box<Self>,
        player: &mut Player,
    ) -> Box<dyn PlayerState> {
        player.pause();
        println!("재생 → 일시정지");

        Box::new(PausedState)
    }

    fn stop(
        self: Box<Self>,
        player: &mut Player,
    ) -> Box<dyn PlayerState> {
        player.pause();
        player.rewind();
        println!("재생 → 정지");

        Box::new(StoppedState)
    }

    fn render(&self, player: &Player) {
        render_status("재생", player);
    }
}

// 모든 상태에서 공통으로 제공하는 조작
impl dyn PlayerState {
    fn next(
        self: Box<Self>,
        player: &mut Player,
    ) -> Box<dyn PlayerState> {
        player.next_track();
        println!("다음 곡 선택");

        // 현재 재생 상태 유지
        self
    }

    fn prev(
        self: Box<Self>,
        player: &mut Player,
    ) -> Box<dyn PlayerState> {
        player.prev_track();
        println!("이전 곡 선택");

        self
    }
}

fn render_status(state: &str, player: &Player) {
    match player.track() {
        Some(track) => {
            println!(
                "[{}] {} | 위치: {}초 / {}초 | 볼륨: {}%",
                state,
                track.title,
                track.cursor,
                track.duration,
                player.volume
            );
        }
        None => println!("[{}] 재생 목록이 비어 있습니다.", state),
    }
}

// 사용자 명령
enum Command {
    Play,
    Stop,
    Prev,
    Next,
}

// 콘텍스트: 플레이어 데이터와 현재 상태를 보관
struct PlayerApplication {
    player: Player,
    state: Box<dyn PlayerState>,
}

impl PlayerApplication {
    fn new() -> Self {
        Self {
            player: Player::default(),
            state: Box::new(StoppedState),
        }
    }

    // self를 받아 이전 상태를 이동시킨 뒤,
    // 다음 상태를 넣은 애플리케이션을 반환한다.
    fn execute(self, command: Command) -> Self {
        let Self {
            mut player,
            state,
        } = self;

        let state = match command {
            Command::Play => state.play(&mut player),
            Command::Stop => state.stop(&mut player),
            Command::Prev => state.prev(&mut player),
            Command::Next => state.next(&mut player),
        };

        Self { player, state }
    }

    fn render(&self) {
        self.state.render(&self.player);
    }
}

fn music_player_ex() -> io::Result<()> {
    println!("\n=== 음악 플레이어 예제 ===");
    println!("p: 재생/일시정지");
    println!("s: 정지");
    println!("n: 다음 곡");
    println!("b: 이전 곡");
    println!("q: 종료");

    let mut app = PlayerApplication::new();
    app.render();

    loop {
        print!("\n명령 입력 > ");
        io::stdout().flush()?;

        let mut input = String::new();

        // 입력 스트림이 끝나면 종료
        if io::stdin().read_line(&mut input)? == 0 {
            break;
        }

        let command = match input.trim() {
            "p" => Command::Play,
            "s" => Command::Stop,
            "n" => Command::Next,
            "b" => Command::Prev,
            "q" => break,
            _ => {
                println!("p / s / n / b / q 중 입력하세요.");
                continue;
            }
        };

        app = app.execute(command);
        app.render();
    }

    Ok(())
}

fn main() -> io::Result<()> {
    fan_ex();
    music_player_ex()
}