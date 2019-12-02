use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mem: Vec<usize> =
        buffer
        .split(",")
        .map(|op| op.parse())
        .filter_map(Result::ok)
        .collect();

    println!("Pt. 1 {}", solve(mem.clone(), 12, 2));

    'outer: for noun in 0..100 {
        for verb in 0..100 {
            if solve(mem.clone(), noun, verb) == 19690720 {
                println!("Pt. 2 {}", 100 * noun + verb);
                break 'outer;
            }
        }
    }

    Ok(())
}

fn solve(mut mem: Vec<usize>, noun: usize, verb: usize) -> usize {
    mem[1] = noun;
    mem[2] = verb;
    for ip in (0..mem.len()).step_by(4) {
        match (mem[ip+0], mem[mem[ip+1]], mem[mem[ip+2]], mem[ip+3]) {
            (1, s1, s2, t) => mem[t] = s1 + s2,
            (2, s1, s2, t) => mem[t] = s1 * s2,
            _ => break,
        };
    }
    mem[0]
}

