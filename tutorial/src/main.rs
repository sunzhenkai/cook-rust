mod games;

use crate::games::guess_number;
fn main() {
    println!("Hello, rust!\n");
    guess_number::play();
}
