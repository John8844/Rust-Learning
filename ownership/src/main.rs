fn main() {
    println!("Hello, world!");
    let mut name = String::from("John");

    let full_name = add_string(name.clone());  // If we pass the name as original, add_string will take a ownership of name. We can't use name after this line. So that, we are passing copy of name.
    println!("First Name: {}",name);
    println!("Full Name: {}",full_name);

    // Slice a string
    let last_name = &full_name[4..=6]; // It will slice the 4th - 6th index values of full_name
    println!("Last Name: {}",last_name);

    // Reference usage
    let joined_name = join_string(&name, last_name); // name passed as reference using keyword &, last_name is already as a reference type.
    println!("Joined Name: {}",joined_name);

}

// Function for join the two string, returning a joined String
fn join_string(first: &str, last: &str) -> String {
    let joined_str = first.to_owned() + last;
    joined_str
}

// Function for add the string in last, returning a added String
fn add_string(mut s: String) -> String {
    s.push_str("son");
    s
}
