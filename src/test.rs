use std::fs;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::fs::File;

#[cfg(test)]

#[test]
fn test_succeeding() {
    let paths = fs::read_dir("test\\valid").unwrap();

    for path in paths {
        let thepath = path.unwrap();
        println!("Name: {}", thepath.path().display());
        show_contents(thepath.path());
    }
}

fn show_contents<P>(filename: P) where P: AsRef<Path> {
    let lines = lines_from_file(filename);
    for line in lines {
        println!("{:?}", line);
    }
}

fn lines_from_file<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
