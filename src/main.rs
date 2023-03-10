#![feature(get_many_mut)]

mod link;
mod object;
mod point;
mod solver;

use link::Link;
use nannou::prelude::*;
use point::Point;
use rand::Rng;
use solver::Solver;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;
const TIME_STEP: f32 = 0.02;
const CONSTRAINT_RADIUS: f32 = 400.0;

struct Model {
    solver: Solver,
}

fn model(_app: &App) -> Model {
    let mut solver = Solver::default();

    let link = Link {
        points: (0..20)
            .map(|i| {
                let x = i as f32 * 20.0;
                let mut p = Point::new([-200.0 + x, -80.0], 10.0, [0.0, 1.0, 0.0]);
                if i == 0 || i == 19 {
                    p.is_fixed = true;
                }
                p
            })
            .collect::<Vec<_>>(),
        target_dist: 21.0,
    };
    solver.objects.push(Box::new(link));

    Model { solver }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let frames = app.elapsed_frames();
    if frames % 15 == 0 && frames < 2000 {
        println!("FPS: {}, frames: {}", app.fps(), (frames as f32).sin());
        let mut rng = rand::thread_rng();
        let radius = rng.gen_range(10.0..30.0);
        let start_pos = [rng.gen_range(-200.0..200.0), 300.0 + radius];
        let col = rng.gen_range(0.4..1.0);
        let point = Point::new(start_pos, radius, [col, col, col]);
        model.solver.objects.push(Box::new(point));
    }
    model.solver.update(TIME_STEP);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    // draw the constraint
    draw.ellipse()
        .x_y(0.0, 0.0)
        .w_h(2.0 * CONSTRAINT_RADIUS, 2.0 * CONSTRAINT_RADIUS)
        .color(BLACK);

    for obj in &model.solver.objects {
        obj.render(&draw);
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
        let mut rng = rand::thread_rng();
        let col = rng.gen_range(0.4..1.0);
        let color = [col, col, col];
        let point = point::Point::new([mouse.x, mouse.y], rng.gen_range(10.0..20.0), color);
        model.solver.objects.push(Box::new(point));
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
