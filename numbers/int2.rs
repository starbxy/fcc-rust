fn main() {
    let v: u16 = 38_u8 as u16; // as allows us to switch variable type

    println!("Success.");

    let x: i32 = 5;
    assert_eq!("i32".to_string(), type_of(&x));

    println!("Success.");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}