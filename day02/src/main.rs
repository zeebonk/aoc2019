use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let ops: Vec<usize> =
        buffer
        .split(",")
        .map(|op| op.parse())
        .filter_map(Result::ok)
        .collect();

    println!("Pt. 1 {}", solve(ops.to_vec(), 12, 2));

    'outer: for noun in 0..100 {
        for verb in 0..100 {
            if solve(ops.to_vec(), noun, verb) == 19690720 {
                println!("Pt. 2 {}", 100 * noun + verb);
                break 'outer;
            }
        }
    }

    Ok(())
}

fn solve(mut ops: Vec<usize>, noun: usize, verb: usize) -> usize {
    ops[1] = noun;
    ops[2] = verb;
    for i in (0..ops.len()).step_by(4) {
        match (ops[i+0], ops[ops[i+1]], ops[ops[i+2]], ops[i+3]) {
            (1, s1, s2, t) => ops[t] = s1 + s2,
            (2, s1, s2, t) => ops[t] = s1 * s2,
            _ => break,
        };
    }
    ops[0]
}

