use std::f64::consts::PI;

use rppal::i2c::I2c;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let i2c_address: u16 = 0x68;
    let mut i2c = I2c::with_bus(1)?;
    i2c.set_slave_address(i2c_address)?;

    i2c.smbus_write_byte(0x6B, 0x00)?;
    loop {
        let mut buf = [0u8; 14];
        i2c.block_read(0x3B, &mut buf)?;

        let accel_x = (((buf[0] as i16) << 8) | buf[1] as i16) as f64;
        let accel_y = (((buf[2] as i16) << 8) | buf[3] as i16) as f64;
        let accel_z = (((buf[4] as i16) << 8) | buf[5] as i16) as f64;

        let accel_x = accel_x / 16384.0;
        let accel_y = accel_y / 16384.0;
        let accel_z = accel_z / 16384.0;

        let roll = accel_y.atan2(accel_z) * 180.0 / PI;
        let pitch = (-accel_x).atan2((accel_y * accel_y + accel_z * accel_z).sqrt()) * 180.0 / PI;

        println!("Roll: {:.2}°", roll);
        println!("Pitch: {:.2}°", pitch);
    }
}

