use crate::color::Color;
use nalgebra_glm::Vec3;

/// Texturas procedurales. La variante LEGO dibuja una retícula de botones
/// (studs) sobre las caras que miran hacia arriba y perturba la normal para
/// que la iluminación los revele con relieve real.
#[derive(Debug, Clone, Copy)]
pub enum Texture {
    Plain,
    Lego {
        /// Separación entre centros de botones.
        pitch: f32,
        /// Radio del botón.
        radius: f32,
    },
}

/// Resultado del muestreo: color difuso y normal de sombreado.
pub struct Sample {
    pub color: Color,
    pub normal: Vec3,
}

/// Proporción del radio donde termina la tapa del botón y empieza la pared.
const CAP_RATIO: f32 = 0.76;
/// Inclinación de la normal en la pared del botón.
const WALL_TILT: f32 = 1.9;
/// Ancho relativo del surco de moldeo alrededor del botón.
const GROOVE_RATIO: f32 = 1.12;

impl Texture {
    pub fn sample(&self, base: Color, point: &Vec3, normal: &Vec3) -> Sample {
        match *self {
            Texture::Plain => Sample {
                color: base,
                normal: *normal,
            },
            Texture::Lego { pitch, radius } => lego(base, point, normal, pitch, radius),
        }
    }
}

fn lego(base: Color, point: &Vec3, normal: &Vec3, pitch: f32, radius: f32) -> Sample {
    // Los botones solo existen en la cara superior.
    if normal.y < 0.5 {
        return Sample {
            color: base * side_shade(point, normal, pitch),
            normal: *normal,
        };
    }

    let (offset_x, offset_z) = (cell_offset(point.x, pitch), cell_offset(point.z, pitch));
    let distance = (offset_x * offset_x + offset_z * offset_z).sqrt();

    // Superficie plana entre botones: surco de moldeo y retícula tenue.
    if distance > radius {
        let groove = if distance < radius * GROOVE_RATIO {
            0.82
        } else {
            1.0
        };
        let seam = seam_shade(point, pitch);
        return Sample {
            color: base * (groove * seam),
            normal: *normal,
        };
    }

    let ratio = distance / radius;

    // Tapa del botón: plana y apenas más clara por el brillo del plástico.
    if ratio <= CAP_RATIO {
        return Sample {
            color: base * 1.07,
            normal: *normal,
        };
    }

    // Pared cilíndrica del botón: se inclina la normal hacia afuera.
    let wall = (ratio - CAP_RATIO) / (1.0 - CAP_RATIO);
    let outward = Vec3::new(offset_x, 0.0, offset_z) / distance.max(1e-5);
    let tilted = (normal + outward * (wall * WALL_TILT)).normalize();

    Sample {
        color: base * (1.0 - 0.10 * wall),
        normal: tilted,
    }
}

/// Desplazamiento del punto respecto al centro de su celda.
fn cell_offset(value: f32, pitch: f32) -> f32 {
    let cell = (value / pitch).floor();
    value - (cell + 0.5) * pitch
}

/// Línea de unión entre celdas de la base.
fn seam_shade(point: &Vec3, pitch: f32) -> f32 {
    let border_x = (cell_offset(point.x, pitch).abs() - pitch * 0.5).abs();
    let border_z = (cell_offset(point.z, pitch).abs() - pitch * 0.5).abs();

    if border_x.min(border_z) < pitch * 0.02 {
        0.93
    } else {
        1.0
    }
}

/// Bisel de las caras laterales: se oscurece cerca del borde superior.
fn side_shade(point: &Vec3, normal: &Vec3, pitch: f32) -> f32 {
    if normal.y < -0.5 {
        return 0.85;
    }

    // Variación vertical muy leve para simular el molde del plástico.
    let ripple = ((point.y / pitch) * 6.0).sin() * 0.015;
    1.0 + ripple
}
