use std::io;

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read line");
    let time_in_seconds : i64 = input_line.trim().parse().unwrap();

    let mut minutes = time_in_seconds / 60;
    let seconds = time_in_seconds % 60;
    let mut hours = 0;

    if minutes > 60 {
        hours = minutes / 60;
        minutes = minutes % 60;
    }


    println!("{}:{}:{}", hours, minutes, seconds);
}

