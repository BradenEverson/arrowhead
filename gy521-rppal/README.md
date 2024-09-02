# gy521-rppal

`gy521-rppal` is a Rust crate that provides an easy-to-use interface for interacting with the GY-521 module using the [rppal](https://crates.io/crates/rppal) library. The GY-521 module typically features the MPU-6050 sensor, which combines a 3-axis accelerometer and a 3-axis gyroscope, making it ideal for applications in robotics, motion tracking, and more.

## Installation

Add `gy521-rppal` to your `Cargo.toml`:

```rust
[dependencies]
gy521-rppal = "0.1.0" 
rppal = "0.19.0"
```

## Usage

Here's a basic example demonstrating how to initialize the GY-521 sensor, wake it up, and read accelerometer data along with calculating roll and pitch angles.

```rust
use std::f64::consts::PI;
use gy521_rppal::Gy521;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the GY-521 on I2C bus 1 with the default address 0x68
    let gy521 = Gy521::new(1, 0x68)?;
    
    // Wake up the sensor
    gy521.wakeup()?;
    
    // Read raw accelerometer data
    let mut raw_data = gy521.read_raw()?;
    println!("Raw Accelerometer Data: {:?}", raw_data);
    
    // Normalize to g's
    raw_data.normalize_to_gs();
    println!("Normalized Accelerometer Data: {:?}", raw_data);
    
    // Calculate roll and pitch
    let (roll, pitch) = gy521.read_raw_poll_pitch()?;
    println!("Roll: {:.2}°, Pitch: {:.2}°", roll, pitch);
    
    Ok(())
}
```

### Example Output

```bash
Raw Accelerometer Data: RawAccelData { x: 16384.0, y: 0.0, z: 16384.0 } 
Normalized Accelerometer Data: RawAccelData { x: 1.0, y: 0.0, z: 1.0 } Roll: 0.00°, Pitch: -45.00°
```

## Acknowledgements

- [rppal](https://crates.io/crates/rppal) for providing the I2C communication capabilities.
- [MPU-6050](https://invensense.tdk.com/products/motion-tracking/6-axis/mpu-6050/) for the sensor hardware.

