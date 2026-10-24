use std::fmt;
use std::rc::Rc;

// 운전자 → 자동차: 단방향 연관 관계
struct Driver {
    name: String,
    car: Option<Rc<Car>>,
}

impl Driver {
    fn new(name: String) -> Self {
        Self {
            name,
            car: None, // 처음에는 자동차 없음
        }
    }

    fn set_car(&mut self, car: Rc<Car>) {
        self.car = Some(car);
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_car(&self) -> Option<&Car> {
        self.car.as_deref()
    }
}

// Java의 toString()에 대응
impl fmt::Display for Driver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let model = match self.get_car() {
            Some(car) => car.get_model(),
            None => "No car",
        };

        write!(f, "Driver: {}, Car: {}", self.get_name(), model)
    }
}

struct Car {
    model: String,
}

impl Car {
    fn new(model: String) -> Self {
        Self { model }
    }

    fn get_model(&self) -> &str {
        &self.model
    }
}

impl fmt::Display for Car {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Car: {}", self.model)
    }
}

fn main() {
    let mut driver1 = Driver::new("GilDong".to_string());
    let car1 = Rc::new(Car::new("KIA".to_string()));

    // 자동차를 복사하지 않고 같은 자동차를 함께 참조
    driver1.set_car(Rc::clone(&car1));

    println!("{}", driver1);
    println!("{}", car1);
}