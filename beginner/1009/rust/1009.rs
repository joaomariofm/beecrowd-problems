use std::io;

fn main() {
    let mut employee = String::new();
    io::stdin().read_line(&mut employee ).expect("Failed to read line");

    let mut salary_input = String::new();
    io::stdin().read_line(&mut salary_input).expect("Failed to read line");
    let salary : f64 = salary_input.trim().parse().unwrap();

    let mut total_sales_input = String::new();
    io::stdin().read_line(&mut total_sales_input).expect("Failed to read line");
    let total_sales : f64 = total_sales_input.trim().parse().unwrap();

    println!("TOTAL = R$ {:.2}", salary + (total_sales * 0.15)); 
}

