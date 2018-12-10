extern crate regex;

use regex::Regex;
use std::cmp;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Read};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Pt {
    x: i64,
    y: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Light {
    pos: Pt,
    velocity: Pt,
}

fn parse_reading(s: &str) -> Result<Light, Box<Error>> {
    let re = Regex::new(r"position=< *(-?\d+), *(-?\d+)> velocity=< *(-?\d+), *(-?\d+)>").unwrap();

    let caps = match re.captures(s) {
        Some(c) => c,
        None => {
            return Err(io::Error::new(
                ErrorKind::InvalidInput,
                format!("Unrecognized Reading \"{}\"", s),
            )
            .into())
        }
    };

    Ok(Light {
        pos: Pt {
            x: caps.get(1).unwrap().as_str().parse()?,
            y: caps.get(2).unwrap().as_str().parse()?,
        },
        velocity: Pt {
            x: caps.get(3).unwrap().as_str().parse()?,
            y: caps.get(4).unwrap().as_str().parse()?,
        },
    })
}

fn advance_lights(lights: &mut Vec<Light>) {
    for l in lights {
        l.pos.x += l.velocity.x;
        l.pos.y += l.velocity.y;
    }
}

fn calc_bounding_box(lights: &Vec<Light>) -> (Pt, Pt) {
    let mut min = Pt { x: std::i64::MAX, y: std::i64::MAX };
    let mut max = Pt { x: std::i64::MIN, y: std::i64::MIN };

    for l in lights {
        min.x = cmp::min(min.x, l.pos.x);
        min.y = cmp::min(min.y, l.pos.y);
        max.x = cmp::max(max.x, l.pos.x);
        max.y = cmp::max(max.y, l.pos.y);
    }
    (min, max)
}

// Here we assume the the message will appear in the frame with the a
// bounding box of the smallest area.
fn calc_min_frame(lights: &Vec<Light>) -> i64 {
    let mut frame = 0;
    let mut prev_area = std::i64::MAX;
    let mut mlights = lights.clone();

    loop {
        let (min, max) = calc_bounding_box(&mlights);
        let area = (max.x - min.x) * (max.y - min.y);
        if area > prev_area {
            return frame - 1;
        }
        advance_lights(&mut mlights);
        prev_area = area;
        frame += 1;
    }
}

fn render(lights: &Vec<Light>) -> String {
    let (min, max) = calc_bounding_box(&lights);
    let h = (max.y - min.y + 1) as usize;
    let w = (max.x - min.x + 1) as usize;
    let mut buf = Vec::with_capacity(h * w);

    for _ in 0..w {
        for _ in 0..h {
            buf.push('.');
        }
    }

    for l in lights {
        let x = (l.pos.x - min.x) as usize;
        let y = (l.pos.y - min.y) as usize;
        buf[y * w + x] = '#';
    }

    let mut res = String::new();
    for y in 0..h {
        for x in 0..w {
            res.push(buf[y * w + x]);
        }
        res.push('\n');
    }
    res
}

fn read<R: Read>(io: R) -> Result<Vec<Light>, Box<Error>> {
    let br = BufReader::new(io);
    let mut lights = Vec::new();
    for line in br.lines() {
        lights.push(parse_reading(&line?)?);
    }

    Ok(lights)
}

fn main() -> Result<(), Box<Error>> {
    let mut lights = read(File::open("input.txt")?)?;
    let iterations = calc_min_frame(&lights);
    for _ in 0..iterations {
        advance_lights(&mut lights);
    }
    println!("{}", render(&lights));
    println!("Seconds: {}", iterations);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_readings() -> Vec<Light> {
        vec![
            parse_reading("position=< 9,  1> velocity=< 0,  2>").unwrap(),
            parse_reading("position=< 7,  0> velocity=<-1,  0>").unwrap(),
            parse_reading("position=< 3, -2> velocity=<-1,  1>").unwrap(),
            parse_reading("position=< 6, 10> velocity=<-2, -1>").unwrap(),
            parse_reading("position=< 2, -4> velocity=< 2,  2>").unwrap(),
            parse_reading("position=<-6, 10> velocity=< 2, -2>").unwrap(),
            parse_reading("position=< 1,  8> velocity=< 1, -1>").unwrap(),
            parse_reading("position=< 1,  7> velocity=< 1,  0>").unwrap(),
            parse_reading("position=<-3, 11> velocity=< 1, -2>").unwrap(),
            parse_reading("position=< 7,  6> velocity=<-1, -1>").unwrap(),
            parse_reading("position=<-2,  3> velocity=< 1,  0>").unwrap(),
            parse_reading("position=<-4,  3> velocity=< 2,  0>").unwrap(),
            parse_reading("position=<10, -3> velocity=<-1,  1>").unwrap(),
            parse_reading("position=< 5, 11> velocity=< 1, -2>").unwrap(),
            parse_reading("position=< 4,  7> velocity=< 0, -1>").unwrap(),
            parse_reading("position=< 8, -2> velocity=< 0,  1>").unwrap(),
            parse_reading("position=<15,  0> velocity=<-2,  0>").unwrap(),
            parse_reading("position=< 1,  6> velocity=< 1,  0>").unwrap(),
            parse_reading("position=< 8,  9> velocity=< 0, -1>").unwrap(),
            parse_reading("position=< 3,  3> velocity=<-1,  1>").unwrap(),
            parse_reading("position=< 0,  5> velocity=< 0, -1>").unwrap(),
            parse_reading("position=<-2,  2> velocity=< 2,  0>").unwrap(),
            parse_reading("position=< 5, -2> velocity=< 1,  2>").unwrap(),
            parse_reading("position=< 1,  4> velocity=< 2,  1>").unwrap(),
            parse_reading("position=<-2,  7> velocity=< 2, -2>").unwrap(),
            parse_reading("position=< 3,  6> velocity=<-1, -1>").unwrap(),
            parse_reading("position=< 5,  0> velocity=< 1,  0>").unwrap(),
            parse_reading("position=<-6,  0> velocity=< 2,  0>").unwrap(),
            parse_reading("position=< 5,  9> velocity=< 1, -2>").unwrap(),
            parse_reading("position=<14,  7> velocity=<-2,  0>").unwrap(),
            parse_reading("position=<-3,  6> velocity=< 2, -1>").unwrap(),
        ]
    }
    #[test]
    fn parse_reading_test() {
        assert_eq!(
            Light { pos: Pt { x: 9, y: 1 }, velocity: Pt { x: 0, y: 2 } },
            parse_reading("position=< 9,  1> velocity=< 0,  2>").unwrap()
        );
    }

    #[test]
    fn calc_min_frame_test() {
        let lights = get_readings();
        assert_eq!(3, calc_min_frame(&lights));
    }

    #[test]
    fn render_test() {
        let mut lights = get_readings();
        advance_lights(&mut lights);
        advance_lights(&mut lights);
        advance_lights(&mut lights);
        let expected = "#...#..###\n".to_string()
            + "#...#...#.\n"
            + "#...#...#.\n"
            + "#####...#.\n"
            + "#...#...#.\n"
            + "#...#...#.\n"
            + "#...#...#.\n"
            + "#...#..###\n";

        assert_eq!(expected, render(&lights));
    }
}
