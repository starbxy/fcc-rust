use std::mem::size_of_val;
fn main() {
    let c1: char = 'a';
    assert_eq!(size_of_val(&c1),4); // size of c1 = 4 bytes

    println!("{}", size_of_val(&c1)); // 4 bytes

    let c2: char = '中';
    assert_eq!(size_of_val(&c2),4); 

    println!("{}", size_of_val(&c2));

    println!("Success!");
} 


// char = character!