extern crate serialize;
extern crate garnet;

use serialize::json;
use std::io::net::ip::{SocketAddr,Ipv4Addr};

fn main() {
    let host = match garnet::Client::new() {
        Ok(h) => h,
        Err(e) => panic!("Could not create client {}",e),
    };
    let server = SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 12345 };

    host.set_read_timeout(None);
    host.set_write_timeout(None);

    loop {
        let msg = host.recv();

        match msg.peer {
            Some(peer) => {
                println!("{}",msg.raw);
            }
            None => {}
        };

        println!("Enter A Message: ");
        let mut reader = std::io::stdin();
        let input = reader.read_line().ok().expect("Failed to read line");

        host.send_to(server,0,garnet::SendKind::RELIABLE);
    }
}