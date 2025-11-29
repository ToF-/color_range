use std::process::exit;
use gtk4::gdk_pixbuf::{Colorspace, Pixbuf};
use gtk4::gdk::MemoryFormat;
use gtk4::gdk::Texture;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Picture};
use std::rc::Rc;

fn main() {
    let app = Application::builder()
        .application_id("example.rgba")
        .build();

    app.connect_activate(|app| {
        let width: u32 = 4096;
        let height: u32 = 4096;
        let mut pixels = vec![0u8; (width * height * 3) as usize];

        for y in 0..height {
            for x in 0..width {
                let r = (x % 256) as u8;
                let g = (y % 256) as u8;
                let b = (((x / 256) * 16) + (y / 256)) as u8; 

                let idx = ((y * width + x) * 3) as usize;
                pixels[idx] = r;
                pixels[idx + 1] = g;
                pixels[idx + 2] = b;
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

        win.present();
    });

    app.run();
}

