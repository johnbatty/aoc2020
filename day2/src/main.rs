use anyhow::Result;
use recap::Recap;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Recap)]
#[recap(regex = r"(?x)
        \s*
        (?P<policy_value1>\d+)
        -
        (?P<policy_value2>\d+)
        \s+
        (?P<policy_letter>[a-z])
        :
        \s*
        (?P<password>\S+)
        \s*
    ")]
struct PwdEntry {
    policy_value1: u32,
    policy_value2: u32,
    policy_letter: char,
    password: String,
}

impl PwdEntry {
    fn password_char(&self, pos: usize) -> Option<char> {
        self.password.chars().nth(pos - 1)
    }

    fn password_valid_v1(&self) -> bool {
        let count = self.password.matches(self.policy_letter).count() as u32;
        (count >= self.policy_value1) && (count <= self.policy_value2)
    }

    fn password_valid_v2(&self) -> bool {
        let m1 = self.password_char(self.policy_value1 as usize) == Some(self.policy_letter);
        let m2 = self.password_char(self.policy_value2 as usize) == Some(self.policy_letter);
        m1 ^ m2
    }
}

struct PwdDb {
    entries: Vec<PwdEntry>,
}

fn parse_input(filename: &str) -> Result<PwdDb> {
    let data = std::fs::read_to_string(filename)?;
    let entries: Vec<PwdEntry> = data
        .lines()
        .filter_map(|line| line.parse::<PwdEntry>().ok())
        .collect();
    Ok(PwdDb { entries })
}

fn valid_password_count(db: &PwdDb, password_validator: impl Fn(&PwdEntry) -> bool) -> usize {
    db.entries
        .iter()
        .filter(|&entry| password_validator(entry))
        .count()
}

fn main() -> Result<()> {
    let db = parse_input("data.txt")?;

    let part1_valid_passwords = valid_password_count(&db, PwdEntry::password_valid_v1);
    let part2_valid_passwords = valid_password_count(&db, PwdEntry::password_valid_v2);

    println!("Part 1 valid passwords: {}", part1_valid_passwords);
    println!("Part 2 valid passwords: {}", part2_valid_passwords);

    Ok(())
}
