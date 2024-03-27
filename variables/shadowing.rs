fn main() {
    let x: i32 = 5;
    { // within this scope, it's like a new world. x is now 12, so assert_eq must be 12 as well
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5); // x is 5 again (out of scope)

    let x = 42;
    println!("{}", x); // prints "42". new value assigned
}