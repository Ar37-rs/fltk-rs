use fltk::{prelude::*, *};

struct RoundImageBox;

impl RoundImageBox {
    pub fn new(radius: i32, image: image::RgbImage) -> Self {
        let frame = frame::Frame::new(0, 0, radius * 2, radius * 2, None);
        frame.set_frame(enums::FrameType::FlatBox);
        frame.draw(move |f| {
            image.scale(f.w(), f.h(), false, true);
            image.draw(f.x(), f.y(), f.w(), f.h());
            let color = f.color().to_rgb();
            let s = format!(
                "<?xml version='1.0' encoding='UTF-8' standalone='no'?>\n
              <svg width='{}' height='{}'>\n
                <rect x='{}' 
                    y='{}' 
                    rx='{}' 
                    ry='{}' 
                    width='{}' 
                    height='{}' 
                    fill='none' 
                    stroke='rgb({}, {}, {})' 
                    stroke-width='{}' />\n
              </svg>\n",
                f.w(),
                f.h(),
                -f.w() / 2,
                -f.w() / 2,
                f.w(),
                f.w(),
                f.w() + f.w(),
                f.h() + f.w(),
                color.0,
                color.1,
                color.2,
                f.w()
            );
            let s = image::SvgImage::from_data(&s).unwrap();
            s.draw(f.x(), f.y(), f.w(), f.h());
        });
        Self
    }
}

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    app::background(0, 0, 0);
    let image = image::SharedImage::load("screenshots/calc2.jpg")
        .unwrap()
        .to_rgb()
        .unwrap();

    let wind = window::Window::new(100, 100, 800, 400, "Hello from rust");
    let pack = group::Pack::default()
        .with_size(800, 200)
        .center_of_parent();
    pack.set_type(group::PackType::Horizontal);
    for i in 1..=4 {
        let color_depth = enums::ColorDepth::from_u8(i).unwrap();
        let image = image.convert(color_depth).unwrap();
        RoundImageBox::new(100, image);
    }
    pack.end();
    wind.end();
    wind.show();

    app.run().unwrap();
}
