use std::env;
use std::fs::File;

pub fn read_file(file_path: &str) -> File {
    let current_dir = env::current_dir().unwrap();
    let path = current_dir.join(file_path);
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    file
}
