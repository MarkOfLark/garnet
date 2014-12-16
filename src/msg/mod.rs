use std::io::net::ip::{SocketAddr};

pub struct Message {
    pub peer: SocketAddr,
    pub raw: u32,
}

