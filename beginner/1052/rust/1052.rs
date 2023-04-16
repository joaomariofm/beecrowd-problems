use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let number : i64 = input.trim().parse().unwrap();

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    
    let index : i64 = number - 1;

    println!("{}", months[index as usize]); 
}

