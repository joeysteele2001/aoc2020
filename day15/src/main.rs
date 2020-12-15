use std::collections::HashMap;

const INPUT: [u64; 7] = [0, 12, 6, 13, 20, 1, 17];

fn main() {
    part1();
    part2();
}

fn part1() {
    println!("*** PART 1 ***");

    let res = run_elf_game(&INPUT, 2020);

    println!("{}", res);
}

fn part2() {
    println!("*** PART 2 ***");

    let res = run_elf_game(&INPUT, 30_000_000);

    println!("{}", res);
}

fn run_elf_game(start: &[u64], end: usize) -> u64 {
    let mut list = HashMap::new();

    for (i, &n) in start.iter().enumerate().take(start.len() - 1) {
        list.insert(n, i);
    }

    let mut prev = start[start.len() - 1];

    for i in start.len()..end {
        let new = match list.get(&prev) {
            Some(idx) => (i - 1 - idx) as u64,
            None => 0,
        };

        list.insert(prev, i - 1);
        prev = new;
    }

    prev
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_elf_game(start: &[u64], end: usize, expected: u64) {
        assert_eq!(run_elf_game(start, end), expected);
    }

    #[test]
    fn test_elf_game_0() {
        let input = [0, 3, 6];
        assert_eq!(run_elf_game(&input, 10), 0);
    }

    #[test]
    fn test_elf_game_part1() {
        let inputs = [
            [1, 3, 2],
            [2, 1, 3],
            [1, 2, 3],
            [2, 3, 1],
            [3, 2, 1],
            [3, 1, 2],
        ];

        let expecteds = [1, 10, 27, 78, 438, 1836];

        for (input, &expected) in inputs.iter().zip(expecteds.iter()) {
            test_elf_game(input, 2020, expected);
        }
    }

    #[test]
    fn test_elf_game_part2() {
        let inputs = [
            [0, 3, 6],
            [1, 3, 2],
            [2, 1, 3],
            [1, 2, 3],
            [2, 3, 1],
            [3, 2, 1],
            [3, 1, 2],
        ];

        let expecteds = [175594, 2578, 3544142, 261214, 6895259, 18, 362];

        for (input, &expected) in inputs.iter().zip(expecteds.iter()) {
            test_elf_game(input, 30_000_000, expected);
        }
    }

    #[test]
    fn test_elf_game_big() {
        let input = [0, 3, 6];
        let expected = 175594;

        test_elf_game(&input, 30_000_000, expected);
    }
}
