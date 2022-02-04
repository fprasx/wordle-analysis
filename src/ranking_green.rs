use std::fs::{read_to_string, OpenOptions};
use std::collections::{BTreeMap};
use std::io::{BufWriter, Write};
use crate::{is_dedup, cfreqs_indices};
pub fn main() {
    let freqs = cfreqs_indices();

    // If we want all letters to be correct on the first try
    // We maximize the probability that each letter is correct
    // This is basically going for all greens

    let buf = read_to_string("all_words.txt").expect("Could not read all_words.txt!");
    let all_words = buf
        .split(" ")
        .map(|x| x.trim_end().trim_start())
        .filter(|x| is_dedup(x))
        .collect::<Vec<&str>>();

    // Add up the frequency for each letter at its spot to get word's score
    let eval_word = |word: &str| {
        freqs[0].get(&word.chars().nth(0).unwrap()).unwrap_or(&0)
            + freqs[1].get(&word.chars().nth(1).unwrap()).unwrap_or(&0)
            + freqs[2].get(&word.chars().nth(2).unwrap()).unwrap_or(&0)
            + freqs[3].get(&word.chars().nth(3).unwrap()).unwrap_or(&0)
            + freqs[4].get(&word.chars().nth(4).unwrap()).unwrap_or(&0)
    };

    let mut wmap = BTreeMap::<usize, Vec<&str>>::new();
    for word in &all_words {
        let list = wmap.entry(eval_word(word)).or_insert(Vec::<&str>::new());
        list.push(word);
    }

    // Flatten BTreeMap into a list that is sored by value
    let mut ordered_words = Vec::<(usize, &str)>::new();
    ordered_words.reserve(all_words.len());
    for (value, words) in wmap {
        for word in words {
            ordered_words.push((value, word));
        }
    }

    // Write the output out
    let dst = OpenOptions::new()
        .write(true)
        .open("word_ranking_green.txt")
        .expect("Could not open word_ranking_green.txt");
    let mut writer = BufWriter::new(dst);
    for (i, (value, word)) in ordered_words.iter().rev().enumerate() {
        writer
            .write(format!("{}: {} ({})\n", i + 1, word, value).as_bytes())
            .expect("Could not write to word_ranking_green.txt");
    }
    writer.flush().expect("Could not flush bufwriter");
}