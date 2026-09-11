use crate::ray_intersect::{Intersect, Material, RayIntersect};
use nalgebra_glm::Vec3;

pub struct Cube {
    pub min: Vec3,   // esquina mínima (-h,-h,-h)
    pub max: Vec3,   // esquina máxima ( h, h, h)
    pub material: Material,
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect> {
        let mut t_min = f32::NEG_INFINITY;
        let mut t_max = f32::INFINITY;
        let mut normal = Vec3::new(0.0, 0.0, 0.0);

        let axes = [
            (ray_origin.x, ray_direction.x, self.min.x, self.max.x,
             Vec3::new(-1.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0)),
            (ray_origin.y, ray_direction.y, self.min.y, self.max.y,
             Vec3::new(0.0, -1.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),
            (ray_origin.z, ray_direction.z, self.min.z, self.max.z,
             Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 0.0, 1.0)),
        ];

        for (o, d, lo, hi, n_lo, n_hi) in axes {
            if d.abs() < 1e-6 {
                // si el origen ya está fuera de la franja, nunca hay intersección posible
                if o < lo || o > hi {
                    return None;
                }
                continue;
            }

            let mut t1 = (lo - o) / d;
            let mut t2 = (hi - o) / d;
            let mut n1 = n_lo;
            let mut n2 = n_hi;

            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
                std::mem::swap(&mut n1, &mut n2);
            }

            if t1 > t_min {
                t_min = t1;
                normal = n1;
            }
            if t2 < t_max {
                t_max = t2;
            }

            if t_min > t_max {
                return None;
            }
        }

        // Si el t resultante no es positivo, se descarta.
        let t = if t_min > 1e-4 { t_min } else { t_max };
        if t <= 0.0 {
            return None;
        }

        let point = ray_origin + ray_direction * t;

        Some(Intersect {
            point,
            normal,
            distance: t,
            material: self.material,
        })
    }
}