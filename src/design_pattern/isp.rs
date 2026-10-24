/*
ISP (Interface Segregation Principle)

인터페이스 분리 원칙

- 클라이언트는 사용하지 않는 메서드에 의존하면 안 된다.
- 하나의 거대한 인터페이스보다 역할에 맞게 분리한다.
*/

// 학생 인터페이스
trait StudentInterface {
    fn course_registration(&self);
    fn grade_inquiry(&self);
}

// 교수 인터페이스
trait ProfessorInterface {
    fn grade_entry(&self);
    fn attendance_inquiry(&self);
}

// 직원 인터페이스
trait EmployeeInterface {
    fn set_registration_period(&self);
    fn set_grade_inquiry_period(&self);
}

// 학사관리 클래스
struct AcademicManagement;

// 학생 인터페이스 구현
impl StudentInterface for AcademicManagement {
    fn course_registration(&self) {
        println!("수강 신청 완료");
    }

    fn grade_inquiry(&self) {
        println!("성적 조회 완료");
    }
}

// 교수 인터페이스 구현
impl ProfessorInterface for AcademicManagement {
    fn grade_entry(&self) {
        println!("성적 등록 완료");
    }

    fn attendance_inquiry(&self) {
        println!("출석부 조회 완료");
    }
}

// 직원 인터페이스 구현
impl EmployeeInterface for AcademicManagement {
    fn set_registration_period(&self) {
        println!("수강 신청 기간 설정 완료");
    }

    fn set_grade_inquiry_period(&self) {
        println!("성적 조회 기간 설정 완료");
    }
}

// 학생
struct Student;

impl Student {
    fn execute(&self, system: &impl StudentInterface) {
        system.course_registration();
        system.grade_inquiry();
    }
}

// 교수
struct Professor;

impl Professor {
    fn execute(&self, system: &impl ProfessorInterface) {
        system.grade_entry();
        system.attendance_inquiry();
    }
}

// 직원
struct Employee;

impl Employee {
    fn execute(&self, system: &impl EmployeeInterface) {
        system.set_registration_period();
        system.set_grade_inquiry_period();
    }
}

fn main() {
    let academic = AcademicManagement;

    let student = Student;
    let professor = Professor;
    let employee = Employee;

    student.execute(&academic);
    professor.execute(&academic);
    employee.execute(&academic);
}