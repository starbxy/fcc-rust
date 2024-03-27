fn main() {
    let x: i32 = 10;

    { // inner scope
        let y: i32 = 5;
        println!("The value of x is {} and the value of y is {}", x, y);
    }

    println!("The value of x is {} and the value of y is {}", x, y); // y no longer valid
}

// variable is only valid within scope which it was declared