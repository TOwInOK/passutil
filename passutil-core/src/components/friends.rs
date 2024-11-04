use super::utils::{draw_image, get_image_data};
use piet::{kurbo::Rect, RenderContext};
use std::path::PathBuf;

pub fn draw_friends(rc: &mut impl RenderContext) -> Result<(), Box<dyn std::error::Error>> {
    let x0 = 1017.;
    let y0 = 24.;
    let height = 74.;

    let dir_data = std::fs::read_dir("friends")?;
    let mut paths: Vec<PathBuf> = vec![];

    for entry in dir_data {
        let Ok(entry) = entry else {
            continue;
        };
        let path = entry.path().clone();
        let Ok(metadata) = entry.metadata() else {
            continue;
        };
        if metadata.is_file() {
            paths.push(path);
        }
    }

    if paths.len() < 4 {
        let size = (height + height, height + height);

        let mut x = x0;
        for i in 0..paths.len() {
            let image = get_image_data(paths[i].clone())?;
            let _ = draw_image(rc, image, Rect::from_origin_size((x, y0), size));

            x += height + height;
        }
    } else {
        let size = (height, height);
        let mut x = x0;
        let y = y0;
        for i in 0..paths.len().min(7) {
            let image = get_image_data(paths[i].clone())?;
            let _ = draw_image(rc, image, Rect::from_origin_size((x, y), size));

            x += height;
        }
        x = x0;
        let y = y0 + height;

        for i in 7..paths.len().min(14) {
            let image = get_image_data(paths[i].clone())?;
            let _ = draw_image(rc, image, Rect::from_origin_size((x, y), size));

            x += height;
        }
    }

    Ok(())
}
