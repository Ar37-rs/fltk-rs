use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};

fn main() {
    let app = app::App::default();
    let wind = Window::default().with_size(400, 300);
    let frame = Frame::default().with_size(200, 100).center_of(&wind);
    let but = Button::new(160, 210, 80, 40, "Click me!");
    wind.end();
    wind.show();

    but.set_callback(move |_| frame.set_label("Hello world"));

    app.run().unwrap();
}
