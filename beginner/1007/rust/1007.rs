use std::io;

fn main() {
    let mut a_input = String::new();
    io::stdin().read_line(&mut a_input).expect("Failed to read line");
    let a : i64 = a_input.trim().parse().unwrap();

    let mut b_input = String::new();
    io::stdin().read_line(&mut b_input).expect("Failed to read line");
    let b : i64 = b_input.trim().parse().unwrap();

    let mut c_input = String::new();
    io::stdin().read_line(&mut c_input).expect("Failed to read line");
    let c : i64 = c_input.trim().parse().unwrap();

    let mut d_input = String::new();
    io::stdin().read_line(&mut d_input).expect("Failed to read line");
    let d : i64 = d_input.trim().parse().unwrap();
     
    let x = a * b - c * d;
    
    println!("DIFERENCA = {:.1}", x); 
}

