# Actividad Ray Tracer Cube

Raytracer escrito en Rust que renderiza un cubo con iluminación difusa y una cámara orbital controlable desde el teclado.

<img width="990" height="778" alt="image" src="https://github.com/user-attachments/assets/5bb54962-bdd0-4bc6-8c66-10737aedbb77" />

## Estructura del proyecto

```
src/
├── main.rs          
├── camera.rs        # cámara orbital 
├── cube.rs          # intersección rayo-cubo 
├── ray_intersect.rs # infraestructura de intersección 
├── light.rs         # luz puntual
├── color.rs         # utilidades de color
└── framebuffer.rs   
```
