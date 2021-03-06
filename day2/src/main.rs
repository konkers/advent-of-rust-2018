use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};

fn analyze_id(id: &str) -> (bool, bool) {
    let mut counts: HashMap<char, i64> = HashMap::new();

    for c in id.chars() {
        let count = match counts.get(&c) {
            Some(n) => *n,
            None => 0,
        };

        counts.insert(c, count + 1);
    }

    let mut has_two = false;
    let mut has_three = false;

    for (_, n) in &counts {
        if *n == 2 {
            has_two = true;
        }
        if *n == 3 {
            has_three = true;
        }
    }

    (has_two, has_three)
}

fn checksum(ids: &Vec<String>) -> i64 {
    let mut twos = 0;
    let mut threes = 0;

    for id in ids {
        let res = analyze_id(&id);
        if res.0 {
            twos += 1;
        }
        if res.1 {
            threes += 1;
        }
    }

    twos * threes
}

fn find_common(a: &str, b: &str) -> Option<String> {
    if a.len() != b.len() {
        return None;
    }

    let mut deviations = 0;
    let mut common = String::new();
    for i in a.chars().zip(b.chars()) {
        if i.0 == i.1 {
            common.push(i.0);
        } else {
            deviations += 1;
            if deviations > 1 {
                return None;
            }
        }
    }

    if deviations == 1 {
        Some(common)
    } else {
        None
    }
}

fn find_common_in_list(ids: &Vec<String>) -> Option<String> {
    // We're assuming there are only two matching box IDs as inferred
    // by the question "What letters are common between the two correct
    // box IDs?"

    // We're using an O(n^2) algorithm.  There is probably something fancy
    // we can do with hashing/caching.  Since n == 250 in the input, let's
    // stay simple.
    for i in 0..(ids.len() - 1) {
        for j in (i + 1)..ids.len() {
            if let Some(s) = find_common(&ids[i], &ids[j]) {
                return Some(s);
            }
        }
    }

    None
}

fn read<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);
    br.lines().collect()
}

fn main() -> Result<(), Error> {
    let lines = read(File::open("input.txt")?)?;
    println!("Pt 1 answer: {}", checksum(&lines));
    println!("Pt 2 answer: {}", find_common_in_list(&lines).unwrap());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example_test() {
        assert_eq!((false, false), analyze_id("abcdef"));
        assert_eq!((true, true), analyze_id("bababc"));
        assert_eq!((true, false), analyze_id("abbcde"));
        assert_eq!((false, true), analyze_id("abcccd"));
        assert_eq!((true, false), analyze_id("aabcdd"));
        assert_eq!((true, false), analyze_id("abcdee"));
        assert_eq!((false, true), analyze_id("ababab"));

        let strings = ["abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(12, checksum(&strings));
    }

    #[test]
    fn part2_example_test() {
        assert_eq!(None, find_common("a", "ab"));
        assert_eq!(None, find_common("abcde", "axcye"));
        assert_eq!(Some("fgij".to_string()), find_common("fghij", "fguij"));

        let strings = ["abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        assert_eq!(Some("fgij".to_string()), find_common_in_list(&strings));
    }
}
