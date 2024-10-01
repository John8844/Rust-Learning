use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Roll the dice!");

    loop {
        println!("Please enter 0 to roll the dice!");

        let mut roll = String::new();

        io::stdin()
            .read_line(&mut roll)
            .expect("Failed to read line");

        let roll: i32 = match roll.trim().parse() {
            Ok(num) => num, 
            Err(_) => continue,
        };

        if roll ==  0 {
            let secret_number = rand::thread_rng().gen_range(1..=6);
            match secret_number {
                1 => {
                    println!("Dice No: {}", secret_number);
                    println!("You Loss!");
                    continue;
                },
                2 => {
                    println!("Dice No: {}", secret_number);
                    println!("You Loss!");
                    continue;
                },
                3 => {
                    println!("Dice No: {}", secret_number);
                    println!("You Loss!");
                    continue;
                },
                4 => {
                    println!("Dice No: {}", secret_number);
                    println!("You Loss!");
                    continue;
                },
                5 => {
                    println!("Dice No: {}", secret_number);
                    println!("You Loss!");
                    continue;
                },
                6 => {
                    println!("Dice No: {}", secret_number);
                    println!("You Won!");
                    break;
                },
                _ => continue
            }
        }
    }
}