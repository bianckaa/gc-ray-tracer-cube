# Actividad Ray Tracer Cube

Raytracer escrito en Rust que renderiza un cubo con iluminación difusa, **textura** de ladrillo de LEGO y una cámara orbital controlable desde el teclado.

![Render](docs/render-lego.png)

## Textura LEGO


Las texturas no viene de una imagen, se generan calculando una función en cada punto donde el rayo golpea. Esto permite cambiar el color difuso y también perturbar la normal para crear relieve sin necesidad de texturas de imagen.

## Estructura del proyecto

```
src/
├── main.rs          
├── camera.rs        # cámara orbital 
├── cube.rs          # intersección rayo-cubo 
├── ray_intersect.rs # infraestructura de intersección y materiales
├── texture.rs       # texturas procedurales (LEGO)
├── light.rs         # luz puntual
├── color.rs         # utilidades de color
└── framebuffer.rs   
```
