// Client

use std::env;
use std::net::{TcpStream,SocketAddr};
use std::io::{Write, Read, stdin};
//use std::str::from_utf8;
use parse_net_args_lib::parse_net_args;

fn get_entry() -> String {
    let mut buf = String::new();

    stdin().read_line(&mut buf).unwrap();
    buf.replace("\n", "").replace("\r", "")
}

fn exchange_with_server(mut stream: TcpStream) {
    let stdout = std::io::stdout();
    let mut io = stdout.lock();
    let buf = &mut [0; 3];

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
