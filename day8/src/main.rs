#[macro_use]
extern crate nom;

use nom::types::CompleteStr;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};

fn from_dec(input: CompleteStr) -> Result<usize, std::num::ParseIntError> {
    usize::from_str_radix(input.0, 10)
}

named!(parse_usize<CompleteStr, usize>,
       do_parse!(
           res: map_res!(nom::digit, from_dec) >>
           tag!(" ") >>
           (res)
       )
);

fn sum(v: &Vec<usize>) -> usize {
    v.iter().sum()
}

named!(parse_license<CompleteStr, usize>,
    dbg_dmp!(do_parse!(
        num_nodes: parse_usize >>
        num_meta: parse_usize >>
        nodes: count!(parse_license, num_nodes) >>
        metas: count!(parse_usize, num_meta) >>
        (sum(&metas) + sum(&nodes))
    ))
);

fn calc_node(nodes: &Vec<usize>, metas: &Vec<usize>) -> usize {
    if nodes.len() == 0 {
        sum(&metas)
    } else {
        let mut val = 0;
        for m in metas {
            if *m > 0 && *m <= nodes.len() {
                val += nodes[*m - 1];
            }
        }
        val
    }
}

named!(parse_license2<CompleteStr, usize>,
    dbg_dmp!(do_parse!(
        num_nodes: parse_usize >>
        num_meta: parse_usize >>
        nodes: count!(parse_license2, num_nodes) >>
        metas: count!(parse_usize, num_meta) >>
        (calc_node(&nodes,&metas))
    ))
);

fn read<R: Read>(io: R) -> Result<String, Error> {
    let br = BufReader::new(io);
    let mut p = String::new();
    for line in br.lines() {
        let l = line?;
        p.push_str(&l);
    }
    Ok(p)
}

fn main() -> Result<(), Error> {
    let input = read(File::open("input.txt")?)? + " ";
    println!("Pt 1 answer: {}", parse_license(CompleteStr(&input)).unwrap().1);
    println!("Pt 2 answer: {}", parse_license2(CompleteStr(&input)).unwrap().1);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_licence_test() {
        assert_eq!(
            138,
            parse_license(CompleteStr("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2 ")).unwrap().1
        );
    }

    #[test]
    fn parse_licence2_test() {
        assert_eq!(
            66,
            parse_license2(CompleteStr("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2 ")).unwrap().1
        );
    }
}
