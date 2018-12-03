#[macro_use]
extern crate text_io;

use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq)]
struct Rectangle {
    x: i64,
    y: i64,
    x1: i64,
    y1: i64,
}

impl Rectangle {
    pub fn new(x: i64, y: i64, w: i64, h: i64) -> Rectangle {
        Rectangle { x: x, y: y, x1: x + w, y1: y + h }
    }
    pub fn w(&self) -> i64 {
        self.x1 - self.x
    }

    pub fn h(&self) -> i64 {
        self.y1 - self.y
    }

    pub fn intersect(&self, other: &Rectangle) -> Option<Rectangle> {
        if self.x >= other.x1 || self.y >= other.y1 || other.x >= self.x1 || other.y >= self.y1 {
            return None;
        }

        Some(Rectangle {
            x: cmp::max(self.x, other.x),
            y: cmp::max(self.y, other.y),
            x1: cmp::min(self.y1, other.y1),
            y1: cmp::min(self.y1, other.y1),
        })
    }
}

#[derive(Debug, PartialEq)]
struct Claim {
    id: i64,
    rect: Rectangle,
}

fn num_overlaps(claims: &Vec<Claim>) -> usize {
    let mut square_claims = HashMap::new();
    for claim in claims {
        for x in claim.rect.x..claim.rect.x1 {
            for y in claim.rect.y..claim.rect.y1 {
                let pt = Point { x: x, y: y };
                let count = match square_claims.get(&pt) {
                    Some(n) => *n,
                    None => 0,
                };

                square_claims.insert(pt, count + 1);
            }
        }
    }

    let mut overlaps = 0;
    for (_, count) in square_claims {
        if count > 1 {
            overlaps += 1;
        }
    }
    overlaps
}

fn parse_claim(s: &str) -> Result<Claim, text_io::Error> {
    let id: i64;
    let x: i64;
    let y: i64;
    let w: i64;
    let h: i64;
    try_scan!(s.bytes() => "#{} @ {},{}: {}x{}", id, x, y, w, h);
    Ok(Claim { id: id, rect: Rectangle::new(x, y, w, h) })
}

fn read<R: Read>(io: R) -> Result<Vec<Claim>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| {
            line.and_then(|s| parse_claim(&s).map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        }).collect()
}

fn main() -> Result<(), Error> {
    let input = read(File::open("input.txt")?)?;
    println!("Pt 1 answer: {}", num_overlaps(&input));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn intersect_test() {
        assert_eq!(None, Rectangle::new(1, 1, 1, 1).intersect(&Rectangle::new(2, 1, 1, 1)));
        assert_eq!(None, Rectangle::new(2, 1, 1, 1).intersect(&Rectangle::new(1, 1, 1, 1)));
        assert_eq!(None, Rectangle::new(1, 2, 1, 1).intersect(&Rectangle::new(1, 1, 1, 1)));
        assert_eq!(None, Rectangle::new(1, 1, 1, 1).intersect(&Rectangle::new(1, 2, 1, 1)));
        assert_eq!(None, Rectangle::new(2, 2, 1, 1).intersect(&Rectangle::new(1, 1, 1, 1)));
        assert_eq!(None, Rectangle::new(1, 1, 1, 1).intersect(&Rectangle::new(2, 2, 1, 1)));

        assert_eq!(
            Some(Rectangle::new(2, 2, 1, 1)),
            Rectangle::new(1, 1, 5, 5).intersect(&Rectangle::new(2, 2, 1, 1))
        );
    }

    #[test]
    fn parse_claim_test() {
        assert_eq!(
            Claim { id: 0, rect: Rectangle::new(0, 0, 0, 0) },
            parse_claim("#0 @ 0,0: 0x0").unwrap()
        );

        assert_eq!(
            Claim { id: 1, rect: Rectangle::new(1, 3, 4, 4) },
            parse_claim("#1 @ 1,3: 4x4").unwrap()
        );
    }

    #[test]
    fn overlaps_test() {
        let claims = vec![
            parse_claim("#1 @ 1,3: 4x4").unwrap(),
            parse_claim("#2 @ 3,1: 4x4").unwrap(),
            parse_claim("#3 @ 5,5: 2x2").unwrap(),
        ];
        assert_eq!(4, num_overlaps(&claims));
    }
}
