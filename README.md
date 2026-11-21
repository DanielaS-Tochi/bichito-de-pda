# Bichito de PDA

Proyecto final de WayLearn Solana Engineering Track (Advanced).
Mascota virtual (Nube) cuyo estado vive en una PDA en Solana. Proyecto de práctica, no un producto.

## Estado

Proyecto Anchor inicial (plantilla de contador) con tests en Rust funcionando. Próximo paso: reemplazar el contador por la cuenta `Pet`.

## Requisitos (versiones con las que se probó)

- Solana CLI 3.1.10, Anchor 1.2.1, Rust 1.89 (build-sbf), Node 24
- WSL2 (Ubuntu) en Windows 11

## Cómo compilar y testear

```bash
anchor build --arch v0
cargo test
```

Nota: se compila con `--arch v0` porque el build por defecto de `anchor build` generó SBPF v3 y LiteSVM 0.10.0 lo rechazó con `InvalidAccountData` en `add_program`. Con v0 el test pasa.
