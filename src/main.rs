#![feature(portable_simd)]
use std::fs;
use wordle::entropy::word::Word;
use wordle::{COLOR_PATTERNS, NUM_WORDS};
fn main() {
    let buf = fs::read_to_string("all_words.txt").expect("Could not read wordle_words.txt!");
    let words = buf.split(" ").collect::<Vec<&str>>();
    for word in words.clone().into_iter().take(10) {
        get_entropy(&word, &words);
    }
}

fn get_entropy(word: &str, words: &Vec<&str>) -> f32 {
    let mut entropy = 0f32;
    for pattern in COLOR_PATTERNS {
        let mut count = 0;
        for w in words.iter() {
            if Word::matches(word, &w, pattern) {
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
