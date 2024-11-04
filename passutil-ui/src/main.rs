use messages::call;
use ui::render;

mod backend;
mod error;
mod messages;
mod result;
mod state;
mod ui;

fn main() -> iced::Result {
    // TODO: check assets before load app
    iced::run("Passutil", call, render)
}
