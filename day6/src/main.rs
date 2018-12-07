extern crate regex;
#[macro_use]
extern crate more_asserts;

use regex::Regex;
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Read};

// Big Assumption that I can't prove:
//   Once areas reach the edges of the bounding box of the coordinates,
//   those areas are infinite.

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Pt {
    x: i64,
    y: i64,
}

impl Pt {
    pub fn dist(&self, other: &Pt) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Rect {
    p0: Pt,
    p1: Pt,
}

fn parse_coord(s: &str) -> Result<Pt, Box<Error>> {
    let re = Regex::new(r"^(\d+), (\d+)$").unwrap();

    let caps = match re.captures(s) {
        Some(c) => c,
        None => {
            return Err(io::Error::new(
                ErrorKind::InvalidInput,
                format!("Unrecognized record \"{}\"", s),
            )
            .into())
        }
    };

    Ok(Pt { x: caps.get(1).unwrap().as_str().parse()?, y: caps.get(2).unwrap().as_str().parse()? })
}

fn bounding_box(coords: &Vec<Pt>) -> Rect {
    let mut bounds = Rect { p0: coords[0].clone(), p1: coords[0].clone() };

    for c in &coords[1..] {
        bounds.p0.x = cmp::min(c.x, bounds.p0.x);
        bounds.p0.y = cmp::min(c.y, bounds.p0.y);
        bounds.p1.x = cmp::max(c.x, bounds.p1.x);
        bounds.p1.y = cmp::max(c.y, bounds.p1.y);
    }

    bounds
}

fn calc_owner_of_point(pt: &Pt, coords: &Vec<Pt>) -> Option<Pt> {
    let mut min_dist = pt.dist(&coords[0]);
    let mut owner = Some(coords[0].clone());

    for c in &coords[1..] {
        let dist = c.dist(&pt);
        if dist < min_dist {
            owner = Some(c.clone());
            min_dist = dist;
        } else if dist == min_dist {
            owner = None;
        }
    }

    owner
}

fn calc_infinite_owners(coords: &Vec<Pt>, bounds: &Rect) -> HashSet<Pt> {
    let mut infinite_coords = HashSet::new();

    // First find all coords with areas at the edges of the bounding box
    // and discard them.
    for x in bounds.p0.x..=bounds.p1.x {
        if let Some(owner) = calc_owner_of_point(&Pt { x: x, y: bounds.p0.y }, &coords) {
            infinite_coords.insert(owner);
        }
        if let Some(owner) = calc_owner_of_point(&Pt { x: x, y: bounds.p1.y }, &coords) {
            infinite_coords.insert(owner);
        }
    }
    for y in bounds.p0.y + 1..bounds.p1.y {
        if let Some(owner) = calc_owner_of_point(&Pt { x: bounds.p0.x, y: y }, &coords) {
            infinite_coords.insert(owner);
        }
        if let Some(owner) = calc_owner_of_point(&Pt { x: bounds.p1.x, y: y }, &coords) {
            infinite_coords.insert(owner);
        }
    }

    infinite_coords
}

fn calc_finite_areas(coords: &Vec<Pt>) -> HashMap<Pt, i64> {
    let bounds = bounding_box(&coords);
    let infinite_owners = calc_infinite_owners(&coords, &bounds);

    let mut areas = HashMap::new();
    for x in bounds.p0.x + 1..bounds.p1.x {
        for y in bounds.p0.y + 1..bounds.p1.y {
            let pt = Pt { x: x, y: y };
            if let Some(owner) = calc_owner_of_point(&pt, &coords) {
                if !infinite_owners.contains(&owner) {
                    let area = match areas.get(&owner) {
                        Some(n) => *n,
                        None => 0,
                    };
                    areas.insert(owner, area + 1);
                }
            }
        }
    }

    areas
}

fn calc_largest_finite_area(coords: &Vec<Pt>) -> (Pt, i64) {
    let finite_areas = calc_finite_areas(&coords);

    let mut max_area = 0;
    let mut max_pt = None;
    for (pt, area) in finite_areas {
        if area > max_area {
            max_area = area;
            max_pt = Some(pt);
        }
    }

    (max_pt.unwrap(), max_area)
}

fn calc_total_distance(pt: &Pt, coords: &Vec<Pt>) -> i64 {
    let mut total = 0;
    for c in coords {
        total += c.dist(&pt);
    }
    total
}

fn verify_area_contained_in_bounding_box(points: &Vec<Pt>, limit: i64) {
    let bounds = bounding_box(&points);
    for x in bounds.p0.x..=bounds.p1.x {
        assert_le!(limit, calc_total_distance(&Pt { x: x, y: bounds.p0.y }, &points));
        assert_le!(limit, calc_total_distance(&Pt { x: x, y: bounds.p1.y }, &points));
    }
    for y in bounds.p0.y + 1..bounds.p1.y {
        assert_le!(limit, calc_total_distance(&Pt { x: bounds.p0.x, y: y }, &points));
        assert_le!(limit, calc_total_distance(&Pt { x: bounds.p1.x, y: y }, &points));
    }
}

fn calc_area_size(points: &Vec<Pt>, limit: i64) -> i64 {
    let bounds = bounding_box(&points);
    let mut size = 0;

    for x in bounds.p0.x + 1..bounds.p1.x {
        for y in bounds.p0.y + 1..bounds.p1.y {
            let pt = Pt { x: x, y: y };
            let dist = calc_total_distance(&pt, &points);
            if dist < limit {
                size += 1;
            }
        }
    }
    size
}

fn read<R: Read>(io: R) -> Result<Vec<Pt>, Box<Error>> {
    let br = BufReader::new(io);
    let mut points = Vec::new();
    for line in br.lines() {
        points.push(parse_coord(&line?)?);
    }

    Ok(points)
}

fn main() -> Result<(), Box<Error>> {
    let input = read(File::open("input.txt")?)?;
    let limit = 10000;
    println!("Pt 1 answer: {:?}", calc_largest_finite_area(&input));
    verify_area_contained_in_bounding_box(&input, limit);
    println!("Pt 2 answer: {:?}", calc_area_size(&input, limit));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_points() -> Vec<Pt> {
        vec![
            Pt { x: 1, y: 1 }, // A
            Pt { x: 1, y: 6 }, // B
            Pt { x: 8, y: 3 }, // C
            Pt { x: 3, y: 4 }, // D
            Pt { x: 5, y: 5 }, // E
            Pt { x: 8, y: 9 }, // F
        ]
    }

    #[test]
    fn parse_coord_test() {
        assert_eq!(Pt { x: 1, y: 1 }, parse_coord("1, 1").unwrap());
        assert_eq!(Pt { x: 1, y: 6 }, parse_coord("1, 6").unwrap());
        assert_eq!(Pt { x: 8, y: 3 }, parse_coord("8, 3").unwrap());
        assert_eq!(Pt { x: 3, y: 4 }, parse_coord("3, 4").unwrap());
        assert_eq!(Pt { x: 5, y: 5 }, parse_coord("5, 5").unwrap());
        assert_eq!(Pt { x: 8, y: 9 }, parse_coord("8, 9").unwrap());
    }

    #[test]
    fn bounding_box_test() {
        let points = get_points();
        assert_eq!(Rect { p0: Pt { x: 1, y: 1 }, p1: Pt { x: 8, y: 9 } }, bounding_box(&points));
    }

    #[test]
    fn dist() {
        assert_eq!(2, Pt { x: 1, y: 1 }.dist(&Pt { x: 2, y: 2 }));
        assert_eq!(2, Pt { x: 2, y: 2 }.dist(&Pt { x: 1, y: 1 }));
    }

    #[test]
    fn calc_owner_of_point_test() {
        let points = get_points();
        assert_eq!(None, calc_owner_of_point(&Pt { x: 5, y: 0 }, &points));
        assert_eq!(Some(Pt { x: 8, y: 3 }), calc_owner_of_point(&Pt { x: 6, y: 0 }, &points));
        assert_eq!(Some(Pt { x: 1, y: 1 }), calc_owner_of_point(&Pt { x: 4, y: 0 }, &points));
    }

    #[test]
    fn calc_infinite_owners_test() {
        let points = get_points();
        let bounds = bounding_box(&points);
        let owners = calc_infinite_owners(&points, &bounds);

        assert_eq!(4, owners.len());
        assert_eq!(true, owners.contains(&Pt { x: 1, y: 1 }));
        assert_eq!(true, owners.contains(&Pt { x: 1, y: 6 }));
        assert_eq!(true, owners.contains(&Pt { x: 8, y: 3 }));
        assert_eq!(true, owners.contains(&Pt { x: 8, y: 9 }));
    }

    #[test]
    fn calc_finite_areas_test() {
        let points = get_points();
        let areas = calc_finite_areas(&points);

        assert_eq!(2, areas.len());
        assert_eq!(9, *areas.get(&Pt { x: 3, y: 4 }).unwrap());
        assert_eq!(17, *areas.get(&Pt { x: 5, y: 5 }).unwrap());
    }

    #[test]
    fn calc_largest_finite_area_test() {
        let points = get_points();
        assert_eq!((Pt { x: 5, y: 5 }, 17), calc_largest_finite_area(&points));
    }

    #[test]
    fn calc_total_distance_test() {
        let points = get_points();
        assert_eq!(30, calc_total_distance(&Pt { x: 4, y: 3 }, &points));
    }

    #[test]
    fn calc_area_size_test() {
        let points = get_points();
        assert_eq!(16, calc_area_size(&points, 32));
    }

}
