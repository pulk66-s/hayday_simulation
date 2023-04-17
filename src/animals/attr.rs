use crate::game::context::Context;

pub trait Animal {
    fn buy(&self, ctx: &mut Context) -> bool;
    fn feed(&mut self, ctx: &mut Context) -> bool;
    fn receive(&mut self, ctx: &mut Context) -> bool;
}

