/*State Pattern
자주바뀌는것이 State라면 상태를 클래스로 바꾸고 interface타입의 상속 구조를 만들어라
*/

trait State {
    fn on_button(&self);
    fn off_button(&self);
}
struct ElectricFan {
    state: Box<dyn State>,
}
fn main() {}
