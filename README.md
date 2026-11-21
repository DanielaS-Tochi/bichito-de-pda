# Bichito de PDA

Proyecto final de WayLearn Solana Engineering Track (Advanced).
Mascota virtual cuyo estado vive en una PDA en Solana. Pompon es el nombre de nuestra mascota de demostración; cada persona podrá elegir el suyo. Proyecto de práctica, no un producto.

## Estado

El programa registra a la mascota en una PDA asociada a la wallet de su dueña. La cuenta `Pet` guarda la autoridad, el nombre, el número de cuidados y el bump de la PDA. La dueña puede registrar y cuidar a su mascota hasta completar tres cuidados; se rechazan las acciones de otra wallet y los cuidados posteriores al límite. La PDA solo puede cerrarse al completar los tres cuidados, y sus lamports vuelven a la dueña. El frontend todavía no está implementado.

La prueba de LiteSVM verifica el registro de Pompon, los permisos, el límite de cuidados y el cierre condicionado, incluido el reembolso del saldo de la PDA.

## Requisitos (versiones con las que se probó)

- Solana CLI 3.1.10, Anchor 1.2.1, Rust 1.89 (build-sbf), Node 24
- WSL2 (Ubuntu) en Windows 11

## Cómo compilar y testear

```bash
anchor build --arch v0
cargo test
```

Nota: `--arch v0` fue la configuración con la que funcionó la prueba inicial del contador y LiteSVM 0.10.0. Hay que volver a validar la prueba de `Pet` con esa configuración en el entorno de desarrollo.
