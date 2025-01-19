use fundsp::shared::Shared;
use iced::{
    alignment,
    widget::{
        button, column, container, horizontal_rule, horizontal_space, row, scrollable, slider,
        text, text_input, vertical_slider, Column,
    },
    Element, Length, Padding, Settings,
};

use iced::settings;
use iced::window;
use iced::Size;
use iced::Task;
use iced::Theme;

use iced::{executor, Application};

use std::sync::mpsc::*;
use std::thread;
use std::time::Duration;

mod audio;
use audio::*;

#[derive(Clone)]
struct State {
    volume: Shared,
    lowpass: Shared,
    q: Shared,
}

#[derive(Debug, Clone)]
enum Message {
    VolumeChanged(f32),
    LowPassChanged(f32),
    QChanged(f32),
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
        }
    }

    fn view(&self) -> Element<Message> {
        container(
            column!(
                container(text("Brown Noise Player").size(25)).padding(30),
                row!(
                    column!(
                        text("Vol"),
                        vertical_slider(0.0..=100.0, self.volume.value() * 100.0, |value| {
                            Message::VolumeChanged(value / 100.0)
                        })
                    ),
                    horizontal_space(),
                    column!(
                        text("Depth").align_x(alignment::Horizontal::Center),
                        vertical_slider(0.0..=7000.0, self.lowpass.value(), |value| {
                            Message::LowPassChanged(value)
                        })
                    ),
                    horizontal_space(),
                    column!(
                        text("Bal"),
                        vertical_slider(0.5..=50.0, self.q.value() * 10.0, |value| {
                            Message::QChanged(value / 10.0)
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
    let state: State = State {
        volume: Shared::new(1.0),
        lowpass: Shared::new(1000.0),
        q: Shared::new(1.5),
    };

    audio::run(state.clone());

    let settings = iced::window::settings::Settings {
        size: iced::Size::new(500.0, 700.0),
        resizable: false,
        ..iced::window::Settings::default()
    };
    iced::application("Brown Noise Player", State::update, State::view)
        .theme(|_| Theme::Dark)
        .centered()
        .window(settings)
        .run_with(move || (state, Task::none()))
}
