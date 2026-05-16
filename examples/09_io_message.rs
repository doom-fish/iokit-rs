use iokit::{io_message_bridged_constant_count, IoMessage};

fn main() {
    let bridged_count = io_message_bridged_constant_count();
    println!("bridged constant count = {bridged_count}");
    for message in IoMessage::all_known().iter().take(5) {
        let raw = message.as_raw();
        println!("{message:?} = {raw}");
    }
}
