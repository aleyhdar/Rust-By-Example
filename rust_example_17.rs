use std::io;

fn main() {
    print!("Echo:");
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            print!("{}",input);
        }
        Err(e) => {
            print!("Something went wrong !{}",e);
        }
    }
}
