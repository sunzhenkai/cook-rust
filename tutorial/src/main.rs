mod concepts;
mod games;

use crate::concepts::{control, variables};
use crate::games::guess_number;

fn need_input() {
    // https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
    guess_number::play();
}

fn normal() {
    variables::play();
    control::play();
}

fn main() {
    println!("Hello, rust!\n");

    normal();
    need_input();
}
