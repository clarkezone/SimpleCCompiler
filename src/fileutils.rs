use std::io::{BufRead, BufReader};
use std::path::Path;
use std::fs::File;
use std::io::Write;

pub fn lines_from_file<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn write_lines<P>(filename: P, lines: &Vec<String>)
where
    P: AsRef<Path>,
{
    let mut file = File::create(filename).expect("error creating file");
    for i in lines.iter().rev() {
        let result = file.write_all(i.as_bytes());
        match result.err() {
            Some(_l) => panic!("error writing file"),
            _ => {}
        }

        let res2 = file.write(String::from("\r\n").as_bytes());
        match res2.err() {
            Some(_l) => panic!("error writing to file"),
            _ => {}
        }
    }
}
