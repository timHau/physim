use nannou::prelude::*;
use ndarray::{arr1, Array1};

#[derive(Debug, Clone)]
pub struct Point {
    pub pos_cur: Array1<f32>,
    pub pos_old: Array1<f32>,
    pub acc: Array1<f32>,
    pub radius: f32,
}

impl Point {
    pub fn render(&self, draw: &Draw) {
        draw.ellipse()
            .x_y(self.pos_cur[0], self.pos_cur[1])
            .w_h(self.radius, self.radius)
            .color(BLUE);
    }

    pub fn update_position(&mut self, dt: f32) {
        let vel = &self.pos_cur - &self.pos_old;
        self.pos_old = self.pos_cur.clone();
        self.pos_cur = &self.pos_cur + &vel + &self.acc * dt * dt; // verlet integration
        self.acc = arr1(&[0.0, 0.0]); // reset acceleration
    }

    pub fn accelerate(&mut self, acc: &Array1<f32>) {
        self.acc = &self.acc + acc.clone();
    }
}

impl Default for Point {
    fn default() -> Self {
        Self {
            pos_cur: arr1(&[150.0, 0.0]),
            pos_old: arr1(&[150.0, 0.0]),
            acc: arr1(&[0.0, 0.0]),
            radius: 50.0,
        }
    }
}
