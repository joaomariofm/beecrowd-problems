use std::io;

fn is_even(number: i64) -> bool {
    if (number % 2) == 0 {
        return true;
    }

    return false
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let inputs : Vec<i64> = input.trim().split(" ").map(|x| x.parse().expect("Not a float")).collect();

    let a : i64 = inputs[0];
    let b : i64 = inputs[1];
    let c : i64 = inputs[2];
    let d : i64 = inputs[3];

    if b > c && d > a && (c + d) > (a + b) && c > 0 && d > 0 && is_even(a) {
        return println!("Valores aceitos");
    }

    println!("Valores nao aceitos");
}

