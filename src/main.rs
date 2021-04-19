use std::cmp::Ordering;
use std::io;

use rand::Rng;

mod test;

fn main() {

    //create the secret num
    let secret = rand::thread_rng().gen_range(1, 101);

    //make a loop
    loop {
        println!("enter your guess number");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("this is not a number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("your guess number is {}", guess);
        match guess.cmp(&secret) {
            Ordering::Greater => println!("To Big try again"),
            Ordering::Less => println!("To Small try again"),
            Ordering::Equal => {
                println!(" You WIN ");
                break;
            }
        }
    }
}
