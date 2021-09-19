// Client

use std::env;
use std::net::{TcpStream,SocketAddr};
use std::io::{Write, Read, stdin};
use mouse_rs::{types::keys::Keys, Mouse};
use parse_net_args_lib::parse_net_args;

fn parse_server_msg(buf: [u8; 12]) {
    // declare vars
    let (mut x, mut y): (i32, i32) = (0, 0);
    let mut b: bool = true;
    let mouse: Mouse = Mouse::new();
    // calculate mouse positions
    for c in buf {
        if c == 0 {
            break;
        }
        if c == 120 {
            b = false;
            continue;
        }
        if b {
            // x
            x = (x * 10) + (c as i32) - 0x30;
        } else {
            // y
            y = (y * 10) + (c as i32) - 0x30;
        }
    }
    // move mouse to position
    mouse.move_to(x.into(), y.into()).expect("Unable to move mouse");
}

fn get_entry() -> String {
    let mut buf = String::new();

    stdin().read_line(&mut buf).unwrap();
    buf.replace("\n", "").replace("\r", "")
}

fn exchange_with_server(mut stream: TcpStream) {
    let stdout = std::io::stdout();
    let mut io = stdout.lock();
    let buf = &mut [0; 12];

    println!("Enter 'quit' when you want to leave");
    loop {
        write!(io, "> ").unwrap();
        // pour afficher de suite
        io.flush().unwrap();
        match &*get_entry() {
            "quit" => {
                println!("bye !");
                return;
            }
            line => {
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
                println!("Server response : {:?}", buf);
                parse_server_msg(*buf);
            }
        }
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