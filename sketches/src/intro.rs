extern crate nannou;
use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    moon: Rect,
    texture: wgpu::Texture
}

fn model(app: &App) -> Model {
    let assets = app.assets_path().unwrap();
    let img_path = assets.join("crumpled-paper-25.png");
    let texture = wgpu::Texture::from_path(app, img_path).unwrap();

    Model {
        moon: Rect::from_w_h(200.0, 200.0),
        texture: texture
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
}

fn view(app: &App, model: &Model, frame: Frame) {
    let win = app.window_rect();
    let draw = app.draw();
    frame.clear(MIDNIGHTBLUE);

    let half = Rect::from_w_h(model.moon.w() / 2.0, model.moon.h());
    let left = draw.scissor(half.mid_left_of(model.moon));
    let right = draw.scissor(half.mid_right_of(model.moon));

    left.ellipse()
        .wh(model.moon.wh())
        .color(CORNSILK);

    right.ellipse()
        .wh(model.moon.wh())
        .color(SLATEBLUE);

    // TODO (daz): how to clip the texture to only this shape?
    left.texture(&model.texture);

    draw.to_frame(app, &frame).unwrap();
}
