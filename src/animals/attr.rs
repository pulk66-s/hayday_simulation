use crate::game::context::Context;

pub trait Animal {
    fn buy(&self, ctx: &mut Context) -> bool;
}

