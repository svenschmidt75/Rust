use crate::render_context::RenderContext;

pub trait Renderable {
    fn render(&self, ctx: &mut RenderContext);
}