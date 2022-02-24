/**
 * For a word ([char; 5]), is it faster to convert the word to hashset
 * and then see if a character is in the set, or is it faster to just check every
 * character (only 5 of them . . . hmm)
 */

#[cfg(test)]
mod bench {
    /*
    WOW!
    test bench::bench::hash_contains        ... bench:          78 ns/iter (+/- 22)
    test bench::bench::hash_doesnt_contain  ... bench:          74 ns/iter (+/- 2)
    test bench::bench::naive_contains       ... bench:           0 ns/iter (+/- 0)
    test bench::bench::naive_doesnt_contain ... bench:           0 ns/iter (+/- 0)
    Even after increasing the word size to 100_000 chars,
    linear search is several (4 or 5 +) orders of magnitude faster
    Guess hashing is super slow
    */
    use std::collections::HashSet;
    use test::{black_box, Bencher};

    #[bench]
    fn hash_doesnt_contain(bencher: &mut Bencher) {
        let mut set = HashSet::<char>::new();
        set.reserve(5);
        let word = black_box(['b', 'c', 'd', 'e', 'f']);
        bencher.iter(|| {
            word.iter().for_each(|x| {
                set.insert(*x);
            });
            set.contains(&'a')
        });
        set.shrink_to(0);
    }

    #[bench]
    fn hash_contains(bencher: &mut Bencher) {
        let mut set = HashSet::<char>::new();
        set.reserve(5);
        let word = black_box(['a', 'b', 'c', 'd', 'e']);
        bencher.iter(|| {
            word.iter().for_each(|x| {
                set.insert(*x);
            });
            set.contains(&'a')
        });
        set.shrink_to(0);
    }

    #[bench]
    fn naive_contains(bencher: &mut Bencher) {
        let word = black_box(['a', 'b', 'c', 'd', 'e']);
        bencher.iter(|| word.iter().any(|x| *x == 'a'))
    }

    #[bench]
    fn naive_doesnt_contain(bencher: &mut Bencher) {
        let word = black_box(['b', 'c', 'd', 'e', 'f']);
        bencher.iter(|| word.iter().any(|x| *x == 'a'))
    }
}
