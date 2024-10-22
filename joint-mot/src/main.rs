use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;

const GPIO_PWM_1: u8 = 4;
const GPIO_PWM_2: u8 = 5;
const GPIO_PWM_3: u8 = 6;

const PERIOD_MS: u64 = 20;
const PULSE_MIN_US: u64 = 350;
const PULSE_MAX_US: u64 = 2645;

fn angle_to_pulse(angle: u8) -> u64 {
    let angle = angle.clamp(0, 180);
    PULSE_MIN_US + ((PULSE_MAX_US - PULSE_MIN_US) as u64 * angle as u64) / 180
}

fn set_angle(pin: &mut rppal::gpio::OutputPin, angle: u8) -> Result<(), Box<dyn Error>> {
    let pulse_width = angle_to_pulse(angle);
    pin.set_pwm(
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(pulse_width),
    )?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut pin1 = Gpio::new()?.get(GPIO_PWM_1)?.into_output();
    let mut pin2 = Gpio::new()?.get(GPIO_PWM_2)?.into_output();
    let mut pin3 = Gpio::new()?.get(GPIO_PWM_3)?.into_output();

    set_angle(&mut pin1, 90)?;
    set_angle(&mut pin2, 45)?;
    set_angle(&mut pin3, 135)?;

    thread::sleep(Duration::from_millis(500));

    set_angle(&mut pin1, 0)?;
    set_angle(&mut pin2, 180)?;
    set_angle(&mut pin3, 90)?;

    thread::sleep(Duration::from_millis(500));

    Ok(())
}
