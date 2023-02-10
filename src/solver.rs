use crate::{point::Point, CONSTRAINT_RADIUS};
use ndarray::{arr1, Array1};

pub struct Solver {
    pub points: Vec<Point>,
    gravity: Array1<f32>,
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
        for point in self.points.iter_mut() {
            point.update_position(dt);
        }
    }

    pub fn apply_gravity(&mut self) {
        for point in self.points.iter_mut() {
            point.accelerate(&self.gravity);
        }
    }

    pub fn apply_constraint(&mut self) {
        let pos = arr1(&[0.0, 0.0]);
        let radius = CONSTRAINT_RADIUS;
        for point in self.points.iter_mut() {
            let d = &point.pos_cur.clone() - &pos;
            let dist = d.dot(&d).sqrt();
            if dist > radius - point.radius {
                let dir = d / dist;
                point.pos_cur = &pos + &dir * (radius - point.radius);
            }
        }
    }

    pub fn solve_collisions(&mut self) {
        let num_points = self.points.len();

        for i in 0..num_points {
            for k in (i + 1)..num_points {
                if let Ok([p1, p2]) = self.points.get_many_mut([i, k]) {
                    let d = &p1.pos_cur - &p2.pos_cur;
                    let dist = d.dot(&d).sqrt();
                    let min_dist = p1.radius + p2.radius;
                    if dist < min_dist {
                        let dir = d / dist;
                        let delta = 0.5 * (min_dist - dist);
                        p1.pos_cur = &p1.pos_cur + 0.5 * &dir * delta;
                        p2.pos_cur = &p2.pos_cur - 0.5 * &dir * delta;
                    }
                }
            }
        }
    }
}

impl Default for Solver {
    fn default() -> Self {
        Self {
            gravity: arr1(&[0.0, -1000.0]),
            points: vec![],
            sub_steps: 8,
        }
    }
}
