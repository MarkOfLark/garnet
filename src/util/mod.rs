/*
//        _________    ____  _   ______________
//       / ____/   |  / __ \/ | / / ____/_  __/
//      / / __/ /| | / /_/ /  |/ / __/   / /
//     / /_/ / ___ |/ _, _/ /|  / /___  / /
//     \____/_/  |_/_/ |_/_/ |_/_____/ /_/
*/

use std::io::net::udp::UdpSocket;
use std::io::net::ip::{Ipv4Addr, SocketAddr};
use std::sync::{Once, ONCE_INIT};

// This is not pretty but it is the best way I could find to have this computed once and only once.
static mut global_max_message_size : u32 = 0;
static COMPUTE_MAX_MESSAGE_SIZE: Once = ONCE_INIT;

fn compute_max_message_size() -> Result<u32,&'static str> {

    let addr = SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 34254 };
    let mut socket = match UdpSocket::bind(addr) {
        Ok(s) => s,
        Err(_) => return Err("Could not bind socket"),
    };

    let mut bottom = 1;
    let mut top = bottom;

    loop {
        let send_size = top;
        let buf = box Vec::from_elem(send_size,0u8);
        match socket.send_to(buf[0..send_size],addr) {
            Ok(_) => {
                top = 2*send_size;
                bottom = send_size;
            }
            Err(_) => {
                break;
            }
        }
    }

    let buf = box Vec::from_elem(top,0u8);
    loop {
        let send_size = bottom+(top-bottom)/2;
        match socket.send_to(buf[0..send_size],addr) {
            Ok(_) => {
                if bottom == send_size {
                    return Ok(send_size as u32);
                }
                bottom = send_size;
            },
            Err(_) => {
                top = send_size;
                if bottom == send_size {
                    return Err("Could not determine send size");
                }
            },
        };
    }
}

pub fn max_message_size() -> u32 {
    unsafe {
        COMPUTE_MAX_MESSAGE_SIZE.doit(|| {
            global_max_message_size = match compute_max_message_size() {
                Ok(max) => max,
                Err(_)  => 0
            }
        });
        global_max_message_size
    }
}


pub trait ToIpPort {
    fn to_ip_port(&self) -> u16;
}

impl ToIpPort for u16 {
    fn to_ip_port(&self) -> u16 { *self }
}

impl ToIpPort for
