use std::io;

fn main() {
    let mut a_input = String::new();
    io::stdin().read_line(&mut a_input).expect("Failed to read line");
    let time_spent : f64 = a_input.trim().parse().unwrap();

    let mut b_input = String::new();
    io::stdin().read_line(&mut b_input).expect("Failed to read line");
    let average_speed : f64 = b_input.trim().parse().unwrap();

    let distance = time_spent * average_speed;

    let liters = distance / 12.0;
     
    println!("{:.3}", liters); 
}

