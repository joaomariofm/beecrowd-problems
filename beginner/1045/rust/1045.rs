use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut numbers : Vec<f64> = input.trim().split(" ").map(|x| x.parse().expect("Not a float")).collect();
    
    numbers.sort_by(|a, b| b.partial_cmp(a).unwrap());
    
    let a : f64 = numbers[0];
    let b : f64 = numbers[1];
    let c : f64 = numbers[2];

    if a >= (b + c) {
        return println!("NAO FORMA TRIANGULO");
    }

    if (a * a) == ((b * b) + (c * c)) {
        println!("TRIANGULO RETANGULO");
    }

    if (a * a) > ((b * b) + (c * c)) {
        println!("TRIANGULO OBTUSANGULO");
    }

    if (a * a) < ((b * b) + (c * c)) {
        println!("TRIANGULO ACUTANGULO");
    }

    if a == b && b == c{
        return println!("TRIANGULO EQUILATERO");
    }

    if a == b || b == c || a == c {
        return println!("TRIANGULO ISOSCELES");
    }
}

