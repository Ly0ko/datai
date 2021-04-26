use audio::Audio;
use deepspeech::Model;
use std::path::Path;
use std::sync::mpsc::{channel, Receiver, Sender};

enum SpeechState {
    Listening,
    Ready,
    Complete,
}

pub struct Speech {
    model: Model,
    state: SpeechState,
}

impl Speech {
    pub fn new() -> Speech {
        let (graph_name, scorer_name) = Speech::get_models();

        let mut model = Model::load_from_files(&graph_name).unwrap();
        if let Some(scorer) = scorer_name {
            println!("Using external scorer `{}`", scorer.to_str().unwrap());
            model.enable_external_scorer(&scorer).unwrap();
        }

        let state = SpeechState::Listening;

        Speech { model, state }
    }

    fn get_models() -> (Box<Path>, Option<Box<Path>>) {
        let dir_path = Path::new("speech/models");
        let mut graph_name: Box<Path> = dir_path.join("output_graph.pb").into_boxed_path();
        let mut scorer_name: Option<Box<Path>> = None;
        for file in dir_path
            .read_dir()
            .expect("Specified model dir is not a dir")
        {
            if let Ok(f) = file {
                let file_path = f.path();
                if file_path.is_file() {
                    if let Some(ext) = file_path.extension() {
                        if ext == "pb" || ext == "pbmm" {
                            graph_name = file_path.into_boxed_path();
                        } else if ext == "scorer" {
                            scorer_name = Some(file_path.into_boxed_path());
                        }
                    }
                }
            }
        }

        (graph_name, scorer_name)
    }

    pub fn start_recognition(&mut self, wake_word: String, text_tx: Sender<String>) {
        let input_device = Audio::new();

        let (tx, rc) = channel();
        let tx1 = Sender::clone(&tx);

        std::thread::spawn(move || {
            input_device.open_input_stream(tx1);
        });

        self.start_stream(wake_word, text_tx, rc);
    }

    fn start_stream(
        &mut self,
        wake_word: String,
        text_tx: Sender<String>,
        buffer_rc: Receiver<Vec<i16>>,
    ) {
        let mut stream = self
            .model
            .create_stream()
            .expect("Failed to create model stream");

        loop {
            let buffer = buffer_rc.recv().unwrap();
            let buffer_slice: &[i16] = buffer.as_ref();
            stream.feed_audio(buffer_slice);
            let decoded = stream.intermediate_decode();

            match decoded {
                Ok(text) => match self.state {
                    SpeechState::Listening => {
                        if text.contains(&wake_word) {
                            self.state = SpeechState::Ready;
                        }
                    }
                    SpeechState::Ready => {
                        println!("{}", text);
                    }
                    SpeechState::Complete => {
                        break;
                    }
                },
                Err(err) => eprintln!("{}", err),
            }
        }

        if let SpeechState::Complete = self.state {
            let text = stream.finish().unwrap();
            text_tx.send(text).unwrap();
            self.state = SpeechState::Listening;
            self.start_stream(wake_word, text_tx, buffer_rc);
        }
    }
}
