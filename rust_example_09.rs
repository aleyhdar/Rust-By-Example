fn main() {
    let numbers = [11,22,33,44,55,66];
    let _numbers: [u8; 4] = [77,88,99,00];
    let zero_vector = [0; 10];

    for i in numbers.iter() {
        println!("{} ",i);
    }
    for i in 0.._numbers.len() {
        print!("{} ",_numbers[i]);
    }
    for i in 1..10 {
        println!("{}",zero_vector[i]);
    }
}
