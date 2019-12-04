use std::io::{self, Read};
use std::error::Error;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<Vec<&str>> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.split(",").collect())
        .collect();

    let steps1 = walk(&lines[0]);
    let steps2 = walk(&lines[1]);

    let pt1 = steps1.keys()
        .filter(|&x| steps2.contains_key(x))
        .map(|&x| x.0.abs() + x.1.abs())
        .min()
        .unwrap_or(0);

    let pt2 = steps1.keys()
        .filter(|pos| steps2.contains_key(&pos))
        .map(|pos| steps1[&pos] + steps2[&pos])
        .min()
        .unwrap_or(0);

    println!("{}", pt1);
    println!("{}", pt2);

    Ok(())
}

fn walk(ops: &Vec<&str>) -> HashMap<(i32, i32), i32> {
    let mut state = HashMap::new();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut d: i32 = 0;
    for op in ops {
        let (dx, dy) = match &op[..1] {
            "L" => (-1, 0),
            "U" => (0, 1),
            "R" => (1, 0),
            "D" => (0, -1),
            _ => (0, 0),
        };
        let steps: i32 = op[1..].parse().unwrap_or(0);
        for _ in 0..steps {
            d += 1;
            x += dx;
            y += dy;
            state.insert((x, y), d);
        }
    }

    state
}
