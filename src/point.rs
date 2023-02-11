use nannou::prelude::*;

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

    pub fn render(&self, draw: &Draw) {
        draw.ellipse()
            .x_y(self.pos_cur[0], self.pos_cur[1])
            .w_h(2.0 * self.radius, 2.0 * self.radius)
            .color(self.color);
    }

    pub fn update_position(&mut self, dt: f32) {
        let vel = self.pos_cur - self.pos_old;
        self.pos_old = self.pos_cur.clone();
        self.pos_cur = self.pos_cur + vel + self.acc * (dt * dt); // verlet integration
        self.acc = Vec2::new(0.0, 0.0); // reset acceleration
    }

    pub fn accelerate(&mut self, acc: &Vec2) {
        self.acc = self.acc + acc.clone();
    }
}
