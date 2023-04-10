use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut numbers : Vec<i64> = input.trim().split(" ").map(|x| x.parse().expect("Not an integer")).collect();

    numbers.sort();

    if numbers[1] % numbers[0] == 0 {
        return println!("Sao Multiplos");
    }
    
    println!("Nao sao Multiplos");
}

