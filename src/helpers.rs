use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::result::Result;
use std::fs::File;

pub fn read(filepath: &str) -> Result<Vec<String>, Error> {
    let file = match File::open(filepath) {
        Ok(f) => f,
        Err(_) => return Err(Error::new(ErrorKind::NotFound,format!("file {} does not exist!", filepath)))
    };
    let br = BufReader::new(file);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}


// pub fn read_and_parse<T>(filepath: &str, parseFn: &dyn Fn(String) -> T) -> Result<Vec<T>, Error> {
//     let file = match File::open(filepath) {
//         Ok(f) => f,
//         Err(_) => return Err(Error::new(ErrorKind::NotFound,format!("file {} does not exist!", filepath)))
//     };
//     let br = BufReader::new(file);
//     br.lines()
//         .map(|line| line.and_then(|v| parseFn::<T>(v).map_err(|e| Error::new(ErrorKind::InvalidData, e))))
//         .map(|parsedLine: T| parsedLine.unwrap().1)
//         .collect()
// }