use crate::{object::PhysicsTarget, point::Point};
use nannou::prelude::*;

pub struct Link {
    pub start: Point,
    pub end: Point,
    pub target_dist: f32,
}

impl Link {
    pub fn render(&self, draw: &Draw) {
        draw.line()
            .start(self.start.pos_cur)
            .end(self.end.pos_cur)
            .weight(1.0)
            .color(WHITE);
        self.start.render(draw);
        self.end.render(draw);
    }
}

// impl PhysicsTarget for Link {
//     fn update_position(&mut self, dt: f32) {
//         let axis = self.end.pos_cur - self.start.pos_cur;
//         let dist = axis.length();
//         let dir = axis / dist;
//         let delta = self.target_dist - dist;

//         if !self.start.is_fixed {
//             self.start.pos_cur = self.start.pos_cur + 0.5 * dir * delta;
//         }
//         if !self.end.is_fixed {
//             self.end.pos_cur = self.end.pos_cur - 0.5 * dir * delta;
//         }
//     }

//     fn accelerate(&mut self, acc: &Vec2) {
//         self.start.accelerate(acc);
//         self.end.accelerate(acc);
//     }
// }
