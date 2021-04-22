use portaudio as pa;
use std::sync::mpsc::Sender;

pub struct InputSettings {
    pub channels: i32,
    pub sample_rate: f64,
    pub frames_per_buffer: u32,
}

pub struct Audio {
    pa: pa::PortAudio,
    input_settings: pa::InputStreamSettings<i16>,
}

impl Audio {
    pub fn new(input_settings: InputSettings) -> Audio {
        let pa = pa::PortAudio::new().expect("Unable to init PortAudio");

        let input_settings = pa
            .default_input_stream_settings(
                input_settings.channels,
                input_settings.sample_rate,
                input_settings.frames_per_buffer,
            )
            .unwrap();

        Audio { pa, input_settings }
    }

    pub fn open_input_stream(&self, tx: Sender<&'static [i16]>) {
        let process_audio = move |pa::InputStreamCallbackArgs { buffer, .. }| match tx.send(buffer)
        {
            Ok(_) => pa::Continue,
            Err(err) => {
                eprintln!("{}", err);
                pa::Complete
            }
        };

        let mut input_stream = self
            .pa
            .open_non_blocking_stream(self.input_settings, process_audio)
            .expect("Unable to create audio stream");

        input_stream.start().expect("Unable to start audio stream");

        while let true = input_stream.is_active().unwrap() {}
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
