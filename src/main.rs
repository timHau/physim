mod point;
mod solver;

use nannou::prelude::*;
use solver::Solver;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

struct Model {
    solver: Solver,
}

fn model(_app: &App) -> Model {
    let solver = Solver::default();
    Model { solver }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.solver.update(0.01);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    draw.ellipse().x_y(0.0, 0.0).w_h(400.0, 400.0).color(BLACK);
    for point in &model.solver.points {
        point.render(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(WIDTH, HEIGHT)
        .run();
}
