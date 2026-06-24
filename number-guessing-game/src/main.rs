use std::{
    cmp,
    io::{self, Write, stdin},
};

use rand::{self, random_range};

fn main() {
    let guess_num = random_range(1..=100);

    loop {
        print!("Enter a number(1-100):");
        io::stdout().flush().unwrap();

        let mut input = Default::default();
        stdin().read_line(&mut input).expect("Error");
        let n = input.trim().parse::<i32>().unwrap_or(0);
        match n.cmp(&guess_num) {
            cmp::Ordering::Greater => println!("The number to guess is lower"),
            cmp::Ordering::Less => println!("The number to guess is greater"),
            cmp::Ordering::Equal => {
                println!("You won!");
                break;
            }
        }
    }
}
