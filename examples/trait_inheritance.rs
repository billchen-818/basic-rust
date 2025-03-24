// 测试trait继承

trait Vehicle {
    fn get_price(&self) -> u64;
}

trait Car: Vehicle {
    fn model(&self) -> String;
}

struct TeslaRoadster {
    model: String,
    release_date: u16,
}

impl TeslaRoadster {
    fn nee(model: &str, release_date: u16) -> Self {
        Self {
            model: model.to_string(),
            release_date,
        }
    }
}

impl Vehicle for TeslaRoadster {
    fn get_price(&self) -> u64 {
        200000
    }
}

impl Car for TeslaRoadster {
    fn model(&self) -> String {
        "Tesla Roadster I".to_string()
    }
}

fn main() {
    let tesla = TeslaRoadster::nee("Tesla Roadster I", 2008);
    println!("Model: {}", tesla.model());
    println!("Release date: {}", tesla.release_date);
    println!("model: {}", tesla.model);
    println!("Price: {}", tesla.get_price());
}
