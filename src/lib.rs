/*
//        _________    ____  _   ______________
//       / ____/   |  / __ \/ | / / ____/_  __/
//      / / __/ /| | / /_/ /  |/ / __/   / /
//     / /_/ / ___ |/ _, _/ /|  / /___  / /
//     \____/_/  |_/_/ |_/_/ |_/_____/ /_/
*/

#![feature(slicing_syntax)]


pub mod util;

pub struct Server {
}

pub struct Client {
}

impl Server {
    pub fn new<P: ToIpPort>(port: P) -> IoResult<Server> {
    }
}
