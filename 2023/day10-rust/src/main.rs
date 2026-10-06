#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    North,
    South,
    East,
    West,
}

#[allow(unused)]
impl Dir {
    /// (row delta, col delta)
    fn delta(self) -> (isize, isize) {
        match self {
            Dir::North => (-1, 0),
            Dir::South => (1, 0),
            Dir::East => (0, 1),
            Dir::West => (0, -1),
        }
    }

    fn opposite(self) -> Dir {
        match self {
            Dir::North => Dir::South,
            Dir::South => Dir::North,
            Dir::East => Dir::West,
            Dir::West => Dir::East,
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Vertical,   // |
    Horizontal, // -
    NorthEast,  // L
    NorthWest,  // J
    SouthWest,  // 7
    SouthEast,  // F
    Ground,     // .
    Start,      // S
}

#[allow(unused)]
impl Tile {
    fn from_char(c: char) -> Option<Tile> {
        Some(match c {
            '|' => Tile::Vertical,
            '-' => Tile::Horizontal,
            'L' => Tile::NorthEast,
            'J' => Tile::NorthWest,
            '7' => Tile::SouthWest,
            'F' => Tile::SouthEast,
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => return None,
        })
    }

    fn connections(self) -> &'static [Dir] {
        use Dir::*;
        match self {
            Tile::Vertical => &[North, South],
            Tile::Horizontal => &[West, East],
            Tile::NorthEast => &[North, East],
            Tile::NorthWest => &[North, West],
            Tile::SouthWest => &[South, West],
            Tile::SouthEast => &[South, East],
            Tile::Ground => &[],
            Tile::Start => &[],
        }
    }

    fn connects(self, dir: Dir) -> bool {
        self.connections().contains(&dir)
    }
}

#[allow(unused)]
fn step(pos: (usize, usize), dir: Dir, rows: usize, cols: usize) -> Option<(usize, usize)> {
    let (dr, dc) = dir.delta();
    let r = pos.0.checked_add_signed(dr)?;
    let c = pos.1.checked_add_signed(dc)?;
    (r < rows && c < cols).then_some((r, c))
}

#[allow(unused)]
fn loop_length(grid: &[Vec<Tile>], start: (usize, usize), mut dir: Dir) -> Option<usize> {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut pos = start;
    let mut steps = 0;

    loop {
        pos = step(pos, dir, rows, cols)?;
        steps += 1;

        let tile = grid[pos.0][pos.1];
        if tile == Tile::Start {
            return Some(steps);
        }

        let came_from = dir.opposite();
        if !tile.connects(came_from) {
            return None;
        }

        dir = *tile.connections().iter().find(|&&d| d != came_from)?;
    }
}

fn walk_loop(
    grid: &[Vec<Tile>],
    start: (usize, usize),
    first_dir: Dir,
) -> Option<(Vec<(usize, usize)>, Dir)> {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut path = vec![start];
    let mut pos = start;
    let mut dir = first_dir;

    loop {
        match step(pos, dir, rows, cols) {
            Some(new_pos) => pos = new_pos,
            None => return None,
        }

        let tile = grid[pos.0][pos.1];
        if tile == Tile::Start {
            return Some((path, dir));
        }
        path.push(pos);

        let entered_from = dir.opposite();
        if !tile.connects(entered_from) {
            return None;
        }

        let openings = tile.connections();
        if openings[0] == entered_from {
            dir = openings[1];
        } else {
            dir = openings[0];
        }
    }
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("input path must be provided as an arg");
    let input = std::fs::read_to_string(path).unwrap();

    let grid: Vec<Vec<Tile>> = input
        .lines()
        .map(|l| l.chars().map(|c| Tile::from_char(c).unwrap()).collect())
        .collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let start = grid
        .iter()
        .enumerate()
        .find_map(|(r, row)| row.iter().position(|&t| t == Tile::Start).map(|c| (r, c)))
        .expect("no S in input");

    for first_dir in [Dir::North, Dir::South, Dir::East, Dir::West] {
        let Some((loop_tiles, last_dir)) = walk_loop(&grid, start, first_dir) else {
            continue;
        };

        println!("Part 1: {}", loop_tiles.len() / 2);

        let mut on_loop = vec![vec![false; cols]; rows];
        for &(r, c) in &loop_tiles {
            on_loop[r][c] = true;
        }

        let s_opens_north = first_dir == Dir::North || last_dir == Dir::South;

        let mut count = 0;
        for r in 0..rows {
            let mut inside = false;
            for c in 0..cols {
                if on_loop[r][c] {
                    let tile = grid[r][c];
                    let opens_north = if tile == Tile::Start {
                        s_opens_north
                    } else {
                        tile.connects(Dir::North)
                    };
                    if opens_north {
                        inside = !inside;
                    }
                } else if inside {
                    count += 1;
                }
            }
        }

        println!("Part 2: {}", count);
        return;
    }

    panic!("no loop found");
}
