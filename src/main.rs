use std::io;
use rand::Rng;
use rand::thread_rng;
use std::collections::HashMap;
use std::collections::hash_map::Entry::Occupied;


const AVAILABLE_WORDS: [&str; 5] = [
    "fish",
    "pizza",
    "bed",
    "shirt",
    "laptop",
];

const MAX_TRIES: i32 = 2;

fn main() {
    loop {
        let win_game = play();
        if !win_game {
            println!("End of game");
        }
        println!("Would you like to restart? [Y/n]");
        let user_input = read_user_input();
        println!("user_input: {}", user_input);
        if user_input.chars().next() != Some('Y') {
            println!("finishing the game");
            break;
        }
    }  
}

// play starts the game, returns true if the user won the game, otherwise returns false
fn play() -> bool{
    let word = select_word();
    print!("-- Word to play: [{}]-- \n", word);
    let mut remaining_tries = MAX_TRIES;
    let mut word_as_map = transform_word_to_map(word.to_string());

    loop {
        // TODO: Create a function that check each value of map and if all of them are 0 then the user won the game
        if remaining_tries < 1 {
            return false
        }
        let user_input = read_user_input();
        print!("{}", user_input);
        let success = search_letter_in_word(
            user_input
                .chars()
                .next()
                .expect("string is empty")
            , &mut word_as_map);
        println!("{success}");
        if !success {
            remaining_tries -= 1;
        }
        println!("Remaining tries: {}", remaining_tries)
    }
    // FIXME: yeah. fixme.
    //true
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
fn search_letter_in_word(letter: char, word: &mut HashMap<char, i8>) -> bool {
    if let Occupied(mut entry) = word.entry(letter) {
        entry.insert(entry.get() - 1);
        return true
    }
    false
}

// Selects a new word to use in the game
fn select_word() -> &'static str {
    let number = thread_rng()
        .gen_range(0, AVAILABLE_WORDS.len().try_into().unwrap());
    let word = AVAILABLE_WORDS[number];
    word
}

fn transform_word_to_map(word: String) -> HashMap<char, i8> {
    let mut letters_map = HashMap::new();
    for l in word.chars() {
        let mut current_value = letters_map.get(&l).copied().unwrap_or(0);
        current_value+=1;
        letters_map.insert(l, current_value);
    }
    letters_map
}
