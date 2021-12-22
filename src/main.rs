use std::path::Path;
use std::path::PathBuf;
use std::{env, fs};
use structopt::StructOpt;

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
            clean_file(&options, &entry.path());
        }
    }
}

fn clean_file<'a>(options: &'a CliOpts, file_path: &'a Path) {
    println!();
    println!("File to read: {}", file_path.as_os_str().to_str().unwrap());

    let mut output_file_path = PathBuf::from(options.input_directory.as_path());
    output_file_path.push("out");
    output_file_path.push(file_path.file_name().unwrap());

    if let Ok(contents) = fs::read_to_string(file_path) {
        let contents = contents.replace("\"", "'");
        fs::write(output_file_path, contents).unwrap();
    } else {
        println!("Failed to read {}", file_path.display());
    }
}

fn main() {
    let opts = CliOpts::from_args();

    run_through_directory(opts);
}
