fn main() {
    /* Replacing */
    {
        let my_string = String::from("Rust is amazing");
        println!("Before replacing : {}",my_string);
        println!("After replace : {}", my_string.replace("amazing", "fantastic"));
    }
    /* Split */
    {
        let my_string = String::from("As+you+wish+my+lord!");
        let tokens: Vec<&str> = my_string.split("+").collect();

        println!("{}", my_string);
        for i in tokens.iter() 
        {
            println!("[ {} ]", i);
        }
    }
    /* Trim */
    {
        let my_string = String::from("     My name is Alexander Yekhuba!");

        println!("Before trim : {}", my_string);
        println!("After trim : {}", my_string.trim());
    
    }
    /* Character */
    {
        let my_string = String::from("Yekhuba");

        println!("{}", my_string);

        match my_string.chars().nth(4) {
            Some(c) => println!("Character at index 4: {}", c),
            None => println!("No character at index 4.")
        }
    }


}
