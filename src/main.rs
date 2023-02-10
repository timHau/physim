#![feature(get_many_mut)]

mod point;
mod solver;

use nannou::prelude::*;
use ndarray::arr1;
use rand::Rng;
use solver::Solver;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const TIME_STEP: f32 = 0.01;

struct Model {
    solver: Solver,
}

fn model(_app: &App) -> Model {
    let solver = Solver::default();
    Model { solver }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.solver.update(TIME_STEP);
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

fn event(app: &App, model: &mut Model, event: Event) {
    if let Event::WindowEvent {
        simple: Some(MousePressed(MouseButton::Left)),
        ..
    } = event
    {
        let mouse = app.mouse.position();
        let pos = arr1(&[mouse.x, mouse.y]);
        let mut rng = rand::thread_rng();
        let point = point::Point::new(pos, rng.gen_range(20.0..50.0));
        model.solver.points.push(point);
    }
}
fn main() {
    nannou::app(model)
        .update(update)
        .event(event)
        .simple_window(view)
        .size(WIDTH, HEIGHT)
        .run();
}
