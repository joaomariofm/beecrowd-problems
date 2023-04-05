use std::io;

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read line");
    let mut age_in_days : i64 = input_line.trim().parse().unwrap();

    let years = age_in_days / 365;
    age_in_days = age_in_days % 365;

    let months = age_in_days / 30;

    let days = age_in_days % 30;

    println!("{} ano(s)", years);
    println!("{} mes(es)", months);
    println!("{} dia(s)", days);
}

