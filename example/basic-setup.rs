extern crate nirt;

fn main() {
    let max_size = nirt::max_message_size();

    println!("Max message size is {}",max_size);
}
