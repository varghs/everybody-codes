use std::fs::read_to_string;

mod quest01;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("{}", quest01::part3(&input));
}
