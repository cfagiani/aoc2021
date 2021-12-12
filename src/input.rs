use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

/// Reads a file from the directory passed in and returns the contents as a Lines Iterator.
pub fn lines_from_file(
    directory: &str,
    filename: &str,
) -> io::Result<io::Lines<io::BufReader<File>>> {
    let mut infile = PathBuf::new();
    infile.push(directory);
    infile.push(filename);
    let file = File::open(infile)?;
    Ok(io::BufReader::new(file).lines())
}

/// gets all data from a file by applying the parser function to each line
/// returns a Vector of whatever type the parser function returns.
pub fn get_data_from_file<T, F>(input_root: &str, filename: &str, parser: F) -> Vec<T>
where
    F: Fn(String) -> T,
{
    let lines = lines_from_file(input_root, filename);
    lines.unwrap().map(|res| parser(res.unwrap())).collect()
}

/// parses a line of input into a vector of integers interpreting each digit separately.
pub fn parse_as_digits(line: String) -> Vec<u32> {
    line.chars().map(|c| c.to_digit(10).unwrap()).collect()
}
