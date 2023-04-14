use std::io;

fn check_taxes_value(salary : f64) -> f64 {
    let mut salary_under_3000 = 0.0;
    let mut salary_under_4500 = 0.0;
    let mut salary_over_4500 = 0.0;
    
    if salary > 2000.00 && salary <= 3000.00 {
        salary_under_3000 = salary - 2000.00;
    }

    if salary > 3000.00 && salary <= 4500.00 {
        salary_under_3000 = 1000.00;
        salary_under_4500 = salary - 3000.00;
    }

    if salary > 4500.00 {
        salary_under_3000 = 1000.00;
        salary_under_4500 = 1500.00;
        salary_over_4500 = salary - 4500.00;
    }

    return (salary_under_3000 * 0.08) + (salary_under_4500 * 0.18) + (salary_over_4500 * 0.28); 
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let salary : f64 = input.trim().parse().unwrap();

    let tax_value = check_taxes_value(salary);

    if tax_value == 0.0 {
        return println!("Isento");
    }

    println!("R$ {:.2}", tax_value);
}

