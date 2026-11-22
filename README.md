# Bichito de PDA

**Bichito de PDA** es un proyecto educativo para practicar Solana con Anchor. Cada persona registra una mascota asociada a su wallet, consulta su estado y puede cuidarla hasta tres veces. El estado vive en una cuenta PDA en Solana.

El nombre inicial es **Pompon** y se puede personalizar.

> Proyecto desarrollado como práctica para el Solana Engineering Track Advanced de WayLearn. No es un producto de producción.

## Red y programa

- **Red:** Solana Devnet
- **Program ID:** `ECoVYdegYcMrm8tC7sW1FKa8fAjY1bsQqwttj5xG6Kwr`
- **Explorar el programa:** [Solana Explorer](https://explorer.solana.com/address/ECoVYdegYcMrm8tC7sW1FKa8fAjY1bsQqwttj5xG6Kwr?cluster=devnet)
- **Evidencia de creación desde el frontend:** [Transacción confirmada en Devnet](https://explorer.solana.com/tx/5AQA3GYBtoP1DhEWCDYzz8ppMCzpCeDtURzGjHHxBKaiBMmKdk4PbXp7yLRHo2bGGJHrS1jWsdF42NSy3Bkv5VDG?cluster=devnet)

## Cómo funciona

La PDA de cada mascota se deriva usando la semilla `pet` y la dirección de su dueña. Su cuenta guarda la autoridad, el nombre, el número de cuidados y el bump.

El programa ofrece tres instrucciones:

- `initialize_pet`: crea la mascota con un nombre de entre 1 y 32 bytes UTF-8.
- `care_for_pet`: permite que solo la autoridad incremente sus cuidados, hasta un máximo de tres.
- `close_pet`: permite que solo la autoridad cierre la cuenta después de completar los tres cuidados. El alquiler de la cuenta se devuelve a la autoridad.

El frontend permite conectar una wallet, crear y consultar la mascota, registrar cuidados y solicitar el cierre. El programa Anchor valida las reglas y permisos en cadena; los controles de la interfaz solo ayudan a guiar el flujo.

## Tecnologías

- Rust y Anchor
- Solana Devnet
- LiteSVM para pruebas del programa
- Codama para generar el cliente TypeScript desde el IDL
- React, TypeScript, Vite y Solana Kit

## Estructura del repositorio

```text
programs/bichito-de-pda/   Programa Anchor y pruebas LiteSVM
clients/js/                Cliente TypeScript generado por Codama
app/                       Frontend React
target/idl/                IDL generado al compilar el programa
```

## Requisitos

- Rust y Cargo
- Solana CLI
- Anchor CLI
- Node.js y npm
- Una wallet compatible con Solana

## Preparación y ejecución

Desde la raíz del repositorio:

```bash
anchor build --arch v0
cargo test
npm install
npm --prefix app install
npm --prefix app run dev -- --host 0.0.0.0
```

Abrí la URL que muestra Vite, normalmente `http://localhost:5173`.

Para usar el frontend, conectá una wallet con **Solana Devnet** seleccionado y SOL de prueba. Podés obtener Devnet SOL desde el faucet de Solana.

## Cliente generado

Después de compilar el programa y generar el IDL, se puede regenerar el cliente TypeScript con:

```bash
npm run generate:client
```

## Verificaciones

```bash
anchor build --arch v0
cargo test
npm --prefix app run lint
npm --prefix app run build
```

Las pruebas LiteSVM cubren el registro, los permisos de autoridad, el límite de cuidados y las condiciones de cierre.

## Seguridad y alcance

La wallet firma las transacciones; la aplicación no almacena claves privadas. Las reglas importantes se aplican en el programa Anchor y no dependen únicamente de la interfaz.

Este repositorio es una demostración educativa para Devnet. No usar con fondos reales.

## Contribuciones

Las sugerencias y mejoras son bienvenidas mediante issues o pull requests. Para cambios en el programa, incluí o actualizá pruebas y regenerá el cliente Codama si cambia el IDL.

## Agradecimientos y créditos

Bichito de PDA es un proyecto educativo y lúdico realizado por Daniela como proyecto final del Solana Engineering Track (Advanced) de WayLearn. Agradezco a WayLearn y a Solana Foundation por el contexto y los recursos formativos.

El proyecto es una práctica independiente en Devnet; no es un producto oficial ni implica patrocinio, respaldo o revisión técnica por parte de WayLearn o Solana Foundation.

ChatGPT (OpenAI) colaboró con la exploración de ideas, explicaciones técnicas, sugerencias de implementación y documentación, y con la creación inicial de la ilustración de Pompon. Las decisiones, integración y pruebas finales estuvieron a cargo de Daniela.