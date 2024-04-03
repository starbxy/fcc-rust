fn main() {
    let mut sum: i32 = 0;
    for i in -3..3 { // endpoint is not included in range
        sum += i
    }

    assert!(sum == -3);

    for c in 'a'..='z' { // ascii table: 97 = a, 122 = z then output as u8
        println!("{}",c as u8);
    }
}