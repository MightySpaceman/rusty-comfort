use fundsp::{funutd::noise::Noise, hacker::Shared};
use iced::{
    ContentFit, alignment, daemon::DefaultStyle, widget::{image, button, column, container, horizontal_space, pick_list, row, text, vertical_slider, vertical_space}, Element, Length, Point, Task, Theme
};
use std::sync::mpsc;
mod audio;
mod config;

#[derive(Clone, Default)]
struct State {
    volume: Shared,
    lowpass: Shared,
    q: Shared,
    mode: Option<NoiseMode>,
    muted: bool,
}

#[derive(Debug, Clone)]
enum Message {
    VolumeChanged(f32),
    LowPassChanged(f32),
    QChanged(f32),
    ModeChanged(NoiseMode),
    MuteToggle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum NoiseMode {
    #[default]
    Brown,
    White,
    Pink,
}

impl NoiseMode {
    const ALL: [NoiseMode; 3] = [
        Self::Brown,
        Self::White,
        Self::Pink,
    ];
}

impl std::fmt::Display for NoiseMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Brown => "Brown",
                Self::White => "White",
                Self::Pink => "Pink",
            }
        )
    }
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::VolumeChanged(volume) => {
                self.volume.set(volume);
            }
            Message::LowPassChanged(pass) => {
                self.lowpass.set(pass);
            }
            Message::QChanged(q) => {
                self.q.set(q);
            }
            Message::ModeChanged(mode) => {
                self.mode = Some(mode);
                // println!("{}", mode);
                match mode {
                    NoiseMode::Brown => { 
                        println!("Brown"); 
                    },
                    NoiseMode::White => {
                        println!("White");
                    }
                    NoiseMode::Pink => {
                        println!("Pink");
                    }
                }
            }
            Message::MuteToggle => {
                println!("Mute toggled");
            }
        }
    }

    fn view(&self) -> Element<Message> {
        container(
            column!(
                container(text("Rusty-Comfort").size(25)).padding(30),
                row!(
                    pick_list(&NoiseMode::ALL[..], self.mode, Message::ModeChanged),
                    horizontal_space(),
                    button(iced::widget::Image::new("mute.png").content_fit(ContentFit::Cover)).width(50).height(35).on_press(Message::MuteToggle)
                ),
                row!(
                    column!(
                        text("Vol"),
                        vertical_slider(0.0..=1000.0, self.volume.value() * 1000.0, |value| {
                            Message::VolumeChanged(value / 1000.0)
                        })
                    ),
                    horizontal_space(),
                    column!(
                        text("Depth").align_x(alignment::Horizontal::Center),
                        vertical_slider(0.0..=4000.0, self.lowpass.value(), |value| {
                            Message::LowPassChanged(value)
                        })
                    ),
                    horizontal_space(),
                    column!(
                        text("Soft"),
                        vertical_slider(0.5..=700.0, self.q.value() * 1000.0, |value| {
                            Message::QChanged(value / 1000.0)
                        })
                    ),
                )
            )
            .padding(100)
            .height(Length::Fill)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Center),
        )
        .into()
    }
}

fn main() -> iced::Result {
    let config = config::read();

    let window_settings = iced::window::settings::Settings {
        size: iced::Size::new(500.0, 700.0),
        resizable: true,
        transparent: true,
        ..iced::window::Settings::default()
    };

    let audio_state: State = State {
        volume: Shared::new(config.volume),
        lowpass: Shared::new(config.lowpass),
        q: Shared::new(config.q),
        mode: Some(NoiseMode::Brown),
        muted: false,
    };

    audio::run(audio_state.clone());

    iced::application("Rusty-Comfort", State::update, State::view)
        .theme(|_| Theme::Dark)
        .centered()
        .window(window_settings)
        .run_with(move || (audio_state, Task::none()))
}
