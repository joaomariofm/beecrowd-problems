use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut sides : Vec<f64> = input.trim().split(" ").map(|x| x.parse().expect("Not a float")).collect();

    let a : f64 = sides[0];
    let b : f64 = sides[1];
    let c : f64 = sides[2];

    sides.sort_by(|a, b| b.partial_cmp(a).unwrap());
    
    // It is not possible to form a triangle
    if sides[0] >= sides[1] + sides[2] {
       return println!("Area = {:.1}", ((a + b) * c) / 2.0); 
    }

    // It is possible to form a triangle
    println!("Perimetro = {:.1}", sides[0] + sides[1] + sides[2]);
}

