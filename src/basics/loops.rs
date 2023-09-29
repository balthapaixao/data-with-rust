use rand::Rng;

fn get_random_numbers() {
    for n in 1..10 {
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(0..n);

        println!("Random number {}", random_number);
    }
}

fn programming_languages() {
    let languages = ["Python", "SQL", "Rust"];
    for language in languages.iter() {
        println!("{} is amazing!", language);
    }
}

fn for_loops() {
    get_random_numbers();
    programming_languages();
}

fn loop_loops() {
    let mut cnt = 0;

    loop {
        // similar to while true
        cnt += 1;

        if (cnt < 10) {
            println!("Loading... {}", cnt);
        } else if (cnt == 10) {
            println!("Ready!");
            break;
        }
    }
}
fn while_loops() {
    let mut cnt = 0;
    while cnt < 10 {
        cnt += 1;
        println!("Loading... {}", cnt);
    }
    println!("Ready!");
}
fn main() {
    for_loops();
    loop_loops();
    while_loops();
}
