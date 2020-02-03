//Formatted print macros

fn main(){
    println!("{} days",31);
    println!("{0}, this is {1}. {1},this is {0}","Alice","Bob");
    println!("{subject} {verb} {object}",
            object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    //:b --> tranform to binary format
    println!("{} of {:b} people know binary the other half doesn't",1,2);
    
    //Appending a white space
    println!("{number:>width$}",number=1 ,width=6);

    //Padding number with extra zero 
    println!("{number:>0width$}",number=2 ,width=7);
     
    println!("My name is {0}, {1} {0}","Bond","James");
    
    //Activities
    //let pi:f32 = 3.141592;
    //println!("Pi is roughly {pi:.3}",pi);

}









