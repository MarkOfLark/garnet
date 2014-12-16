#![feature(slicing_syntax)]

extern crate garnet;


fn main() {
    let host = garnet::Server.new("12345");
    host.set_read_timeout(Some(1));
    host.set_write_timeout(None);

    loop {
        let msg = host.recv();

        match msg.peer {
            Some(peer) => {
                println!("Got a message! {}",json.decode(msg.raw));
                host.send_to(msg.peer,msg.raw,garnet::RELIABLE);
            }
            None => {}
        };
    }
}