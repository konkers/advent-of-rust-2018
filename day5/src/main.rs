use std::collections::HashSet;
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

fn get_unique_units(poly: &VecDeque<char>) -> HashSet<char> {
    let mut units = HashSet::new();

    for c in poly {
        let lower = c.to_lowercase().next().unwrap();
        units.insert(lower);
    }

    units
}

fn remove_unit(poly: &VecDeque<char>, r: char) -> VecDeque<char> {
    let lower = r.to_lowercase().next().unwrap();
    let upper = r.to_uppercase().next().unwrap();

    poly.iter().cloned().filter(|c| *c != lower && *c != upper).collect()
}

fn find_best_removal(poly: &VecDeque<char>) -> (char, usize) {
    let units = get_unique_units(&poly);

    let mut best_unit = None;
    let mut best_len = poly.len();
    for c in units {
        let mut new_poly = remove_unit(&poly, c);
        process_poly(&mut new_poly);
        if new_poly.len() < best_len {
            best_unit = Some(c);
            best_len = new_poly.len();
        }
    }

    (best_unit.unwrap(), best_len)
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
    let input2 = input.clone();
    println!("Pt 1: {}", process_poly(&mut input));
    let (unit, len) = find_best_removal(&input2);
    println!("Pt 2: {}, {}", unit, len);
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

    #[test]
    fn get_unique_units_test() {
        let poly = parse_poly("dabAcCaCBAcCcaDA");
        let u = get_unique_units(&poly);
        let mut sorted: Vec<char> = u.iter().cloned().collect();
        sorted.sort();

        assert_eq!(vec!('a', 'b', 'c', 'd'), sorted);
    }

    #[test]
    fn remove_unit_test() {
        let poly = parse_poly("dabAcCaCBAcCcaDA");

        assert_eq!(parse_poly("dbcCCBcCcD"), remove_unit(&poly, 'a'));
        assert_eq!(parse_poly("daAcCaCAcCcaDA"), remove_unit(&poly, 'b'));
        assert_eq!(parse_poly("dabAaBAaDA"), remove_unit(&poly, 'c'));
        assert_eq!(parse_poly("abAcCaCBAcCcaA"), remove_unit(&poly, 'd'));
    }

    #[test]
    fn find_best_removal_test() {
        let poly = parse_poly("dabAcCaCBAcCcaDA");
        assert_eq!(('c', 4), find_best_removal(&poly));
    }
}
