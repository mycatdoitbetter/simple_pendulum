mod pendulum;

use crate::pendulum::*;
use nannou::prelude::*;

struct Model {
    first_pendulum: Pendulum,
    second_pendulum: Pendulum,
}

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(500, 700)
        .run();
}

fn model(app: &App) -> Model {
    let boundaries = app.window_rect();

    let origin = vec2(boundaries.x(), boundaries.y());
    let position = vec2(boundaries.x(), boundaries.y.end - 60.0);
    let radius = boundaries.y.end * 0.6;

    let first_pendulum = Pendulum {
        angle: PI / 2.0,
        radius,
        origin,
        position,
        angular_acceleration: 0.0,
        angular_velocity: 0.0,
        friction: 0.999,
        gravity: 0.8,
        mass: 35.0,
    };

    let second_pendulum = Pendulum {
        radius: radius * 0.9,
        mass: 30.0,
        ..first_pendulum
    };

    Model {
        first_pendulum,
        second_pendulum,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.first_pendulum.update();
    model.second_pendulum.update();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(rgb8(27, 36, 48)); // rgb(27, 36, 48)

    draw.ellipse()
        .color(rgb8(214, 213, 168)) // rgb(214, 213, 168)
        .w_h(5.0, 5.0);

    model.first_pendulum.draw(&draw);
    model.second_pendulum.draw(&draw);

    draw.to_frame(app, &frame).unwrap();
}
