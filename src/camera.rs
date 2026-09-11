use nalgebra_glm::Vec3;
use std::f32::consts::PI;

const PITCH_LIMIT: f32 = PI / 2.0 - 0.1;

pub struct Camera {
    pub eye: Vec3,
    pub center: Vec3,
    pub up: Vec3,
    pub min_eye_y: f32,
}

impl Camera {
    pub fn new(eye: Vec3, center: Vec3, up: Vec3, min_eye_y: f32) -> Self {
        Camera {
            eye,
            center,
            up,
            min_eye_y,
        }
    }

    pub fn basis_change(&self, vector: &Vec3) -> Vec3 {
        let forward = (self.center - self.eye).normalize();
        let right = forward.cross(&self.up).normalize();

        let up = right.cross(&forward).normalize();

        let rotated = vector.x * right + vector.y * up - vector.z * forward;

        rotated.normalize()
    }

    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        let radius_vector = self.eye - self.center;
        let radius = radius_vector.magnitude();

        let current_yaw = radius_vector.z.atan2(radius_vector.x);
        let radius_xz =
            (radius_vector.x * radius_vector.x + radius_vector.z * radius_vector.z).sqrt();
        let current_pitch = (-radius_vector.y).atan2(radius_xz);

        let new_yaw = (current_yaw + delta_yaw) % (2.0 * PI);

        // El ojo no puede bajar más que el nivel del suelo (min_eye_y)
        let max_down_pitch = ((self.center.y - self.min_eye_y) / radius)
            .clamp(-1.0, 1.0)
            .asin()
            .min(PITCH_LIMIT);
        let new_pitch = (current_pitch + delta_pitch).clamp(-PITCH_LIMIT, max_down_pitch);

        self.eye = self.center
            + Vec3::new(
                radius * new_yaw.cos() * new_pitch.cos(),
                -radius * new_pitch.sin(),
                radius * new_yaw.sin() * new_pitch.cos(),
            );
    }
}