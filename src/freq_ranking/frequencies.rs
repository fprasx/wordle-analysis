use super::utilities::{add_color, cfreqs, cfreqs_indices};
use std::fs::read_to_string;
pub fn main() {
    // Reading in words
    let buf = read_to_string("wordle_words.txt").expect("Could not read wordle_words.txt!");
    let words = buf.split(" ").collect::<Vec<&str>>();
    let numwordsf32: f32 = words.len() as f32;
    let freqs = cfreqs_indices();
    let absolute_freqs = cfreqs();

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
            add_color((*freqs[0].get(&c).unwrap_or(&0) as f32) / numwordsf32)
        );
        print!(
            " {} |",
            add_color((*freqs[2].get(&c).unwrap_or(&0) as f32) / numwordsf32)
        );
        print!(
            " {} |",
            add_color((*freqs[2].get(&c).unwrap_or(&0) as f32) / numwordsf32)
        );
        print!(
            " {} |",
            add_color((*freqs[3].get(&c).unwrap_or(&0) as f32) / numwordsf32)
        );
        println!(
            " {} |",
            add_color((*freqs[4].get(&c).unwrap_or(&0) as f32) / numwordsf32)
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
            add_color((*freqs[0].get(&c).unwrap_or(&0) as f32) / numwordsf32)
        );
        print!(
            " {} |",
            add_color((*freqs[1].get(&c).unwrap_or(&0) as f32) / numwordsf32)
        );
        print!(
            " {} |",
            add_color((*freqs[2].get(&c).unwrap_or(&0) as f32) / numwordsf32)
        );
        print!(
            " {} |",
            add_color((*freqs[3].get(&c).unwrap_or(&0) as f32) / numwordsf32)
        );
        println!(
            " {} |",
            add_color((*freqs[4].get(&c).unwrap_or(&0) as f32) / numwordsf32)
        );
        println!("+---+------+------+------+------+------+");
    }
    // TODO: write output to files
}
