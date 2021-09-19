// Server

use std::env;
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{Read, Write};
use std::thread;
use device_query::{DeviceQuery, DeviceState, MouseState};
use parse_net_args_lib::parse_net_args;

fn fill_from_str(mut bytes: &mut [u8], s: &str) {
    bytes.write(s.as_bytes()).unwrap();
}

fn handle_client(mut stream: TcpStream, adresse: &str) {
    let mut msg: Vec<u8> = Vec::new();
    let device_state: DeviceState = DeviceState::new();
    let mut mouse: MouseState;
    let (mut mouse_x, mut mouse_y): (i32, i32);
    let mut bytes: [u8; 12];

    loop {
        let buf = &mut [0; 12];
        bytes = [0; 12];

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
                        let _client_msg = String::from_utf8(msg).unwrap();
                        /*println!("message {} : {}",
                            adresse,
                            // convert buffer into string
                            client_msg
                        );*/
                        // end
                        mouse = device_state.get_mouse();
                        mouse_x = mouse.coords.0;
                        mouse_y = mouse.coords.1;
                        let mut pressed_buttons_string: String = "".to_owned();
                        if mouse.button_pressed[1] {
                            pressed_buttons_string = "l".to_owned();
                        }
                        if mouse.button_pressed[2] {
                            pressed_buttons_string = pressed_buttons_string.clone() + "r";
                        }
                        if mouse.button_pressed[3] {
                            pressed_buttons_string = format!("{}{}", pressed_buttons_string, "m".to_owned());
                        }
                        let mouse_pos_string = format!("{x}x{y}{buttons}",
                            x = mouse_x,
                            y = mouse_y,
                            buttons = pressed_buttons_string);
                        fill_from_str(&mut bytes, &mouse_pos_string);
                        stream.write(&mut bytes).unwrap();
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