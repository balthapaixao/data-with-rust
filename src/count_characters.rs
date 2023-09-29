use rand::Rng;

fn count_caracters(input: &str, car: char) -> usize {
    input.chars().filter(|x| *x == car).count() as usize
}

fn main() {
    let mut rng = rand::thread_rng();
    let input_string: &str = "Club de Regatas Vasco da Gama";

    if rng.gen_bool(0.5) {
        count_caracters(input_string, 'B');
    } else {
        count_caracters(input_string, '9');
    }

    //    let input_string = "Hey everyone!";
    //    let mystery_cnt = count_consonants(&input_string);
    //    println!("The mystery count is: {}", mystery_cnt);
}
