use crate::game::context::Context;

pub trait Crop: Clone {
    fn harvest(&self, context: &mut Context) -> bool;
}
