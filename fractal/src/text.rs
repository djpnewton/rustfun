use quicksilver::{
    combinators::result,
    geom::Shape,
    graphics::{Background::Img, Color, Font, FontStyle, Image},
    lifecycle::{Asset, Window},
    Future, Result,
};

use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
pub enum Text {
    Space,
    R,
    C,
    Arrow,
    ZX,
    Number(i32),
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
}

impl From<char> for Text {
    fn from(ch: char) -> Self {
        if !ch.is_digit(10) {
            panic!("Cannot convert something else than a digit to a Text");
        } else {
            match ch {
                '0' => Text::Digit0,
                '1' => Text::Digit1,
                '2' => Text::Digit2,
                '3' => Text::Digit3,
                '4' => Text::Digit4,
                '5' => Text::Digit5,
                '6' => Text::Digit6,
                '7' => Text::Digit7,
                '8' => Text::Digit8,
                '9' => Text::Digit9,
                _ => Text::Digit0,
            }
        }
    }
}

pub struct TextRenderer {
    renders: HashMap<Text, Asset<Image>>,
}

impl TextRenderer {
    pub fn new() -> Self {
        let mut renders: HashMap<Text, Asset<Image>> = HashMap::new();

        let font = "Pixeled.ttf";

        renders.insert(
            Text::Space,
            Asset::new(Font::load(font).and_then(|font| {
                let style = FontStyle::new(32.0, Color::RED);
                result(font.render("press 'space' to start/stop", &style))
            })),
        );

        renders.insert(
            Text::R,
            Asset::new(Font::load(font).and_then(|font| {
                let style = FontStyle::new(32.0, Color::RED);
                result(font.render("press 'r' to randomize", &style))
            })),
        );

        renders.insert(
            Text::C,
            Asset::new(Font::load(font).and_then(|font| {
                let style = FontStyle::new(32.0, Color::RED);
                result(font.render("press 'c' to change fractal", &style))
            })),
        );

        renders.insert(
            Text::Arrow,
            Asset::new(Font::load(font).and_then(|font| {
                let style = FontStyle::new(32.0, Color::RED);
                result(font.render("press 'arrow' to transate fractal", &style))
            })),
        );

        renders.insert(
            Text::ZX,
            Asset::new(Font::load(font).and_then(|font| {
                let style = FontStyle::new(32.0, Color::RED);
                result(font.render("press 'z/x' to zoom fractal", &style))
            })),
        );

        renders.insert(
            Text::Digit0,
            Asset::new(Font::load(font).and_then(|font| {
                let style = FontStyle::new(32.0, Color::RED);
                result(font.render("0", &style))
            })),
        );

        renders.insert(
            Text::Digit1,
            Asset::new(Font::load(font).and_then(|font| {
                let style = FontStyle::new(32.0, Color::RED);
                result(font.render("1", &style))
            })),
        );

        renders.insert(
            Text::Digit2,
            Asset::new(Font::load(font).and_then(|font| {
                let style = FontStyle::new(32.0, Color::RED);
                result(font.render("2", &style))
            })),
        );

        renders.insert(
            Text::Digit3,
            Asset::new(Font::load(font).and_then(|font| {
                let style = FontStyle::new(32.0, Color::RED);
                result(font.render("3", &style))
            })),
        );

        renders.insert(
            Text::Digit4,
            Asset::new(Font::load(font).and_then(|font| {
                let style = FontStyle::new(32.0, Color::RED);
                result(font.render("4", &style))
            })),
        );

        renders.insert(
            Text::Digit5,
            Asset::new(Font::load(font).and_then(|font| {
                let style = FontStyle::new(32.0, Color::RED);
                result(font.render("5", &style))
            })),
        );

        renders.insert(
            Text::Digit6,
            Asset::new(Font::load(font).and_then(|font| {
                let style = FontStyle::new(32.0, Color::RED);
                result(font.render("6", &style))
            })),
        );

        renders.insert(
            Text::Digit7,
            Asset::new(Font::load(font).and_then(|font| {
                let style = FontStyle::new(32.0, Color::RED);
                result(font.render("7", &style))
            })),
        );

        renders.insert(
            Text::Digit8,
            Asset::new(Font::load(font).and_then(|font| {
                let style = FontStyle::new(32.0, Color::RED);
                result(font.render("8", &style))
            })),
        );

        renders.insert(
            Text::Digit9,
            Asset::new(Font::load(font).and_then(|font| {
                let style = FontStyle::new(32.0, Color::RED);
                result(font.render("9", &style))
            })),
        );

        TextRenderer { renders }
    }

    pub fn draw(&mut self, window: &mut Window, pos: (f32, f32), text: &Text) -> Result<()> {
        match text {
            Text::Number(number) => {
                let spacing = 15.;

                let digits = number.to_string();
                for (i, d) in digits.chars().enumerate() {
                    let render = self
                        .renders
                        .get_mut(&d.into())
                        .expect("Cannot get render from hashmap");

                    render.execute(|image| {
                        window.draw(
                            &image
                                .area()
                                .with_center((pos.0 + (i as f32 * spacing), pos.1)),
                            Img(&image),
                        );
                        Ok(())
                    })?;
                }
            }
            _ => {
                let render = self
                    .renders
                    .get_mut(&text)
                    .expect("Cannot get text from hashmap");

                render.execute(|image| {
                    window.draw(&image.area().with_center(pos), Img(&image));
                    Ok(())
                })?;
            }
        }

        Ok(())
    }
}
