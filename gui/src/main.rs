use messages::call;
use ui::render;

mod backend;
mod error;
mod messages;
mod result;
mod state;
mod ui;

fn main() -> iced::Result {
    iced::run("Passutil", call, render)
}
