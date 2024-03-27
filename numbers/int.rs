// signed - i8/i16 etc... can be both positive or negative. 
// unsigned - u8/u16 etc... can only be positive

fn main() {
    let x: i32 = 5;
    let mut y: u32 = 5;

    // y = x; not possible as we're assigning a signed to an unsigned variable

    let mut y = 5; // automatically i32 type

    y = x;
    
    println!("Success.")
}