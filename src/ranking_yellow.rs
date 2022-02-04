use crate::{cfreqs, flip, is_dedup};
use std::fs::{read_to_string, OpenOptions};
use std::collections::BTreeMap;
use std::io::{BufWriter, Write};

pub fn main() {
    let buf = read_to_string("all_words.txt").expect("Could not read all_words.txt!");
    let all_words = buf
        .split(" ")
        .map(|x| x.trim_end().trim_start())
        .filter(|x| is_dedup(x))
        .collect::<Vec<&str>>();

    let absolute_freqs_flipped = flip(cfreqs());
    // Add up the frequency for each letter at its spot to get word's score
    let eval_word = |word: &str| {
        let mut total= 0;
        for i in 0..5 {
            // We know the word has 5 characters so we can unwrap
            let c = word.chars().nth(i).unwrap();
            total += absolute_freqs_flipped.get(&c).unwrap()
        }
        total
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
        .open("word_ranking_yellow.txt")
        .expect("Could not open word_ranking_yellow.txt");
    let mut writer = BufWriter::new(dst);
    for (i, (value, word)) in ordered_words.iter().rev().enumerate() {
        writer
            .write(format!("{}: {} ({})\n", i + 1, word, value).as_bytes())
            .expect("Could not write to word_ranking_yellows.txt");
    }
    writer.flush().expect("Could not flush bufwriter");
}