use crate::{
    object::{PhysicsObject, PhysicsTarget},
    point::Point,
};
use nannou::prelude::*;
use std::any::Any;

pub struct Link {
    pub start: Point,
    pub end: Point,
    pub target_dist: f32,
}

impl PhysicsTarget for Link {
    fn update_position(&mut self, dt: f32) {
        let axis = self.end.pos_cur - self.start.pos_cur;
        let dist = axis.length();
        if dist == 0.0 {
            return;
        }

        let dir = axis.normalize();
        let delta = dist - self.target_dist;
        if delta == 0.0 {
            return;
        }

        self.start.pos_cur = self.start.pos_cur + 0.5 * dir * delta;
        self.end.pos_cur = self.end.pos_cur - 0.5 * dir * delta;
    }

    fn accelerate(&mut self, acc: &Vec2) {
        self.start.accelerate(acc);
        self.end.accelerate(acc);
    }

    fn render(&self, draw: &Draw) {
        draw.line()
            .start(self.start.pos_cur)
            .end(self.end.pos_cur)
            .weight(1.0)
            .color(WHITE);
        self.start.render(draw);
        self.end.render(draw);
    }

    fn apply_constraints(&mut self, constraint: &Point) {
        self.start.apply_constraints(constraint);
        self.end.apply_constraints(constraint);
    }

    fn solve_collisions(&mut self, other: &mut Box<dyn PhysicsTarget>) {
        self.start.solve_collisions(other);
        self.end.solve_collisions(other);
    }

    fn object_type(&mut self) -> &PhysicsObject {
        &PhysicsObject::Link
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
