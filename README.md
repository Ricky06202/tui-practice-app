# TUI Practice App 🦀

Este proyecto es una aplicación de terminal (TUI) diseñada para aprender a construir interfaces de usuario desde cero utilizando **Rust**, sin depender de librerías externas (como `ratatui` o `tui-rs`). El objetivo principal es profundizar en el lenguaje Rust mientras se resuelven diversos problemas lógicos y matemáticos.

## 🚀 Cómo ejecutar el proyecto

Para poner en marcha la aplicación, asegúrate de tener instalado Rust y Cargo en tu sistema. Luego, simplemente ejecuta el siguiente comando en la raíz del proyecto:

```bash
cargo run
```

Cargo se encargará automáticamente de compilar todas las dependencias (si las hubiera en el futuro, aunque actualmente no usa ninguna) y ejecutar el binario resultante.

## 📂 Estructura del Proyecto

La lógica está organizada de manera modular para facilitar el aprendizaje:

- **`src/main.rs`**: El punto de entrada de la aplicación que coordina la ejecución.
- **`src/ui.rs`**: Contiene todo lo relacionado con la visualización y el manejo de la interfaz de usuario en la terminal.
- **Problemas**: Cada archivo individual representa un desafío o funcionalidad específica:
  - `game_of_life.rs`: Implementación del Juego de la Vida de Conway.
  - `magic_square.rs`: Generación o validación de cuadrados mágicos.
  - `unit_convert.rs`: Conversor de unidades.
  - `pass_gen.rs`: Generador de contraseñas seguras.
  - `parenthesis.rs`: Lógica de validación de paréntesis.

## 🛠️ Filosofía de Desarrollo

Este proyecto sigue una filosofía de **"Zero Dependencies"** para la UI. Todo el renderizado en consola, manejo de escapes ANSI y gestión de estado de la terminal se realiza utilizando únicamente la biblioteca estándar de Rust y scripts personalizados. Esto garantiza un aprendizaje profundo de cómo interactúa el software con el emulador de terminal.

---

> **Tip para el futuro:** Si el proyecto llega a requerir persistencia de datos local, te sugeriría utilizar **SQLite**, ya que es ligero, no requiere un servidor independiente y encaja perfectamente con la naturaleza "portable" de una herramienta CLI escrita en Rust.
