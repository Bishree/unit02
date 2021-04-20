use std::cmp::Ordering;
use std::io;

use rand::Rng;

mod test;

fn main() {
    //creat a secret num
    let mut secret = rand::thread_rng().gen_range(1,101);
    //make a loop to hole process
    loop {
        //capture user input
        let mut guess = String::new();
        io::stdin().read_line(& mut guess).expect("this is not a num");
        //match user input to number
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)=> {
                println!("this is not a number");
                continue;
            }
        };
        // match the correct number
        match guess.cmp(& mut secret) {
            Ordering::Less => println!("Too Small Try again"),
            Ordering::Greater => println!("Too Big Try again"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
    }

    }

}
