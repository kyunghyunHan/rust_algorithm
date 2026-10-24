/*
OCP
개방패쇄원칙
- 변경에는 닫혀잇어야 하고 확장에는 열려있어야한다
- 어떤 부분은 개방 , 어떤부분은 패쇄
- 변화가 자주 일어나는 요소를 식별 
- 변경에 닫혀있는 설계 , 비즈니스 코드는 수정할 필요가없다 
*/
// 코덱의 공통 동작
trait PlayCodec {
    fn play(&self);
}

// MP4 코덱
struct Mp4;

impl PlayCodec for Mp4 {
    fn play(&self) {
        println!("MP4 재생");
    }
}

// AVI 코덱: 기존 플레이어 수정 없이 추가
struct Avi;

impl PlayCodec for Avi {
    fn play(&self) {
        println!("AVI 재생");
    }
}

// 플레이어는 구체적인 코덱 대신 trait에 의존
struct VideoPlayer {
    codec: Box<dyn PlayCodec>,
}

impl VideoPlayer {
    fn new(codec: Box<dyn PlayCodec>) -> Self {
        Self { codec }
    }

    fn set_codec(&mut self, codec: Box<dyn PlayCodec>) {
        self.codec = codec;
    }

    fn start(&self) {
        self.codec.play();
    }
}

fn main() {
    let mut player = VideoPlayer::new(Box::new(Mp4));
    player.start(); // MP4 재생

    player.set_codec(Box::new(Avi));
    player.start(); // AVI 재생
}