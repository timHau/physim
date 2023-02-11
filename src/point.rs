use crate::{
    link::Link,
    object::{PhysicsObject, PhysicsTarget},
};
use nannou::prelude::*;
use std::any::Any;

#[derive(Debug, Clone)]
pub struct Point {
    pub pos_cur: Vec2,
    pub pos_old: Vec2,
    pub acc: Vec2,
    pub radius: f32,
    pub is_fixed: bool,
    color: Rgba,
}

impl Point {
    pub fn new(pos: [f32; 2], radius: f32, color: [f32; 3]) -> Self {
        Self {
            pos_cur: Vec2::from(pos),
            pos_old: Vec2::from(pos),
            acc: Vec2::new(0.0, 0.0),
            radius,
            is_fixed: false,
            color: Rgba::new(color[0], color[1], color[2], 1.0),
        }
    }

    fn solve_point_collisions(&mut self, other: &mut Point) {
        let d = self.pos_cur - other.pos_cur;
        let dist = d.length();
        let min_dist = self.radius + other.radius;
        if dist < min_dist {
            let dir = d / dist;
            let delta = 0.5 * (min_dist - dist);
            self.pos_cur = self.pos_cur + 0.5 * dir * delta;
            other.pos_cur = other.pos_cur - 0.5 * dir * delta;
        }
    }

    fn solve_link_collisions(&mut self, other: &mut Link) {
        self.solve_point_collisions(&mut other.start);
        self.solve_point_collisions(&mut other.end);
    }
}

impl PhysicsTarget for Point {
    fn update_position(&mut self, dt: f32) {
        let vel = self.pos_cur - self.pos_old;
        self.pos_old = self.pos_cur.clone();
        self.pos_cur = self.pos_cur + vel + self.acc * (dt * dt); // verlet integration
        self.acc = Vec2::new(0.0, 0.0); // reset acceleration
    }

    fn accelerate(&mut self, acc: &Vec2) {
        self.acc = self.acc + acc.clone();
    }

    fn apply_constraints(&mut self, constraint: &Point) {
        let pos = constraint.pos_cur;
        let radius = constraint.radius;

        let d = self.pos_cur - pos;
        let dist = d.length();
        if dist > radius - self.radius {
            let dir = d / dist;
            self.pos_cur = pos + dir * (radius - self.radius);
        }
    }

    fn solve_collisions(&mut self, other: &mut Box<dyn PhysicsTarget>) {
        match other.object_type() {
            PhysicsObject::Point => {
                let other = other.as_any().downcast_mut::<Point>().unwrap();
                self.solve_point_collisions(other);
            }
            PhysicsObject::Link => {
                let other = other.as_any().downcast_mut::<Link>().unwrap();
                self.solve_link_collisions(other);
            }
        }
    }

    fn object_type(&mut self) -> &PhysicsObject {
        &PhysicsObject::Point
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn render(&self, draw: &Draw) {
        draw.ellipse()
            .x_y(self.pos_cur[0], self.pos_cur[1])
            .w_h(2.0 * self.radius, 2.0 * self.radius)
            .color(self.color);
    }
}
