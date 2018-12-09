#[macro_use]
extern crate intrusive_collections;

use intrusive_collections::{LinkedList, LinkedListLink};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn do_game(num_marbles: usize, players: usize) -> usize {
    let mut scores = vec![0; players];
    let mut circle = vec![0, 1];
    let mut cur = 1;
    let mut player = 0;

    for m in 2..=num_marbles {
        if (m % 23) == 0 {
            scores[player] += m;
            for i in 0..7 {
                if cur == 0 {
                    cur = circle.len() - 1;
                } else {
                    cur -= 1;
                }
            }
            scores[player] += circle[cur];
            circle.remove(cur);
        } else {
            cur = (cur + 1) % (circle.len()) + 1;
            circle.insert(cur, m);
        }
        player = (player + 1) % players;
    }

   // println!("{:?}", circle);
    *scores.iter().max().unwrap()
}

fn main() -> Result<(), Box<Error>> {

    println!("Pt 1: {:?}", do_game(71944, 423));
    println!("Pt 2: {:?}", do_game(71944 * 100, 423));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn do_game_test() {
        //10 players; last marble is worth 1618 points: high score is 8317
//13 players; last marble is worth 7999 points: high score is 146373
//17 players; last marble is worth 1104 points: high score is 2764
//21 players; last marble is worth 6111 points: high score is 54718
//30 players; last marble is worth 5807 points: high score is 37305
        assert_eq!(32, do_game(25, 9));
        assert_eq!(8317, do_game(1618, 10));
        assert_eq!(146373, do_game(7999, 13));
        assert_eq!(2764, do_game(1104, 17));
        assert_eq!(54718, do_game(6111, 21));
        assert_eq!(37305, do_game(5807, 30));
    }
}
