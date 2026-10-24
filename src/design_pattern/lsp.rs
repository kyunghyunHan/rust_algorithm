/*
LSP
- 상위 클래스의 객체는 언제나 자신의 하위 클래스의 객체로 교체가 가능해야한다

*/

// 코드 6-13: 강사의 공통 동작
/*
LSP: 리스코프 치환 원칙

Lecturer를 사용하는 자리에 각 강사 구현체를 넣어도
공통으로 약속한 동작이 정상적으로 수행되어야 한다.

여기서 공통 약속:
- 강사 종류와 해당 강사료를 출력한다.
*/

trait Lecturer {
    fn lecture(&self);
}

// 일반 강사
struct LecturerNomal {
    lecturer: String,
    charge: i32,
}

impl LecturerNomal {
    fn new() -> Self {
        println!("[리스코프 교체의 원칙]");

        Self {
            lecturer: String::from("일반 강사"),
            charge: 90000,
        }
    }
}

impl Lecturer for LecturerNomal {
    fn lecture(&self) {
        println!("{}", self.lecturer);
        println!("강사료: {}원\n", self.charge);
    }
}

// 야간 강사
struct LecturerNight {
    lecturer: String,
    charge: i32,
}

impl LecturerNight {
    fn new() -> Self {
        println!("[리스코프 교체의 원칙]");

        Self {
            lecturer: String::from("야간 강사"),
            charge: 90000,
        }
    }
}

impl Lecturer for LecturerNight {
    fn lecture(&self) {
        println!("{}", self.lecturer);
        println!("강사료: {}원\n", self.charge as f64 * 1.5);
    }
}

// 대학원 강사
struct LecturerAtGS {
    lecturer: String,
    charge: i32,
}

impl LecturerAtGS {
    fn new() -> Self {
        println!("[리스코프 교체의 원칙]");

        Self {
            lecturer: String::from("대학원 강사"),
            charge: 90000,
        }
    }
}

impl Lecturer for LecturerAtGS {
    fn lecture(&self) {
        println!("{}", self.lecturer);
        println!("강사료: {}원\n", self.charge * 2);
    }
}

// 직업이 있는 강사
struct LecturerWithWork {
    lecturer: String,
    charge: i32,
}

impl LecturerWithWork {
    fn new() -> Self {
        println!("[리스코프 교체의 원칙]");

        Self {
            lecturer: String::from("직업이 있는 강사"),
            charge: 90000,
        }
    }
}

impl Lecturer for LecturerWithWork {
    fn lecture(&self) {
        println!("{}", self.lecturer);
        println!("강사료: {}원\n", self.charge / 2);
    }
}

fn main() {
    let lect01 = LecturerAtGS::new();
    lect01.lecture();

    let lect02 = LecturerNight::new();
    lect02.lecture();

    let lect03 = LecturerWithWork::new();
    lect03.lecture();

    let lect04 = LecturerNomal::new();
    lect04.lecture();
}