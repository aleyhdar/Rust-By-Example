fn main() {
    let mut _x = 10;
    
    {
    let _x = 15;
    let _y = 20;
    println!("x is {} y is {}",_x,_y);
    }

    //Cant acces the y is isolated in curly brackets
    //println!("x is {} y is {}",x,y);

    let _x = "Is now a String";
    println!("x {}",_x);

    let _x = true;
    println!("x is now boolean '{}' ",_x);
}
