enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}
const MAX_NUMBER : u8 = 20;

fn main () {

    //Tuples has multiple variable type 
    let tuples = (10,20,"Rust",3.14,true,(1,2,3));
    //put the separete variables
    let (a, b, c, d, e, f) = tuples;

    //Accessing the elements of tuples we are use .
    println!("{}",tuples.3);
    println!("{}",(tuples.5).2);


    for n in 1..MAX_NUMBER{
        println!("{}",n);
    }

    let player_direction:Direction = Direction::UP;

    match player_direction {
        Direction::UP => println!("We are heading up!"),
        Direction::DOWN => println!("We are going all the way down.."),
        Direction::LEFT => println!("Left is right!"),   
        Direction::RIGHT => println!("Moving towards the right!"),
    
    }

}
