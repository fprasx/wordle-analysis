#![feature(portable_simd)]
// To allow for benchmarking
#![feature(test)]
extern crate test;

pub const GREY: usize = 0;
pub const YELLOW: usize = 1;
pub const GREEN: usize = 2;
pub const NUM_WORDS: usize = 12972;
pub const NUM_WORDLE_WORDS: usize = 2315;
// Represents all 243 color states that can happen after a guess
// Technically, some are impossible, like [YELLOW, GREEN x 4]
// But it's now worth dealing with all of those
pub const COLOR_PATTERNS: [[usize; 5]; 243] = {
    // 3 to the 5th is 243
    let mut x = [[0; 5]; 243];
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;
    let mut e = 0;
    // Rustc doesn't like for loops (at least ones with ranges)
    while a < 3 {
        while b < 3 {
            while c < 3 {
                while d < 3 {
                    while e < 3 {
                        x[a * 1 + b * 3 + c * 9 + d * 27 + e * 81] = [a, b, c, d, e];
                        e += 1;
                    }
                    e = 0;
                    d += 1;
                }
                d = 0;
                c += 1;
            }
            c = 0;
            b += 1;
        }
        b = 0;
        a += 1;
    }
    x
};

/**
 * For finding the best starting guess using informational entropy
 * Following 3Blue1Brown's approach but hopefully searching to depth level 3
 */
pub mod entropy {
    pub mod word;
}
pub mod bench {
    /**
     * How much faster is comparing a integer to 0 than an float?
     */
    pub mod cmp;
    /**
     * For a word ([char; 5]), is it faster to convert the word to hashset
     * and then see if a character is in the set, or is it faster to just check every
     * character (only 5 of them . . . hmm)
     */
    pub mod contains;
}
pub mod freq_ranking {
    /**
     * Finding letter frequencies for individual indices and in general
     */
    pub mod frequencies;
    /**
     * Finding best words by maximizing chances of yellows, discounting words with duplicates
     */
    pub mod ranking_green;
    /**
     * Finding best words by maximizing chances of greens, discounting words with duplicates
     */
    pub mod ranking_yellow;
    /**
     * Miscellaneous utility functions
     */
    pub mod utilities;
}
