use std::f64::consts::PI;

use rppal::i2c::{self, I2c};

pub const CONVERSION_TO_GS: f64 = 16384.0;

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

    pub fn read_raw(&self) -> i2c::Result<(RawAccelData, RawGyroData)> {
        let mut buf = [0u8; 28];
        self.0.block_read(0x3B, &mut buf)?;

        let accel_x = (((buf[0] as i16) << 8) | buf[1] as i16) as f64;
        let accel_y = (((buf[2] as i16) << 8) | buf[3] as i16) as f64;
        let accel_z = (((buf[4] as i16) << 8) | buf[5] as i16) as f64;

        let gyro_x = (((buf[6] as i16) << 8) | buf[7] as i16) as f64;
        let gyro_y = (((buf[8] as i16) << 8) | buf[9] as i16) as f64;
        let gyro_z = (((buf[10] as i16) << 8) | buf[11] as i16) as f64;


        Ok(((accel_x, accel_y, accel_z).into(), (gyro_x, gyro_y, gyro_z).into()))
    }

    pub fn read_raw_poll_pitch(&self) -> i2c::Result<((f64, f64), (f64, f64))> {
        let (mut accel, mut gyro) = self.read_raw()?;
        accel.normalize_to_gs();
        gyro.normalize_to_gs();

        let roll_accel = accel.y.atan2(accel.z) * 180.0 / PI;
        let pitch_accel = (-accel.x).atan2((accel.y * accel.y + accel.z * accel.z).sqrt()) * 180.0 / PI;

        let roll_gyro = gyro.y.atan2(gyro.z) * 180.0 / PI;
        let pitch_gyro = (-gyro.x).atan2((gyro.y * gyro.y + gyro.z * gyro.z).sqrt()) * 180.0 / PI;

        Ok(((roll_accel, pitch_accel), (roll_gyro, pitch_gyro)))
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

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct RawGyroData {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl RawGyroData {
    pub fn normalize_to_gs(&mut self) {
        self.x = self.x / CONVERSION_TO_GS;
        self.y = self.y / CONVERSION_TO_GS;
        self.z = self.z / CONVERSION_TO_GS;
    }
}

impl From<(f64, f64, f64)> for RawGyroData {
    fn from(value: (f64, f64, f64)) -> Self {
        Self { x: value.0, y: value.1, z: value.2 }
    }
}


