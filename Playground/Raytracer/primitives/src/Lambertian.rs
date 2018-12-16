use Material::Material;

#[derive(Copy, Clone)]
pub struct Lambertian {}

impl Lambertian {
    pub fn new() -> Lambertian {
        Lambertian {}
    }
}

impl Material for Lambertian {}
