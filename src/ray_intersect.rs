use crate::color::Color;
use crate::texture::{Sample, Texture};
use nalgebra_glm::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: Color,
    pub albedo: f32,
    pub texture: Texture,
}

impl Material {
    pub fn new(diffuse: Color, albedo: f32) -> Self {
        Material {
            diffuse,
            albedo,
            texture: Texture::Plain,
        }
    }

    pub fn textured(diffuse: Color, albedo: f32, texture: Texture) -> Self {
        Material {
            diffuse,
            albedo,
            texture,
        }
    }

    pub fn sample(&self, point: &Vec3, normal: &Vec3) -> Sample {
        self.texture.sample(self.diffuse, point, normal)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Intersect {
    pub point: Vec3,
    pub normal: Vec3,
    pub distance: f32,
    pub material: Material,
}

pub trait RayIntersect {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect>;
}