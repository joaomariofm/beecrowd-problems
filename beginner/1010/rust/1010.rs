use std::io;

fn main() {
    let mut total_value = 0.0;

    for _number in 1..3 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        let inputs : Vec<f64> = input.trim().split(" ").map(|x| x.parse().expect("Not an integer")).collect();

        total_value += inputs[1] * inputs[2];
    }

    println!("VALOR A PAGAR: R$ {:.2}", total_value);
}

