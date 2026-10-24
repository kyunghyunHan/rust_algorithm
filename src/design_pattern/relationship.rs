struct Vehicle {
    brand: String,
    year: i32,
}

// 교통수단이 제공하는 공통 기능
trait VehicleFn {
    fn display_info(&self);
    fn start(&self);
}

impl Vehicle {
    fn new(brand: String, year: i32) -> Self {
        Self { brand, year }
    }
}

impl VehicleFn for Vehicle {
    fn display_info(&self) {
        println!("Brand: {}, Year: {}", self.brand, self.year);
    }

    fn start(&self) {
        println!("{} vehicle is starting.", self.brand);
    }
}

// 버스: Vehicle을 포함하는 합성 구조
struct Bus {
    vehicle: Vehicle,
    capacity: usize,
}

impl Bus {
    fn new(brand: String, year: i32, capacity: usize) -> Self {
        Self {
            vehicle: Vehicle::new(brand, year),
            capacity,
        }
    }
}

impl VehicleFn for Bus {
    fn display_info(&self) {
        // 공통 정보 출력은 내부 Vehicle에 맡김
        self.vehicle.display_info();
    }

    fn start(&self) {
        println!(
            "{} bus with capacity of {} passengers is starting.",
            self.vehicle.brand, self.capacity
        );
    }
}

// 택시
struct Taxi {
    vehicle: Vehicle,
    taxi_number: String,
}

impl Taxi {
    fn new(brand: String, year: i32, taxi_number: String) -> Self {
        Self {
            vehicle: Vehicle::new(brand, year),
            taxi_number,
        }
    }
}

impl VehicleFn for Taxi {
    fn display_info(&self) {
        self.vehicle.display_info();
    }

    fn start(&self) {
        println!(
            "{} taxi with number {} is starting.",
            self.vehicle.brand, self.taxi_number
        );
    }
}

// 지하철
struct Subway {
    vehicle: Vehicle,
    line_number: i32,
}

impl Subway {
    fn new(brand: String, year: i32, line_number: i32) -> Self {
        Self {
            vehicle: Vehicle::new(brand, year),
            line_number,
        }
    }
}

impl VehicleFn for Subway {
    fn display_info(&self) {
        self.vehicle.display_info();
    }

    fn start(&self) {
        println!(
            "{} subway on line {} is starting.",
            self.vehicle.brand, self.line_number
        );
    }
}

fn main() {
    let bus = Bus::new("Hyundai".to_string(), 2025, 50);
    let taxi = Taxi::new("KIA".to_string(), 2025, "TX1234".to_string());
    let subway = Subway::new("Rotem".to_string(), 2025, 1);

    bus.display_info();
    bus.start();

    taxi.display_info();
    taxi.start();

    subway.display_info();
    subway.start();
}