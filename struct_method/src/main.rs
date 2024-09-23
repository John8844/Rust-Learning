use std::io;

#[derive(Debug)]
struct Profile {
    name: String,
    age: u32,
    role: String
}

impl Profile {
    fn add_initial(&mut self, initial: char) -> &Self{
        let name = self.name.clone()+"_"+&initial.to_string();
        self.name = name;
        self
    }
}

fn main() {
    println!("Hello, world!");

    let mut user = Profile {
        name: String::from("Johnson"),
        age: 22,
        role: String::from("Software Engineer Intern")
    };

    println!("User {:?}", user);
    println!("Do you want to promote this user?");
    println!("If Yes press 1, or 0 to quit!");

    let mut choice = String::new();

    io::stdin().read_line(&mut choice).expect("Failed read line");

    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    println!("Enter your initial:");

    let mut initial = String::new();

    io::stdin().read_line(&mut initial).expect("Failed read line");

    let initial: char = match initial.trim().parse() {
        Ok(ch) => ch,
        Err(_) => '#',
    };

    if choice == 1 {
        let mut promoted_user: Profile = promote(user);
        promoted_user.add_initial(initial);
        println!("User promoted successfully! {:?}", promoted_user);
    } else {
        user.add_initial(initial);
        println!("User not promoted: {:?}", user);
    }

}

fn promote(user: Profile) -> Profile {
    Profile {
        role: "Software Engineer".to_string(),
        ..user
    }
}
