use crate::{config, AppState, AudioState, NoiseMode};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, FromSample, SampleFormat, SizedSample, StreamConfig};
use fundsp::hacker::lowpass;
use fundsp::prelude::*;
use std::sync::mpsc::*;

pub fn run(state: AudioState, rx: Receiver<NoiseMode>) {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("Error: failed to find a default output device");
    let config = device.default_output_config().unwrap();
    match config.sample_format() {
        SampleFormat::F32 => run_graph::<f32>(device, config.into(), state, rx),
        SampleFormat::I16 => run_graph::<i16>(device, config.into(), state, rx),
        SampleFormat::U16 => run_graph::<u16>(device, config.into(), state, rx),

        _ => panic!("Error: unsupported system audio format"),
    }
}

fn run_graph<T: SizedSample + FromSample<f32>>(
    device: Device,
    config: StreamConfig,
    state: AudioState,
    rx: Receiver<NoiseMode>,
) {
    std::thread::spawn(move || {
        let mut net = Net::new(0, 2);
        let id = net.push(generate_brown(&state));
        net.pipe_output(id);
        let mut backend = net.backend();
        backend.set_sample_rate(config.sample_rate.0 as f64);

        let closure_state = state.clone();

        let mut next_value = move || {
            let poll = rx.try_recv();
            if let Ok(mode) = poll {
                let new_graph: Box<dyn AudioUnit> = match mode {
                    NoiseMode::Brown => { generate_brown(&closure_state) },
                    NoiseMode::White => { generate_white(&closure_state) },
                    NoiseMode::Pink => { generate_pink(&closure_state) },
                    _ => { generate_brown(&closure_state) },
                };
                // net.replace(id, new_graph);
                net.crossfade(id, Fade::Smooth, 1.0, new_graph);
                net.commit();
            }
            backend.get_stereo()
        };

        let channels = config.channels as usize;
        let err_fn = |err| eprintln!("An error occurred on stream: {err}");
        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                    write_to_speaker(data, channels, &mut next_value)
                },
                err_fn,
                None,
            )
            .unwrap();

        stream.play().unwrap();
        loop {
            std::thread::sleep(std::time::Duration::from_millis(1000));
            config::write(state.clone().into());
        }
    });
}

fn write_to_speaker<T: SizedSample + FromSample<f32>>(
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

fn generate_brown(state: &AudioState) -> Box<dyn AudioUnit> {
    let response_time = 0.1;

    let lowpass_var = var(&state.lowpass) >> follow(response_time);
    let q_var = var(&state.q) >> follow(response_time);
    let volume_var = var(&state.volume) >> follow(response_time);

    let noise = brown::<f64>();
    let mono_graph =
        (noise | lowpass_var | q_var) >> lowpass() * volume_var;
    Box::new(mono_graph.clone() | mono_graph)
}

fn generate_white(state: &AudioState) -> Box<dyn AudioUnit> {
    let response_time = 0.1;

    let lowpass_var = var(&state.lowpass) >> follow(response_time);
    let q_var = var(&state.q) >> follow(response_time);
    let volume_var = var(&state.volume) >> follow(response_time);

    let noise = white();
    let mono_graph =
        (noise | lowpass_var | q_var) >> lowpass() * volume_var;
    Box::new(mono_graph.clone() | mono_graph)
}

fn generate_pink(state: &AudioState) -> Box<dyn AudioUnit> {
    let response_time = 0.1;

    let lowpass_var = var(&state.lowpass) >> follow(response_time);
    let q_var = var(&state.q) >> follow(response_time);
    let volume_var = var(&state.volume) >> follow(response_time);

    let noise = pink::<f64>();
    let mono_graph =
        (noise | lowpass_var | q_var) >> lowpass() * volume_var;
    Box::new(mono_graph.clone() | mono_graph)
}
