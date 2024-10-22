// There are some key paths:
// 1. For USER
//   - /frineds/*.png
//   - avatar.png
// 2. For DEVELOPER
//   - /backgrounds/*.png
//   - /banners/*.png
//   - /badges/*.png
//   - /frames/*.png

use encoding_rs::WINDOWS_1251;
use piet::kurbo::Size;
use piet::RenderContext;

mod components;

use components::{
    draw_avatar, draw_background, draw_badges, draw_banners, draw_frame, draw_friends,
    draw_text_blobs, Config,
};

pub const SIZE: Size = Size::new(3200., 1800.);

use std::{fmt::Error, fs::File, io::Write, path::Path};

use piet_common::Device;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let size = SIZE;
    let scale = 2.0;
    let save_path = Path::new("res.png");

    let config = make_config()?;

    let s = serde_json::to_string(&config)?;
    println!("{s}");

    make_image(size, scale, save_path, config)
}
fn make_config() -> Result<Config, Box<dyn std::error::Error>> {
    let s = read_file("config.json".into())?;
    Ok(serde_json::from_str(&s)?)
}

fn read_file(path: String) -> Result<String, Box<dyn std::error::Error>> {
    match std::fs::read_to_string(path.clone()) {
        Ok(s) => Ok(s),
        Err(_) => {
            let bytes = std::fs::read(path)?;
            let (cow, _, _) = WINDOWS_1251.decode(bytes.as_slice());
            Ok(cow.into_owned())
        }
    }
}

fn make_image(
    size: Size,
    scale: f64,
    save_path: &Path,
    config: Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut device = Device::new()?;
    let mut target = device.bitmap_target(size.width as usize, size.height as usize, scale)?;
    let mut piet_context = target.render_context();

    draw(&mut piet_context, config)?;

    piet_context.finish()?;
    std::mem::drop(piet_context);

    target.save_to_file(save_path).map_err(Into::into)
}

fn handle_err(
    f: &mut File,
    e: Box<dyn std::error::Error>,
) -> Result<(), Box<dyn std::error::Error>> {
    log_str(f, &format!("Error found: {e}"))?;
    std::io::stdin().read_line(&mut "".into())?;
    Err(Box::new(Error))
}

fn log_str(f: &mut File, s: &str) -> Result<(), Box<dyn std::error::Error>> {
    f.write_fmt(format_args!("{s}\n"))?;
    println!("{s}");
    Ok(())
}

fn draw(rc: &mut impl RenderContext, config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let _ = std::fs::remove_file("logs.txt");
    let mut logs = File::create("logs.txt")?;

    log_str(&mut logs, "Background render start")?;
    match draw_background(rc, &config.background) {
        Ok(_) => println!("No errors emitted"),
        Err(e) => return handle_err(&mut logs, e),
    }

    log_str(&mut logs, "Badges render start")?;
    match draw_badges(rc, &config.main_badges, &config.badges) {
        Ok(_) => println!("No errors emitted"),
        Err(e) => return handle_err(&mut logs, e),
    }

    log_str(&mut logs, "Banners render start")?;
    match draw_banners(rc, &config.banners) {
        Ok(_) => println!("No errors emitted"),
        Err(e) => return handle_err(&mut logs, e),
    }

    log_str(&mut logs, "Text render start")?;
    match draw_text_blobs(rc, &config.nickname, &config.entry_time, &config.about) {
        Ok(_) => println!("No errors emitted"),
        Err(e) => return handle_err(&mut logs, e),
    }

    log_str(&mut logs, "Avatar render start")?;
    let _ = draw_avatar(rc);

    log_str(&mut logs, "Friends render start")?;
    let _ = draw_friends(rc);

    log_str(&mut logs, "Frame render start")?;
    match draw_frame(rc, &config.frame) {
        Ok(_) => println!("No errors emitted"),
        Err(e) => return handle_err(&mut logs, e),
    }

    Ok(())
}
