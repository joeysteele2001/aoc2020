use std::fs;
use std::io;

fn main() {
    println!("*** PART 1 ***");
    part1();

    println!("\n*** PART 2 ***");
    part2();
}

fn part1() {
    const EXPECTED_SUM: u64 = 2020;

    let input = read_input("input.txt").unwrap();
    let numbers = parse_numbers(&input);
    let (m, n) = find_addends(&numbers, EXPECTED_SUM).unwrap();
    println!("{}", m * n);
}

fn part2() {
    const EXPECTED_SUM: u64 = 2020;

    let input = read_input("input.txt").unwrap();
    let numbers = parse_numbers(&input);
    let (l, m, n) = find_3_addends(&numbers, EXPECTED_SUM).unwrap();
    println!("{}", l * m * n);
}

fn find_addends(numbers: &[u64], expected_sum: u64) -> Option<(u64, u64)> {
    for (i, &n) in numbers.iter().enumerate() {
        for &m in numbers.iter().skip(i + 1) {
            if m + n == expected_sum {
                return Some((m, n));
            }
        }
    }

    None
}

fn find_3_addends(numbers: &[u64], expected_sum: u64) -> Option<(u64, u64, u64)> {
    for (i, &l) in numbers.iter().enumerate() {
        for (j, &m) in numbers.iter().skip(i + 1).enumerate() {
            for &n in numbers.iter().skip(j + 1) {
                if l + m + n == expected_sum {
                    return Some((l, m, n));
                }
            }
        }
    }

    None
}

fn parse_numbers(s: &str) -> Vec<u64> {
    s.lines()
        .map(|l| l.parse().unwrap())
        .collect()
}

fn read_input(filename: &str) -> io::Result<String> {
    fs::read_to_string(filename)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        const EXPECTED_SUM: u64 = 2020;
        let list_entries = vec![1721, 979, 366, 299, 675, 1456];
        let (m, n) = find_addends(&list_entries, EXPECTED_SUM).unwrap();
        assert_eq!(m * n, 514579)
    }

    #[test]
    fn test_part2() {
        const EXPECTED_SUM: u64 = 2020;
        let list_entries = vec![1721, 979, 366, 299, 675, 1456];
        let (l, m, n) = find_3_addends(&list_entries, EXPECTED_SUM).unwrap();
        assert_eq!(l * m * n, 241861950)
    }
}
