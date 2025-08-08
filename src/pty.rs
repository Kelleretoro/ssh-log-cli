// Código de ejemplo que incluye las correcciones necesarias

// Aquí debes incluir las correcciones específicas necesarias para el archivo src/pty.rs.
// Asegúrate de reemplazar las ocurrencias de `.map_err(PTYParserError::WriteError)?;` por `.map_err(|e| PTYParserError::WriteError(e))?;`

// Ejemplo de cómo podría lucir:

// fn write_to_pty(...) {
//     // Código anterior...
//     .map_err(|e| PTYParserError::WriteError(e))?;
//     // Código posterior...
// }

// Asegúrate de que todas las funciones donde se maneja escritura se actualicen.
