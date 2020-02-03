fn main (){

    //let variables are immutable
    //Add this keyword and can mutable
    let mut x = 50; /*i32*/
    println!("X's value : {}",x);

    x = 80; 
    println!("X's new value : {}",x);
    
    //If conditions
    if x < 70 {
        println!("X is less than 70");
    } 
    else if x >50 {
        println!("rust!");
    }
    else {
        println!("X is bigger than 70");
    }

    //Type of loops
    let mut n = 0;

    loop {
        n += 1;
        if n == 7 { continue;}
        if n > 10 { break; }
        println!("The value of n is {}",n);
    }

    let mut i = 1;
    
    while i<50 {
        println!("{}",i);
        i += 1;
    }

    let numbers = 30..41;
    for j in numbers{
        println!("The number is {}", j);
    }

    let animals = vec!["Dog", "Cow", "Cat"];
    for a in animals.iter(){
        println!("The animal name is  {}", a);
    }

    for (index, a) in animals.iter().enumerate(){
        println!("The index is {} the animal name is {}", index, a);
    }
}
