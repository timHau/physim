use crate::point::Point;
use nannou::{prelude::Vec2, Draw};
use std::any::Any;

pub enum PhysicsObject {
    Point,
    Link,
}

pub trait PhysicsTarget {
    fn update_position(&mut self, dt: f32);
    fn accelerate(&mut self, acc: &Vec2);
    fn apply_constraints(&mut self, constraint: &Point);
    fn solve_collisions(&mut self, other: &mut Box<dyn PhysicsTarget>);
    fn object_type(&mut self) -> &PhysicsObject;
    fn render(&self, draw: &Draw);
    fn as_any(&mut self) -> &mut dyn Any;
}
