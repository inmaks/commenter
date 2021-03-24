use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("work.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    file.read_to_string(&mut s)
                            .expect("unable to read file");
    s.insert_str(0, "/*");
    s.insert_str(s.len(), "*/");
    let mut i = 0;
    while i < s.len() {
        let b: u8 = s.as_bytes()[i];
        if b == 10 {
            s.insert_str(i - 1, "*/");
            s.insert_str(i + 3, "/*");
            i += 2;
        }
        i += 1
    }
    let mut outfile = File::create("withcomm.txt")
                            .expect("unable to create file");
    outfile.write_all(s.as_bytes());
}
