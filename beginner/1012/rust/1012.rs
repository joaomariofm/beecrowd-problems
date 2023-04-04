use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let inputs : Vec<f64> = input.trim().split(" ").map(|x| x.parse().expect("Not a float")).collect();

    let a : f64 = inputs[0];
    let b : f64 = inputs[1];
    let c : f64 = inputs[2];

    // The area of the rectangled triangle that has base a and height c
    let area_rectangled_triangle = (a * c) / 2.0;

    // The area of the radius's circle c. (pi = 3.14159)
    let pi = 3.14159;
    let area_circle = pi * (c * c);

    // The area of the trapezium which has a and b by base, and c by height
    let area_trapezium = (a + b) * c / 2.0;

    // The area of the square that has side b
    let area_square = b * b;

    // The area of the rectangle that has side a and b
    let area_rectangle = a * b;

    println!("TRIANGULO: {:.3}", area_rectangled_triangle);
    println!("CIRCULO: {:.3}", area_circle);
    println!("TRAPEZIO: {:.3}", area_trapezium);
    println!("QUADRADO: {:.3}", area_square);
    println!("RETANGULO: {:.3}", area_rectangle);
}

