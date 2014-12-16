#![feature(slicing_syntax)]

extern crate serialize;
extern crate garnet;

use serialize::json;

fn main() {
    let host = match garnet::Server::new("12345") {
        Ok(h) => h,
        Err(e) => panic!("Could not create server {}",e),
    };
    host.set_read_timeout(Some(1));
    host.set_write_timeout(None);

    loop {
        let msg = match host.recv() {
            Ok(m) => {
                println!("Got a message! {}",m.raw);
                host.send_to(m.peer,m.raw,garnet::SendKind::RELIABLE);
            },
            Err(e) => { break; },
        };
    }
}