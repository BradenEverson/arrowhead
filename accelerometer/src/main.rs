use rppal::i2c::I2c;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let i2c_address: u16 = 0x68; // MPU-6050 I2C address

    let mut i2c = I2c::with_bus(1)?;
    i2c.set_slave_address(i2c_address)?;

    i2c.smbus_write_byte(0x6B, 0x00)?; // Write 0 to the power management register

    let mut buf = [0u8; 14];
    i2c.block_read(0x3B, &mut buf)?;

    let accel_x = ((buf[0] as i16) << 8) | buf[1] as i16;
    let accel_y = ((buf[2] as i16) << 8) | buf[3] as i16;
    let accel_z = ((buf[4] as i16) << 8) | buf[5] as i16;

    println!("Accelerometer X: {}", accel_x);
    println!("Accelerometer Y: {}", accel_y);
    println!("Accelerometer Z: {}", accel_z);

    Ok(())
}

