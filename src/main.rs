use std::env;
use std::fs::File;
use std::io::Error;
use std::io::Read;

fn read_file_lines(path_de_archivo: &str) -> Result<Vec<String>, Error> {
    let mut archivo_a_leer = File::open(path_de_archivo)?;

    let mut datos_como_string = String::new();
    archivo_a_leer.read_to_string(&mut datos_como_string)?;

    let datos_en_vector_de_strings: Vec<String> = datos_como_string
        .split('\n')
        .map(|s| s.to_string())
        .collect();

    Ok(datos_en_vector_de_strings)
}

fn main() {
    let prompt_args: Vec<String> = env::args().collect();
    println!("{:?}", prompt_args);
    if prompt_args.len() != 3 {
        panic!("Se recibieron demasiados/muy pocos argumentos.\nSe necesita el nombre de los dos archivos a procesar.\nArgumentos recibidos: {:?}", prompt_args);
    };

    let lineas_f1: Vec<String> = match read_file_lines(&prompt_args[1]) {
        Ok(lineas) => lineas,
        Err(err) => {
            println!("Error al obtener lineas del primer archivo pasado por parametro.\nDetalle de error:\n {:?}", err);
            return;
        }
    };

    let lineas_f2: Vec<String> = match read_file_lines(&prompt_args[2]) {
        Ok(lineas) => lineas,
        Err(err) => {
            println!("Error al obtener lineas del segundo archivo pasado por parametro.\nDetalle de error:\n {:?}", err);
            return;
        }
    };

    println!("Lecturas del primer archivo: {:?}", lineas_f1);
    println!("Lecturas del segundo archivo: {:?}", lineas_f2);
}
