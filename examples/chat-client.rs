extern crate garnet;


fn main() {
    let host = garnet::Client.new();
    let server = garnet::Peer.new("127.0.0.1:12345");

    host.set_read_timeout(None);
    host.set_write_timeout(None);

    loop {
        let msg = host.recv();

        match msg.peer {
            Some(peer) => {
                println!("{}",json.deserialize(msg.raw));
            }
            None => {}
        };

        println!("Enter A Message: ");
        let mut reader = io::stdin();
        let input = reader.read_line().ok().expect("Failed to read line");

        host.send_to(server,json.encode(input),garnet::RELIABLE);
    }
}