use std::error::Error;

use byteorder::{ByteOrder, LittleEndian};
use gy521_rppal::Gy521;
use tungstenite::{connect, Message};

fn main() -> Result<(), Box<dyn Error>> {
    let (mut socket, _) = connect("ws://192.168.10.134:7878").expect("Can't connect");
    let gyro = Gy521::new(1, 0x68)?;
    gyro.wakeup()?;

    loop {
        let ((_, _), (pitch, poll)) = gyro.read_raw_poll_pitch()?;

        let mut buffer = [0u8; 8];
        LittleEndian::write_f32_into(&[pitch as f32, poll as f32], &mut buffer);

        socket.send(Message::Binary(buffer.to_vec())).expect("Failed to send message");
        println!("Pitch: {:.2}\nPoll: {:.2}", pitch, poll);
    }
}
