fn main() {
    let mut _x = 10;

    //Unmutable references and multiple references
    let xr = &_x;
    let xr2 = &_x;
    println!("x is {}",xr);

    //Mutable references
    let xrm = &mut _x;
    *xrm += 1 ;
    println!("new x is {}",xrm);

} 
