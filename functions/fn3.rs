// don't let println! work
fn main() {
    never_return();

    println!("Failed!");
}

fn never_return() -> ! { // !: diverging function, doesn't return to user
    panic!() // quits process
}