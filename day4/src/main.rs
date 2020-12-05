use anyhow::Result;
use log::error;
use recap::Recap;
use regex::Regex;
use serde::Deserialize;
use std::str::FromStr;
#[macro_use]
extern crate lazy_static;

#[derive(Debug, Clone, Deserialize, Recap)]
#[recap(regex = r"(?x)
        \s*
        ((
            byr:(?P<byr>\S+) |
            iyr:(?P<iyr>\S+) |
            eyr:(?P<eyr>\S+) | 
            hgt:(?P<hgt>\S+) |
            hcl:(?P<hcl>\S+) |
            ecl:(?P<ecl>\S+) |
            pid:(?P<pid>\S+) | 
            cid:(?P<cid>\S+)
        )\s*)+
    ")]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

// Alternative Part2 implementation using regexes to do validation
#[derive(Debug, Clone, Deserialize, Recap)]
#[recap(regex = r"(?x)
        \s*
        ((
            # byr (Birth Year) - four digits; at least 1920 and at most 2002.
            byr:(?P<byr>
                19[2-9][0-9] |
                200[0-2]
            ) |
            # iyr (Issue Year) - four digits; at least 2010 and at most 2020.
            iyr:(?P<iyr>
                201[0-9] |
                2020
            ) |
            # eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
            eyr:(?P<eyr>
                202[0-9] |
                2030
            ) | 
            # hgt (Height) - a number followed by either cm or in:
            # If cm, the number must be at least 150 and at most 193.
            # If in, the number must be at least 59 and at most 76.       
            hgt:(?P<hgt>
                1[5-8][0-9]cm |
                19[0-3]cm |
                59in | 
                6[0-9]in |
                7[0-6]in    
            ) |
            # hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
            hcl:(?P<hcl>
                (?-x:#)[0-9a-f]{6}
            ) |
            # ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
            ecl:(?P<ecl>
                amb|blu|brn|gry|grn|hzl|oth
            ) |
            # pid (Passport ID) - a nine-digit number, including leading zeroes.
            pid:(?P<pid>
                \d{9}
            ) | 
            # cid (Country ID) - ignored, missing or not.
            cid:(?P<cid>\S+)
        )\s*)+
        $
    ")]
struct PassportV2 {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

impl Passport {
    fn validate_year(year: &str, min: &str, max: &str) -> bool {
        (year >= min) && (year <= max)
    }

    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    fn byr_valid(&self) -> bool {
        Passport::validate_year(&self.byr, "1920", "2002")
    }

    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    fn iyr_valid(&self) -> bool {
        Passport::validate_year(&self.iyr, "2010", "2020")
    }

    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    fn eyr_valid(&self) -> bool {
        Passport::validate_year(&self.eyr, "2020", "2030")
    }

    // hgt (Height) - a number followed by either cm or in:
    // If cm, the number must be at least 150 and at most 193.
    // If in, the number must be at least 59 and at most 76.
    fn hgt_valid(&self) -> bool {
        lazy_static! {
            static ref HGT_REGEX: Regex = Regex::new(r"^(?P<value>\d+)(?P<units>cm|in)$").unwrap();
        }

        match HGT_REGEX.captures(&self.hgt) {
            Some(caps) => {
                let units = caps.name("units").unwrap().as_str();
                let value = caps
                    .name("value")
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap_or(0);
                match units {
                    "cm" => (value >= 150) && (value <= 193),
                    "in" => (value >= 59) && (value <= 76),
                    _ => false,
                }
            }
            None => false,
        }
    }

    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    fn hcl_valid(&self) -> bool {
        lazy_static! {
            static ref HCL_REGEX: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        }
        HCL_REGEX.is_match(&self.hcl)
    }

    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    fn ecl_valid(&self) -> bool {
        const VALID_ECLS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        VALID_ECLS.contains(&self.ecl.as_str())
    }

    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    fn pid_valid(&self) -> bool {
        lazy_static! {
            static ref PID_REGEX: Regex = Regex::new(r"^\d{9}$").unwrap();
        }
        PID_REGEX.is_match(&self.pid)
    }

    // cid (Country ID) - ignored, missing or not.
    fn cid_valid(&self) -> bool {
        true
    }

    fn is_valid(&self) -> bool {
        self.byr_valid()
            && self.iyr_valid()
            && self.eyr_valid()
            && self.hgt_valid()
            && self.hcl_valid()
            && self.ecl_valid()
            && self.pid_valid()
            && self.cid_valid()
    }
}

fn parse_input<T>(filename: &str) -> Result<Vec<T>>
where
    T: FromStr + std::fmt::Debug,
    <T as std::str::FromStr>::Err: std::error::Error,
{
    let data = std::fs::read_to_string(filename)?;
    let values = data
        .split("\n\n")
        .filter_map(|v| {
            v.parse::<T>()
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

    let passports = parse_input::<Passport>("data.txt")?;
    let part1_count = passports.len();
    println!("Part 1: {}", part1_count);

    let part2_count = passports.iter().filter(|p| p.is_valid()).count();
    println!("Part 2: {}", part2_count);

    let passports_v2 = parse_input::<PassportV2>("data.txt")?;
    let part2_v2_count = passports_v2.len();
    println!("Part 2 (v2): {}", part2_v2_count);

    Ok(())
}
