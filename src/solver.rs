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
            if dist > radius - point.radius / 2.0 {
                let dir = d / dist;
                point.pos_cur = &pos + &dir * (radius - point.radius / 2.0);
            }
        }
    }
}

impl Default for Solver {
    fn default() -> Self {
        Self {
            gravity: arr1(&[0.0, -9800.0]),
            points: vec![Point::default(); 1],
        }
    }
}
