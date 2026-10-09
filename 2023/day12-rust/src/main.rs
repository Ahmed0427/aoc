use rayon::prelude::*;
use std::collections::HashMap;

fn count(
    bytes: &[u8],
    lengths: &[usize],
    i: usize,
    g: usize,
    run: usize,
    memo: &mut HashMap<(usize, usize, usize), u64>,
) -> u64 {
    if i == bytes.len() {
        let ok = (g == lengths.len() && run == 0) || (g + 1 == lengths.len() && run == lengths[g]);
        return ok as u64;
    }
    if let Some(&v) = memo.get(&(i, g, run)) {
        return v;
    }

    let mut total = 0;
    let c = bytes[i];

    if c == b'#' || c == b'?' {
        if g < lengths.len() && run < lengths[g] {
            total += count(bytes, lengths, i + 1, g, run + 1, memo);
        }
    }
    if c == b'.' || c == b'?' {
        if run == 0 {
            total += count(bytes, lengths, i + 1, g, 0, memo);
        } else if run == lengths[g] {
            total += count(bytes, lengths, i + 1, g + 1, 0, memo);
        }
    }

    memo.insert((i, g, run), total);
    total
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_path = args.get(1).unwrap();
    let input = std::fs::read_to_string(input_path).unwrap();
    let result: u64 = input
        .par_lines()
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            let springs = [parts[0]; 5].join("?");
            let groups = [parts[1]; 5].join(",");
            let lengths: Vec<usize> = groups
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();

            count(
                &mut springs.as_bytes().to_vec(),
                &lengths,
                0,
                0,
                0,
                &mut HashMap::new(),
            )
        })
        .sum();
    dbg!(result);
}
