use crate::{object::PhysicsTarget, point::Point, CONSTRAINT_RADIUS};
use nannou::prelude::Vec2;

pub struct Solver {
    pub objects: Vec<Box<dyn PhysicsTarget>>,
    gravity: Vec2,
    sub_steps: usize,
}

impl Solver {
    pub fn update(&mut self, dt: f32) {
        let sub_dt = dt / self.sub_steps as f32;
        for _ in 0..self.sub_steps {
            self.apply_gravity();
            self.apply_constraint();
            self.solve_collisions();
            self.update_positions(sub_dt);
        }
    }

    pub fn update_positions(&mut self, dt: f32) {
        for obj in self.objects.iter_mut() {
            obj.update_position(dt);
        }
    }

    pub fn apply_gravity(&mut self) {
        for obj in self.objects.iter_mut() {
            obj.accelerate(&self.gravity);
        }
    }

    pub fn apply_constraint(&mut self) {
        let constraint = Point::new([0.0, 0.0], CONSTRAINT_RADIUS, [0.0, 0.0, 0.0]);
        for obj in self.objects.iter_mut() {
            obj.apply_constraints(&constraint);
        }
    }

    pub fn solve_collisions(&mut self) {
        let num_objs = self.objects.len();

        for i in 0..num_objs {
            for k in (i + 1)..num_objs {
                if let Ok([p1, p2]) = self.objects.get_many_mut([i, k]) {
                    p1.solve_collisions(p2);
                }
            }
        }
    }
}

impl Default for Solver {
    fn default() -> Self {
        Self {
            gravity: Vec2::new(0.0, -1000.0),
            objects: vec![],
            sub_steps: 8,
        }
    }
}
