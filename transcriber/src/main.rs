use hc_sr04::{HcSr04, Unit};

fn main() {
    let mut ultrasonic = HcSr04::new(
        24,          // TRIGGER -> Gpio pin 24
        23,          // ECHO -> Gpio pin 23
        Some(23_f32) // Ambient temperature (if `None` defaults to 20.0C)
        ).unwrap();

    loop {
        let val = ultrasonic.measure_distance(Unit::Meters);
        println!("{:?}", val)
    }
}
