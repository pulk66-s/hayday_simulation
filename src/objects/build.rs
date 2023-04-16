use crate::game::context::Context;

pub trait Building {
    fn build(&self, ctx: &mut Context) -> bool;
    fn name(&self) -> String;
}
