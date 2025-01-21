use std::io;
fn main() {

    let mut input = String::new();
    print!("please enter a number ");

    io::stdin().read_line(&mut input)
    .expect("failed to read line");
    let number: i32 = input.trim().parse()
    .expect("please enter a valid number");

    println!("you entered the number {number}");
}
