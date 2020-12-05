use anyhow::Result;
use itertools::Itertools;

fn find_two_entries(values: &[i64], target_sum: i64) -> Option<(i64, i64)> {
    for x in values {
        for y in values {
            if x + y == target_sum {
                return Some((*x, *y));
            }
        }
    }
    None
}

fn find_three_entries(values: &[i64], target_sum: i64) -> Option<(i64, i64, i64)> {
    for x in values {
        for y in values {
            for z in values {
                if x + y + z == target_sum {
                    return Some((*x, *y, *z));
                }
            }
        }
    }
    None
}

fn find_n_entries(values: &[i64], n: usize, target_sum: i64) -> Option<Vec<&i64>> {
    values
        .iter()
        .combinations(n)
        .find(|v| v.iter().cloned().sum::<i64>() == target_sum)
}

fn parse_input(filename: &str) -> Result<Vec<i64>> {
    let data = std::fs::read_to_string(filename)?
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    Ok(data)
}

const TARGET_SUM: i64 = 2020;

fn main() -> Result<()> {
    let values = parse_input("data.txt")?;

    println!("Basic approach...");
    println!("PART1");
    match find_two_entries(&values, TARGET_SUM) {
        Some((a, b)) => {
            let result = a * b;
            println!("{} x {} = {}", a, b, result);
        }
        None => println!("No solution found!"),
    }

    println!("PART2");
    match find_three_entries(&values, TARGET_SUM) {
        Some((a, b, c)) => {
            let result = a * b * c;
            println!("{} x {} x {} = {}", a, b, c, result)
        }
        None => println!("No solution found!"),
    }

    println!("===");
    println!("Alternative more general approach...");
    for n in 2..=3 {
        println!("n = {}", n);
        match find_n_entries(&values, n, TARGET_SUM) {
            Some(v) => {
                let result: i64 = v.iter().cloned().product();
                println!("product {} = {}", v.iter().format(" x "), result)
            }
            None => println!("No solution found!"),
        }
    }

    Ok(())
}
