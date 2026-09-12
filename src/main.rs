mod camera;
mod color;
mod framebuffer;
mod light;
mod ray_intersect;
mod cube;
mod texture;

use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{dot, normalize, Vec3};
use std::f32::consts::PI;
use std::time::Duration;

use crate::camera::Camera;
use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::light::Light;
use crate::ray_intersect::{Intersect, Material, RayIntersect};
use crate::cube::Cube;
use crate::texture::Texture;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const BACKGROUND_COLOR: u32 = 0xDCE9E5;

const FOV: f32 = PI / 3.0;

const ROTATION_SPEED: f32 = PI / 60.0;

const SHADOW_BIAS: f32 = 1e-3;
const GROUND_Y: f32 = -1.75;
const CAMERA_GROUND_MARGIN: f32 = 0.5;
const AMBIENT_INTENSITY: f32 = 0.34;

// Paleta LEGO
const LEGO_YELLOW: u32 = 0xFFD500;
const LEGO_BASEPLATE: u32 = 0x7EB3C1;

// Retícula de botones (studs)
const BRICK_STUD_PITCH: f32 = 1.0;
const BRICK_STUD_RADIUS: f32 = 0.31;
const PLATE_STUD_PITCH: f32 = 0.5;
const PLATE_STUD_RADIUS: f32 = 0.155;

pub fn cast_shadow(
    intersect: &Intersect,
    light_direction: &Vec3,
    light: &Light,
    objects: &[Box<dyn RayIntersect>],
) -> bool {
    let shadow_ray_origin = intersect.point + intersect.normal * SHADOW_BIAS;
    let light_distance = (light.position - intersect.point).magnitude();

    objects.iter().any(|object| {
        object
            .ray_intersect(&shadow_ray_origin, light_direction)
            .is_some_and(|blocker| blocker.distance < light_distance)
    })
}

pub fn shade(
    intersect: &Intersect,
    light: &Light,
    objects: &[Box<dyn RayIntersect>],
) -> Color {
    // La textura entrega el color difuso y una normal con el relieve del botón.
    let sample = intersect.material.sample(&intersect.point, &intersect.normal);

    let light_direction = (light.position - intersect.point).normalize();

    let light_intensity = if cast_shadow(intersect, &light_direction, light, objects) {
        0.0
    } else {
        light.intensity
    };

    let diffuse_intensity = dot(&sample.normal, &light_direction).max(0.0);

    let direct =
        sample.color * (diffuse_intensity * intersect.material.albedo * light_intensity);
    let ambient = sample.color * AMBIENT_INTENSITY;

    direct + ambient
}

pub fn cast_ray(
    ray_origin: &Vec3,
    ray_direction: &Vec3,
    objects: &[Box<dyn RayIntersect>],
    light: &Light,
) -> Color {
    let mut closest: Option<Intersect> = None;

    for object in objects {
        if let Some(intersect) = object.ray_intersect(ray_origin, ray_direction) {
            if closest.is_none_or(|current| intersect.distance < current.distance) {
                closest = Some(intersect);
            }
        }
    }

    let Some(intersect) = closest else {
        return Color::from_hex(BACKGROUND_COLOR);
    };

    shade(&intersect, light, objects)
}

pub fn render(
    framebuffer: &mut Framebuffer,
    objects: &[Box<dyn RayIntersect>],
    camera: &Camera,
    light: &Light,
) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    let perspective_scale = (FOV / 2.0).tan();

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            let screen_x = screen_x * aspect_ratio * perspective_scale;
            let screen_y = screen_y * perspective_scale;

            let ray_direction = normalize(&Vec3::new(screen_x, screen_y, -1.0));
            let ray_direction = camera.basis_change(&ray_direction);

            framebuffer.set_current_color(
                cast_ray(&camera.eye, &ray_direction, objects, light).to_hex(),
            );
            framebuffer.point(x, y);
        }
    }
}

fn main() {
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let mut window = Window::new("Ray Tracer Cube", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    let brick = Material::textured(
        Color::from_hex(LEGO_YELLOW),
        1.0,
        Texture::Lego {
            pitch: BRICK_STUD_PITCH,
            radius: BRICK_STUD_RADIUS,
        },
    );
    let baseplate = Material::textured(
        Color::from_hex(LEGO_BASEPLATE),
        1.05,
        Texture::Lego {
            pitch: PLATE_STUD_PITCH,
            radius: PLATE_STUD_RADIUS,
        },
    );

    let objects: Vec<Box<dyn RayIntersect>> = vec![
        Box::new(Cube {
            min: Vec3::new(-1.0, GROUND_Y, -1.0),
            max: Vec3::new(1.0, 0.25, 1.0),
            material: brick,
        }),
        Box::new(Cube {
            min: Vec3::new(-10.0, -2.0, -10.0),
            max: Vec3::new(10.0, GROUND_Y, 10.0),
            material: baseplate,
        }),
    ];

    let light = Light::new(Vec3::new(-6.0, 7.0, 8.0), Color::new(255, 255, 255), 1.0);

    let mut camera = Camera::new(
        Vec3::new(0.0, 3.0, 6.5),
        Vec3::new(0.0, -0.6, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        GROUND_Y + CAMERA_GROUND_MARGIN,
    );

    let mut camera_moved = true;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let orbit = [
            (Key::Left, ROTATION_SPEED, 0.0),
            (Key::Right, -ROTATION_SPEED, 0.0),
            (Key::Up, 0.0, -ROTATION_SPEED),
            (Key::Down, 0.0, ROTATION_SPEED),
        ];

        for (key, delta_yaw, delta_pitch) in orbit {
            if window.is_key_down(key) {
                camera.orbit(delta_yaw, delta_pitch);
                camera_moved = true;
            }
        }

        if camera_moved {
            render(&mut framebuffer, &objects, &camera, &light);
            camera_moved = false;
        }

        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}