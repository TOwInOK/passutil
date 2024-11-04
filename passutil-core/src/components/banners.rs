use piet::{
    kurbo::{Rect, Size},
    RenderContext,
};
use serde::{Deserialize, Serialize};

use super::{utils::draw_image, ImageEnum};
// x1 - 710
// x2 - 860

// width - 140
// height - 306

pub fn draw_banners(
    rc: &mut impl RenderContext,
    banners: &[Banner; 2],
) -> Result<(), Box<dyn std::error::Error>> {
    let x0 = 710.0;
    let x1 = 860.0;
    let y = 0.0;

    let width = 140.0;
    let height = 306.0;
    let size = Size::new(width, height);

    'first: {
        let image = banners[0].get_image();
        let Ok(image) = image else {
            break 'first;
        };
        draw_image(rc, image, Rect::from_origin_size((x0, y), size))?;
    }

    'second: {
        let image = banners[1].get_image();
        let Ok(image) = image else {
            break 'second;
        };
        draw_image(rc, image, Rect::from_origin_size((x1, y), size))?;
    }

    Ok(())
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum Banner {
    none,
    all(usize),
    creators(usize),
    events(usize),
    holidays(usize),
    limited(usize),
    seasons(usize),
    shop(usize),
}

impl ImageEnum for Banner {
    fn get_path(&self) -> String {
        match self {
            Banner::none => "".into(),
            Banner::all(num) => format!("assets/banners/all/{}.png", num),
            Banner::creators(num) => format!("assets/banners/creators/{}.png", num),
            Banner::events(num) => format!("assets/banners/events/{}.png", num),
            Banner::holidays(num) => format!("assets/banners/holidays/{}.png", num),
            Banner::limited(num) => format!("assets/banners/limited/{}.png", num),
            Banner::seasons(num) => format!("assets/banners/seasons/{}.png", num),
            Banner::shop(num) => format!("assets/banners/shop/{}.png", num),
        }
    }
}
