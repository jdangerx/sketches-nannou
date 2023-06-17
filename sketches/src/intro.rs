extern crate nannou;
use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    my_box: Rect
}

fn model(_app: &App) -> Model {
    Model {
        my_box: Rect::from_w_h(100.0, 100.0)
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let win = app.window_rect();
    let sine = app.time.sin();
    let cosine = app.time.cos();

    let boundary = win
        .pad_left(model.my_box.w() / 2.0)
        .pad_right(model.my_box.w() / 2.0)
        .pad_top(model.my_box.h() / 2.0)
        .pad_bottom(model.my_box.h() / 2.0)
    ;

    let x = map_range(cosine, -1.0, 1.0, boundary.left(), boundary.right());
    let y = map_range(sine, -1.0, 1.0, boundary.top(), boundary.bottom());
    let w = map_range(cosine, -1.0, 1.0, 50.0, 150.0);
    let h = map_range(sine, -1.0, 1.0, 50.0, 150.0);
    model.my_box = Rect::from_x_y_w_h(x, y, w, h);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    frame.clear(WHEAT);

    draw.rect()
        .xy(model.my_box.xy())
        .wh(model.my_box.wh())
        .z_degrees(app.time * 10.0)
        .color(SLATEBLUE);

    draw.to_frame(app, &frame).unwrap();
}
