use std::io;

fn main() {
    loop {
        println!("If you want Number, press '1'");
        println!("If you want Alphabet, press '2'");

        println!("Please enter your choice:");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Usage of IF ELSE condition
        if choice == 1 {
            get_numbers();
            println!("");
            println!("Want to EXIT enter 0.");
        } else if choice == 2 {
            get_alphabets();
            println!("");
            println!("Want to EXIT enter 0.");
        } else if choice == 0 {
            break;
        } else {
            println!("Wrong choice! Please choose 1 or 2.");
        }
    }
}

// Usage of WHILE loop
fn get_alphabets() {
    let letters = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    let mut index = 0;
    print!("The alphabets are : ");
    while index < 26 {
        print!("{},", letters[index]);
        index += 1;
    }
}

// Usage of FOR loop
fn get_numbers() {
    print!("The numbers are : ");
    for element in 1..=10 {
        print!("{},",element);
    }
}
