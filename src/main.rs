use std::fs;

fn scan_and_delete(path: &str, pattern: &str) {
    let dir = match fs::read_dir(&path) {
        Ok(dir) => dir,
        Err(err) => panic!("{}", err),
    };

    for entry in dir {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => panic!("{}", err),
        };

        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        if path.is_dir() {
            if file_name == pattern {
                fs::remove_dir_all(&path).unwrap();
            } else {
                scan_and_delete(&path.to_str().unwrap(), pattern);
            }
        } else if file_name == pattern {
            match fs::remove_file(&path) {
                Ok(_) => println!("{}", file_name),
                Err(err) => panic!("{}", err),
            }
        }
        if file_name.contains(pattern) {
            if path.is_dir() {
                fs::remove_dir_all(&path).unwrap();
            }
        }
    }
}

fn main() {
    let path = "./";
    let pattern = "test";

    scan_and_delete(path, pattern);
}
