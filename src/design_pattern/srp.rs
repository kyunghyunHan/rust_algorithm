/*
단일책임 SRP

- 책임은 하나다
- 클래스를설계할떄 변경하는 이유가 하나여야한다
- 그 책임을 완전히 캡슐화한다(내부에)
- 만약에 클래스에 두개의 책임이 존재한다면 두개의 클래스로 변경해야한다

예를들어 운전자 정보를 관리하는 클래스인데 파일까지 저장하는거면 따로 클래스를 만들수있다
*/

struct Driver {
    name: String,
}

impl Driver {
    fn change_name(){}
}
struct DriverFileStorage;

impl DriverFileStorage{
    fn save(){}
}