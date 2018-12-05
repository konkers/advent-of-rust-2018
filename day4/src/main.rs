extern crate chrono;
extern crate regex;
#[macro_use]
extern crate text_io;

use chrono::{NaiveDateTime, Timelike};
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

type DateTime = NaiveDateTime;

#[derive(Debug, PartialEq, Eq)]
enum Action {
    BeginShift { id: i64 },
    Asleep,
    Awake,
}

#[derive(Debug, Eq)]
struct Record {
    time: DateTime,
    action: Action,
}

impl Ord for Record {
    fn cmp(&self, other: &Record) -> Ordering {
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Record) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Record {
    fn eq(&self, other: &Record) -> bool {
        self.time.eq(&other.time)
    }
}

#[derive(Debug)]
struct Span {
    id: i64,
    start: DateTime,
    end: DateTime,
}

fn parse_time(s: &str) -> Result<DateTime, Error> {
    let dt = DateTime::parse_from_str(&s, "%Y-%m-%d %H:%M");
    dt.map_err(|e| Error::new(ErrorKind::InvalidData, e))
}

// We have to break this out because the text_io macros
// back in either .unwrap() or ?.
fn parse_begin(s: &str) -> Result<Action, text_io::Error> {
    let id: i64;
    try_scan!(s.bytes() => "Guard #{} begins shift", id);
    Ok(Action::BeginShift { id })
}

fn parse_action(s: &str) -> Result<Action, Error> {
    if let Ok(action) = parse_begin(&s) {
        return Ok(action);
    }

    match s {
        "falls asleep" => Ok(Action::Asleep),
        "wakes up" => Ok(Action::Awake),
        _ => Err(Error::new(ErrorKind::InvalidInput, format!("Unrecognized action \"{}\"", s))),
    }
}

fn parse_record(s: &str) -> Result<Record, Error> {
    let re = Regex::new(r"^\[(.*)\] (.*)$").unwrap();
    let caps = match re.captures(s) {
        Some(c) => c,
        None => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Unrecognized record \"{}\"", s),
            ))
        }
    };

    let time = parse_time(&caps.get(1).unwrap().as_str())?;
    let action = parse_action(&caps.get(2).unwrap().as_str())?;
    Ok(Record { time: time, action: action })
}

fn calc_spans(recs: &mut Vec<Record>) -> Vec<Span> {
    let mut spans = Vec::new();

    let mut id = None;
    let mut start = None;

    recs.sort();
    for rec in recs {
        let time = rec.time;
        match rec.action {
            Action::BeginShift { id: a } => id = Some(a),
            Action::Asleep => start = Some(time),
            Action::Awake => {
                if let (Some(i), Some(s)) = (id, start) {
                    spans.push(Span { id: i, start: s, end: time });
                }
            }
        }
    }

    spans
}

fn calc_sleep_min(spans: &Vec<Span>) -> HashMap<i64, i64> {
    let mut totals = HashMap::new();

    for span in spans {
        let dur = span.end.signed_duration_since(span.start);
        let total = match totals.get(&span.id) {
            Some(n) => *n,
            None => 0,
        };

        totals.insert(span.id, total + dur.num_minutes());
    }

    totals
}

fn get_sleep_histograms(spans: &Vec<Span>) -> HashMap<i64, [i64; 60]> {
    let mut hists = HashMap::new();

    for s in spans {
        if !hists.contains_key(&s.id) {
            let h = [0; 60];
            hists.insert(s.id, h);
        }
        let mut hist = hists.get_mut(&s.id).unwrap();

        for m in s.start.minute()..s.end.minute() {
            hist[m as usize] += 1;
        }
    }

    hists
}

fn find_most_slept_min(recs: &mut Vec<Record>) -> (i64, i64) {
    let spans = calc_spans(recs);
    let totals = calc_sleep_min(&spans);
    let mut sleepiest = None;
    let mut slept_max = 0;
    for (id, slept) in totals {
        if slept > slept_max {
            sleepiest = Some(id);
            slept_max = slept;
        }
    }

    let id = sleepiest.unwrap();

    let hists = get_sleep_histograms(&spans);
    let hist = hists[&id];

    let mut max_min = 0;
    let mut max_overlap = 0;
    for m in 0..hist.len() {
        let overlap = hist[m];

        if overlap > max_overlap {
            max_min = m;
            max_overlap = overlap
        }
    }

    (id, max_min as i64)
}

