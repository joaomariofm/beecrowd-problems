use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let inputs : Vec<i64> = input.trim().split(" ").map(|x| x.parse().expect("Not an integer")).collect();

    let n1 : i64 = inputs[0];
    let n2 : i64 = inputs[1];
    let n3 : i64 = inputs[2];
    let n4 : i64 = inputs[3];

    if n1 == n3 && n2 == n4 { 
        return println!("O JOGO DUROU 24 HORA(S) E 0 MINUTO(S)");
    }

    if n1 > n3 || (n1 == n3 && n2 > n4) {
        let mut horas = (24 - n1) + n3;
        let mut minutos = n4 - n2;

        if n4 < n2 {
            horas = horas - 1;
            minutos = (60 + n4) - n2; 
        }

        return println!("O JOGO DUROU {} HORA(S) E {} MINUTO(S)", horas, minutos);
    }

    let mut horas = n3 - n1;
    let mut minutos = n4 - n2;

    if n4 < n2 {
        horas = horas - 1;
        minutos = (60 + n4) - n2; 
    }

    println!("O JOGO DUROU {} HORA(S) E {} MINUTO(S)", horas, minutos);
}

