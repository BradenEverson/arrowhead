use std::f64::consts::PI;

use rppal::i2c::{self, I2c};

pub const CONVERSION_TO_GS: f64 = 16384.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gyro = Gy521::new(1, 0x68)?;

    gyro.wakeup()?;
    loop {
        let (roll, pitch) = gyro.read_raw_poll_pitch()?;

        println!("Roll: {:.2}°", roll);
        println!("Pitch: {:.2}°", pitch);
    }
}

#[derive(Debug)]
pub struct Gy521(I2c);

impl Gy521 {
    pub fn wakeup(&self) -> i2c::Result<()> {
        self.0.smbus_write_byte(0x6B, 0x00)?;
        Ok(())
    }

    pub fn new(bus: u8, addr: u16) -> i2c::Result<Self> {
        let mut i2c = I2c::with_bus(bus)?;
        i2c.set_slave_address(addr)?;

        Ok(Self(i2c))
    }

    pub fn read_raw(&self) -> i2c::Result<RawAccelData> {
        let mut buf = [0u8; 14];
        self.0.block_read(0x3B, &mut buf)?;

        let accel_x = (((buf[0] as i16) << 8) | buf[1] as i16) as f64;
        let accel_y = (((buf[2] as i16) << 8) | buf[3] as i16) as f64;
        let accel_z = (((buf[4] as i16) << 8) | buf[5] as i16) as f64;

        Ok((accel_x, accel_y, accel_z).into())
    }

    pub fn read_raw_poll_pitch(&self) -> i2c::Result<(f64, f64)> {
        let mut accel = self.read_raw()?;
        accel.normalize_to_gs();

        let roll = accel.y.atan2(accel.z) * 180.0 / PI;
        let pitch = (-accel.x).atan2((accel.y * accel.y + accel.z * accel.z).sqrt()) * 180.0 / PI;

        Ok((roll, pitch))
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct RawAccelData {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl RawAccelData {
    pub fn normalize_to_gs(&mut self) {
        self.x = self.x / CONVERSION_TO_GS;
        self.y = self.y / CONVERSION_TO_GS;
        self.z = self.z / CONVERSION_TO_GS;
    }
}

impl From<(f64, f64, f64)> for RawAccelData {
    fn from(value: (f64, f64, f64)) -> Self {
        Self { x: value.0, y: value.1, z: value.2 }
    }
}
