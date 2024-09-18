use std::io::{self, Write};
use tungstenite::{connect, Message};
use byteorder::{ByteOrder, LittleEndian};

fn main() {
    let (mut socket, _) = connect("ws://localhost:7878").expect("Can't connect");

    loop {
        let (num1, num2) = get_f32_input();

        let mut buffer = [0u8; 8];
        LittleEndian::write_f32_into(&[num1, num2], &mut buffer);
        println!("{}, {}", num1, num2);

        socket.send(Message::Binary(buffer.to_vec())).expect("Failed to send message");

        println!("Sent: {:?}", buffer);
    }
}

fn get_f32_input() -> (f32, f32) {
    let mut input = String::new();
    
    print!("Enter the first f32: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let num1: f32 = input.trim().parse().expect("Invalid input");

    input.clear();

    print!("Enter the second f32: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let num2: f32 = input.trim().parse().expect("Invalid input");

    (num1, num2)
}
