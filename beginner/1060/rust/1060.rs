use std::io;

fn main() {
    let mut numbers : [f64; 6] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let mut positive_numbers = 0;

    for _i in 1..7 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let number : f64 = input.trim().parse().unwrap();

        numbers[(_i - 1) as usize] = number;
    }

    for number in numbers.iter() {
        if *number > 0.0 {
            positive_numbers += 1;
        }
    }

    println!("{} valores positivos", positive_numbers);
}

