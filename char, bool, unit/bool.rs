fn main() {
    let _f: bool = false;

    let t: bool = true;
    if !t { // !t = not true, so won't be printed.
        println!("Success!");
    }

    if t {
        println!("Success!");
    } 
}