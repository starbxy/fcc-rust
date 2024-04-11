fn main() {
    let x = String::from("Hello world");
    let y = x.clone(); // if you just say y=x, x gets overwritten. using .clone allows both to exist simultaneously!
    println!("{}, {}",x, y);
}