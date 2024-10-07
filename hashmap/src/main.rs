use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let mut languages: HashMap<&str, &str> = HashMap::new();
    languages.insert("Frontend", "ReactJs");
    languages.insert("Backend", "NodeJs");
    languages.insert("Database", "Mongodb");
    languages.insert("Name", "John");
    // languages.insert("Design", "CSS");
    languages.entry("Design").or_insert("Figma");

    println!("Backend: {}", languages["Backend"]);

    if let Some(frontend) = languages.get("Frontend") {
        println!("Frontend: {}", frontend);
    } else {
        println!("Frontend: Not Found!");
    }

    languages.remove("Name");

    for (key, value) in languages {
        println!("The {} language is {}!", key, value);
    }

}
