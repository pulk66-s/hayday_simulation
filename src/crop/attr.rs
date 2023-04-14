use crate::game::context::Context;

pub trait Crop {
    fn harvest(&self, context: &mut Context) -> bool;
    fn new() -> Self;
}