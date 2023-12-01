// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<ReadLines>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub type ReadLines = io::Lines<io::BufReader<File>>;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
