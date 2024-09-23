enum Coin {
    USD,
    NEAR,
    INR,
    John
}

fn main() {
    println!("Hello, world!");

    let coin = Coin::John;
    match coin {
        Coin::USD => println!("Coin is USD!"),
        Coin::NEAR => println!("Coin is NEAR!"),
        Coin::INR => println!("Coin is INR!"),
        _ => println!("Coin is invalid!"),
    }
}
