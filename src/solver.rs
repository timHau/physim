use crate::point::Point;
use ndarray::{arr1, Array1};

pub struct Solver {
    pub points: Vec<Point>,
    gravity: Array1<f32>,
}

impl Solver {
    pub fn update(&mut self, dt: f32) {
        self.apply_gravity();
        self.apply_constraint();
        self.solve_collisions();
        self.update_positions(dt);
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
        let radius = 200.0;
        for point in self.points.iter_mut() {
            let d = point.pos_cur.clone();
            let dist = d.dot(&d).sqrt();
            if dist > radius - 0.5 * point.radius {
                let dir = d / dist;
                point.pos_cur = &pos + &dir * (radius - 0.5 * point.radius);
            }
        }
    }

    pub fn solve_collisions(&mut self) {
        let num_points = self.points.len();
        let mut points = self.points.clone();

        for i in 0..num_points {
            let p1 = points[i].clone();
            for k in i + 1..num_points {
                let p2 = points[k].clone();
                let d = &p1.pos_cur - &p2.pos_cur;
                let dist = d.dot(&d).sqrt();
                if dist < 0.5 * (p1.radius + p2.radius) {
                    let dir = d / dist;
                    let delta = 0.5 * (p1.radius + p2.radius) - dist;
                    points[i].pos_cur = &p1.pos_cur + &dir * delta;
                    points[k].pos_cur = &p2.pos_cur - &dir * delta;
                }
            }
        }

        self.points = points;
    }
}

impl Default for Solver {
    fn default() -> Self {
        Self {
            gravity: arr1(&[0.0, -9800.0]),
            points: vec![
                Point::new(arr1(&[100.0, 50.0]), 60.0),
                Point::new(arr1(&[-100.0, 50.0]), 60.0),
            ],
        }
    }
}
