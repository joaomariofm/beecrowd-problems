use std::io;

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read line");
    let mut total_value : i64 = input_line.trim().parse().unwrap();

    let banknotes = [100, 50, 20, 10, 5, 2, 1];
   
    println!("{}", total_value);

    for banknote in banknotes.iter() {
        let quo = total_value / banknote;

        total_value = total_value - quo * banknote;
        println!("{} nota(s) de R$ {},00", quo, banknote);
    }
}

