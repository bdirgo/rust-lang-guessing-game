use std::io;

fn main() {
    println!("Guess The number!");
    println!("Please input now:");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line!");

    println!("You guessed: {}", guess);
}
