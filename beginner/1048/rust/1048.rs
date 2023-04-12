use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let salary : f64 = input.trim().parse().unwrap();

    let mut readjustment_rate : f64 = 0.0;
    let mut raise : f64 = 0.0;

    if salary <= 400.00 {
        readjustment_rate = 0.15;
        raise = salary * readjustment_rate;
    }

    if salary <= 800.00 && readjustment_rate == 0.0 {
        readjustment_rate = 0.12;
        raise = salary * readjustment_rate;
    }

    if salary <= 1200.00 && readjustment_rate == 0.0{
        readjustment_rate = 0.10;
        raise = salary * readjustment_rate;
    }

    if salary <= 2000.00 && readjustment_rate == 0.0 {
        readjustment_rate = 0.07;
        raise = salary * readjustment_rate;
    }

    if salary > 2000.00 && readjustment_rate == 0.0 {
        readjustment_rate = 0.04;
        raise = salary * readjustment_rate;
    }
        
    println!("Novo salario: {:.2}", salary + raise); 
    println!("Reajuste ganho: {:.2}", raise); 
    println!("Em percentual: {:.0} %", readjustment_rate * 100.00); 
}

