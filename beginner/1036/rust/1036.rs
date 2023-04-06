use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let inputs : Vec<f64> = input.trim().split(" ").map(|x| x.parse().expect("Not a float")).collect();

    let a : f64 = inputs[0];
    let b : f64 = inputs[1];
    let c : f64 = inputs[2];

    let delta = (b * b) - 4.0 * a * c;

    if a == 0.0 || delta <= 0.0 {
        return println!("Impossivel calcular");
    }

    let r1 = (-b + delta.sqrt()) / (2.0 * a);
    let r2 = (-b - delta.sqrt()) / (2.0 * a);

    println!("R1 = {:.5}", r1);
    println!("R2 = {:.5}", r2);
}

