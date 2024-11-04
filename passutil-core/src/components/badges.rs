use super::{utils::draw_image, ImageEnum};
use piet::{
    kurbo::{Rect, Size},
    RenderContext,
};
use serde::{Deserialize, Serialize};

pub fn draw_badges(
    rc: &mut impl RenderContext,
    main_badges: &[Badge; 2],
    badges: &[[Badge; 4]; 5],
) -> Result<(), Box<dyn std::error::Error>> {
    draw_common_badges(rc, badges)?;

    draw_main_badges(rc, main_badges)
}

fn draw_common_badges(
    rc: &mut impl RenderContext,
    badges: &[[Badge; 4]; 5],
) -> Result<(), Box<dyn std::error::Error>> {
    let x0 = 1015.0;
    let y0 = 196.0;
    let width = 124.0;
    let height = 124.0;
    let size = Size::new(width, height);

    let x1 = 1160.0;
    let y1 = 336.0;
    let dx = x1 - x0;
    let dy = y1 - y0;
    let mut y = y0;
    for row in badges {
        let mut x = x0;
        for badge in row {
            let image = badge.get_image();
            let Ok(image) = image else {
                continue;
            };
            draw_image(rc, image, Rect::from_origin_size((x, y), size))?;

            x += dx;
        }

        y += dy;
    }

    Ok(())
}

fn draw_main_badges(
    rc: &mut impl RenderContext,
    main_badges: &[Badge; 2],
) -> Result<(), Box<dyn std::error::Error>> {
    let x0 = 710.0;
    let x1 = 710.0 + 291.0;
    let y0 = 337.0;
    let y1 = 337.0 + 263.0;

    let y2 = 617.0;
    let y3 = 617.0 + 263.0;

    let width = 263.0;
    let size = Size::new(width, width);

    'first: {
        let image = main_badges[0].get_image();
        let Ok(image) = image else {
            break 'first;
        };
        draw_image(
            rc,
            image,
            Rect::from_center_size(((x0 + x1) / 2.0, (y0 + y1) / 2.0), size),
        )?;
    }

    'second: {
        let image = main_badges[1].get_image();
        let Ok(image) = image else {
            break 'second;
        };
        draw_image(
            rc,
            image,
            Rect::from_center_size(((x0 + x1) / 2.0, (y2 + y3) / 2.0), size),
        )?;
    }

    Ok(())
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum Badge {
    none,
    achievements(usize),
    all(usize),
    anime(usize),
    coders(usize),
    codes(usize),
    coops(usize),
    creators(usize),
    events(usize),
    halloween(usize),
    holidays(usize),
    iwas(usize),
    limited(usize),
    messages(usize),
    music(usize),
    newyear(usize),
    oldpoints(usize),
    oldseasons(usize),
    oldsides(usize),
    paws(usize),
    rp(usize),
    shop(usize),
    support(usize),
    tournaments(usize),
}

impl Default for Badge {
    fn default() -> Self {
        Self::none
    }
}

impl ImageEnum for Badge {
    fn get_path(&self) -> String {
        match self {
            Badge::none => "".into(),
            Badge::achievements(num) => format!("badges/achievements/{}.png", num),
            Badge::all(num) => format!("badges/all/{}.png", num),
            Badge::anime(num) => format!("badges/anime/{}.png", num),
            Badge::coders(num) => format!("badges/coders/{}.png", num),
            Badge::codes(num) => format!("badges/codes/{}.png", num),
            Badge::coops(num) => format!("badges/coop/{}.png", num),
            Badge::creators(num) => format!("badges/creators/{}.png", num),
            Badge::events(num) => format!("badges/events/{}.png", num),
            Badge::halloween(num) => format!("badges/halloween/{}.png", num),
            Badge::holidays(num) => format!("badges/holidays/{}.png", num),
            Badge::iwas(num) => format!("badges/iwas/{}.png", num),
            Badge::limited(num) => format!("badges/limited/{}.png", num),
            Badge::messages(num) => format!("badges/messages/{}.png", num),
            Badge::music(num) => format!("badges/music/{}.png", num),
            Badge::newyear(num) => format!("badges/newyear/{}.png", num),
            Badge::oldpoints(num) => format!("badges/oldpoints/{}.png", num),
            Badge::oldseasons(num) => format!("badges/oldseasons/{}.png", num),
            Badge::oldsides(num) => format!("badges/oldsides/{}.png", num),
            Badge::paws(num) => format!("badges/paws/{}.png", num),
            Badge::rp(num) => format!("badges/rp/{}.png", num),
            Badge::shop(num) => format!("badges/shop/{}.png", num),
            Badge::support(num) => format!("badges/support/{}.png", num),
            Badge::tournaments(num) => format!("badges/tournaments/{}.png", num),
        }
    }
}
