use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_input(p: &str) -> String {
    let path = Path::new(p);
    let display = path.display();

    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => s,
    }
}
