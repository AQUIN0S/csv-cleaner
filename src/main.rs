use std::fs::File;
use std::io::{self, BufRead, LineWriter, Write};
use std::path::Path;
use std::path::PathBuf;
use std::{env, fs};
use structopt::StructOpt;

enum CleaningResult<T> {
    Clean(T),
    Dirty(T),
}

/// Options for calling this program on the command line
#[derive(StructOpt, Debug)]
#[structopt(
    name = "CSV Cleaner",
    about = "A utility program that cleans up csv files"
)]
struct CliOpts {
    /// Input directory to find CSV files in
    #[structopt(parse(from_os_str))]
    input_directory: PathBuf,

    /// Use Linux style line endings ("\n"). If not set, use Windows style ("\r\n").
    #[structopt(short, long)]
    linux: bool,

    /// Quotes are to be unescaped. I can't really think of a situation where this would be set though...
    #[structopt(short, long)]
    unescaped: bool,

    /// Don't close quotes in cells. Usually best not to set this as it will result in less clean data, though it will
    /// also mean the result is closer to the original.
    #[structopt(short, long)]
    unclosed_quotes: bool,

    /// Remove quotes altogether. This may or may not help...
    #[structopt(short, long)]
    remove_quotes: bool,

    /// Tabs are used instead of commas
    #[structopt(short, long)]
    tabs: bool,
}

use CleaningResult::{Clean, Dirty};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn run_through_directory(options: CliOpts) {
    println!(
        "Directory: {}",
        options.input_directory.as_os_str().to_str().unwrap()
    );
    println!();

    let mut output_directory = PathBuf::from(&options.input_directory);
    output_directory.push("out");

    println!("input: {}", options.input_directory.display());
    println!("output: {}", output_directory.display());

    let mut full_dir_path = env::current_dir().unwrap();
    full_dir_path.push(&options.input_directory);

    // Replace the <input_dir>/out/ directory, making for a clean output
    let _ = fs::remove_dir_all(&output_directory); // Cleanish way of getting rid of an unneeded warning on this line
    fs::create_dir(&output_directory).unwrap();

    let mut problems: Vec<String> = Vec::new();

    for entry in match fs::read_dir(&options.input_directory) {
        Ok(read_dir) => read_dir,
        Err(_) => {
            eprintln!(
                "Could not find the input directory specified at {}",
                full_dir_path.display()
            );
            return;
        }
    } {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("Some io error occurred during iteration of the files in the directory {}. See https://doc.rust-lang.org/std/fs/struct.ReadDir.html for details. You might be able to retry this operation", full_dir_path.display());
                eprintln!("Error details are as follows: {}", e);
                return;
            }
        };

        if entry.file_name().to_str().unwrap().ends_with(".csv") {
            problems.append(&mut clean_file(&options, &entry.path()));
        }
    }

    if !problems.is_empty() {
        println!();
        println!("The following problems were found and could not be solved by this tool. Please review:");
        println!();
        for problem in &problems {
            println!("{}", problem);
        }
    }
}

fn clean_file<'a>(options: &'a CliOpts, file_path: &'a Path) -> Vec<String> {
    println!();
    println!("File to read: {}", file_path.as_os_str().to_str().unwrap());
    let mut read_first_line = false;
    let mut num_columns: usize = 0;
    let mut line_number = 0;

    let mut output_file_path = PathBuf::from(options.input_directory.as_path());
    output_file_path.push("out");
    output_file_path.push(file_path.file_name().unwrap());

    let mut problems: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(file_path) {
        let output_file = File::create(output_file_path).unwrap();
        let mut file_writer = LineWriter::new(output_file);

        for result in lines {
            line_number += 1;

            if let Ok(line) = result {
                if read_first_line {
                    if line.matches(if options.tabs { '\t' } else { ',' }).count() + 1
                        != num_columns
                    {
                        let error = format!("Please review line {} of file {}, number of columns in row doesn't match the number of columns in header! This line has not been written into the output file",
                            line_number,
                            file_path.as_os_str().to_str().unwrap());
                        eprintln!("{}", error);
                        problems.push(error);
                        continue;
                    }
                } else {
                    num_columns = line.matches(if options.tabs { '\t' } else { ',' }).count() + 1;
                    println!("Number of columns: {}", num_columns);
                    read_first_line = true;
                }
                let mut cleaned_line = match clean_line(options, line) {
                    Clean(cleaned_line) => cleaned_line,
                    Dirty(cleaned_line) => {
                        println!();
                        println!("Needed to clean line {}:", line_number);
                        println!("{}", cleaned_line);
                        cleaned_line
                    }
                };

                cleaned_line += if options.linux { "\n" } else { "\r\n" };

                match file_writer.write_all(cleaned_line.as_bytes()) {
                    Err(e) => {
                        let error = format!(
                            "Error writing line {} of file {} due to {}",
                            line_number,
                            file_path.file_name().unwrap().to_str().unwrap(),
                            e.to_string()
                        );
                        eprintln!("{}", error);
                        problems.push(error);
                    }
                    _ => {}
                };
            } else {
                let error = format!(
                    "Error in reading line {} of file \"{}\". Line not written into output file, please review",
                    line_number,
                    file_path.as_os_str().to_str().unwrap()
                );
                eprintln!("{}", error);
                problems.push(error);
            }
        }

        match file_writer.flush() {
            Err(e) => {
                eprintln!(
                    "Error writing line {} of file {} due to {}",
                    line_number,
                    file_path.file_name().unwrap().to_str().unwrap(),
                    e.to_string()
                );

                let error = format!(
                    "Error writing line {} of file {} due to {}",
                    line_number,
                    file_path.file_name().unwrap().to_str().unwrap(),
                    e.to_string()
                );
                eprintln!("{}", error);
                problems.push(error);
            }
            _ => {}
        };
    } else {
        let problem = format!("Could not read file {}", file_path.display());
        eprintln!("{}", problem);
        problems.push(problem);
    }

    problems
}

fn clean_line(opts: &CliOpts, line: String) -> CleaningResult<String> {
    let mut result = String::new();

    let mut open_quote = false;
    let mut was_dirty = false;
    let mut prev_char: Option<char> = None;

    for character in line.chars() {
        if character == '\"' {
            if opts.remove_quotes {
                was_dirty = true;
                continue;
            }
            if prev_char == None || prev_char == Some(if opts.tabs { '\t' } else { ',' }) {
                // TODO: Implement a thing
            } else {
                // TODO: Implement another thing
            }
            open_quote = !open_quote;
            result += if opts.unescaped { "" } else { "\\" };
        } else if character == if opts.tabs { '\t' } else { ',' }
            && open_quote
            && !opts.unclosed_quotes
        {
            result += if opts.unescaped { "\"" } else { "\\\"" };
            open_quote = false;
            was_dirty = true;
        } else if character == '\n' || character == '\r' {
            if opts.unescaped || opts.remove_quotes || opts.unclosed_quotes {
                was_dirty = true;
                continue;
            }
        } else if character == '\\' {
            was_dirty = true;
            continue;
        }
        result += character.to_string().as_str();
        prev_char = Some(character);
    }

    if was_dirty {
        Dirty(result)
    } else {
        Clean(result)
    }
}

fn main() {
    let opts = CliOpts::from_args();

    run_through_directory(opts);
}
