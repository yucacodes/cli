use std::env;
use std::fs;
use std::io::Write;

fn main() {
  
    let args: Vec<String> = env::args().collect();


    if args.len() < 4 {
        eprintln!("Uso: {} <palabra_clave> crear_template <tipo>", args[0]);
        std::process::exit(1);
    }


    let palabra_clave = &args[1];
    if palabra_clave != "cli_tool" {
        eprintln!("Error: la palabra clave debe ser 'cli_tool'.");
        std::process::exit(1);
    }


    let accion = &args[2];


    match accion.as_str() {
        "crear_template" => {
            let tipo = &args[3];
            crear_template(tipo);
        }
        _ => {
            eprintln!("Error: acción desconocida '{}'. Las acciones disponibles son: crear_template.", accion);
            std::process::exit(1);
        }
    }
}

fn crear_template(tipo: &str) {
    let (archivo, contenido) = match tipo {
        "caso_de_uso" => (
            "caso_de_uso.ts",
            "// Caso de Uso\n\n/**\n * Descripción del caso de uso:\n * Actores:\n * Flujo principal:\n * Flujo alternativo:\n */\n"
        ),
        "modelo" => (
            "modelo.ts",
            "// Modelo\n\n/**\n * Representación de datos del modelo:\n * Atributos:\n * Relaciones:\n */\n"
        ),
        _ => {
            eprintln!("Error: tipo desconocido '{}'. Los tipos disponibles son: caso_de_uso, modelo.", tipo);
            std::process::exit(1);
        }
    };

    match fs::File::create(archivo) {
        Ok(mut file) => {
            if let Err(err) = file.write_all(contenido.as_bytes()) {
                eprintln!("Error al escribir en el archivo: {}", err);
                std::process::exit(1);
            }
            println!("Archivo '{}' generado exitosamente.", archivo);
        }
        Err(err) => {
            eprintln!("Error al crear el archivo: {}", err);
            std::process::exit(1);
        }
    }
}
