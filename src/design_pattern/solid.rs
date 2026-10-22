
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
