extern crate nannou;
use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
    let win_rect = app.window_rect();

    // get canvas to draw on
    let draw = app.draw();

    let win_p = win_rect.pad(25.0);
    draw.rect()
        .xy(win_p.xy())
        .wh(win_p.wh())
        .color(rgba(0.3, 0.4, 0.7, 0.5));

    // set background to blue
    draw.background().color(GAINSBORO);
    draw.ellipse().x_y(0.0, 0.0).color(DEEPPINK);

    let rectangle = Rect::from_w_h(100.0, 100.0).top_left_of(win_p);
    // let circle = rectangle.below(rectangle);
    let circle = Rect::from_w_h(200.0, 200.0)
        .top_left_of(win_p)
        .below(rectangle);

    draw.ellipse().xy(circle.xy()).wh(circle.wh()).color(SALMON);

    draw.rect()
        .xy(rectangle.xy())
        .wh(rectangle.wh())
        .color(PLUM);

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}
