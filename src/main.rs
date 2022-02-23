use std::fs;
use wordle::optimizer::entropy::Word;
fn main() {
    let buf = fs::read_to_string("wordle_words.txt").expect("Could not read wordle_words.txt!");
    let words = buf
        .split(" ")
        .map(|x| x.trim_start().trim_end())
        .filter(|x| !x.is_empty())
        .map(|x| x.into())
        .collect::<Vec<Word>>();
    for word in words.iter().take(5) {
        println!("{}", word)
    }
}