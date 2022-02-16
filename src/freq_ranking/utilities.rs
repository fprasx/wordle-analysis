use std::fs::read_to_string;
use std::collections::{BTreeMap, HashMap};
/*
* Returns an array of hashmaps which contain the letter frequencies for each index
* The first map contains the frequencies for the first spot, etc.
*/
pub fn cfreqs_indices() -> [HashMap<char, usize>; 5] {
    let buf = read_to_string("wordle_words.txt").expect("Could not read wordle_words.txt!");
    let words = buf
        .split(" ")
        .map(|x| x.trim_start().trim_end())
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>();

    // Get frequencies of each letter at each position
    let mut cfreqs: [HashMap<char, usize>; 5] = Default::default();
    // For each index
    for index in 0usize..5 {
        // Make a new map
        let mut freqs = HashMap::<char, usize>::new();
        // For each word
        for word in &words {
            // For the desired character in the word
            for c in word.chars().nth(index) {
                // Increment the correct entry in the map
                // If there is no entry, add one
                let count = freqs.entry(c).or_insert(0);
                *count += 1;
            }
        }
        cfreqs[index] = freqs;
    }
    cfreqs
}

/*
Returns the absolute frequencies of each letter accross all spots
*/
pub fn cfreqs() -> BTreeMap<usize, char> {
    let cfreqs_i = cfreqs_indices();
    let mut absolute_freqs = BTreeMap::<usize, char>::new();
    for c in 'a'..'{' {
        absolute_freqs.insert(
            (0..5)
                // Maps index to value stored freqs for character at position `index` in word
                .map(|x| *cfreqs_i[x].get(&c).unwrap_or(&0))
                // Add 'em all up
                .reduce(|acc, elem| acc + elem)
                // We know each letter occurs at least once to we can unwrap
                .unwrap(),
            c,
        );
    }
    absolute_freqs
}

/*
Take in a float, and based on how small/large it is
assign it a color, higher means greener
 */
pub fn add_color<'a>(num: f32) -> String {
    if num < 0.03 {
        format!("\x1B[38;2;150;150;250m{:.2}\x1B[0m", num)
    } else if num < 0.06 {
        format!("\x1B[38;2;0;170;0m{:.2}\x1B[0m", num)
    } else if num < 0.09 {
        format!("\x1B[38;2;0;190;0m{:.2}\x1B[0m", num)
    } else if num < 0.12 {
        format!("\x1B[38;2;0;210;0m{:.2}\x1B[0m", num)
    } else {
        format!("\x1B[38;2;0;255;0m{:.2}\x1B[0m", num)
    }
}
/*
Flips a BTreeMap from mapping K to V to V to K
*/
pub fn flip<K: Clone, V: Ord + Clone>(map: BTreeMap<K, V>) -> BTreeMap<V, K> {
    let mut ret = BTreeMap::<V, K>::new();
    for (k, v) in map.into_iter() {
        ret.insert(v, k);
    }
    ret
}

/*
Check if a word (must have length 5) has repeated characters
*/
pub fn is_dedup(x: &str) -> bool {
    assert!(x.len() == 5);
    let mut vec = x.chars().collect::<Vec<char>>();
    vec.sort_unstable();
    vec.dedup();
    vec.len() == 5
}