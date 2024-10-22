use piet::{
    kurbo::Point, Color, FontFamily, RenderContext, Text, TextAttribute, TextLayout,
    TextLayoutBuilder,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct TextBlob {
    pub text: String,
    pub font: String,
    pub color: Color,
}

const FONT_SIZE: f64 = 20.0;
const LINE_HEIGHT: f64 = 20.0;

pub fn draw_text_blobs(
    rc: &mut impl RenderContext,
    nickname: &TextBlob,
    entry_time: &TextBlob,
    about: &TextBlob,
) -> Result<(), Box<dyn std::error::Error>> {
    let x0 = 40.;
    let x1 = 173.;
    #[cfg(target_os = "windows")]
    let y0 = 442. - LINE_HEIGHT;
    #[cfg(not(target_os = "windows"))]
    let y0 = 446. - LINE_HEIGHT;
    let y1 = 479. - LINE_HEIGHT;
    let mut y = 518. - LINE_HEIGHT;

    let dy = 40.;
    {
        let font = nickname.font.clone();
        let text = nickname.text.clone();
        let color = nickname.color;

        draw_line(rc, text, font.clone(), color, (x1, y0).into())?;
    }

    {
        let font = entry_time.font.clone();
        let text = entry_time.text.clone();
        let color = entry_time.color;

        draw_line(rc, text, font.clone(), color, (x1, y1).into())?;
    }

    {
        let about = about.clone();
        let font = about.font.clone();
        let color = about.color;
        let lines = about
            .text
            .split("\n")
            .map(String::from)
            .collect::<Vec<String>>();

        let font = rc
            .text()
            .font_family(&font)
            .unwrap_or(FontFamily::SANS_SERIF);

        let mut line_x = x1;
        let max_x = x0 + 627.;

        for line in lines {
            let mut x = line_x;
            let words = line.split(" ").map(String::from).collect::<Vec<String>>();

            let mut is_first_in_line = true;

            for word in words {
                let layout = rc
                    .text()
                    .new_text_layout(if is_first_in_line {
                        word
                    } else {
                        " ".to_owned() + &word
                    })
                    .font(font.clone(), FONT_SIZE)
                    .default_attribute(TextAttribute::TextColor(color))
                    .build()?;
                is_first_in_line = false;

                let current_width = layout.size().width;
                if current_width + x > max_x {
                    line_x = x0;
                    x = line_x;
                    y += dy;
                }

                rc.draw_text(&layout, (x, y));

                x += current_width;
            }

            y += dy;
            line_x = x0;
        }
    }

    Ok(())
}

fn draw_line(
    rc: &mut impl RenderContext,
    text: String,
    font: String,
    color: Color,
    position: Point,
) -> Result<(), Box<dyn std::error::Error>> {
    let layout = rc
        .text()
        .new_text_layout(text.clone())
        .font(FontFamily::new_unchecked(font), FONT_SIZE)
        .default_attribute(TextAttribute::TextColor(color))
        .build();
    let layout = match layout {
        Ok(layout) => layout,
        Err(_) => rc
            .text()
            .new_text_layout(text)
            .font(FontFamily::SYSTEM_UI, FONT_SIZE)
            .default_attribute(TextAttribute::TextColor(color))
            .build()?,
    };

    rc.draw_text(&layout, position);

    Ok(())
}
