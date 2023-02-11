use crate::point::Point;
use nannou::prelude::*;

pub struct Link {
    pub start: Point,
    pub end: Point,
    pub target_dist: f32,
}

impl Link {
    pub fn apply(&mut self) {
        let axis = self.end.pos_cur - self.start.pos_cur;
        let dist = axis.length();
        let dir = axis / dist;
        let delta = self.target_dist - dist;
        self.start.pos_cur = self.start.pos_cur + 0.5 * dir * delta;
        self.end.pos_cur = self.end.pos_cur - 0.5 * dir * delta;
    }

    pub fn render(&self, draw: &Draw) {
        // draw.line().start(start).end(end).weight(1.0).color(WHITE);
    }
}
