use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let inputs : Vec<i64> = input.trim().split(" ").map(|x| x.parse().expect("Not an integer")).collect();

    let n1 : i64 = inputs[0];
    let n2 : i64 = inputs[1];

    if n1 == n2 { 
        return println!("O JOGO DUROU 24 HORA(S)");
    }

    if n1 > n2 {
        return println!("O JOGO DUROU {} HORA(S)", (24 - n1) + n2);
    }

    println!("O JOGO DUROU {} HORA(S)", n2 - n1);
}

