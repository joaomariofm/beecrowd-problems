fn main() {
    let mut total_value : i64;

    for number in 1..3 {
        let mut code_input = String::new();
        io::stdin().read_line(&mut code_input).expect("Failed to read line");
        let code : i64 = code_input .trim().parse().unwrap();

        let mut unit_input = String::new();
        io::stdin().read_line(&mut unit_input).expect("Failed to read line");
        let unit : i64 = unit_input.trim().parse().unwrap();
    }
}
