use crate::{config, State};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, FromSample, SampleFormat, SizedSample, StreamConfig};
use fundsp::hacker::lowpass;
use fundsp::prelude::*;

pub fn run(state: State) {
    let audio_graph = create_brown_noise(state.clone());
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("Error: failed to find a default output device");
    let config = device.default_output_config().unwrap();
    match config.sample_format() {
        SampleFormat::F32 => write_to_speaker::<f32>(audio_graph, device, config.into(), state),
        SampleFormat::I16 => write_to_speaker::<i16>(audio_graph, device, config.into(), state),
        SampleFormat::U16 => write_to_speaker::<u16>(audio_graph, device, config.into(), state),

        _ => panic!("Error: unsupported system audio format"),
    }
}

fn write_to_speaker<T: SizedSample + FromSample<f32>>(
    mut audio_graph: Box<dyn AudioUnit>,
    device: Device,
    config: StreamConfig,
    state: State,
) {
    std::thread::spawn(move || {
        let sample_rate = config.sample_rate.0 as f64;
        audio_graph.set_sample_rate(sample_rate);

        let mut net = Net::new(0, 2);
        let id = net.push(audio_graph.clone());
        net.pipe_output(id);
        let mut backend = net.backend();
        backend.set_sample_rate(sample_rate);


        let mut next_value = move || { 
            backend.get_stereo()
            // audio_graph.get_stereo()
        };

        let channels = config.channels as usize;
        let err_fn = |err| eprintln!("An error occurred on stream: {err}");
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
            config::write(&state);
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

fn create_brown_noise(state: State) -> Box<dyn AudioUnit> {
    let fade_in_time = 3.0;
    let response_time = 0.1;

    let lowpass_var = var(&state.lowpass) >> follow(response_time);
    let q_var = var(&state.q) >> follow(response_time);
    let volume_var = var(&state.volume) >> follow(response_time);

    let noise = brown::<f64>();
    let mono_graph =
        (noise | lowpass_var | q_var) >> lowpass() * volume_var >> declick_s(fade_in_time);
    Box::new(mono_graph.clone() | mono_graph)
}
