fn main() {
    println!("Hello, world!");
    let mut vec = Vec::from([1,2,3]);

    for i in 0..5 {
        match vec.get(i) {
            Some(e) => vec[i] = e + 1,
            None => vec.push(i+2)
        }
    }

    for i in 0..5 {
        println!("value : {}", vec[i]);
    }

}
