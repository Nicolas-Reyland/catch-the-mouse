// Server

use std::env;
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{Read, Write};
use std::thread;
use std::time::{Duration, Instant};
use mouse_rs::Mouse;
use device_query::{DeviceQuery, DeviceState, MouseState};
use parse_net_args_lib::parse_net_args;

static BETWEEN_CLICK_DURATION_MS: u128 = 250;

fn fill_from_str(mut bytes: &mut [u8], s: &str) {
    bytes.write(s.as_bytes()).unwrap();
}

fn handle_client(mut stream: TcpStream, adresse: &str) {
    //let mut msg: Vec<u8> = Vec::new();
    let mut bytes: [u8; 12];

    let device_state: DeviceState = DeviceState::new();
    let mut mouse: MouseState = device_state.get_mouse();
    let mouse_rs_obj: Mouse = Mouse::new();

    let (mouse_origin_x, mouse_origin_y): (i32, i32) = mouse.coords;
    let (mut th_mouse_x, mut th_mouse_y): (i32, i32) = mouse.coords;
    let (mut mouse_mvt_x, mut mouse_mvt_y): (i32, i32) = (0, 0);

    let (max_x, max_y): (i32, i32) = (1920, 1080);
    let mut now: Instant = Instant::now();
    let mut old_now: Instant = now;
    let mut last_pressed_instant: Instant = now;
    let mut last_pressed_flag: u8 = 0;

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
                // get mouse info
                mouse = device_state.get_mouse();
                // reset mouse position to origin
                mouse_rs_obj.move_to(mouse_origin_x, mouse_origin_y).unwrap();
                // get mouse movement on x & y
                mouse_mvt_x = mouse.coords.0 - mouse_origin_x;
                mouse_mvt_y = mouse.coords.1 - mouse_origin_y;
                // apply movement on theoretical position
                th_mouse_x += mouse_mvt_x;
                th_mouse_y += mouse_mvt_y;
                // clip the values
                if th_mouse_x > max_x {
                    th_mouse_x = max_x;
                } else if th_mouse_x < 0 {
                    th_mouse_x = 0;
                }
                if th_mouse_y > max_y {
                    th_mouse_y = max_y;
                } else if th_mouse_y < 0 {
                    th_mouse_y = 0;
                }
                // get time info
                now = Instant::now();
                let duraton: u128 = now.duration_since(old_now).as_millis();
                let click_duration: u128 = now.duration_since(last_pressed_instant).as_millis();
                // check for button presses
                let mut pressed_buttons_string: String = "".to_owned();
                if mouse.button_pressed[1] && (last_pressed_flag & 1 == 0 || click_duration < BETWEEN_CLICK_DURATION_MS) {
                    pressed_buttons_string = "l".to_owned();
                    last_pressed_instant = now;
                    last_pressed_flag = 1;
                }
                if mouse.button_pressed[2] && (last_pressed_flag & 2 == 0 || click_duration < BETWEEN_CLICK_DURATION_MS) {
                    pressed_buttons_string = pressed_buttons_string.clone() + "r";
                    last_pressed_instant = now;
                    last_pressed_flag += 2;
                }
                if mouse.button_pressed[3] && (last_pressed_flag & 4 == 0 || click_duration < BETWEEN_CLICK_DURATION_MS) {
                    pressed_buttons_string = format!("{}{}", pressed_buttons_string, "m".to_owned());
                    last_pressed_instant = now;
                    last_pressed_flag += 4;
                }
                // format final msg
                let mouse_pos_string = format!("{x}x{y}{buttons}",
                    x = th_mouse_x,
                    y = th_mouse_y,
                    buttons = pressed_buttons_string);
                // fill byte-buffer with padding (zeros) using the str
                fill_from_str(&mut bytes, &mouse_pos_string);
                // write to socket stream
                stream.write(&mut bytes).unwrap();
                // set new time
                old_now = now;
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