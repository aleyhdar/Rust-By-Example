fn main() {
    let hash_string = String::from("be53aac026d11be8bcb25d3c374c4f745e7161d63a9ec81e30f804ded2e1a0a0");
    println!("\nHash: {}",hash_string);
    println!("Hash size: {}bit", hash_string.len()*4);
    
    let mut test_string = String::from("The quick brown fox jumps over the lazy dog.");
    for token in test_string.split_whitespace() {
        println!("{}",token);
    }

    test_string.push_str("cow");
    println!("My strings contains 'cow'? {} ",test_string.contains("cow"));
}
