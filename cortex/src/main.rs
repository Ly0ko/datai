use std::{
    sync::mpsc::{channel, Sender},
    thread,
};

use speech::Speech;

fn main() {
    let (tx, rc) = channel();

    thread::spawn(move || {
        let mut speech = Speech::new();
        speech.start_recognition(String::from("computer"), Sender::clone(&tx));
    });

    loop {
        let text = rc.recv().unwrap();
        println!("Final text: {}", text);
    }
}
