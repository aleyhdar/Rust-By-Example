fn main() {
    let number = 19;
    let name = "SAKAT";

    match number {
        1 => println!("This is one !"),
        2 => println!("There is two of them!"),
        3...20 => println!("It greater than one!"),
        //1 | 10 => println!("It is either 1 or 11"),
        _ => println!("It doesn't match!")
    }

    match name {
        "SAKAT" => println!("Nice name buddy!"),
        "TAKAS" => println!("Nah bro!"),
        _ => println!("It doesn't match!")
    }

}
