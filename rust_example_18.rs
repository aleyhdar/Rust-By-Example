use std::collections::HashMap;
use std::io;
fn main() {
    let mut marks = HashMap::new();

    marks.insert("Intro to Rust Programming","BSM206"); 
    marks.insert("Web Development","BSM207");
    marks.insert("Data Structures","BSM208");
    marks.insert("Reverse Engineering","BSM209");

    //Find length of HashMap

    println!("How many subject have you studied? {}",marks.len());
    
    let mut lecture = String::new();

    io::stdin().read_line(&mut lecture);

    match marks.get(&lecture) {
        Some(marks) => println!("You got {}",lecture),
        None => println!("You didn't study {} lecture",lecture)
    }


}
