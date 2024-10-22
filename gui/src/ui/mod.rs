use crate::messages::Messages;
use iced::alignment::Horizontal::Center;
use iced::widget::{button, column, text, Column};

/// File picker for user image
mod filepicker;
/// footer
mod footer;
/// header
mod header;
/// Image board of images by path
mod imagegrid;
/// Selector for choose path
mod selector;

/// Render app
pub fn render(state: &crate::state::State) -> Column<Messages> {
    column![
        button("Increment").on_press(Messages::Increment),
        text(state.count).size(50),
        button("Decrement").on_press(Messages::Decrement)
    ]
    .align_x(Center)
}
