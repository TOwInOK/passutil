mod avatar;
mod background;
mod badges;
mod banners;
mod frame;
mod friends;
mod text;
pub mod utils;

use std::path::PathBuf;

use piet::ImageBuf;
use serde::{Deserialize, Serialize};
use utils::get_image_data;

pub use {
    avatar::draw_avatar,
    background::{draw_background, Background},
    badges::{draw_badges, Badge},
    banners::{draw_banners, Banner},
    frame::{draw_frame, Frame},
    friends::draw_friends,
    text::{draw_text_blobs, TextBlob},
};

pub trait ImageEnum {
    fn get_path(&self) -> String;

    fn get_image(&self) -> Result<ImageBuf, Box<dyn std::error::Error>> {
        get_image_data(PathBuf::new().join(self.get_path()))
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub nickname: TextBlob,
    pub entry_time: TextBlob,
    pub about: TextBlob,
    pub background: Background,
    pub banners: [Banner; 2],
    pub main_badges: [Badge; 2],
    pub badges: [[Badge; 4]; 5],
    pub frame: Frame,
}
