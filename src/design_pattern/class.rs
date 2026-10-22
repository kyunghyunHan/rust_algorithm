// 공통 데이터: Java 부모 클래스의 필드에 해당
struct VehicleInfo {
    brand: String,
    year: i32,
}

impl VehicleInfo {
    fn new(brand: &str, year: i32) -> Self {
        Self {
            brand: brand.to_string(),
            year,
        }
    }
}

// 교통수단의 공통 기능
trait Vehicle {
    fn info(&self) -> &VehicleInfo;

    // 공통 메서드: 각 타입에서 그대로 사용 가능
    fn display_info(&self) {
        let info = self.info();
        println!("Brand: {}, Year: {}", info.brand, info.year);
    }

    // 기본 동작: 각 타입에서 다르게 구현 가능
    fn start(&self) {
        println!("{} vehicle is starting.", self.info().brand);
    }
}

// 버스
struct Bus {
    info: VehicleInfo,
    passenger_capacity: i32,
}

impl Bus {
    fn new(brand: &str, year: i32, passenger_capacity: i32) -> Self {
        Self {
            info: VehicleInfo::new(brand, year),
            passenger_capacity,
        }
    }
}

impl Vehicle for Bus {
    fn info(&self) -> &VehicleInfo {
        &self.info
    }

    fn start(&self) {
        println!(
            "{} bus with capacity of {} passengers is starting.",
            self.info.brand, self.passenger_capacity
        );
    }
}

// 택시
struct Taxi {
    info: VehicleInfo,
    taxi_number: String,
}

impl Taxi {
    fn new(brand: &str, year: i32, taxi_number: &str) -> Self {
        Self {
            info: VehicleInfo::new(brand, year),
            taxi_number: taxi_number.to_string(),
        }
    }
}

impl Vehicle for Taxi {
    fn info(&self) -> &VehicleInfo {
        &self.info
    }

    fn start(&self) {
        println!(
            "{} taxi with number {} is starting.",
            self.info.brand, self.taxi_number
        );
    }
}

// 지하철
struct Subway {
    info: VehicleInfo,
    line_number: i32,
}

impl Subway {
    fn new(brand: &str, year: i32, line_number: i32) -> Self {
        Self {
            info: VehicleInfo::new(brand, year),
            line_number,
        }
    }
}

impl Vehicle for Subway {
    fn info(&self) -> &VehicleInfo {
        &self.info
    }

    fn start(&self) {
        println!(
            "{} subway on line {} is starting.",
            self.info.brand, self.line_number
        );
    }
}

fn main() {
    let my_bus = Bus::new("Hyundai", 2025, 50);
    let my_taxi = Taxi::new("KIA", 2025, "TX1234");
    let my_subway = Subway::new("Rotem", 2025, 1);

    my_bus.display_info();
    my_bus.start();

    my_taxi.display_info();
    my_taxi.start();

    my_subway.display_info();
    my_subway.start();
}