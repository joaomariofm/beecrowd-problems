use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let number : f64 = input.trim().parse().unwrap();

    if number >= 0.00 && number <= 25.00 {
        return println!("Intervalo [0,25]");
    }

    if number > 25.00 && number <= 50.00 {
        return println!("Intervalo (25,50]");
    }

    if number > 50.00 && number <= 75.00 {
        return println!("Intervalo (50,75]");
    }

    if number > 75.00 && number <= 100.00 {
        return println!("Intervalo (75,100]");
    }

    println!("Fora de intervalo");
}

