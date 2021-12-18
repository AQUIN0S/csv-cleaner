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

## Quirks

* This tool assumes that there are no commas in any of the actual cells of the data. What I mean by this is that in any row of a `.csv` file, each comma (whether within quotes or not) is considered to be a column delimiter. If a row looks to have the right number of columns but the wrong number/placement of quotes, it will helpfully place new quotes just before the next comma. That's, pretty much the purpose of this whole thing.
* The count of columns is derived from the first row (considered the header row), so the above point means that if any header data actually contains commas, that will mess up that file's whole output. There is no way currently to counter that except to remove commas in the header row's data elements itself, but it should be relatively obvious if most of the lines in the file look to have the wrong number of columns.
