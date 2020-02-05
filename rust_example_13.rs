use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::create("rust_example_13.txt")
        .expect("Could not create file!");

    file.write_all(b"Welcome to the RUST side!!")
        .expect("Could not write file!");
    
    let mut contents = String::new();

    file = File::open("rust_example_13.txt")
        .expect("Could not read file!!");

    file.read_to_string(&mut contents)
        .expect("Could not read file!");

    println!("File contents: \n {}",contents);

}
