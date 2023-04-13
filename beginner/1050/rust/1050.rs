use std::io;

fn ddd_converter(ddd: i64) -> String {
    if ddd == 61 {
        return "Brasilia".to_string();
    }

    if ddd == 71 {
        return "Salvador".to_string();
    }

    if ddd == 11 {
        return "Sao Paulo".to_string();
    }

    if ddd == 21 {
        return "Rio de Janeiro".to_string();
    }

    if ddd == 32 {
        return "Juiz de Fora".to_string();
    }

    if ddd == 19 {
        return "Campinas".to_string();
    }

    if ddd == 27 {
        return "Vitoria".to_string();
    }

    if ddd == 31 {
        return "Belo Horizonte".to_string();
    }

    return "DDD nao cadastrado".to_string()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let ddd : i64 = input.trim().parse().unwrap();
    
    println!("{}", ddd_converter(ddd)); 
}

