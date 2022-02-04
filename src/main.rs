use std::collections::{BTreeMap, HashMap};
use std::fs::read_to_string;
use wordle::add_color;
fn main() {
    // Reading in words
    let buf = read_to_string("wordle.txt").expect("Problem reading file!");
    let words = buf
        .split(" ")
        .map(|x| x.trim_start())
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>();
    let numwordsf32: f32 = words.len() as f32;

    // Get frequencies of each letter at each position
    let mut cfreqs: [HashMap<char, usize>; 5] = Default::default();
    // For each index
    for index in 0usize..5 {
        // Make a new map
        let mut freqs = HashMap::<char, usize>::new();
        // For ech word
        for word in &words {
            // For the desired character in the word
            for c in word.chars().nth(index) {
                // Increment the correct entry in the map
                // If there is not entry, add one
                let count = freqs.entry(c).or_insert(0);
                *count += 1;
            }
        }
        cfreqs[index] = freqs;
    }

    // Add up the frequencies from each position, for each letter
    // NOTE: the result is the frequency time 5, because floats can't be used as keys
    let mut absolute_freqs = BTreeMap::<usize, char>::new();
    for c in 'a'..'z' {
        absolute_freqs.insert(
            (0..5)
                // Maps index to value stored freqs for character at position `index` in word
                .map(|x| *cfreqs[x].get(&c).unwrap_or(&0))
                // Add 'em all up
                .reduce(|acc, elem| acc + elem)
                // We know each letter occurs at least once to we can unwrap
                .unwrap(),
            c,
        );
    }

    // Print out absolute letter frequencies
    println!("Absolute letter frequencies:");
    println!("+---+------+");
    for (num, c) in absolute_freqs.iter().rev() {
        println!(
            "| {} | {} |",
            c,
            add_color((*num as f32) / (5f32 * numwordsf32))
        );
        println!("+---+------+")
    }
    println!();

    // Print out subcomponents of letter frequencies (ordered by frequency)
    println!("Frequencies of letters at each position (ordered by frequency):");
    println!("    +------+------+------+------+------+");
    println!("    |  1   |  2   |  3   |  4   |  5   |");
    println!("+---+------+------+------+------+------+");
    for (_num, c) in absolute_freqs.iter().rev() {
        print!("| {} |", c);
        print!(
            " {} |",
            add_color((*cfreqs[0].get(&c).unwrap_or(&0) as f32) / numwordsf32)
        );
        print!(
            " {} |",
            add_color((*cfreqs[2].get(&c).unwrap_or(&0) as f32) / numwordsf32)
        );
        print!(
            " {} |",
            add_color((*cfreqs[2].get(&c).unwrap_or(&0) as f32) / numwordsf32)
        );
        print!(
            " {} |",
            add_color((*cfreqs[3].get(&c).unwrap_or(&0) as f32) / numwordsf32)
        );
        println!(
            " {} |",
            add_color((*cfreqs[4].get(&c).unwrap_or(&0) as f32) / numwordsf32)
        );
        println!("+---+------+------+------+------+------+");
    }
    println!();

    // Print out subcomponents of letter frequencies (ordered by letter)
    println!("Frequencies of letters at each position (ordered by letter):");
    println!("    +------+------+------+------+------+");
    println!("    |  1   |  2   |  3   |  4   |  5   |");
    println!("+---+------+------+------+------+------+");
    for c in 'a'..'z' {
        print!("| {} |", c);
        print!(
            " {} |",
            add_color((*cfreqs[0].get(&c).unwrap_or(&0) as f32) / numwordsf32)
        );
        print!(
            " {} |",
            add_color((*cfreqs[1].get(&c).unwrap_or(&0) as f32) / numwordsf32)
        );
        print!(
            " {} |",
            add_color((*cfreqs[2].get(&c).unwrap_or(&0) as f32) / numwordsf32)
        );
        print!(
            " {} |",
            add_color((*cfreqs[3].get(&c).unwrap_or(&0) as f32) / numwordsf32)
        );
        println!(
            " {} |",
            add_color((*cfreqs[4].get(&c).unwrap_or(&0) as f32) / numwordsf32)
        );
        println!("+---+------+------+------+------+------+");
    }
}
