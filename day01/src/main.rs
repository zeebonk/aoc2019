use std::io::{self, BufRead};

fn main() {
    let weights: Vec<i32> = io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|l| l.parse::<i32>())
        .filter_map(Result::ok)
        .collect();

    println!("Pt. 1: {}", weights.iter().fold(0, |a, w| a + fuel(w)));
    println!("Pt. 2: {}", weights.iter().fold(0, |a, w| a + fuel_rec(w)));
}

fn fuel(mass: &i32) -> i32 {
    (mass / 3) - 2
}

fn fuel_rec(mass: &i32) -> i32 {
    match fuel(mass) {
        f if f <= 0 => 0,
        f => f + fuel_rec(&f),
    }
}
