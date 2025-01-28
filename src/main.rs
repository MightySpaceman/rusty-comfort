use fundsp::{funutd::noise::Noise, hacker::Shared};
use iced::{
    alignment,
    daemon::DefaultStyle,
    widget::{
        button, column, container, horizontal_space, image, pick_list, row, text, vertical_slider,
        vertical_space,
    },
    ContentFit, Element, Length, Point, Task, Theme,
};
use std::sync::mpsc::*;
mod audio;
mod config;

#[derive(Clone)]
struct AppState {
    audiostate: AudioState,
    mode: Option<NoiseMode>,
    muted: bool,
    sender: Sender<NoiseMode>,
}

#[derive(Clone)]
struct AudioState {
    volume: Shared,
    lowpass: Shared,
    q: Shared,
}

impl Into<AudioState> for AppState {
    fn into(self) -> AudioState {
        self.audiostate
    }
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
    Muted,
}

impl NoiseMode {
    const ALL: [NoiseMode; 3] = [Self::Brown, Self::Pink, Self::White];
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
                Self::Muted => "Muted"
            }
        )
    }
}

impl AppState {
    fn update(&mut self, message: Message) {
        match message {
            Message::VolumeChanged(volume) => {
                self.audiostate.volume.set(volume);
            }
            Message::LowPassChanged(pass) => {
                self.audiostate.lowpass.set(pass);
            }
            Message::QChanged(q) => {
                self.audiostate.q.set(q);
            }
            Message::ModeChanged(mode) => {
                self.mode = Some(mode);
                self.sender.send(mode);
            }
            Message::MuteToggle => {
                if !self.muted {
                    self.sender.send(NoiseMode::Muted);
                } else {
                    self.sender.send(self.mode.unwrap());
                }
                self.muted = !self.muted;

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
                    button(iced::widget::Image::new("mute.png").content_fit(ContentFit::Cover))
                        .width(50)
                        .height(35)
                        .on_press(Message::MuteToggle)
                ),
                row!(
                    column!(
                        text("Vol"),
                        vertical_slider(0.0..=1000.0, self.audiostate.volume.value() * 1000.0, |value| {
                            Message::VolumeChanged(value / 1000.0)
                        })
                    ),
                    horizontal_space(),
                    column!(
                        text("Depth").align_x(alignment::Horizontal::Center),
                        vertical_slider(0.0..=4000.0, self.audiostate.lowpass.value(), |value| {
                            Message::LowPassChanged(value)
                        })
                    ),
                    horizontal_space(),
                    column!(
                        text("Soft"),
                        vertical_slider(0.5..=700.0, self.audiostate.q.value() * 1000.0, |value| {
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

    let (tx, rx) = channel::<NoiseMode>();

    let state = AppState {
        audiostate: AudioState {
            volume: Shared::new(config.volume),
            lowpass: Shared::new(config.lowpass),
            q: Shared::new(config.q),
        },
        mode: Some(NoiseMode::Brown),
        muted: false,
        sender: tx,
    };

    audio::run(state.clone().into(), rx);

    iced::application("Rusty-Comfort", AppState::update, AppState::view)
        .theme(|_| Theme::Dark)
        .centered()
        .window(window_settings)
        .run_with(move || (state, Task::none()))
}
