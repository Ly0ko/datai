use std::sync::mpsc;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub struct Audio {
    device: cpal::Device
}

impl Audio {
    pub fn new() -> Audio{
        let host = cpal::default_host();
        let device = host.default_input_device().expect("Unable to find input device");

        Audio { device }
    }

    pub fn open_input_stream(&self, tx: mpsc::Sender<Vec<i16>>) {
        let mut supported_configs_range = self.device.supported_input_configs()
            .expect("Unable to get supported input configs");

        let config = supported_configs_range.next()
            .expect("No support configs")
            .with_sample_rate(cpal::SampleRate(16000));
        
            let stream = self.device.build_input_stream(
                &config.into(),
                move |data: &[i16], _: &cpal::InputCallbackInfo| {
                    let buffer = data.to_vec();
                    tx.send(buffer).unwrap();
                },
                move |err| {
                    eprint!("{}", err);
                },
            ).unwrap();

            stream.play().unwrap();

            loop {}
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
