use crate::{
    object::{PhysicsObject, PhysicsTarget},
    point::Point,
};
use nannou::prelude::*;
use std::any::Any;

pub struct Link {
    pub points: Vec<Point>,
    pub target_dist: f32,
}

impl PhysicsTarget for Link {
    fn update_position(&mut self, dt: f32) {
        for i in 1..self.points.len() {
            if let Ok([prev, point]) = self.points.get_many_mut([i - 1, i]) {
                let axis = point.pos_cur - prev.pos_cur;
                let dist = axis.length();
                if dist == 0.0 {
                    return;
                }

                let dir = axis.normalize();
                let delta = dist - self.target_dist;
                if delta == 0.0 {
                    return;
                }

                if !point.is_fixed {
                    point.pos_cur = point.pos_cur - 0.5 * dir * delta;
                }
                if !prev.is_fixed {
                    prev.pos_cur = prev.pos_cur + 0.5 * dir * delta;
                }
                point.update_position(dt);
            }
        }
    }

    fn accelerate(&mut self, acc: &Vec2) {
        self.points.iter_mut().for_each(|p| p.accelerate(acc));
    }

    fn render(&self, draw: &Draw) {
        for i in 1..self.points.len() {
            let prev = &self.points[i - 1];
            let point = &self.points[i];
            draw.line()
                .start(prev.pos_cur)
                .end(point.pos_cur)
                .weight(1.0)
                .color(WHITE);
        }

        self.points.iter().for_each(|p| p.render(draw));
    }

    fn apply_constraints(&mut self, constraint: &Point) {
        self.points
            .iter_mut()
            .for_each(|p| p.apply_constraints(constraint));
    }

    fn solve_collisions(&mut self, other: &mut Box<dyn PhysicsTarget>) {
        self.points
            .iter_mut()
            .for_each(|p| p.solve_collisions(other));
    }

    fn object_type(&mut self) -> &PhysicsObject {
        &PhysicsObject::Link
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
