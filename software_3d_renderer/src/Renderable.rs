use crate::RenderContext::RenderContext;

trait Renderable {
    fn render(&self, ctx: &mut RenderContext);
}