fn main() {
//    let c1: char = "中"; // double quote = string, single quote = variable
    let c1: char = '中';
    print_char(c1);
} 

fn print_char(c : char) { // takes argument c of type char
    println!("{}", c);
}