// Server

use std::env;
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{Read, Write};
use std::thread;
use mouse_rs::{types::keys::Keys, Mouse};
use parse_net_args_lib::parse_net_args;

fn handle_client(mut stream: TcpStream, adresse: &str) {
    let mut msg: Vec<u8> = Vec::new();
    let mouse = Mouse::new();
    let mut position = mouse.get_position().unwrap();

    loop {
        let buf = &mut [0; 10];

        match stream.read(buf) {
            Ok(received) => {
                // if we get 0 bytes, the client is considered disconnected
                if received < 1 {
                    println!("Client disconnected {}", adresse);
                    return;
                }
                let mut x = 0;

                for c in buf {
                    // don't need to go further than number of received bytes
                    if x >= received {
                        break;
                    }
                    x += 1;
                    if *c == '\n' as u8 {
                        let client_msg = String::from_utf8(msg).unwrap();
                        println!("message {} : {}",
                            adresse,
                            // convert buffer into string
                            client_msg
                        );
                        // end
                        position = mouse.get_position().unwrap();
                        let mouse_pos_string = format!("{}x{}", position.x, position.y);
                        stream.write(mouse_pos_string.as_bytes()).unwrap();
                        msg = Vec::new();
                        break; // only read one line
                    } else {
                        msg.push(*c);
                    }
                }
            }
            Err(_) => {
                println!("Client disconnected {}", adresse);
                return;
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let addr: SocketAddr = parse_net_args(args);
    let listener = TcpListener::bind(addr).unwrap();

    println!("Waiting for connection on {:?} ...", addr);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let adresse = match stream.peer_addr() {
                    Ok(addr) => format!("[address : {}]", addr),
                    Err(_) => "unknown".to_owned()
                };

                println!("New client {}", adresse);
                thread::spawn(move|| {
                    handle_client(stream, &*adresse)
                });
            }
            Err(e) => {
                println!("Connection has failed : {}", e);
            }
        }
        println!("Waiting for another client...");
    }
}