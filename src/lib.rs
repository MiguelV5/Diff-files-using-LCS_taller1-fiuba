use std::fs::File;
use std::io::Error;
use std::io::Read;

const ARGS_NECESARIOS_PARA_DIFF: usize = 3;

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

pub fn parsear_args(args: &[String]) -> Option<(String, String)> {
    if args.len() != ARGS_NECESARIOS_PARA_DIFF {
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
        Err(err) => return Err(format!("Error al obtener lineas del primer archivo pasado por parametro.\nDetalle de error:\n {:?}", err)),
    };

    let lineas_f2: Vec<String> = match read_file_lines(&path_archivo_2) {
        Ok(lineas) => lineas,
        Err(err) => return Err(format!("Error al obtener lineas del segundo archivo pasado por parametro.\nDetalle de error:\n {:?}", err)),
    };

    println!("Lecturas del primer archivo: {:?}", &lineas_f1);
    println!("Lecturas del segundo archivo: {:?}", &lineas_f2);

    let grilla_con_resultado_lcs = lcs_para_lineas(&lineas_f1, &lineas_f2);

    print_diff(
        &grilla_con_resultado_lcs,
        &lineas_f1,
        &lineas_f2,
        lineas_f1.len(),
        lineas_f2.len(),
    );

    Ok(())
}

fn lcs_para_lineas(lineas_f1: &[String], lineas_f2: &[String]) -> Vec<Vec<u32>> {
    let cantidad_filas_para_grilla = lineas_f1.len() + 1;
    let cantidad_columnas_para_grilla = lineas_f2.len() + 1;

    let mut grilla: Vec<Vec<u32>> =
        vec![vec![0; cantidad_columnas_para_grilla]; cantidad_filas_para_grilla];

    for i in 0..cantidad_filas_para_grilla - 1 {
        for (j, _) in lineas_f2
            .iter()
            .enumerate()
            .take(cantidad_columnas_para_grilla - 1)
        {
            //Forma de iterar añadida por clippy, creo que es falso positivo ya que piensa que se usa j para indexar SOLO lineas_f2, cuando tambien se usa para grid.
            if lineas_f1[i] == lineas_f2[j] {
                grilla[i + 1][j + 1] = grilla[i][j] + 1;
            } else {
                grilla[i + 1][j + 1] = numero_max_entre(grilla[i + 1][j], grilla[i][j + 1])
            }
        }
    }

    grilla
}

fn numero_max_entre(num1: u32, num2: u32) -> u32 {
    if num1 > num2 {
        num1
    } else {
        num2
    }
}

/// C es la grilla computada por lcs()
/// X e Y son las secuencias
/// i y j especifican la ubicacion dentro de C que se quiere buscar cuando
///    se lee el diff. Al llamar a estar funcion inicialmente, pasarle
///    i=len(X) y j=len(Y)
fn print_diff(grilla: &[Vec<u32>], lineas_f1: &[String], lineas_f2: &[String], i: usize, j: usize) {
    if (i > 0) && (j > 0) && (lineas_f1[i - 1] == lineas_f2[j - 1]) {
        print_diff(grilla, lineas_f1, lineas_f2, i - 1, j - 1);
        println!("    {}", lineas_f1[i - 1]);
    } else if (j > 0) && ((i == 0) || (grilla[i][j - 1] >= grilla[i - 1][j])) {
        print_diff(grilla, lineas_f1, lineas_f2, i, j - 1);
        println!(">   {}", lineas_f2[j - 1]);
    } else if (i > 0) && ((j == 0) || (grilla[i][j - 1] < grilla[i - 1][j])) {
        print_diff(grilla, lineas_f1, lineas_f2, i - 1, j);
        println!("<   {}", lineas_f1[i - 1]);
    } else {
        println!()
    }
}
