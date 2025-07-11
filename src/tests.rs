#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::decodificar_base64;

    #[test]
    fn test_decodificar_base64() {
        // "hola" en base64 es "aG9sYQ=="
        let resultado = decodificar_base64("aG9sYQ==").unwrap();
        assert_eq!(resultado, b"hola");

        // Prueba con una cadena vacía
        let resultado = decodificar_base64("").unwrap();
        assert_eq!(resultado, b"");

        // Prueba con base64 inválido
        assert!(decodificar_base64("%%%").is_err());
    }
}#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suma_basica() {
        assert_eq!(2 + 2, 4);
    }

    // Aquí puedes agregar más pruebas para tus funciones reales
}
