fn main() {
    let s = give_ownership();
    println!("{}", s);
}

fn give_ownership() -> String {
    let s = String::from("Hello world");
    // Convert String to Vec
    // let _s = s.into_bytes(); // using into_bytes() means that s is now consumed !
    let _s = s.as_bytes();
    s
}