fn main() {
    
    {
        let my_string = String::from("Rust is amazing");
        println!("Before replacing : {}",my_string);
        println!("After replace : {}", my_string.replace("amazing", "fantastic"));
    }

}
