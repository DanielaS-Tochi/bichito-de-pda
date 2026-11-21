# Bichito de PDA

Proyecto final de WayLearn Solana Engineering Track (Advanced).
Mascota virtual cuyo estado vive en una PDA en Solana. Pompon es el nombre de nuestra mascota de demostración; cada persona podrá elegir el suyo. Proyecto de práctica, no un producto.

## Estado

Primer paso de implementación: el programa registra a la mascota en una PDA asociada a la wallet de su dueña. La cuenta `Pet` guarda la autoridad, el nombre, el número de cuidados y el bump de la PDA. En esta etapa están implementados el registro y una prueba de inicialización; luego agregaremos los cuidados, las reglas de autorización y el cierre condicionado. El frontend todavía no está implementado.

## Requisitos (versiones con las que se probó)

- Solana CLI 3.1.10, Anchor 1.2.1, Rust 1.89 (build-sbf), Node 24
- WSL2 (Ubuntu) en Windows 11

## Cómo compilar y testear

```bash
anchor build --arch v0
cargo test
```

Nota: `--arch v0` fue la configuración con la que funcionó la prueba inicial del contador y LiteSVM 0.10.0. Hay que volver a validar la prueba de `Pet` con esa configuración en el entorno de desarrollo.
