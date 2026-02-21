use crate::matrix4::Matrix4;
use crate::render_context::RenderContext;
use crate::triangle::Triangle;

pub trait Renderable {
    fn render(&self, ctx: &mut RenderContext, delta: f32);
//    fn add_transform(&mut self, f: Box<dyn Fn(Triangle, f32) -> Triangle>);
}
