use std::io;
use rand::Rng;
use rand::thread_rng;
use std::collections::HashMap;
use std::collections::hash_map::Entry::Occupied;


const AVAILABLE_WORDS: [&str; 1] = [
    //"fish",
    "pizza",
    //"bed",
    //"shirt",
    //"laptop",
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

   let result: bool = loop {
        if remaining_tries < 1 {
            break false
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
        let victory = check_victory(&word_as_map);
        if victory {
            println!("Congratulations you had won the game!!");
            break true
        }
        println!("Remaining tries: {}", remaining_tries)
    };
    result
}

// check_victory Checks key by key in map to if there are negative values.
fn check_victory(word: &HashMap<char, i8>) -> bool {
    for (key, value) in word.iter() {
        if *value != 0 {
            return false
        }
    }
    true
}

fn read_user_input() -> String {
    println!("Enter a new key:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Falied to read the input");
    input
}

// Search for a letter existed in the current word. If the letter exists in the word, then the
// letters count is reducced
fn search_letter_in_word(letter: char, word: &mut HashMap<char, i8>) -> bool {
    if let Occupied(mut entry) = word.entry(letter) {
        entry.insert(0);
        return true;
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
