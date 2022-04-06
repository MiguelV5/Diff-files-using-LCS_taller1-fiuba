use std::fs::File;
use std::io::Error;
use std::io::Read;

pub fn read_file_lines(path_de_archivo: &str) -> Result<Vec<String>, Error> {
    let mut archivo_a_leer = File::open(path_de_archivo)?;

    let mut datos_como_string = String::new();
    archivo_a_leer.read_to_string(&mut datos_como_string)?;

    let datos_en_vector_de_strings: Vec<String> = datos_como_string
        .split('\n')
        .map(|s| s.to_string())
        .collect();

    Ok(datos_en_vector_de_strings)
}


pub fn parsear_args(args: &[String]) -> Option<(String, String)>{

    if args.len() != 3 {
        println!("Se recibieron demasiados/muy pocos argumentos.\nSe necesita el nombre de los dos archivos a procesar.\nArgumentos recibidos: {:?}", args);
        return None;
    };

    let path_archivo_1 = args[1].clone();
    let path_archivo_2 = args[2].clone();

    Some((path_archivo_1, path_archivo_2))

}

pub fn ejecutar_diff(path_archivo_1: String, path_archivo_2: String) -> Result<(), String> {

    
    let lineas_f1: Vec<String> = match read_file_lines(&path_archivo_1) {
        Ok(lineas) => lineas,
        Err(err) => {
            return Err(format!("Error al obtener lineas del primer archivo pasado por parametro.\nDetalle de error:\n {:?}", err));
        }
    };

    let lineas_f2: Vec<String> = match read_file_lines(&path_archivo_2) {
        Ok(lineas) => lineas,
        Err(err) => {
            return Err(format!("Error al obtener lineas del segundo archivo pasado por parametro.\nDetalle de error:\n {:?}", err));
        }
    };

    println!("Lecturas del primer archivo: {:?}", lineas_f1);
    println!("Lecturas del segundo archivo: {:?}", lineas_f2);

    Ok(())

}