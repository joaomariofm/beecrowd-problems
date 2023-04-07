use std::io;

fn main() {
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let inputs : Vec<f64> = input1.trim().split(" ").map(|x| x.parse().expect("Not a float")).collect();

    let n1 : f64 = inputs[0];
    let n2 : f64 = inputs[1];
    let n3 : f64 = inputs[2];
    let n4 : f64 = inputs[3];

    let mut average = ((n1 * 2.0) + (n2 * 3.0) + (n3 * 4.0) + (n4 * 1.0)) / 10.0;

    if average < 5.0 {
        println!("Media: {:.1}", average);
        return println!("Aluno reprovado.");
    }

    if average > 6.9 {
        println!("Media: {:.1}", average);
        return println!("Aluno aprovado.");
    }

    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let n5 : f64 = input2.trim().parse().unwrap();

    println!("Media: {:.1}", average);
    println!("Aluno em exame.");
    println!("Nota do exame: {:.1}", n5);

    average = (average + n5) / 2.0;

    if average >= 5.0 {
        println!("Aluno aprovado.");
    } else {
        println!("Aluno reprovado.");
    }
    
    println!("Media final: {:.1}", average);
}

