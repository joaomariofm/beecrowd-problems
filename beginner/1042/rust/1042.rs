use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let numbers : Vec<i64> = input.trim().split(" ").map(|x| x.parse().expect("Not an integer")).collect();

    let mut sorted_numbers = numbers.clone();
    sorted_numbers.sort();

    println!("{}", sorted_numbers[0]);
    println!("{}", sorted_numbers[1]);
    println!("{}", sorted_numbers[2]);

    println!("");

    println!("{}", numbers[0]);
    println!("{}", numbers[1]);
    println!("{}", numbers[2]);
}

