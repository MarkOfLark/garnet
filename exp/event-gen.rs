

use std::sync::mpsc;

struct Message<'a> {
    id : u32,
    data : &'a[u32],
}

fn generator<'a>(id : u32, tx : mpsc::Sender<Message<'a>>) {
    let gen = [0;64];
    let m = 'a Message{id : id, data : gen.as_slice()};
    tx.send(m);
}


fn main() {

    let (tx, rx) : (mpsc::Sender<Message>, mpsc::Receiver<Message>) = mpsc::channel();

    std::thread::Thread::spawn(move || { generator(0,tx) }).detach();
    

}
