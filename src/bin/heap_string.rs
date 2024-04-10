fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s2);
    copying_instead()

}

fn copying_instead() {
    let s1 = String::from("what's up");
let s2 = s1.clone();
println!("{}, world! I said {}", s1, s2);

}