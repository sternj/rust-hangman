use std::io::{self, Write};
use std::io::stdin;
use rand::seq::SliceRandom;
use std::collections::HashSet;
fn main() {
    let words = vec!["snakes", "thanks", "granted", "awkward", "bagpipes", "banjo", "bungler", "croquet", "crypt"];
    let states  = [r#"
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
    "#
    ];
    let current_word = words.choose(&mut rand::thread_rng()).unwrap();
    let mut states_ind = 0;
    let mut s;
    let mut guessed = HashSet::<char>::new();
    let mut current_word_set = HashSet::new();
    for c in current_word.chars() {
        current_word_set.insert(c);
    }
    let mut current_state = vec!["_"; (*current_word).len()];
    while states_ind != states.len() - 1 {
        s = String::new();
        println!("{}", states[states_ind]);
        println!("{}", current_state.iter().map({|c| format!("{} ", c)}).collect::<String>());
        println!("Make a guess!");
        print!("> ");
        io::stdout().flush().unwrap();
        stdin().read_line(&mut s).unwrap();
        let c = s.chars().next().unwrap();
        if guessed.contains(&c) {
            println!("You already guessed that!");
            continue;
        }
        if ! current_word_set.contains(&c) {
            current_word_set.insert(c);
            states_ind += 1;
            println!("Incorrect");
        } else {
            for (i, val) in current_word.chars().enumerate() {
                if c == val{
                    current_state[i] = &format!("{}", val.to_string());
                }
            }
        }
        guessed.insert(c);
    }
}
