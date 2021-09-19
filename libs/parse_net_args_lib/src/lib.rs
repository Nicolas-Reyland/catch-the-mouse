use std::net::{ToSocketAddrs,SocketAddr};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn parse_net_args(args: Vec<String>) -> SocketAddr {
    // Check arg count
    let num_args = args.len() - 1;
    if num_args != 1 {
        panic!("Must include one argument (only one): ip-addr:port");
    }

    let connection: Vec<_> = args[1]
        .to_socket_addrs()
        .expect("Unablle to resolve address")
        .collect();

    assert_eq!(connection.len(), 1, "Assert that there is only one connection given.");

    println!("{:?}", connection[0]);

    return connection[0];
}
