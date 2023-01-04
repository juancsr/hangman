use std::io;
use rand::Rng;
use rand::thread_rng;


const AVAILABLE_WORDS: [&str; 5] = [
    "fish",
    "pizza",
    "bed",
    "shirt",
    "laptop",
];

const MAX_TRIES: i32 = 10;

//let game_word = select_word();

fn main() {
    let word = select_word();
    print!("-- Word to play: [{}]-- \n", word);
    let mut remaining_tries = MAX_TRIES;
    let user_input = read_user_input();
    print!("{}", user_input);
}

fn read_user_input() -> String {
    println!("Enter a new key:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Falied to read the input");
    input
}

// Search for a letter existed in the current word
fn search_letter_in_word() -> bool {
    true
}

// Selects a new word to use in the game
fn select_word() -> String {
    let number = thread_rng().gen_range(0, AVAILABLE_WORDS.len().try_into().unwrap());
    let word = AVAILABLE_WORDS[number];
    word.to_string()
}
