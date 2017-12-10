#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use fileutils::lines_from_file;

    #[test]
    fn test_succeeding() {
        let paths = fs::read_dir("test\\valid").unwrap();

        for path in paths {
            let thepath = path.unwrap();
            println!("Name: {}", thepath.path().display());
            show_contents(thepath.path());
        }
    }

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
