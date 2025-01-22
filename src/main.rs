use fundsp::hacker::Shared;
use iced::{
    alignment,
    widget::{column, container, horizontal_space, row, text, vertical_slider},
    Element, Length, Task, Theme,
};
mod audio;
mod config;

#[derive(Clone, Default)]
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
                container(text("Rusty-Comfort").size(25)).padding(30),
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
        ..iced::window::Settings::default()
    };

    let audio_state: State = State {
        volume: Shared::new(config.volume),
        lowpass: Shared::new(config.lowpass),
        q: Shared::new(config.q),
    };

    audio::run(audio_state.clone());

    iced::application("Rusty-Comfort", State::update, State::view)
        .theme(|_| Theme::Dark)
        .centered()
        .window(window_settings)
        .run_with(move || (audio_state, Task::none()))
}
