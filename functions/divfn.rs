fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp { // match means switch lock
        1 => { // if tp is 1, this code block is executed
            // TODO
        }
        _ => { // else, this code block is executed
            // TODO
        }
    };
    
    never_return_fn()
}

fn never_return_fn() -> ! {
    unimplemented!() // can use this or todo! if function not implemented yet
}