fn dip() {
    /*의존 관계원칙
    Game 이라는 약속읆 만듬


    */

    trait Games {
        fn start(&self);
    }
    struct Racing;
    struct Puzzle;
    impl Games for Racing {
        fn start(&self) {
            println!("자동차 경주 시작");
        }
    }

    impl Games for Puzzle {
        fn start(&self) {
            println!("퍼즐 시작");
        }
    }

    fn game_play(game: &dyn Games) {
        game.start();
    }
    let racing = Racing;
    let puzzle = Puzzle;

    game_play(&racing);
    game_play(&puzzle);
}
fn lsp() {
    /*예 오름차순으로 약속햇으면 오른차순으로 약속 */

    trait Animal {
        fn speak(&self);
    }

    struct Dog;
    struct Cat;

    impl Animal for Dog {
        fn speak(&self) {
            println!("멍멍");
        }
    }

    impl Animal for Cat {
        fn speak(&self) {
            println!("야옹");
        }
    }

    let dog = Dog;
    let cat = Cat;
    let mut animal: &dyn Animal = &Dog;
    animal.speak();

    animal = &cat;
    animal.speak();
}
pub fn example() {}
