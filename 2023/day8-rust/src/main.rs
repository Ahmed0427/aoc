use std::collections::HashMap;

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    if a != 0 && b != 0 {
        (a / gcd(a, b)) * b
    } else {
        0
    }
}

fn parse_line(line: &str) -> (&str, &str, &str) {
    let (node, rest) = line.split_once(" = (").expect("bad line");
    let (l, r) = rest.split_once(", ").expect("bad line");
    (node, l, r.trim_end_matches(')'))
}

fn main() {
    let path = std::env::args().nth(1).expect("input file must exist");
    let input = std::fs::read_to_string(path).expect("can't read file");
    let mut lines = input.lines();

    let instrs = lines.next().expect("empty input").as_bytes();
    lines.next().unwrap();

    let graph: HashMap<&str, (&str, &str)> = lines
        .map(|l| {
            let (n, l, r) = parse_line(l);
            (n, (l, r))
        })
        .collect();

    let result = graph
        .keys()
        .filter(|n| n.ends_with('A'))
        .map(|&start| {
            let mut cur = start;
            let mut steps = 0u64;
            for &i in instrs.iter().cycle() {
                steps += 1;
                let (l, r) = graph[cur];
                cur = match i {
                    b'L' => l,
                    b'R' => r,
                    _ => panic!("unexpected instruction"),
                };
                if cur.ends_with('Z') {
                    break;
                }
            }
            steps
        })
        .fold(1, lcm);

    println!("steps: {result}");
}
