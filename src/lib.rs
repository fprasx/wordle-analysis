#![feature(test)]
/**
 * For finding the best starting guess using informational entropy
 * Following 3Blue1Brown's approach but hopefully searching to depth level 3
 */
pub mod optimizer;
pub mod bench {
    /**
     * For a word ([char; 5]), is it faster to convert the word to hashset
     * and then see if a character is in the set, or is it faster to just check every
     * character (only 5 of them . . . hmm)
     */
    pub mod contains;
}
pub mod freq_ranking {
    /**
     * Miscellaneous utility functions
     */
    pub mod utilities;
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
}
