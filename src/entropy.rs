use std::convert::From;
use std::fs;
use std::fmt::{Formatter, Error, Display};
use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq, Eq)]
pub struct Word([char; 5]);

impl From<String> for Word {
    fn from(s: String) -> Self {
        assert!(s.len() == 5, "Can only convert Strings of length 5 to Word");
        let mut word = ['a'; 5];
        s.chars().enumerate().for_each(|(i, c)| word[i] = c);
        Word(word)
    }
}

impl From<&str> for Word {
    fn from(s: &str) -> Self {
        assert!(s.len() == 5, "Can only convert Strings of length 5 to Word");
        let mut word = ['a'; 5];
        s.chars().enumerate().for_each(|(i, c)| word[i] = c);
        Word(word)
    }
}

impl Index<usize> for Word {
    type Output = char;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Word {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}{}{}{}{}", self[0], self[1], self[2], self[3], self[4])
    }
}

fn main() {
    let buf = fs::read_to_string("wordle_words.txt").expect("Could not read wordle_words.txt!");
    let words = buf
        .split(" ")
        .map(|x| x.trim_start().trim_end())
        .filter(|x| !x.is_empty())
        .map(|x| x.into())
        .collect::<Vec<Word>>();
}
