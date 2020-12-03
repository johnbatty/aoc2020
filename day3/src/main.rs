use anyhow::Result;
use std::ops::Add;

type MapRow = Vec<u8>;

struct Map {
    rows: Vec<MapRow>,
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

impl Add<&Slope> for Pos {
    type Output = Self;

    fn add(self, slope: &Slope) -> Pos {
        Pos {
            x: self.x + slope.x,
            y: self.y + slope.y,
        }
    }
}

impl Map {
    fn tree_at_pos(&self, pos: &Pos) -> bool {
        match self.rows.get(pos.y) {
            Some(row) => row[pos.x % row.len()] == b'#',
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
            pos = pos + slope;
        }

        tree_count
    }
}

fn parse_input(filename: &str) -> Result<Map> {
    let rows = std::fs::read_to_string(filename)?
        .lines()
        .map(|s| s.bytes().collect::<MapRow>())
        .collect();

    Ok(Map { rows })
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
