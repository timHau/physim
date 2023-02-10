use nannou::prelude::*;
use ndarray::{arr1, Array1};

#[derive(Debug, Clone)]
pub struct Point {
    pub pos_cur: Array1<f32>,
    pub pos_old: Array1<f32>,
    pub acc: Array1<f32>,
    pub radius: f32,
    color: Rgba,
}

impl Point {
    pub fn new(pos: Array1<f32>, radius: f32, color: [f32; 3]) -> Self {
        Self {
            pos_cur: pos.clone(),
            pos_old: pos,
            acc: arr1(&[0.0, 0.0]),
            radius,
            color: Rgba::new(color[0], color[1], color[2], 1.0),
        }
    }

    pub fn render(&self, draw: &Draw) {
        draw.ellipse()
            .x_y(self.pos_cur[0], self.pos_cur[1])
            .w_h(2.0 * self.radius, 2.0 * self.radius)
            .color(self.color);
    }

    pub fn update_position(&mut self, dt: f32) {
        let vel = &self.pos_cur - &self.pos_old;
        self.pos_old = self.pos_cur.clone();
        self.pos_cur = &self.pos_cur + &vel + &self.acc * (dt * dt); // verlet integration
        self.acc = arr1(&[0.0, 0.0]); // reset acceleration
    }

    pub fn accelerate(&mut self, acc: &Array1<f32>) {
        self.acc = &self.acc + acc.clone();
    }
}
