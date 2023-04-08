use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let inputs : Vec<f64> = input.trim().split(" ").map(|x| x.parse().expect("Not a float")).collect();

    let x : f64 = inputs[0];
    let y : f64 = inputs[1];

    if x == 0.0 && y == 0.0 {
        return println!("Origem");
    }

    if x == 0.0 {
        return println!("Eixo Y");
    }
    
    if y == 0.0 {
        return println!("Eixo X");
    }

    if x > 0.0 && y > 0.0 {
        return println!("Q1");
    }

    if x > 0.0 && y < 0.0 {
        return println!("Q4");
    }

    if x < 0.0 && y > 0.0 {
        return println!("Q2");
    }

    if x < 0.0 && y < 0.0 {
        return println!("Q3");
    }
}

