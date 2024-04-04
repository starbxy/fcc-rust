fn main() {
    let v: i32 = {
        let x: i32 = 3;
        x
    };
 
    assert!(v == 3);
 
    println!("Success!");
 }