// Naming Fail
fn find_most_slept_min_pt2(recs: &mut Vec<Record>) -> (i64, i64) {
    let spans = calc_spans(recs);
    let hists = get_sleep_histograms(&spans);

    let mut sleepiest_id = None;
    let mut max_min = 0;
    let mut max_overlap = 0;
    for (id, hist) in hists {
        for m in 0..hist.len() {
            let overlap = hist[m];

            if overlap > max_overlap {
                max_min = m;
                max_overlap = overlap;
                sleepiest_id = Some(id);
            }
        }
    }


    (sleepiest_id.unwrap(), max_min as i64)
}


fn read<R: Read>(io: R) -> Result<Vec<Record>, Error> {
    let br = BufReader::new(io);
    br.lines().map(|line| line.and_then(|s| parse_record(&s))).collect()
}

fn main() -> Result<(), Error> {
    let mut input = read(File::open("input.txt")?)?;
    let res = find_most_slept_min(&mut input);
    println!("Pt 1 answer: {}", res.0 * res.1);
    let res2 = find_most_slept_min_pt2(&mut input);
    println!("Pt 2 answer: {}", res2.0 * res2.1);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_time_test() {
        assert_eq!(
            chrono::NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 0, 0),
            parse_time("1518-11-01 00:00").unwrap()
        );
        assert_eq!(
            chrono::NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 5, 0),
            parse_time("1518-11-01 00:05").unwrap()
        );
        assert_eq!(
            chrono::NaiveDate::from_ymd(1518, 11, 1).and_hms(23, 58, 0),
            parse_time("1518-11-01 23:58").unwrap()
        );
    }

    #[test]
    fn parse_action_test() {
        assert_eq!(Action::BeginShift { id: 10 }, parse_action("Guard #10 begins shift").unwrap());
        assert_eq!(Action::Asleep, parse_action("falls asleep").unwrap());
        assert_eq!(Action::Awake, parse_action("wakes up").unwrap());
    }

    #[test]
    fn parse_record_test() {
        assert_eq!(
            Record {
                time: chrono::NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 0, 0),
                action: Action::BeginShift { id: 10 }
            },
            parse_record("[1518-11-01 00:00] Guard #10 begins shift").unwrap()
        );
    }

    fn get_recs() -> Vec<Record> {
        vec![
            parse_record("[1518-11-05 00:55] wakes up").unwrap(),
            parse_record("[1518-11-01 00:00] Guard #10 begins shift").unwrap(),
            parse_record("[1518-11-01 00:05] falls asleep").unwrap(),
            parse_record("[1518-11-01 00:25] wakes up").unwrap(),
            parse_record("[1518-11-01 00:30] falls asleep").unwrap(),
            parse_record("[1518-11-01 00:55] wakes up").unwrap(),
            parse_record("[1518-11-01 23:58] Guard #99 begins shift").unwrap(),
            parse_record("[1518-11-02 00:40] falls asleep").unwrap(),
            parse_record("[1518-11-02 00:50] wakes up").unwrap(),
            parse_record("[1518-11-03 00:05] Guard #10 begins shift").unwrap(),
            parse_record("[1518-11-03 00:24] falls asleep").unwrap(),
            parse_record("[1518-11-03 00:29] wakes up").unwrap(),
            parse_record("[1518-11-04 00:02] Guard #99 begins shift").unwrap(),
            parse_record("[1518-11-04 00:36] falls asleep").unwrap(),
            parse_record("[1518-11-04 00:46] wakes up").unwrap(),
            parse_record("[1518-11-05 00:03] Guard #99 begins shift").unwrap(),
            parse_record("[1518-11-05 00:45] falls asleep").unwrap(),
        ]
    }

    #[test]
    fn calc_sleep_mins_test() {
        let mut recs = get_recs();
        let spans = calc_spans(&mut recs);
        let totals = calc_sleep_min(&spans);
        assert_eq!(50, *totals.get(&10).unwrap());
        assert_eq!(30, *totals.get(&99).unwrap());
    }

    #[test]
    fn find_most_slept_min_test() {
        let mut recs = get_recs();
        assert_eq!((10, 24), find_most_slept_min(&mut recs));
    }

    #[test]
    fn find_most_slept_min_pt_2test() {
        let mut recs = get_recs();
        assert_eq!((99, 45), find_most_slept_min_pt2(&mut recs));
    }
}
