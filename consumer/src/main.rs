use std::error::Error;

use gy521_rppal::Gy521;

fn main() -> Result<(), Box<dyn Error>> {
    let gyro = Gy521::new(1, 0x68)?;
    gyro.wakeup()?;

    loop {
        let ((_, _), (pitch, poll)) = gyro.read_raw_poll_pitch()?;

        println!("Pitch: {:.2}\nPoll: {:.2}", pitch, poll);
    }
}
