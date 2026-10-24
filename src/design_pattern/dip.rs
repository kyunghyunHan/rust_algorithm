/*
DIP

클라이언트는 구채 클래스가 아닌 추상클래스에 의존해야한다

*/


trait Games {
    fn return_title(&self) -> &str;
    fn return_version(&self) -> &str;

    // Java의 Games.Start()에 해당하는 공통 구현
    fn start(&self) {
        println!("게임명 : {}", self.return_title());
        println!("버전  : {}", self.return_version());
        println!("{} 을 시작합니다.\n", self.return_title());
    }
}

struct GameServer;

impl GameServer {
    fn new() -> Self {
        Self
    }

    fn game_play(&self, game: &dyn Games) {
        game.start();
    }
}

struct LoL {
    title: String,
    version: String,
}

impl LoL {
    fn new() -> Self {
        Self {
            title: String::from("League of Legends"),
            version: String::from("v.23"),
        }
    }
}

impl Games for LoL {
    fn return_title(&self) -> &str {
        &self.title
    }

    fn return_version(&self) -> &str {
        &self.version
    }
}

struct SuperMario {
    title: String,
    version: String,
}

impl SuperMario {
    fn new() -> Self {
        Self {
            title: String::from("SuperMario"),
            version: String::from("v.23"),
        }
    }
}

impl Games for SuperMario {
    fn return_title(&self) -> &str {
        &self.title
    }

    fn return_version(&self) -> &str {
        &self.version
    }
}

struct Tetris {
    title: String,
    version: String,
}

impl Tetris {
    fn new() -> Self {
        Self {
            title: String::from("Tetris"),
            version: String::from("v.23"),
        }
    }
}

impl Games for Tetris {
    fn return_title(&self) -> &str {
        &self.title
    }

    fn return_version(&self) -> &str {
        &self.version
    }
}

fn main() {
    let game_server = GameServer::new();

    game_server.game_play(&SuperMario::new());
    game_server.game_play(&Tetris::new());
    game_server.game_play(&LoL::new());
}