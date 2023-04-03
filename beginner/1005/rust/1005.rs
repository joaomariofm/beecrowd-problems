use std::io;

fn main() {
    let mut a_input = String::new();
    io::stdin().read_line(&mut a_input).expect("Failed to read line");
    let a : f64 = a_input.trim().parse().unwrap();

    let mut b_input = String::new();
    io::stdin().read_line(&mut b_input).expect("Failed to read line");
    let b : f64 = b_input.trim().parse().unwrap();
     
    let x = ((a * 3.5) + (b * 7.5))/11.0;
    
    println!("MEDIA = {:.5}", x); 
}

