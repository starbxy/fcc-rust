fn main() {
    let mut s: String = String::from(""); // init empty string
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success!");
}