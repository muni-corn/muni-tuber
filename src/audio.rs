use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    InputCallbackInfo, SampleFormat, Stream,
};
use std::sync::{Arc, Mutex};

pub struct AudioState {
    pub volume: Arc<Mutex<f32>>,
}

/// Ensure to store the resulting `Stream`! It will be dropped otherwise and mic input will stop.
pub fn start_default_stream() -> (AudioState, Stream) {
    let host = cpal::default_host();

    let device = host
        .default_input_device()
        .expect("no input device available");

    let config = device
        .default_input_config()
        .expect("no input config available");

    let sample_format = config.sample_format();

    let volume = Arc::new(Mutex::new(0.0));
    let volume_clone = volume.clone();
    let err_fn = |e| eprintln!("error occurred on stream: {}", e);
    let stream = match sample_format {
        SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data: &[f32], _: &InputCallbackInfo| {
                let max_sample_value = data
                    .iter()
                    .map(|sample| (sample.abs() * u16::MAX as f32) as u16)
                    .max()
                    .unwrap_or(0);
                *volume_clone.lock().unwrap() = u16_to_dbfs(max_sample_value)
            },
            err_fn,
            None,
        ),
        _ => unimplemented!(),
    }
    .unwrap();

    stream.play().expect("failed to play stream");

    (AudioState { volume }, stream)
}

fn u16_to_dbfs(volume: u16) -> f32 {
    let normalized = volume as f32 / u16::MAX as f32;
    20.0 * normalized.log10()
}
