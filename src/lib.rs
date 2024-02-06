use crate::r#match::MoveStatus;
use clap::Parser;

mod fs_utils;
mod input_validation;
mod r#match;

/// Arguments for mmv execution
#[derive(Parser, Debug)]
#[clap(name = "MassMove2000")]
#[clap(author = "Alexey Datskovskiy")]
#[clap(version = "1.1.0")]
#[clap(
    about = "Mass move utility",
    long_about = "Analog to standard mmv, matches files by pattern and moves them. Supports *
Usage example:
./mmv '*/2023_*.txt' '2023_#1/#2.txt' --- will match 'archive/2023_08.txt' and move it to '2023_archive/08.txt'
"
)]
pub struct Cli {
    /// Mask of files to move
    pub old_file_mask: String,
    /// Mask of file destinations
    pub new_file_mask: String,
    /// Is move forced
    #[clap(short, long)]
    pub force: bool,
}

fn main() {
    let cli = Cli::parse();
    run_mmv(cli);
}

pub fn run_mmv(cli: Cli) {
    let mut old_file_mask = cli.old_file_mask;
    let mut new_file_mask = cli.new_file_mask;
    let is_forced = cli.force;
    input_validation::normalize_mask_slashes(&mut old_file_mask);
    input_validation::normalize_mask_slashes(&mut new_file_mask);
    input_validation::validate_patterns(&old_file_mask, &new_file_mask);
    let file_matches = fs_utils::find_matches(&old_file_mask, &new_file_mask);
    if file_matches.is_empty() {
        panic!("No files matching \"{}\" found", old_file_mask);
    }

    if !is_forced {
        for file_match in file_matches.clone() {
            if !fs_utils::check_destination_availability(file_match.new_file_path.as_str()) {
                panic!(
                    "File {} already exists. If you want to overwrite it, add flag -f",
                    file_match.new_file_path.as_str()
                );
            }
        }
    }

    let mut successful_moves = 0;
    for file_match in file_matches.clone() {
        let move_result = file_match.execute_move();
        match move_result {
            Ok(move_status) => {
                println!(
                    "{} -> {}{}",
                    file_match.old_file_path,
                    file_match.new_file_path,
                    match move_status {
                        MoveStatus::MovedOverwritten => " - overwritten",
                        MoveStatus::MovedNotOverwritten => "",
                    }
                );
                successful_moves += 1;
            }
            Err(error) => println!(
                "Moving {} was unsuccessful: {:?}",
                file_match.old_file_path, error
            ),
        }
    }
    println!("Successfully moved {} file(s)", successful_moves);
}
