use std::io;

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read line");
    let total_value : f64 = input_line.trim().parse().unwrap();

    let mut dolars = total_value.trunc() as i64;
    let mut cents = ((total_value * 100.00) as i64) - (dolars * 100);

    let banknotes = [100, 50, 20, 10, 5, 2];
    let coins = [100, 50, 25, 10, 5, 1];
   
    println!("NOTAS:");

    for banknote in banknotes.iter() {
        let quo = dolars / banknote;

        dolars -= quo * banknote;
        println!("{} nota(s) de R$ {}.00", quo, banknote);
    }

    if dolars != 0 {
        cents += 100;
    }

    println!("MOEDAS:");

    for coin in coins.iter() {
        let quo = cents / coin;

        cents -= quo * coin;

        let coin_as_float = (*coin as f64) / 100.00;

        println!("{} moeda(s) de R$ {:.2}", quo, coin_as_float);
    }
}

