use std::fs::File;
use std::io::{self, BufRead, BufReader, LineWriter, Write};
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
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
    // #[structopt(parse(from_os_str))]
    // input_directory: PathBuf,

    /// Expect linux style line endings in the input ("\n"). If not set, defaults to Windows style ("\r\n").
    #[structopt(short, long)]
    read_linux: bool,

    /// Write with linux style line endings to the output ("\n"). If not set, defaults to Windows style ("\r\n").
    #[structopt(short, long)]
    write_linux: bool,

    /// The character to use as a separator (default ',')
    #[structopt(short, long, default_value = ",")]
    separator: char,

    /// The size of the data chunks being read at once. Larger chunks mean less reads, resulting in faster results, but requires more memory. Defaults to 1MB (1e6 Bytes)
    #[structopt(short, long, default_value = "1_000_000")]
    chunk_size: u32,
}

use CleaningResult::{Clean, Dirty};

fn run_through_directory(options: CliOpts) {
    let input_directory = PathBuf::from_str("C:\\Downloads\\").expect("Couldn't parse path");

    println!(
        "Directory: {}",
        input_directory.as_os_str().to_str().unwrap()
    );
    println!();

    let mut output_directory = PathBuf::from(&input_directory);
    output_directory.push("out");

    println!("input: {}", input_directory.display());
    println!("output: {}", output_directory.display());

    let mut full_dir_path = env::current_dir().unwrap();
    full_dir_path.push(&input_directory);

    // Replace the <input_dir>/out/ directory, making for a clean output
    let _ = fs::remove_dir_all(&output_directory); // Cleanish way of getting rid of an unneeded warning on this line
    fs::create_dir(&output_directory).unwrap();

    let mut problems: Vec<String> = Vec::new();

    for entry in match fs::read_dir(input_directory) {
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

/// Use the provided reader to parse a single csv line. An assumption made is that every call to this
/// is made at the very start of a new line.
fn read_csv_line<'a>(options: &'a CliOpts, reader: BufReader<File>, column_count: Option<u32>) {}

fn clean_file<'a>(options: &'a CliOpts, file_path: &'a Path) -> Vec<String> {
    let input_directory = PathBuf::from_str("C:\\Downloads\\").expect("Couldn't parse path");

    println!();
    println!("File to read: {}", file_path.as_os_str().to_str().unwrap());
    let mut read_first_line = false;
    let mut num_columns: usize = 0;
    let mut line_number = 0;

    let mut output_file_path = PathBuf::from(input_directory.as_path());
    output_file_path.push("out");
    output_file_path.push(file_path.file_name().unwrap());

    let mut problems: Vec<String> = Vec::new();

    let file = match match File::open(file_path) {
        Ok(file) => Some(file),
        Err(e) => {
            let error = format!(
                "Couldn't open file {} due to {}",
                file_path.as_os_str().to_str().unwrap(),
                e,
            );
            eprintln!("{}", error);
            problems.push(error);
            None
        }
    } {
        Some(file) => file,
        None => return problems,
    };

    let reader = BufReader::new(file);

    let mut header_count = 0;

    problems
}

fn main() {
    let opts = CliOpts::from_args();

    run_through_directory(opts);
}
