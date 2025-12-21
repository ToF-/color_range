use gtk::glib::{ControlFlow, Propagation};

use std::fmt;
use std::process::exit;
use gtk::gdk_pixbuf::{Colorspace, Pixbuf};
use gtk::gdk::MemoryFormat;
use gtk::gdk::Texture;
use gtk::prelude::*;
use gtk::{CssProvider, Application, ApplicationWindow, EventControllerKey, Picture, StyleContext};
use gtk::gdk::Display;
use std::rc::Rc;


use std::num::ParseIntError;
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn from_string(s: &str) -> Result<Self, ParseIntError> {
        u8::from_str_radix(&s[0..2], 16).and_then(|r| {
            u8::from_str_radix(&s[2..4], 16)
                .and_then(|g| u8::from_str_radix(&s[4..6], 16).and_then(|b| Ok(Color { r, g, b })))
        })
    }
}

#[derive(Debug)]
pub struct ParseColorRangeError;

impl fmt::Display for ParseColorRangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error parsing color range")
    }
}

impl std::error::Error for ParseColorRangeError {}

#[derive(Debug, Clone, Default)]
pub struct ColorRange {
    pub color_min: Color,
    pub color_max: Color,
}

impl ColorRange {
    pub fn default() -> Self {
        ColorRange {
            color_min: Color::default(),
            color_max: Color {
                r: 255,
                g: 255,
                b: 255,
            },
        }
    }

    pub fn from_string(s: &str) -> Result<Self, ParseColorRangeError> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        match parts.len() {
            2 => match Color::from_string(parts[0]) {
                Ok(color_min) => match Color::from_string(parts[1]) {
                    Ok(color_max) => 
                        Ok(ColorRange {
                            color_min,
                            color_max,
                        })
                    ,
                    Err(_) => Err(ParseColorRangeError),
                },
                Err(_) => Err(ParseColorRangeError),
            },
            _ => Err(ParseColorRangeError),
        }
    }

    pub fn matches(&self, color: Color) -> bool {
        let red_range = self.color_min.r..=self.color_max.r;
        let green_range = self.color_min.g..=self.color_max.g;
        let blue_range = self.color_min.b..=self.color_max.b;
        red_range.contains(&color.r)
            && green_range.contains(&color.g)
            && blue_range.contains(&color.b)
    }
}
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    min: String,
    max: String,
}

fn main() {
    let app = Application::builder()
        .application_id("example.rgba")
        .build();

    let args = Args::parse();

    let color_min = args.min;
    let color_max = args.max;
    match ColorRange::from_string(&format!("{} {}", color_min, color_max)) {
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        },
        Ok(range) => {
            app.connect_activate(move |app| {
                let width: u32 = 4096;
                let height: u32 = 4096;
                let mut pixels = vec![0u8; (width * height * 3) as usize];

                for y in 0..height {
                    for x in 0..width {
                        let r = (x % 256) as u8;
                        let g = (y % 256) as u8;
                        let b = (((x / 256) * 16) + (y / 256)) as u8; 
                        let color = Color {
                            r,
                            g,
                            b,
                        };
                        let idx = ((y * width + x) * 3) as usize;
                        if range.matches(color) {
                            pixels[idx] = r;
                            pixels[idx + 1] = g;
                            pixels[idx + 2] = b;
                        } else {
                            pixels[idx] = 0;
                            pixels[idx + 1] = 0;
                            pixels[idx + 2] = 0;
                        }
                    }
                }

                let bytes = glib::Bytes::from(&pixels);
                let pixbuf = Pixbuf::from_bytes(&bytes,
                    Colorspace::Rgb,
                    false,
                    8,
                    4096,
                    4096,
                    4096 * 3);
                let pic = Picture::for_pixbuf(&pixbuf);

                let win = ApplicationWindow::builder()
                    .application(app)
                    .title("RGB Map")
                    .child(&pic)
                    .build();
                let css = CssProvider::new();
                css.load_from_string(
                    "
            window {
                background-color: black;
            }
            "
                );

                StyleContext::add_provider_for_display(
                    &Display::default().unwrap(),
                    &css,
                    gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
                );
                let key_controller = EventControllerKey::new();
                key_controller.connect_key_pressed({
                    let app = app.clone();
                    move |_, _, _, _| {
                        app.quit();
                        Propagation::Stop
                    }
                });

                win.add_controller(key_controller);
                win.present();
            });

            let no_args: Vec<String> = vec![];
            app.run_with_args(&no_args);
        }
    }

}
