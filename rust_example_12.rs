use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("rust_example_12.txt")
        .expect("Can't open file!!");
    
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Can't read file!!");

    println!("File contents: \n\n{}", contents);
}
