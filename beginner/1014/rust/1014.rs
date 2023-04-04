use std::io;

fn main() {
    let mut a_input = String::new();
    io::stdin().read_line(&mut a_input).expect("Failed to read line");
    let a : f64 = a_input.trim().parse().unwrap();

    let mut b_input = String::new();
    io::stdin().read_line(&mut b_input).expect("Failed to read line");
    let b : f64 = b_input.trim().parse().unwrap();
     
    let average_consumption = a / b;

    println!("{:.3} km/l", average_consumption); 
}

