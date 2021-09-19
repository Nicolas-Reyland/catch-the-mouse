// Client

use std::env;
use std::net::{TcpStream,SocketAddr};
use std::io::{Write, Read};
use mouse_rs::{types::keys::Keys, Mouse};
use parse_net_args_lib::parse_net_args;

fn parse_server_msg(buf: [u8; 12]) {
    // declare vars
    let (mut x, mut y): (i32, i32) = (0, 0);
    let mut b: bool = true;
    let mouse: Mouse = Mouse::new();
    let (mut press_r, mut press_l, mut press_m): (bool, bool, bool) = (false, false, false);
    // calculate mouse positions
    for c in buf {
        // premature end of buffer
        if c == 0 {
            break;
        }
        // 'x' between x and y coords
        if c == 120 {
            b = false;
            continue;
        }
        // button presses
        if c > 0x39 {
            if c == 'l' as u8 {
                press_l = true;
            }
            if c == 'r' as u8 {
                press_r = true;
            }
            if c == 'm' as u8 {
                press_m = true;
            }
            continue;
        }
        // write to x or y coordinates
        if b {
            // x
            x = (x * 10) + ((c - 0x30) as i32);
        } else {
            // y
            y = (y * 10) + ((c - 0x30) as i32);
        }
    }
    // move mouse to position
    //println!("{} x {}", x, y);
    mouse.move_to(x, y).expect("Unable to move mouse");
    if press_l {
        mouse.click(&Keys::LEFT).unwrap();
    }
    if press_r {
        mouse.click(&Keys::RIGHT).unwrap();
    }
    if press_m {
        mouse.click(&Keys::MIDDLE).unwrap();
    }
}

fn exchange_with_server(mut stream: TcpStream) {
    let stdout = std::io::stdout();
    let mut io = stdout.lock();
    let buf = &mut [0; 12];

    println!("Enter 'quit' when you want to leave");
    loop {
        //write!(io, "> ").unwrap();
        io.flush().unwrap();
        let line: String = "_\n".to_string();
        write!(stream, "{}\n", line).unwrap();
        match stream.read(buf) {
            Ok(received) => {
                if received < 1 {
                    println!("Connection to server has been lost (0)");
                    return;
                }
            }
            Err(_) => {
                println!("Connection to server has been lost (e)");
                return;
            }
        }
        //println!("Server response : {:?}", buf);
        parse_server_msg(*buf);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let addr: SocketAddr = parse_net_args(args);

    println!("Trying to connect to server useing params {:?} ...", addr);
    match TcpStream::connect(addr) {
        Ok(stream) => {
            println!("Connexion successful !");
            exchange_with_server(stream);
        }
        Err(e) => {
            println!("Connection has been lost : {}", e);
        }
    }
}