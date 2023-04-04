use std::io;

fn main() {
    let mut radius_input = String::new();
    io::stdin().read_line(&mut radius_input).expect("Failed to read line");
    let radius : f64 = radius_input.trim().parse().expect("Not a float");

    let pi = 3.14159;

    let volume : f64 = (4.0/3.0) * pi * (radius * radius * radius);

    println!("VOLUME = {:.3}", volume);
}

