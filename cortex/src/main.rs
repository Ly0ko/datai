use std::{sync::mpsc::channel, thread};

use speech::Speech;

fn main() {
    let (tx, rc) = channel();

    thread::spawn(move || {
        let mut speech = Speech::new();
        speech.start_recognition(String::from("computer"), tx);
    });

    loop {
        let text = rc.recv().unwrap();
        println!("{}", text);
    }
}
