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

fn read<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);
    br.lines().collect()
}

fn main() -> Result<(), Error> {
    let lines = read(File::open("input.txt")?)?;
    println!("Pt 1 answer: {} ", checksum(&lines));

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

        let strings = [
            "abcdef",
            "bababc",
            "abbcde",
            "abcccd",
            "aabcdd",
            "abcdee",
            "ababab",
        ].iter().map(|s| s.to_string()).collect();
        assert_eq!(12, checksum(&strings));
    }
}
