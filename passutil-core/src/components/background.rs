use super::{utils::draw_image, ImageEnum};
use piet::{kurbo::Rect, RenderContext};
use serde::{Deserialize, Serialize};

pub fn draw_background(
    rc: &mut impl RenderContext,
    background: &Background,
) -> Result<(), Box<dyn std::error::Error>> {
    let width = 1600.;
    let height = 900.;
    let image = background.get_image()?;
    draw_image(
        rc,
        image,
        Rect::from_origin_size((0.0, 0.0), (width, height)),
    )
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum Background {
    all(usize),
    creators(usize),
    events(usize),
    holidays(usize),
    oldpoints(usize),
    oldseasons(usize),
    shop(usize),
    support(usize),
}

impl Default for Background {
    fn default() -> Self {
        Self::all(1)
    }
}

impl ImageEnum for Background {
    fn get_path(&self) -> String {
        match self {
            Background::all(num) => format!("backgrounds/all/{}.png", num),
            Background::creators(num) => format!("backgrounds/creators/{}.png", num),
            Background::events(num) => format!("backgrounds/events/{}.png", num),
            Background::holidays(num) => format!("backgrounds/holidays/{}.png", num),
            Background::oldpoints(num) => format!("backgrounds/oldpoints/{}.png", num),
            Background::oldseasons(num) => format!("backgrounds/oldseasons/{}.png", num),
            Background::shop(num) => format!("backgrounds/shop/{}.png", num),
            Background::support(num) => format!("backgrounds/support/{}.png", num),
        }
    }
}
