use std::cmp::max;
use std::fs::File;
use std::io::Error;
use std::io::Read;

const NEEDED_ARGS_TO_PARSE: usize = 3;

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

pub fn parse_args(args: &[String]) -> Option<(String, String)> {
    if args.len() != NEEDED_ARGS_TO_PARSE {
        println!("Se recibieron demasiados/muy pocos argumentos.\nSe necesita el nombre de los dos archivos a procesar.\nArgumentos recibidos: {:?}", args);
        return None;
    };

    let path_of_file_1 = args[1].clone();
    let path_of_file_2 = args[2].clone();

    Some((path_of_file_1, path_of_file_2))
}

pub fn run_diff(path_of_file_1: String, path_of_file_2: String) -> Result<(), String> {
    let lines_of_file_1: Vec<String> = match read_file_lines(&path_of_file_1) {
        Ok(lines) => lines,
        Err(err) => return Err(format!("Error al obtener lineas del primer archivo pasado por parametro.\nDetalle de error:\n {:?}", err)),
    };

    let lines_of_file_2: Vec<String> = match read_file_lines(&path_of_file_2) {
        Ok(lines) => lines,
        Err(err) => return Err(format!("Error al obtener lineas del segundo archivo pasado por parametro.\nDetalle de error:\n {:?}", err)),
    };

    let grid_with_lcs_results = lcs_for_lines(&lines_of_file_1, &lines_of_file_2);

    print_diff(
        &grid_with_lcs_results,
        &lines_of_file_1,
        &lines_of_file_2,
        lines_of_file_1.len(),
        lines_of_file_2.len(),
    );

    Ok(())
}

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

/// C es la grid computada por lcs()
/// X e Y son las secuencias
/// i y j especifican la ubicacion dentro de C que se quiere buscar cuando
///    se lee el diff. Al llamar a estar funcion inicialmente, pasarle
///    i=len(X) y j=len(Y)
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
