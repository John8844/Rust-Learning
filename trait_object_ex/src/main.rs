trait Bio {
    fn bio(&self) -> String;
    fn get_name(&mut self) -> &mut String;
}

#[derive(Debug)]
struct Developer {
    name: String,
    age: u8
}
impl Bio for Developer {
    fn bio(&self) -> String {
        let bio_str = format!("My name is {}, my age is {}. I'm a software developer", self.name, self.age);
        bio_str
    }
    fn get_name(&mut self) -> &mut String {
        &mut self.name
    }
}

#[derive(Debug)]
struct Designer {
    name: String,
    age: u8
}
impl Bio for Designer {
    fn bio(&self) -> String {
        let bio_str = format!("My name is {}, my age is {}. I'm a graphic designer", self.name, self.age);
        bio_str
    }
    fn get_name(&mut self) -> &mut String {
        &mut self.name
    }
}

fn bio_out<T: Bio>(user: &mut T ) {
    user.get_name().push_str(".J")
}

fn main() {
    println!("Hello, world!");
    let mut dev = Developer {
        name: "Johnson".to_string(),
        age: 22
    };
    let res = dev.bio();
    println!("My Bio: {}", res);

    bio_out(&mut dev);
    println!("Out Res: {:?}", dev);
}
