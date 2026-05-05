fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = format!("{} {}", s1, s2);
    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);
}
