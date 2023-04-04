use std::io;

fn main() {
    let mut first_input_line = String::new();
    io::stdin().read_line(&mut first_input_line).expect("Failed to read line");

    let first_input : Vec<f64> = first_input_line.trim().split(" ").map(|x| x.parse().expect("Not an integer")).collect();

    let first_point_x : f64 = first_input[0];
    let first_point_y : f64 = first_input[1];

    let mut second_input_line = String::new();
    io::stdin().read_line(&mut second_input_line).expect("Failed to read line");
    
    let second_input : Vec<f64> = second_input_line.trim().split(" ").map(|x| x.parse().expect("Not an integer")).collect();

    let second_point_x : f64 = second_input[0];
    let second_point_y : f64 = second_input[1];

    let distance = ((second_point_x - first_point_x).powf(2.0) + (second_point_y - first_point_y).powf(2.0)).sqrt();

    println!("{:.4}", distance);
}

