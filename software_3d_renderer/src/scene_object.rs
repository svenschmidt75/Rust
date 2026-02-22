use crate::matrix4::Matrix4;
use crate::render_context::RenderContext;
use crate::renderable::Renderable;

pub struct SceneObject {
    transforms: Vec<Box<dyn FnMut(f32) -> Matrix4>>,
    scene_object: Box<dyn Renderable>,
}

impl SceneObject {
    pub fn new(scene_object: Box<dyn Renderable>) -> Self {
        Self {
            transforms: vec![],
            scene_object,
        }
    }

    pub fn add_transform(&mut self, transform: Box<dyn FnMut(f32) -> Matrix4>) {
        self.transforms.push(transform);
    }

    pub fn render(&mut self, ctx: &mut RenderContext, delta: f32) {
        // SS: determine aggregated transform matrix
        let transform_matrix = self
            .transforms
            .iter_mut()
            .fold(Matrix4::identity(), |acc, tr| {
                let m = tr(delta);
                m * acc
            });
        self.scene_object.render(ctx, transform_matrix);
    }
}
