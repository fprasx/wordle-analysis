use crate::{GREEN, GREY, YELLOW};

pub trait Wordle {
    fn contains(&self, other: char) -> bool;
    fn matches(&self, other: &Self, pattern: [usize; 5]) -> bool;
}

pub trait FromStr {
    fn from_str(s: &str) -> Self;
}
pub type Word = [char; 5];

impl FromStr for Word {
    fn from_str(s: &str) -> Self {
        let mut iter = s.chars();
        [
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        ]
    }
}

impl Wordle for Word {
    fn contains(&self, item: char) -> bool {
        for char in self {
            if *char == item {
                return true;
            }
        }
        false
    }

    fn matches(&self, word: &Word, pattern: [usize; 5]) -> bool {
        for ((self_letter, word_letter), color) in self.iter().zip(word.iter()).zip(pattern) {
            match color {
                // If it's grey, the word should not contain the current letter
                GREY => {
                    if self.contains(*word_letter) {
                        return false;
                    }
                }
                // If it's yellow, the word should countain the current letter, but it cannot be in the right spot
                YELLOW => {
                    if self_letter == word_letter || !self.contains(*word_letter) {
                        return false;
                    }
                }
                // If it's green, the word must contain the letter at the right spot
                GREEN => {
                    if self_letter != word_letter {
                        return false;
                    }
                }
                // There should only be GREY, YELLOW, or GREEN in the pattern, if not, panic
                _ => {
                    unreachable!()
                }
            }
        }
        true
    }
}
impl Wordle for str {
    fn contains(&self, item: char) -> bool {
        // self.contains(item)
        self.chars().any(|c| c == item)
    }

    fn matches(&self, word: &str, pattern: [usize; 5]) -> bool {
        for ((self_letter, word_letter), color) in self.chars().zip(word.chars()).zip(pattern) {
            match color {
                // If it's grey, the word should not contain the current letter
                GREY => {
                    if self.contains(word_letter) {
                        return false;
                    }
                }
                // If it's yellow, the word should countain the current letter, but it cannot be in the right spot
                YELLOW => {
                    if self_letter == word_letter || !self.contains(word_letter) {
                        return false;
                    }
                }
                // If it's green, the word must contain the letter at the right spot
                GREEN => {
                    if self_letter != word_letter {
                        return false;
                    }
                }
                // There should only be GREY, YELLOW, or GREEN in the pattern, if not, panic
                _ => {
                    unreachable!()
                }
            }
        }
        true
    }
}
