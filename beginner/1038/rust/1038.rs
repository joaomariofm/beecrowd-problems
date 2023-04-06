use std::io;

fn snack_price (code: i64, quantity: i64) -> f64 {
    let snacks = [
        (1, "cachorro quente", 4.00),
        (2, "x-salada", 4.50),
        (3, "x-bacon", 5.00),
        (4, "torrada simples", 2.00),
        (5, "refrigerante", 1.50)
    ];

    for snack in snacks.iter() {
        if snack.0 == code {
            return snack.2 * (quantity as f64);
        }
    }

    return -1.0;
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let inputs : Vec<i64> = input.trim().split(" ").map(|x| x.parse().expect("Not a integer")).collect();

    let a : i64 = inputs[0];
    let b : i64 = inputs[1];

    println!("Total: R$ {:.2}", snack_price(a, b));
}

