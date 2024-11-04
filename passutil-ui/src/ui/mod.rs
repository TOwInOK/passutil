use filepicker::file_picker;
use iced::alignment::Horizontal::{self};
use iced::widget::{column, container, Container};

use crate::state::Message;

pub mod about;
pub mod background;
pub mod banner;
/// File picker for user image
pub mod filepicker;
/// Image board of images by path
pub mod imagegrid;
pub mod pin;
pub mod welcome;

/// Render app
pub fn render(state: &crate::state::State) -> Container<Message> {
    let app = column![file_picker(state)];
    container(app).align_x(Horizontal::Center)
}

#[derive(Debug, Default)]
pub enum Plates {
    #[default]
    Welcome,
    /// Budges, lmao
    Pin,
    Banner,
    Background,
    Frame,
    About,
}
