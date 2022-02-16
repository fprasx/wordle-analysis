/**
 * For finding the best starting guess using informational entropy
 * Following 3Blue1Brown's approach but hopefully searching to depth level 3
 */
pub mod optimizer;

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
