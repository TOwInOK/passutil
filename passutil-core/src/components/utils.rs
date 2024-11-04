use image::{DynamicImage, EncodableLayout, ImageReader};
use piet::{kurbo::Rect, ImageBuf, ImageFormat, RenderContext};
use std::{fs::File, io::BufReader, path::PathBuf};

pub fn draw_image(
    rc: &mut impl RenderContext,
    image: ImageBuf,
    rect: Rect,
) -> Result<(), Box<dyn std::error::Error>> {
    let image = rc.make_image(
        image.width(),
        image.height(),
        image.raw_pixels(),
        image.format(),
    )?;
    rc.draw_image(&image, rect, piet::InterpolationMode::Bilinear);

    Ok(())
}

pub fn get_raw_image_data(path: PathBuf) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let data = ImageReader::with_format(
        BufReader::new(File::open(path.clone())?),
        image::ImageFormat::WebP,
    )
    .decode();
    if let Ok(image) = data {
        return Ok(image);
    }
    let data = ImageReader::with_format(
        BufReader::new(File::open(path.clone())?),
        image::ImageFormat::Png,
    )
    .decode();
    if let Ok(image) = data {
        return Ok(image);
    }
    match ImageReader::with_format(BufReader::new(File::open(path)?), image::ImageFormat::Jpeg)
        .decode()
    {
        Ok(image) => Ok(image),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn get_image_data(path: PathBuf) -> Result<ImageBuf, Box<dyn std::error::Error>> {
    let data = get_raw_image_data(path)?;
    let data = data.into_rgba8();
    let width = data.width();
    let height = data.height();
    let data = data.as_bytes();

    Ok(ImageBuf::from_raw(
        data,
        ImageFormat::RgbaSeparate,
        width as usize,
        height as usize,
    ))
}
