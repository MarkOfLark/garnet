/*
//        _________    ____  _   ______________
//       / ____/   |  / __ \/ | / / ____/_  __/
//      / / __/ /| | / /_/ /  |/ / __/   / /
//     / /_/ / ___ |/ _, _/ /|  / /___  / /
//     \____/_/  |_/_/ |_/_/ |_/_____/ /_/
*/

#![feature(slicing_syntax)]

use std::io::IoResult;
use std::io::net::udp::UdpSocket;
use std::io::net::ip::{Ipv4Addr, SocketAddr};

pub mod util;
pub mod msg;

pub struct Server {
    sock: UdpSocket,
}

pub struct Client {
    sock: UdpSocket,
}

pub enum SendKind{
    RELIABLE,
    UNRELIABLE,
}


impl Server {
    pub fn new<P: util::ToIpPort>(port: P) -> IoResult<Server> {
        let addr = SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: port.to_ip_port() };
        let socket = match UdpSocket::bind(addr) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        Ok(Server{sock:socket})
    }

    pub fn recv(&mut self) -> IoResult<msg::Message> {
        let mut buf = [0, ..10];
        match self.sock.recv_from(&mut buf) {
            Ok((amt, src)) => {
                Ok(msg::Message{peer:src,raw:0})
            },
            Err(e) => Err(e),
        }
    }

    pub fn send_to(&self, addr: SocketAddr, raw: u32, kind: SendKind) -> IoResult<u32> {
        Ok(0)
    }
}



impl Client {
    pub fn new<P: util::ToIpPort>(port: P) -> IoResult<Client> {
        let addr = SocketAddr { ip: Ipv4Addr(0, 0, 0, 0), port: port.to_ip_port() };
        let socket = match UdpSocket::bind(addr) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        Ok(Client{sock:socket})
    }

    pub fn recv(&mut self) -> IoResult<msg::Message> {
        let mut buf = [0, ..10];
        match self.sock.recv_from(&mut buf) {
            Ok((amt, src)) => {
                Ok(msg::Message{peer:src,raw:0})
            },
            Err(e) => Err(e),
        }
    }

    pub fn send_to(&self, addr: SocketAddr, raw: u32, kind: SendKind) -> IoResult<u32> {
        Ok(0)
    }
}


pub trait CanTimeout {
    fn set_read_timeout(&mut self, timeout_ms: Option<u64>);
    fn set_write_timeout(&mut self, timout_ms: Option<u64>);
}

impl CanTimeout for Server {
    fn set_read_timeout(&mut self, timeout_ms: Option<u64>) {
        self.sock.set_read_timeout(timeout_ms);
    }
    fn set_write_timeout(&mut self, timeout_ms: Option<u64>) {
        self.sock.set_write_timeout(timeout_ms);
    }
}

