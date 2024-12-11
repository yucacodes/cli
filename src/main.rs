use std::env;
use std::fs;
use std::io::Write;

fn main() {
    // Obtener los argumentos de la línea de comandos
    let args: Vec<String> = env::args().collect();

    // Verificar que hay suficientes argumentos
    if args.len() < 5 {
        eprintln!("Uso: {} cli_tool generate <tipo> <nombre_base>", args[0]);
        std::process::exit(1);
    }

    // Verificar la palabra clave
    let palabra_clave = &args[1];
    if palabra_clave != "cli_tool" {
        eprintln!("Error: la palabra clave debe ser 'cli_tool'.");
        std::process::exit(1);
    }

    // Obtener la acción
    let accion = &args[2];

    // Procesar la acción
    match accion.as_str() {
        "generate" => {
            let tipo = &args[3];
            let nombre_base = &args[4];
            crear_template(tipo, nombre_base);
        }
        _ => {
            eprintln!("Error: acción desconocida '{}'. Las acciones disponibles son: generate.", accion);
            std::process::exit(1);
        }
    }
}

// Función para crear un template
fn crear_template(tipo: &str, nombre_base: &str) {
    let (extension, contenido) = match tipo {
        "caso_de_uso" => ("usecase.ts", "// Caso de Uso\n\n/**\n * Descripción del caso de uso:\n * Actores:\n * Flujo principal:\n * Flujo alternativo:\n */\n"),
        "modelo" => ("model.ts", "// Modelo\n\n/**\n * Representación de datos del modelo:\n * Atributos:\n * Relaciones:\n */\n"),
        _ => {
            eprintln!("Error: tipo desconocido '{}'. Los tipos disponibles son: caso_de_uso, modelo.", tipo);
            std::process::exit(1);
        }
    };

    let nombre_archivo = format!("{}.{}", nombre_base, extension);

    match fs::File::create(&nombre_archivo) {
        Ok(mut file) => {
            if let Err(err) = file.write_all(contenido.as_bytes()) {
                eprintln!("Error al escribir en el archivo: {}", err);
                std::process::exit(1);
            }
            println!("Archivo '{}' generado exitosamente.", nombre_archivo);
        }
        Err(err) => {
            eprintln!("Error al crear el archivo: {}", err);
            std::process::exit(1);
        }
    }
}
