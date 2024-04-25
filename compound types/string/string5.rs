fn main() {
    let s: &str = "hello, world";
    greetings(s.to_string()) // input must be String type, so use to_string to convert
}

fn greetings(s: String) {
    println!("{}", s)
}