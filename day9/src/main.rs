#[macro_use]
extern crate intrusive_collections;

use intrusive_collections::linked_list::CursorMut;
use intrusive_collections::{LinkedList, LinkedListLink};
use std::cell::Cell;
use std::error::Error;

struct Place {
    link: LinkedListLink,
    value: Cell<usize>,
}

intrusive_adapter!(CircleAdapter = Box<Place>: Place { link: LinkedListLink });

fn next(cur: &mut CursorMut<CircleAdapter>) {
    cur.move_next();
    if let None = cur.get() {
        cur.move_next();
    }
}

fn prev(cur: &mut CursorMut<CircleAdapter>) {
    cur.move_prev();
    if let None = cur.get() {
        cur.move_prev();
    }
}

fn do_game(num_marbles: usize, players: usize) -> usize {
    let mut scores = vec![0; players];
    let mut circle = LinkedList::new(CircleAdapter::new());
    let mut player = 0;

    circle.push_front(Box::new(Place { link: LinkedListLink::new(), value: Cell::new(0) }));
    {
        let mut cur = circle.front_mut();

        for m in 1..=num_marbles {
            if (m % 23) == 0 {
                scores[player] += m;
                for _ in 0..7 {
                    prev(&mut cur);
                }
                if let Some(p) = cur.get() {
                    scores[player] += p.value.get();
                }
                cur.remove();
            } else {
                next(&mut cur);
                cur.insert_after(Box::new(Place {
                    link: LinkedListLink::new(),
                    value: Cell::new(m),
                }));
                next(&mut cur);
            }
            player = (player + 1) % players;
        }
    }
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
        assert_eq!(32, do_game(25, 9));
        assert_eq!(8317, do_game(1618, 10));
        assert_eq!(146373, do_game(7999, 13));
        assert_eq!(2764, do_game(1104, 17));
        assert_eq!(54718, do_game(6111, 21));
        assert_eq!(37305, do_game(5807, 30));
    }
}
