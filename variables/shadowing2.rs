fn main() {
    let mut x: i32 = 1;
    x = 7; // x = 7
    // Shadowing and re-binding
    // let x = x; // we are re-declaring x, so it's immutable
    x += 3;
    println!("{}", x);

    let y: i32 = 4;
    // shadowing
    let y = "I can also be bound to text!"; 

    println!("Success!");
}