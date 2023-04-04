use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let inputs : Vec<i64> = input.trim().split(" ").map(|x| x.parse().expect("Not an integer")).collect();

    let a : i64 = inputs[0];
    let b : i64 = inputs[1];
    let c : i64 = inputs[2];

    let maior_ab = (a + b + (a - b).abs()) / 2;
    let maior_abc = (maior_ab + c + (maior_ab - c).abs()) / 2;

    println!("{} eh o maior", maior_abc);
}

