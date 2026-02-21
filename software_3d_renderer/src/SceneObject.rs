use crate::matrix4::Matrix4;
use crate::renderable::Renderable;

pub struct SceneObject {
    transforms: Vec<Matrix4>,
    scene_object: Box<dyn Renderable>,
}

impl SceneObject {
    pub fn new(scene_object: Box<dyn Renderable>) -> Self {
        Self{
            transforms: vec![],
            scene_object,
        }
    }

    pub fn add_transform(&mut self, matrix: Matrix4) {

    }

    pub fn render(&self, delta: f32) {
        
    }
}