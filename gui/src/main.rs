use crate::state::State;
mod backend;
mod error;
mod messages;
mod result;
mod state;
fn main() -> iced::Result {
    iced::run("Passutil", State::update, State::view)
}
