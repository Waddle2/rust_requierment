use std::io;
fn main() {
    println!("Hello, user!");
    println!("What is your age?");
    let mut age = String::new();

    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read line");

    let age: u32 = age.trim().parse().expect("Please type a number!");

    if age >= 18 {
        println!("You can enter the bar.");
    } else {
        println!("You cannot enter the bar.");
    }
}
