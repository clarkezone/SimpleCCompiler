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
        //println!("Writing {}", &i);
        let result = file.write_all(i.as_bytes());
        file.write(String::from("\r\n").as_bytes());
    }
}
