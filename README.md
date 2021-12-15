# CSV Cleaner

This is a simple Rust script that attempts to clean up .csv files in a directory.

## Prerequisites:

You will need to have Rust installed on your system. Head over to the [Rust installation website](https://www.rust-lang.org/tools/install) for this.

You'll also want Git to be set up on your machine so that you can pull this repository.

## Usage

```bash
git clone git@github.com:AQUIN0S/csv-cleaner.git
cd csv-cleaner
cargo run -- <path_to_csv_files>
```

> Note that `<path_to_csv_files>` should be a directory that contains only csv files,
> and nothing else. The csv files will be cleaned as best as this tool can do and sent
> to an `out/` directory inside the given directory.

## Known Issues

This tool will count the number of headers there are in a `.csv` file, and will not deal with lines where there are more commas in a line than suggested by the number of headers, quoted or not. It should instead just show an error telling which line had the problem, and move on without passing the data in that line to the output file.
