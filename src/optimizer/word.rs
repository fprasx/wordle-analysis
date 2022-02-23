use std::convert::From;
use std::fmt::{Display, Error, Formatter};
use std::ops::{Index, IndexMut};
use std::iter::{Iterator, IntoIterator};

#[derive(Debug, PartialEq, Eq)]
pub struct Word([char; 5]);

impl Word {
    fn contains(&self, item: char) -> bool {
        for i in 0..5 {
            if self[i] == item {
                return true;
            }
        }
        return false;
    }
}

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

pub struct WordIter {
    index: usize,
    data: Word
}

impl Iterator for WordIter {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < 5 {
            Some(self.data[self.index])
        } else {
            None
        }
    }
}

impl IntoIterator for Word {
    type Item = char;
    type IntoIter = WordIter;
    fn into_iter(self) -> Self::IntoIter {
        WordIter {
            index: 0,
            data: self
        }
    }
}