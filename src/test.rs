#![allow(dead_code)]
#[cfg(test)]
mod tests {
    //use std::fs;
    use std::path::Path;
    use fileutils::lines_from_file;

    fn show_contents<P>(filename: P)
    where
        P: AsRef<Path>,
    {
        let lines = lines_from_file(filename);
        for line in lines {
            println!("{:?}", line);
        }
    }
}
