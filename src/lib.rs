//! # Diff - Ejercicio 2, Guia 2, Taller de programación I - FIUBA.
//! Este programa tiene como propósito calcular la subsecuencia común más larga de lineas entre dos archivos distintos con el algoritmo LCS (longest common subsequence) y usar esa información para observar en qué difieren los mismos (como el diff de linux, indica qué contenidos tiene el uno que no tenga el otro).
//! ## Ejecución
//! Para ejecutarlo requiere recibir los dos archivos a comparar por parámetro:
//! ```text
//! cargo run filepath1 filepath2
//! ```

use std::cmp::max;
use std::fs::File;
use std::io::{Error, Read};

const NEEDED_ARGS_TO_PARSE: usize = 3;

/// Interpreta los argumentos recibidos por consola para obtener las rutas de los dos archivos a comparar.
///
/// Devuelve un Option con:
/// - Some( (ruta_primer_archivo,ruta_segundo_archivo) )
/// - None
///
/// # Ejemplo de uso básico:
///
/// ```
/// let prompt_args: Vec<String> = env::args().collect();
/// let (path_of_file_1, path_of_file_2) = match parse_args(&prompt_args) {
///     Some((path_of_file_1, path_of_file_2)) => { ...; (path_of_file_1, path_of_file_2) },
///     None => {...},
/// };
/// ```
///
pub fn parse_args(args: &[String]) -> Option<(String, String)> {
    if args.len() != NEEDED_ARGS_TO_PARSE {
        println!("Se recibieron demasiados/muy pocos argumentos.\nSe necesita el nombre de los dos archivos a procesar.\nArgumentos recibidos: {:?}", args);
        return None;
    };

    let path_of_file_1 = args[1].clone();
    let path_of_file_2 = args[2].clone();

    Some((path_of_file_1, path_of_file_2))
}

/// Dada la ruta de un archivo, lee todas sus lineas.
///
/// Devuelve un Result donde:
/// - El Ok value es un vector con todas las lineas del archivo separadas en Strings.
/// - El Err value es un Error causado en la apertura o en la lectura del archivo.
///
/// # Ejemplos de uso básico:
///
/// ```
/// let lineas_separadas = read_file_lines("foo/testfile.txt");
/// ```
///
pub fn read_file_lines(file_path: &str) -> Result<Vec<String>, Error> {
    let mut file_to_read = File::open(file_path)?;

    let mut file_data_as_string = String::new();
    file_to_read.read_to_string(&mut file_data_as_string)?;

    let separated_lines: Vec<String> = file_data_as_string
        .split('\n')
        .map(|each_ref_str| each_ref_str.to_string())
        .collect();

    Ok(separated_lines)
}

/// Dadas dos rutas de archivos a comparar, devuelve un Result donde:
///
/// - El Ok value es una tupla con dos vectores (secuencias). Cada vector (secuencia) contiene todas las lineas de cada archivo separadas en Strings.
/// - El Err value es un String que concatena una descripción y el error de tipo Error ocasionado en read_file_lines.
///
/// # Ejemplo de uso básico:
///
/// ```
/// let (seq_1, seq_2) = match obtain_line_sequences_to_compare(path_of_file_1, path_of_file_2) {
///     Ok( (seq_f1, seq_f2) ) => (seq_f1, seq_f2),
///     Err(string_with_error_info) => {
///         println!("{}", string_with_error_info);
///         ...
///     }
/// };
/// ```
///
pub fn obtain_line_sequences_to_compare(
    path_of_file_1: String,
    path_of_file_2: String,
) -> Result<(Vec<String>, Vec<String>), String> {
    let lines_of_file_1: Vec<String> = match read_file_lines(&path_of_file_1) {
        Ok(lines) => lines,
        Err(err) => return Err(format!("Error al obtener lineas del primer archivo pasado por parametro.\nDetalle de error:\n {:?}", err)),
    };

    let lines_of_file_2: Vec<String> = match read_file_lines(&path_of_file_2) {
        Ok(lines) => lines,
        Err(err) => return Err(format!("Error al obtener lineas del segundo archivo pasado por parametro.\nDetalle de error:\n {:?}", err)),
    };

    Ok((lines_of_file_1, lines_of_file_2))
}

