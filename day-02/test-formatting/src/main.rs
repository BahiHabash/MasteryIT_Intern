fn main() {
    let s1 = String::from("Hello, ");
    let s2 = "world!";

    let combined = s1.clone() + &s2; 
    println!("{combined}"); 
    
    let joined = format!("{}{}", s1, s2);

    println!("{joined}");

    
    println!("{s1}");
    println!("{s2}");
}