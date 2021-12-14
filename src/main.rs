use std::fs::File;
use std::io::{self, BufRead, LineWriter, Write};
use std::ops::AddAssign;
use std::path::Path;
use std::path::PathBuf;
use std::{env, fs};

enum CleaningResult<T> {
    Clean(T),
    Dirty(T),
}

use CleaningResult::{Clean, Dirty};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn run_through_directory(directory: String) {
    println!("Directory: {}", directory);
    println!();
    let entries = fs::read_dir(&directory)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    fs::remove_dir(String::from(&directory) + "out/").unwrap_or_default();
    fs::create_dir(String::from(&directory) + "out/").unwrap();

    for file in entries {
        clean_file(&directory, &file);
    }
}

fn clean_file(directory: &str, file_path: &PathBuf) {
    println!();
    println!("File to read: {}", file_path.as_os_str().to_str().unwrap());
    let mut read_first_line = false;
    let mut num_columns: usize = 0;
    let mut line_number = 1;

    let file = File::create(
        String::from(directory)
            + "out/"
            + file_path.as_path().file_name().unwrap().to_str().unwrap(),
    )
    .unwrap();

    let mut file = LineWriter::new(file);

    if let Ok(lines) = read_lines(file_path) {
        for result in lines {
            if let Ok(line) = result {
                if read_first_line {
                    if line.matches(',').count() + 1 != num_columns {
                        eprintln!("Houston, we have a problem! Number of commas in line {} of file {} don't match the number of columns!", line_number, file_path.as_os_str().to_str().unwrap());
                        return;
                    }
                } else {
                    num_columns = line.matches(',').count() + 1;
                    println!("Number of columns: {}", num_columns);
                    read_first_line = true;
                }
                let mut cleaned_line = match clean_line(line) {
                    Clean(cleaned_line) => cleaned_line,
                    Dirty(cleaned_line) => {
                        println!();
                        println!("Needed to clean line {}:", line_number);
                        println!("{}", cleaned_line);
                        cleaned_line
                    }
                };

                cleaned_line.add_assign("\n");

                match file.write_all(cleaned_line.as_bytes()) {
                    Err(e) => eprintln!(
                        "Error writing line {} of file {} due to {}",
                        line_number,
                        file_path.as_path().file_name().unwrap().to_str().unwrap(),
                        e.to_string()
                    ),
                    _ => {}
                };
            } else {
                eprintln!(
                    "Error in reading line {} of file {}",
                    line_number,
                    file_path.as_os_str().to_str().unwrap()
                );
                return;
            }
            line_number += 1;
        }
    }

    match file.flush() {
        Err(e) => eprintln!(
            "Error writing line {} of file {} due to {}",
            line_number,
            file_path.as_path().file_name().unwrap().to_str().unwrap(),
            e.to_string()
        ),
        _ => {}
    };
}

fn clean_line(line: String) -> CleaningResult<String> {
    let mut result = String::new();

    let mut open_quote = false;
    let mut was_dirty = false;

    for character in line.chars() {
        if character == '\"' {
            open_quote = !open_quote;
        } else if character == ',' && open_quote {
            result.add_assign("\"");
            open_quote = false;
            was_dirty = true;
        }
        result.add_assign(character.to_string().as_str());
    }

    if was_dirty {
        Dirty(result)
    } else {
        Clean(result)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <directory_path_containing_csv_files>", args[0]);
        return;
    }

    run_through_directory(String::from(args.get(1).unwrap().trim_end_matches('/')) + "/");
}
