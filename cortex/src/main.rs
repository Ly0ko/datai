use speech::Speech;

fn main() {
    let mut speech = Speech::new();
    speech.start_recognition(String::from("computer"));
}
