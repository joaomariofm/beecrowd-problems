use std::io;

fn ave_branching(aminal_feeding_habit: String) -> String {
    if aminal_feeding_habit == "carnivoro" {
        return "aguia".to_string();
    }

    return "pomba".to_string()
}

fn mamifero_branching(aminal_feeding_habit: String) -> String {
    if aminal_feeding_habit == "onivoro" {
        return "homem".to_string();
    }

    return "vaca".to_string()
}

fn inseto_branching(aminal_feeding_habit: String) -> String {
    if aminal_feeding_habit == "hematofago" {
        return "pulga".to_string();
    }

    return "lagarta".to_string()
}

fn anelideo_branching(aminal_feeding_habit: String) -> String {
    if aminal_feeding_habit == "hematofago" {
        return "sanguessuga".to_string();
    }

    return "minhoca".to_string()
}

fn vertebrado_branching(aminal_group: String, aminal_feeding_habit: String) -> String {
    if aminal_group == "ave" {
        return ave_branching(aminal_feeding_habit);
    }

    return mamifero_branching(aminal_feeding_habit);
}

fn invertebrado_branching(aminal_group: String, aminal_feeding_habit: String) -> String {
    if aminal_group == "inseto" {
        return inseto_branching(aminal_feeding_habit);
    }

    return anelideo_branching(aminal_feeding_habit);
}

fn main() {
    let mut first_word = String::new();
    io::stdin().read_line(&mut first_word).expect("Failed to read line");
    first_word = first_word.trim().to_owned();

    let mut second_word = String::new();
    io::stdin().read_line(&mut second_word).expect("Failed to read line");
    second_word = second_word.trim().to_owned();

    let mut third_word = String::new();
    io::stdin().read_line(&mut third_word).expect("Failed to read line");
    third_word = third_word.trim().to_owned();

    if first_word == "vertebrado" {
        return println!("{}", vertebrado_branching(second_word.clone(), third_word.clone()));
    }

    println!("{}", invertebrado_branching(second_word.clone(), third_word.clone()));
}

