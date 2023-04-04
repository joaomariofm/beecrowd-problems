use std::io;

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read line");
    let number : i64 = input_line.trim().parse().unwrap();

    println!("{} minutos", number * 2);
}

