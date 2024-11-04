use piet::{kurbo::Rect, RenderContext};

use super::utils::{draw_image, get_image_data};

pub fn draw_avatar(rc: &mut impl RenderContext) -> Result<(), Box<dyn std::error::Error>> {
    let image = get_image_data("avatar.png".into())?;
    draw_image(rc, image, Rect::from_origin_size((337., 26.), (350., 350.)))
}
