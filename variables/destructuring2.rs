fn main() {
    let (x, y); // already used a let statement, so don't need to again for x, y

    (x,..) = (3, 4);
    [.., y] = [1, 2]; // this is an array
    assert_eq!([x,y], [3,2]);
    println!("Success.")
}