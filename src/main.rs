use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::io::stdin;
use std::io::{self, Write};

fn main() {
    let words = vec![
        "snakes", "thanks", "granted", "awkward", "bagpipes", "banjo", "bungler", "croquet",
        "crypt",
    ];
    let states = [
        r#"
|-----|
|
|
|
|_______
    "#,
        r#"
|-----|
|     o
|
|
|_______
    "#,
        r#"
|-----|
|     o
|     |
|
|_______
    "#,
        r#"
|-----|
|     o
|     |
|    /
|_______
    "#,
        r#"
|-----|
|     o
|     |
|    / \
|_______
    "#,
        r#"
|-----|
|     o
|    /|
|    / \
|_______
    "#,
        r#"
|-----|
|     o
|    /|\
|    / \
|_______
    "#,
    ];
    let current_word = words.choose(&mut rand::thread_rng()).unwrap();
    let mut states_ind = 0;
    let mut current_guess;
    let mut guessed = HashSet::<char>::new();
    let mut current_word_set = HashSet::new();
    for c in current_word.chars() {
        current_word_set.insert(c);
    }
    let mut partially_revealed = "_".repeat(current_word.len());
    while states_ind != states.len() - 1 {
        current_guess = String::new();
        println!("{}", states[states_ind]);
        println!("{}", partially_revealed);
        println!("Make a guess!");
        print!("> ");
        io::stdout().flush().unwrap();
        stdin().read_line(&mut current_guess).unwrap();
        let c = current_guess.chars().next().unwrap();
        if guessed.contains(&c) {
            println!("You already guessed that!");
            continue;
        }
        if !current_word_set.contains(&c) {
            current_word_set.insert(c);
            states_ind += 1;
            println!("Incorrect");
        } else {
            for (i, chr) in current_word.char_indices().filter(|(_, chr)| c == *chr) {
                let chr_string: String = chr.to_string();
                partially_revealed.replace_range(i..i + chr_string.len(), &chr_string);
            }
        }
        guessed.insert(c);
    }
}
