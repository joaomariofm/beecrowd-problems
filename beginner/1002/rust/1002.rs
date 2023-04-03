use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let r : f64 = input.trim().parse().unwrap();

    let n = 3.14159;

    let a = n * (r * r);

    println!("A={:.4}",a);
}
