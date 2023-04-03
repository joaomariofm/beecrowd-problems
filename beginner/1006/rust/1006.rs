use std::io;

fn main() {
    let mut a_input = String::new();
    io::stdin().read_line(&mut a_input).expect("Failed to read line");
    let a : f64 = a_input.trim().parse().unwrap();

    let mut b_input = String::new();
    io::stdin().read_line(&mut b_input).expect("Failed to read line");
    let b : f64 = b_input.trim().parse().unwrap();

    let mut c_input = String::new();
    io::stdin().read_line(&mut c_input).expect("Failed to read line");
    let c : f64 = c_input.trim().parse().unwrap();
     
    let x = ((a * 2.0) + (b * 3.0) + (c * 5.0))/10.0;
    
    println!("MEDIA = {:.1}", x); 
}

