use anyhow::{bail, Result};
use serde::Deserialize;
use std::ops::Range;
use std::str::FromStr;

#[derive(Debug, Clone, Deserialize)]
struct BoardingPass {
    codes: Vec<BSPCode>,
}

#[derive(Debug, Clone, Deserialize)]
enum BSPCode {
    Front,
    Back,
    Left,
    Right,
}

trait Bisect {
    fn slice_a(&self) -> Self;
    fn slice_b(&self) -> Self;
}

impl Bisect for Range<usize> {
    fn slice_a(&self) -> Self {
        self.start..(self.start + self.len() / 2)
    }

    fn slice_b(&self) -> Self {
        (self.start + self.len() / 2)..self.end
    }
}

#[derive(Debug)]
struct PlanePartition {
    rows: std::ops::Range<usize>,
    seats: std::ops::Range<usize>,
}

impl PlanePartition {
    const MAX_ROWS: usize = 128;
    const SEATS_PER_ROW: usize = 8;

    fn new() -> Self {
        PlanePartition {
            rows: 0..PlanePartition::MAX_ROWS,
            seats: 0..PlanePartition::SEATS_PER_ROW,
        }
    }

    fn front(&self) -> Self {
        PlanePartition {
            rows: self.rows.slice_a(),
            seats: self.seats.clone(),
        }
    }

    fn back(&self) -> Self {
        PlanePartition {
            rows: self.rows.slice_b(),
            seats: self.seats.clone(),
        }
    }

    fn left(&self) -> Self {
        PlanePartition {
            rows: self.rows.clone(),
            seats: self.seats.slice_a(),
        }
    }

    fn right(&self) -> Self {
        PlanePartition {
            rows: self.rows.clone(),
            seats: self.seats.slice_b(),
        }
    }

    fn seat_id(&self) -> Result<u32> {
        if (self.rows.len() != 1) || (self.seats.len() != 1) {
            bail!("Partition does not identify a single seat: {:?}", self)
        }
        let row = self.rows.start as u32;
        let seat = self.seats.start as u32;
        Ok(row * (PlanePartition::SEATS_PER_ROW as u32) + seat)
    }
}

impl FromStr for BoardingPass {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let codes = s
            .chars()
            .map(|c| match c {
                'F' => Ok(BSPCode::Front),
                'B' => Ok(BSPCode::Back),
                'L' => Ok(BSPCode::Left),
                'R' => Ok(BSPCode::Right),
                _ => bail!("Invalid BSPCode: {}", c),
            })
            .collect::<Result<Vec<BSPCode>>>()?;
        Ok(BoardingPass { codes })
    }
}

impl BoardingPass {
    fn seat_id(&self) -> Result<u32> {
        let mut partition = PlanePartition::new();
        for code in &self.codes {
            partition = match code {
                BSPCode::Front => partition.front(),
                BSPCode::Back => partition.back(),
                BSPCode::Left => partition.left(),
                BSPCode::Right => partition.right(),
            };
        }
        partition.seat_id()
    }
}

fn parse_input(filename: &str) -> Result<Vec<BoardingPass>> {
    let data = std::fs::read_to_string(filename)?;
    let boarding_passes = data
        .lines()
        .filter_map(|boarding_pass| boarding_pass.parse::<BoardingPass>().ok())
        .collect();
    Ok(boarding_passes)
}

fn main() -> Result<()> {
    let boarding_passes = parse_input("data.txt")?;

    let mut seat_ids: Vec<u32> = boarding_passes
        .iter()
        .map(|bp| bp.seat_id().unwrap())
        .collect();
    seat_ids.sort_unstable();
    let max_seat_id = seat_ids.last().unwrap();
    println!("Part1: {}", max_seat_id);

    let first_seat = *seat_ids.first().unwrap();
    let free_seat = seat_ids
        .iter()
        .zip(first_seat..)
        .find_map(|(a, b)| if *a != b { Some(b) } else { None })
        .unwrap();
    println!("Part2: {}", free_seat);

    Ok(())
}
