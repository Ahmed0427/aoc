use std::cmp::max;
use std::cmp::min;
use std::io::BufRead;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_path = args.get(1).expect("must provide the input path as an arg");
    let f = std::fs::File::open(input_path).unwrap();
    let mut image = Vec::new();
    for line in std::io::BufReader::new(f).lines().map_while(Result::ok) {
        image.push(line.chars().collect::<Vec<char>>());
    }
    let mut is_empty_row = vec![1u8; image.len()];
    let mut is_empty_col = vec![1u8; image.len()];
    let mut galaxies = Vec::new();
    for i in 0..image.len() {
        for j in 0..image[0].len() {
            if image[i][j] == '#' {
                galaxies.push((i, j));
                is_empty_row[i] = 0;
                is_empty_col[j] = 0;
            }
        }
    }
    let mut ans = 0;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let (x1, y1) = galaxies[i];
            let (x2, y2) = galaxies[j];

            let mut steps: u64 = 0;
            for x in min(x1, x2)..max(x1, x2) {
                steps += 1 + (is_empty_row[x] as u64) * 999999;
            }
            for y in min(y1, y2)..max(y1, y2) {
                steps += 1 + (is_empty_col[y] as u64) * 999999;
            }
            ans += steps;
        }
    }
    dbg!(ans);
}
