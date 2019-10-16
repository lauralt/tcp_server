use std::io::{BufRead, BufReader, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

fn handle_client(mut stream: &TcpStream, client_greetings: Arc<Mutex<Vec<String>>>) {
    let mut reader = BufReader::new(stream);
    let mut data = String::new();

    while match reader.read_line(&mut data) {
        Ok(_) => {
            match data.as_ref() {
                "Receive\n" => {
                    let greetings = &mut *client_greetings.lock().unwrap();
                    if !greetings.is_empty() {
                        stream
                            .write(greetings.pop().unwrap().as_bytes())
                            .expect("Could not write bytes to stream.");
                    } else {
                        stream
                            .write(
                                &"No greetings available. Please, send another request.\n"
                                    .as_bytes(),
                            )
                            .expect("Could not write bytes to stream.");
                    }
                }
                "Send\n" => {
                    data.clear();
                    reader
                        .read_line(&mut data)
                        .expect("Could not read user input");
                    println!("Greeting received from client: {:?}", data);
                    let greetings = &mut *client_greetings.lock().unwrap();
                    greetings.push(data.clone());
                    println!("Greeting added successfully to list.");
                    stream
                        .write(&"Greeting added successfully to list.\n".as_bytes())
                        .unwrap();
                }
                _ => {
                    stream
                        .write(&"Bad request. Try again!\n".as_bytes())
                        .expect("Could not send bytes to stream.");
                }
            };
            data.clear();
            true
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    let greetings = Arc::new(Mutex::new(vec![]));

    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!(
                    "New connection: {}",
                    stream.peer_addr().expect("Could not create new connection.")
                );
                let client_greetings = Arc::clone(&greetings);
                thread::spawn( {
                    move || {
                        // connection succeeded
                        handle_client(&stream, client_greetings)
                    }});
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}
