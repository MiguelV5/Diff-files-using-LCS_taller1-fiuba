use std::env;
use std::process;

mod lib;
use lib::*;

const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    let prompt_args: Vec<String> = env::args().collect();

    let (path_of_file_1, path_of_file_2) = match parse_args(&prompt_args) {
        Some((path_of_file_1, path_of_file_2)) => (path_of_file_1, path_of_file_2),
        None => process::exit(ERROR_EXIT_CODE),
    };

    let (lines_of_file_1, lines_of_file_2) =
        match obtain_line_sequences_to_compare(path_of_file_1, path_of_file_2) {
            Ok((lines_f1, lines_f2)) => (lines_f1, lines_f2),
            Err(string_with_error_info) => {
                println!("{}", string_with_error_info);
                process::exit(ERROR_EXIT_CODE);
            }
        };

    run_diff(lines_of_file_1, lines_of_file_2);
}
