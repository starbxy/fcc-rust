fn main() {
    let x: i32 = 5; // i - integer. Uninitialized but used -> error
    let _y: i32; // uninitialized but not used -> just a warning. add an _ to fix

    assert_eq!(x, 5); // x must be equal to 5
    println!("Success.")

}