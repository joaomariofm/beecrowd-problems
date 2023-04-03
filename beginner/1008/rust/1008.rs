use std::io;

fn main() {
    let mut number_input = String::new();
    io::stdin().read_line(&mut number_input).expect("Failed to read line");
    let number : i64 = number_input.trim().parse().unwrap();

    let mut hours_input = String::new();
    io::stdin().read_line(&mut hours_input).expect("Failed to read line");
    let hours : f64 = hours_input.trim().parse().unwrap();

    let mut hourly_payment_input = String::new();
    io::stdin().read_line(&mut hourly_payment_input).expect("Failed to read line");
    let hourly_payment : f64 = hourly_payment_input.trim().parse().unwrap();

    let salary = hours * hourly_payment;
    
    println!("NUMBER = {}", number); 
    println!("SALARY = U$ {:.2}", salary); 
}

