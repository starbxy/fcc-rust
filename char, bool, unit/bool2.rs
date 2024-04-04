fn main() {
    let f: bool = true;
    let t: bool = true && false; // true AND false = false
    assert_eq!(t, f); // f holds true, t holds false, so won't be success.

    println!("Success!");

    // re-writing

    let f: bool = false;
    let t: bool = true && false;
    assert_eq!(t, f);

    println!("Success!");

}