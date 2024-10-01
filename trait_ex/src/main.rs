trait Bio {
    fn bio(&self) -> String;
}

struct Developer {
    name: String,
    skills: String
}
impl Bio for Developer {
    fn bio(&self) -> String{
        format!("{} works as a software engineer at Teckas Technologies! The skills are, {}",self.name, self.skills)
    }
}

struct Designer {
    name: String,
    skills: String
}
impl Bio for Designer {
    fn bio(&self) -> String{
        format!("{} works as a graphic designer at Teckas Technologies! The skills are, {}",self.name, self.skills)
    }
}

fn main() {
    println!("Hello, world!");
    let numbers = vec![44,88,66,33];
    let large_number = largest(&numbers);
    println!("The largest number is {large_number}");

    let characters = vec!['J', 'I', 'F', 'P'];
    let large_char = largest(&characters);
    println!("The largest number is {large_char}");

    let dev = Developer {
        name: String::from("Johnson"),
        skills: "HTML, CSS, Javascript".to_string()
    };
    let developer_bio: String = dev.bio();
    println!("Result: {}", developer_bio);

    let dev1 = Developer {
        name: String::from("Sathish"),
        skills: "HTML, CSS, Javascript, React Js".to_string()
    };
    let developer_bio1: String = dev1.bio();
    println!("Result: {}", developer_bio1);

    let des = Designer {
        name: String::from("Simon"),
        skills: "Ps, Fr, Figma".to_string()
    };
    let designer_bio: String = des.bio();
    println!("Result: {}", designer_bio);
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
