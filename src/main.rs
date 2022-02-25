#![feature(portable_simd)]
use std::fs;
use wordle::entropy::word::{Word, Wordle, FromStr};
use wordle::{COLOR_PATTERNS, NUM_WORDS};
fn main() {
    let buf = fs::read_to_string("all_words.txt").expect("Could not read wordle_words.txt!");
    let words = buf.split(" ").map(|x| Word::from_str(x)).collect::<Vec<Word>>();
    let words = words.as_slice();
    for word in words.iter().take(1) {
        println!("{:?} has entropy {}", word, get_entropy(&word, words));
    }
    println!("{}", get_entropy(&Word::from_str("tares"), words))
}

fn get_entropy(word: &Word, words: &[Word]) -> f32 {
    let mut entropy = 0f32;
    for pattern in COLOR_PATTERNS {
        let mut count = 0;
        for w in words.iter() {
            if Wordle::matches(word, &w, pattern) {
                count += 1;
            }
        }
        if count != 0 {
            let probability = count as f32 / NUM_WORDS as f32;
            entropy += probability * -probability.log2();
        }
    }
    entropy
}
