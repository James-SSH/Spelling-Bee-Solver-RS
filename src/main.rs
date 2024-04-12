use std::{
    fs,
    io::{stdin, Read},
};

use regex::Regex;

fn main() {
    let mut english_words = {
        let mut f: String = String::from("");
        let _ = fs::File::open("./english_reduced.txt")
            .unwrap()
            .read_to_string(&mut f);
        let vec: Vec<String> = f.lines().map(String::from).collect();
        vec
    };

    println!("Enter the list of characters");
    let mut letters = String::from("");
    let _ = stdin().read_line(&mut letters);

    println!("What is the center character?");
    let mut mesiah = String::from("");
    let _ = stdin().read_line(&mut mesiah);

    let letters = letters.trim();
    let mesiah = mesiah.trim();
    let regex = Regex::new(format!("^([{}]){{3,}}$", letters).as_str()).unwrap();

    english_words = english_words
        .into_iter()
        .filter(|w| w.contains(mesiah) && regex.find(&w).is_some())
        .collect();

    println!("All Possible Words: {:?}", english_words);
}
