use anyhow::Result;
use log::error;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Person {
    yes_answers: HashSet<char>,
}

#[derive(Debug, Clone)]
struct Group {
    people: Vec<Person>,
}

impl Group {
    fn yes_answer_union_count(&self) -> usize {
        self.people
            .iter()
            .fold(HashSet::<char>::new(), |set, person| {
                set.union(&person.yes_answers).cloned().collect()
            })
            .len()
    }

    fn yes_answer_intersection_count(&self) -> usize {
        let initial_set = match self.people.get(0) {
            Some(person) => person.yes_answers.clone(),
            None => return 0,
        };

        self.people
            .iter()
            .skip(1)
            .fold(initial_set, |set, person| {
                set.intersection(&person.yes_answers).cloned().collect()
            })
            .len()
    }
}

impl FromStr for Group {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let people = s
            .lines()
            .map(|line| Person {
                yes_answers: line.chars().collect(),
            })
            .collect::<Vec<Person>>();
        Ok(Group { people })
    }
}

fn parse_input(filename: &str) -> Result<Vec<Group>> {
    let data = std::fs::read_to_string(filename)?;
    let values = data
        .split("\n\n")
        .filter_map(|v| {
            v.parse()
                .map_err(|e| {
                    error!("ERROR: Failed to parse:\n{}\nError: {}", v, e);
                    e
                })
                .ok()
        })
        .collect();
    Ok(values)
}

fn main() -> Result<()> {
    env_logger::init();

    let groups = parse_input("data.txt").unwrap();

    let part1_result: usize = groups
        .iter()
        .map(|group| group.yes_answer_union_count())
        .sum();
    println!("Part1: {}", part1_result);

    let part2_result: usize = groups
        .iter()
        .map(|group| group.yes_answer_intersection_count())
        .sum();
    println!("Part2: {}", part2_result);

    Ok(())
}
