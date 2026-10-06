#[allow(dead_code)]

fn next_value(seq: &[i32]) -> i32 {
    if seq.iter().all(|&x| x == seq[0]) {
        return seq[0];
    }
    let diffs: Vec<i32> = seq.windows(2).map(|w| w[1] - w[0]).collect();
    seq.last().unwrap() + next_value(&diffs)
}

fn prev_value(seq: &[i32]) -> i32 {
    if seq.iter().all(|&x| x == seq[0]) {
        return seq[0];
    }
    let diffs: Vec<i32> = seq.windows(2).map(|w| w[1] - w[0]).collect();
    seq.first().unwrap() - prev_value(&diffs)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_path = args.get(1).expect("must provide input as arg");
    let input = std::fs::read_to_string(input_path).unwrap();
    let lines = input.lines();
    let result = lines
        .map(|l| {
            l.split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|seq| prev_value(&seq))
        .fold(0, |a, b| a + b);
    dbg!(result);
}
