// unit type doesn't hold any value

fn main() {
    let _v: () = (); // unit type, 0 bytes

    let v: (i32, i32) = (2, 3);
    assert_eq!(_v, implicitly_ret_unit());

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}