//! State of app
use crate::messages::Messages;
use iced::alignment::Horizontal::Center;
use iced::widget::{button, column, text, Column};
#[derive(Default)]
pub struct State {
    count: isize,
}

impl State {
    pub fn update(&mut self, message: Messages) {
        match message {
            Messages::Increment => {
                self.count += 1;
            }
            Messages::Decrement => {
                self.count -= 1;
            }
        }
    }

    pub fn view(&self) -> Column<Messages> {
        column![
            button("Increment").on_press(Messages::Increment),
            text(self.count).size(50),
            button("Decrement").on_press(Messages::Decrement)
        ]
        .padding(20)
        .align_x(Center)
    }
}
