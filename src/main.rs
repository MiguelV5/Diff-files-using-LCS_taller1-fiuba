use std::env;
use std::process;

mod lib;
use lib::*;

const CODIGO_DE_SALIDA_POR_ERROR: i32 = 1;

fn main() {

    let prompt_args: Vec<String> = env::args().collect();

    let (path_archivo_1, path_archivo_2) = match parsear_args(&prompt_args) {
        Some((path_archivo_1, path_archivo_2)) => (path_archivo_1, path_archivo_2),
        None => process::exit(CODIGO_DE_SALIDA_POR_ERROR),
    };

    match ejecutar_diff(path_archivo_1, path_archivo_2) {
        Ok(()) => (),
        Err(string_con_info_de_error) => {
            println!("{}", string_con_info_de_error);
            process::exit(CODIGO_DE_SALIDA_POR_ERROR);
        }
    }

}


