use std::env;

mod lib;
use lib::*;

const EXIT_ERROR_CODE: i32 = 1;

fn main() -> Result<(), i32> {
    let prompt_args: Vec<String> = env::args().collect();

    let (path_of_file_1, path_of_file_2) = match parse_args(&prompt_args) {
        Some((path_of_file_1, path_of_file_2)) => (path_of_file_1, path_of_file_2),
        None => {
            println!("\nSe recibieron demasiados/muy pocos argumentos.\nSe necesita el nombre de los dos archivos a procesar.\nArgumentos recibidos: {:?}\n", prompt_args);
            return Err(EXIT_ERROR_CODE);
        }
    };

    let (lines_of_file_1, lines_of_file_2) =
        match obtain_line_sequences_to_compare(path_of_file_1, path_of_file_2) {
            Ok((lines_f1, lines_f2)) => (lines_f1, lines_f2),
            Err(string_with_error_info) => {
                println!("\n{}\n", string_with_error_info);
                return Err(EXIT_ERROR_CODE);
            }
        };

    run_diff(lines_of_file_1, lines_of_file_2);

    Ok(())
}
