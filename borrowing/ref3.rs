fn main() {
    let mut s: String = String::from("hello, ");

    let p = &mut s;

    p.push_str("world");

    println!("Success.");


}