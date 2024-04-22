fn main() {
    let s1: String = String::from("hello,");
    let s2: String = String::from("world!");
    let s3: String = s1 + s2.as_str(); // s3 has become owner of data above, so s1 and s2 no longer exist
    assert_eq!(s3, "hello,world!");
    println!("{}", s3);
}