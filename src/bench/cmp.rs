// /**
//  * How much faster is comparing a integer to 0 than an float?
//  */
// #[cfg(test)]
// // These lints are here because some variables are conditionally updated
// // Normally they would be returned, but this is unnecessary for the benchmark
// #[allow(unused_assignments)]
// #[allow(unused_variables)]
// mod test {
//     /*
//     test bench::cmp::test::floatcmp                   ... bench:  16,335,212 ns/iter (+/- 184,265)
//     test bench::cmp::test::intcmp                     ... bench:  16,318,994 ns/iter (+/- 156,333)
//     Seems like there's no difference!
//     */
//     use crate::entropy::word;
//     use crate::COLOR_PATTERNS;
//     use std::fs;
//     use test::Bencher;

//     /**
//      * In this version, we have the count as an i32, and we check if count is nonzero,
//      * if so, update the entropy
//      */
//     #[bench]
//     fn intcmp(bencher: &mut Bencher) {
//         let buf = fs::read_to_string("all_words.txt").expect("Could not read wordle_words.txt!");
//         let words = buf
//             .split(" ")
//             .map(|x| x.into())
//             .collect::<Vec<word::Word>>();
//         let num_words = words.len() as f32;
//         let my_word: word::Word = "tares".into();
//         bencher.iter(|| {
//             let mut entropy = 0f32;
//             for pattern in COLOR_PATTERNS {
//                 let mut count = 0;
//                 for word in words.iter() {
//                     if word::Word::matches(&my_word, &word, pattern) {
//                         count += 1;
//                     }
//                 }
//                 if count != 0 {
//                     // Can't take the log of 0, and this won't add to the entropy anyways
//                     let probability = count as f32 / num_words;
//                     entropy += probability * -probability.log2();
//                 }
//             }
//         });
//     }

//     /**
//      * In this version, we have the count as an f32, and we compare that to 0
//      */
//     #[bench]
//     fn floatcmp(bencher: &mut Bencher) {
//         let buf = fs::read_to_string("all_words.txt").expect("Could not read wordle_words.txt!");
//         let words = buf
//             .split(" ")
//             .map(|x| x.into())
//             .collect::<Vec<word::Word>>();
//         let num_words = words.len() as f32;
//         let my_word: word::Word = "tares".into();
//         bencher.iter(|| {
//             for pattern in COLOR_PATTERNS {
//                 let mut entropy = 0f32;
//                 let mut count = 0f32;
//                 for word in words.iter() {
//                     if word::Word::matches(&my_word, &word, pattern) {
//                         count += 1.0;
//                     }
//                 }
//                 if count != 0f32 {
//                     let probability = count / num_words;
//                     entropy += probability * -probability.log2();
//                 }
//             }
//         });
//     }
// }