/// Dadas dos secuencias de lineas previamente obtenidas de los dos archivos a comparar, se obtiene una grilla con el resultado
/// del algoritmo LCS entre las dos secuencias, y posteriormente se usa ese resultado para imprimir las diferencias entre los dos archivos.
///
/// # Ejemplo de uso básico:
///
/// ```
/// run_diff(lines_of_file_1, lines_of_file_2);
/// ```
///
pub fn run_diff(lines_of_file_1: Vec<String>, lines_of_file_2: Vec<String>) {
    let grid_with_lcs_results = lcs_for_lines(&lines_of_file_1, &lines_of_file_2);

    print_diff(
        &grid_with_lcs_results,
        &lines_of_file_1,
        &lines_of_file_2,
        lines_of_file_1.len(),
        lines_of_file_2.len(),
    );
}

/// Crea y devuelve una grilla para los resultados del algoritmo LCS.
/// Esta implementación aplica el algoritmo longest common subsequence, adaptado a comparar lineas (Strings) entre sí en vez de caracteres.
///
/// # Ejemplo de uso básico:
///
/// ```
/// let grid_with_lcs_results = lcs_for_lines(&lines_of_file_1, &lines_of_file_2);
/// ```
///
fn lcs_for_lines(lines_of_file_1: &[String], lines_of_file_2: &[String]) -> Vec<Vec<u32>> {
    let quantity_of_rows = lines_of_file_1.len() + 1;
    let quantity_of_collumns = lines_of_file_2.len() + 1;

    let mut grid: Vec<Vec<u32>> = vec![vec![0; quantity_of_collumns]; quantity_of_rows];

    for i in 0..quantity_of_rows - 1 {
        for (j, _) in lines_of_file_2
            .iter()
            .enumerate()
            .take(quantity_of_collumns - 1)
        {
            //Forma de iterar sugerida por clippy, creo que el warning es falso positivo ya que piensa que se usa j para indexar SOLO lines_of_file_2, cuando tambien se usa para grid. Originalmente era:  for j in 0..quantity_of_collumns-1
            if lines_of_file_1[i] == lines_of_file_2[j] {
                grid[i + 1][j + 1] = grid[i][j] + 1;
            } else {
                grid[i + 1][j + 1] = max(grid[i + 1][j], grid[i][j + 1])
            }
        }
    }

    grid
}

/// Crea y devuelve una grilla para los resultados del algoritmo LCS.
/// Esta implementación aplica el algoritmo longest common subsequence, adaptado a comparar lineas (Strings) entre sí en vez de caracteres.
///
/// # Ejemplo de uso básico:
///
/// - grid es la grilla que debe haber sido previamente completada por el algoritmo de lcs.
/// - lines_... son las secuencias.
/// - i y j son la ubicación dentro de la grilla que se quiere buscar cuando se lee el diff. Se requiere que los valores iniciales sean la
/// longitud de la 1ra y 2nda secuencia respectivamente.
/// ```
/// print_diff(&grid, &lines_of_file_1, &lines_of_file_2, lines_of_file_1.len(), lines_of_file_2.len());
/// ```
fn print_diff(
    grid: &[Vec<u32>],
    lines_of_file_1: &[String],
    lines_of_file_2: &[String],
    i: usize,
    j: usize,
) {
    if (i > 0) && (j > 0) && (lines_of_file_1[i - 1] == lines_of_file_2[j - 1]) {
        print_diff(grid, lines_of_file_1, lines_of_file_2, i - 1, j - 1);
        println!("    {}", lines_of_file_1[i - 1]);
    } else if (j > 0) && ((i == 0) || (grid[i][j - 1] >= grid[i - 1][j])) {
        print_diff(grid, lines_of_file_1, lines_of_file_2, i, j - 1);
        println!(">   {}", lines_of_file_2[j - 1]);
    } else if (i > 0) && ((j == 0) || (grid[i][j - 1] < grid[i - 1][j])) {
        print_diff(grid, lines_of_file_1, lines_of_file_2, i - 1, j);
        println!("<   {}", lines_of_file_1[i - 1]);
    } else {
        println!()
    }
}
