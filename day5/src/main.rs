use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};

fn parse_poly(s: &str) -> VecDeque<char> {
    let mut poly = VecDeque::new();
    for c in s.chars() {
        poly.push_back(c);
    }
    poly
}

fn process_poly(poly: &mut VecDeque<char>) -> usize {
    let mut i = 0;

    while i < poly.len() - 1 {
        let c = poly[i];
        let c1 = poly[i + 1];
        if (c.is_lowercase() && c.to_uppercase().next().unwrap() == c1)
            || (c.is_uppercase() && c.to_lowercase().next().unwrap() == c1)
        {
            poly.remove(i); // c
            poly.remove(i); // c1

            // The previous character could match it's new neighbor.
            if i > 0 {
                i -= 1;
            }
        } else {
            i += 1;
        }
    }
    poly.len()
}

fn read<R: Read>(io: R) -> Result<VecDeque<char>, Error> {
    let br = BufReader::new(io);
    let mut p = String::new();
    for line in br.lines() {
        let l = line?;
        p.push_str(&l);
    }
    Ok(parse_poly(&p))
}

fn main() -> Result<(), Error> {
    let mut input = read(File::open("input.txt")?)?;
    println!("Pt 1: {}", process_poly(&mut input));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_poly_test() {
        assert_eq!(
            VecDeque::from(vec!(
                'd', 'a', 'b', 'A', 'c', 'C', 'a', 'C', 'B', 'A', 'c', 'C', 'c', 'a', 'D', 'A'
            )),
            parse_poly("dabAcCaCBAcCcaDA")
        );
    }

    #[test]
    fn process_poly_test() {
        let mut poly = parse_poly("dabAcCaCBAcCcaDA");
        let l = process_poly(&mut poly);
        assert_eq!(10, l);
        assert_eq!(VecDeque::from(vec!('d', 'a', 'b', 'C', 'B', 'A', 'c', 'a', 'D', 'A')), poly);
    }
}
