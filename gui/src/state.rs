//! State of app

use std::path::PathBuf;
#[derive(Default)]
pub struct State {
    pub avatar_path: Option<PathBuf>,
}
