use std::io::{stdin, BufRead, BufReader, Write};
use std::net::TcpStream;

pub fn process_request(mut buffer: &mut String, mut stream: &TcpStream) {
    stdin()
        .read_line(&mut buffer)
        .expect("Could not read user input.");
    stream
        .write(buffer.as_bytes())
        .expect("Could not write bytes to stream.");
    if buffer == &String::from("Send\n") {
        println!("Type your greeting.");
        buffer.clear();
        stdin()
            .read_line(&mut buffer)
            .expect("Could not read user input.");
        stream
            .write(buffer.as_bytes())
            .expect("Could not write bytes to stream.");
    }
    buffer.clear();
}


fn main() {
    let stream = &TcpStream::connect("localhost:3333").expect("Failed to connect");

    let mut reader = BufReader::new(stream);
    println!("Successfully connected to server in port 3333");
    println!(
        "If you want to send a greeting, type 'Send', if you want to receive one, type 'Receive'."
    );
    let mut buffer = String::new();
    process_request(&mut buffer, stream);
    buffer.clear();

    while match reader.read_line(&mut buffer) {
        Ok(_) => {
            if buffer == String::from("No greetings available. Please, send another request.\n")
                || buffer == String::from("Greeting added successfully to list.\n")
                || buffer == String::from("Bad request. Try again!\n")
            {
                print!("{}", buffer);
            } else {
                print!("Greeting received from server: {}", buffer);
            }
            buffer.clear();
            process_request(&mut buffer, stream);
            true
        }
        Err(e) => {
            println!("Failed to receive data: {}", e);
            false
        }
    } {}
    println!("Terminated.");
}
