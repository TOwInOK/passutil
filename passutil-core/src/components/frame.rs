use piet::kurbo::Rect;
use piet::RenderContext;
use serde::{Deserialize, Serialize};

use super::utils::draw_image;
use super::ImageEnum;

pub fn draw_frame(
    rc: &mut impl RenderContext,
    frame: &Frame,
) -> Result<(), Box<dyn std::error::Error>> {
    let width = 1600.;
    let height = 900.;
    let image = frame.get_image()?;
    draw_image(
        rc,
        image,
        Rect::from_origin_size((0.0, 0.0), (width, height)),
    )
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum Frame {
    none,
    classic(usize),
    minecraft(usize),
    neon(usize),
    pastel(usize),
}

impl ImageEnum for Frame {
    fn get_path(&self) -> String {
        match self {
            Frame::none => "".into(),
            Frame::classic(num) => format!("frames/classic/{}.png", num),
            Frame::minecraft(num) => format!("frames/minecraft/{}.png", num),
            Frame::neon(num) => format!("frames/neon/{}.png", num),
            Frame::pastel(num) => format!("frames/pastel/{}.png", num),
        }
    }
}
