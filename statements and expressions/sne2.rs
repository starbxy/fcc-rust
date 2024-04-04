fn main() {
    let v: i32 = {
        let mut x: i32 = 1;
        x += 2; // 3
        x // now x is assigned to v
    };
 
    assert_eq!(v, 3);
 
    println!("Success!");
 }