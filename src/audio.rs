use std::sync::mpsc::Receiver;

use crate::State;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, FromSample, SampleFormat, SizedSample, StreamConfig};
use fundsp::hacker::lowpass_hz;
use fundsp::prelude::*;

pub fn run(receiver: Receiver<State>) {
    let audio_graph = create_brown_noise(0.5, 300.0, 1.5);
    println!("Run");
    let mut state: State = State::default();
    run_output(audio_graph, receiver);
}

fn run_output(audio_graph: Box<dyn AudioUnit>, rx: Receiver<State>) {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("failed to find a default output device");
    let config = device.default_output_config().unwrap();
    match config.sample_format() {
        SampleFormat::F32 => run_synth::<f32>(audio_graph, device, config.into(), rx),
        SampleFormat::I16 => run_synth::<i16>(audio_graph, device, config.into(), rx),
        SampleFormat::U16 => run_synth::<u16>(audio_graph, device, config.into(), rx),

        _ => panic!("Unsupported format"),
    }
}

fn run_synth<T: SizedSample + FromSample<f32>>(
    mut audio_graph: Box<dyn AudioUnit>,
    device: Device,
    config: StreamConfig,
    rx: Receiver<State>,
) {
    std::thread::spawn(move || {
        let sample_rate = config.sample_rate.0 as f64;
        audio_graph = create_brown_noise(1.0, 3000.0, 1.5);
        audio_graph.set_sample_rate(sample_rate);

        let mut next_value = move || {
            let poll = rx.try_recv();
            if let Ok(state) = poll {
                // println!("Received new params: {:?}", state.volume/100.0);
                audio_graph =
                    create_brown_noise(state.volume / 100.0, state.lowpass, state.q / 10.0);
            }
            audio_graph.get_stereo()
        };

        let channels = config.channels as usize;
        let err_fn = |err| eprintln!("an error occurred on stream: {err}");
        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                    write_data(data, channels, &mut next_value)
                },
                err_fn,
                None,
            )
            .unwrap();

        stream.play().unwrap();
        loop {
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    });
}

fn write_data<T: SizedSample + FromSample<f32>>(
    output: &mut [T],
    channels: usize,
    next_sample: &mut dyn FnMut() -> (f32, f32),
) {
    for frame in output.chunks_mut(channels) {
        let sample = next_sample();
        let left: T = T::from_sample(sample.0);
        let right: T = T::from_sample(sample.1);

        for (channel, sample) in frame.iter_mut().enumerate() {
            *sample = if channel & 1 == 0 { left } else { right };
        }
    }
}

fn create_brown_noise(volume: f32, pass: f32, q: f32) -> Box<dyn AudioUnit> {
    let brown_stereo = brown::<f64>() | brown::<f64>();
    let lowpass = lowpass_hz(pass, q) | lowpass_hz(pass, q);
    println!("{:?}", (volume, pass, q));
    Box::new((brown_stereo >> lowpass) * volume)
}
