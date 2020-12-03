use anyhow::Result;
use std::ops::AddAssign;
use std::str::FromStr;

type TreeRow = Vec<bool>;

struct TreeMap {
    rows: Vec<TreeRow>,
}

impl FromStr for TreeMap {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s
            .lines()
            .map(|line| line.trim().chars().map(|c| c == '#').collect::<TreeRow>())
            .collect::<Vec<TreeRow>>();
        Ok(TreeMap { rows })
    }
}

struct Pos {
    x: usize,
    y: usize,
}

struct Slope {
    x: usize,
    y: usize,
}

impl Slope {
    fn new(x: usize, y: usize) -> Slope {
        Slope { x, y }
    }
}

impl AddAssign<&Slope> for Pos {
    fn add_assign(&mut self, slope: &Slope) {
        self.x += slope.x;
        self.y += slope.y;
    }
}

impl TreeMap {
    fn tree_at_pos(&self, pos: &Pos) -> bool {
        match self.rows.get(pos.y) {
            Some(row) => row[pos.x % row.len()],
            None => false,
        }
    }

    fn height(&self) -> usize {
        self.rows.len()
    }

    fn check_slope(&self, slope: &Slope) -> u64 {
        let mut pos = Pos { x: 0, y: 0 };
        let mut tree_count = 0;
        while pos.y < self.height() {
            if self.tree_at_pos(&pos) {
                tree_count += 1;
            }
            pos += slope;
        }

        tree_count
    }
}

fn parse_input(filename: &str) -> Result<TreeMap> {
    Ok(std::fs::read_to_string(filename)?.parse()?)
}

fn main() -> Result<()> {
    let map = parse_input("data.txt")?;

    let part1_result = map.check_slope(&Slope::new(3, 1));
    println!("Part 1: {}", part1_result);

    let slopes = vec![
        Slope::new(1, 1),
        Slope::new(3, 1),
        Slope::new(5, 1),
        Slope::new(7, 1),
        Slope::new(1, 2),
    ];

    let part2_result: u64 = slopes.iter().map(|slope| map.check_slope(slope)).product();
    println!("Part 2: {}", part2_result);

    Ok(())
}
