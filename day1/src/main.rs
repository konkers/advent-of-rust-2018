use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn calc_shift(start: i64, shifts: &[i64]) -> i64 {
    let mut val = start;

    for shift in shifts {
        val += shift;
    }

    val
}

fn find_dup_freq(start: i64, shifts: &[i64]) -> i64 {
    let mut val = start;
    let mut seen = HashSet::new();

    seen.insert(val);

    'outer: loop {
        for shift in shifts {
            val += shift;
            if seen.insert(val) == false {
                break 'outer;
            }
        }
    }

    val
}

fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn main() -> Result<(), Error> {
    let shifts = read(File::open("input.txt")?)?;
    println!("Pt 1 answer: {}", calc_shift(0, &shifts));
    println!("Pt 2 answer: {}", find_dup_freq(0, &shifts));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example_test() {
        assert_eq!(3, calc_shift(0, &[1, 1, 1]));
        assert_eq!(0, calc_shift(0, &[1, 1, -2]));
        assert_eq!(-6, calc_shift(0, &[-1, -2, -3]));
    }

    #[test]
    fn part2_example_test() {
        assert_eq!(0, find_dup_freq(0, &[1, -1]));
        assert_eq!(10, find_dup_freq(0, &[3, 3, 4, -2, -4]));
        assert_eq!(5, find_dup_freq(0, &[-6, 3, 8, 5, -6]));
        assert_eq!(14, find_dup_freq(0, &[7, 7, -2, -7, -4]));
    }
}
