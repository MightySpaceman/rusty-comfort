use fundsp::shared::Shared;
use iced::{
    alignment,
    widget::{column, container, horizontal_space, row, text, vertical_slider},
    Element, Length, Task, Theme,
};

use std::sync::mpsc::*;

mod audio;

<<<<<<< HEAD
use std::time::*;

=======
>>>>>>> dev
#[derive(Clone)]
struct State {
    volume: Shared,
    lowpass: Shared,
    q: Shared,
<<<<<<< HEAD
    polled: bool,
    lastpoll: SystemTime,
    tx: Option<Sender<State>>,
=======
>>>>>>> dev
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
<<<<<<< HEAD
                self.tx.as_ref().unwrap().send(self.clone());
            }
            Message::LowPassChanged(pass) => {
                self.lowpass.set(pass);
                self.tx.as_ref().unwrap().send(self.clone());
            }
            Message::QChanged(q) => {
                self.q.set(q);
                self.tx.as_ref().unwrap().send(self.clone());
=======
            }
            Message::LowPassChanged(pass) => {
                self.lowpass.set(pass);
            }
            Message::QChanged(q) => {
                self.q.set(q);
>>>>>>> dev
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
<<<<<<< HEAD
    let (tx, rx) = channel();
    audio::run(rx);

    let now = SystemTime::now();
    
    let state: State = State {
        volume: 100.0,
        lowpass: 1000.0,
        q: 1.5,
        polled: true,
        lastpoll: now,
        tx: Some(tx),
    };

    state.tx.as_ref().unwrap().send(state.clone());
=======
    let state: State = State {
        volume: Shared::new(1.0),
        lowpass: Shared::new(1000.0),
        q: Shared::new(1.5),
    };

    audio::run(state.clone());

>>>>>>> dev
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
