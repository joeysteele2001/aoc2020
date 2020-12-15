use std::collections::HashMap;

const INPUT: [u32; 7] = [0, 12, 6, 13, 20, 1, 17];

fn main() {
    part1();
}

fn part1() {
    println!("*** PART 1 ***");

    let res = run_elf_game(&INPUT, 2020);

    println!("{}", res);
}

fn run_elf_game(start: &[u32], end: usize) -> u32 {
    let mut list = HashMap::new();

    for (i, &n) in start.iter().enumerate().take(start.len() - 1) {
        list.insert(n, i);
    }

    let mut prev = start[start.len() - 1];

    for i in start.len()..end {
        let new = match list.get(&prev) {
            Some(idx) => (i - 1 - idx) as u32,
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

    #[test]
    fn test_elf_game_0() {
        let input = [0, 3, 6];
        assert_eq!(run_elf_game(&input, 10), 0);
    }

    #[test]
    fn test_elf_game_1() {
        let input = [1, 3, 2];
        assert_eq!(run_elf_game(&input, 2020), 1);
    }

    #[test]
    fn test_elf_game_2() {
        let input = [2, 1, 3];
        assert_eq!(run_elf_game(&input, 2020), 10);
    }

    #[test]
    fn test_elf_game_3() {
        let input = [1, 2, 3];
        assert_eq!(run_elf_game(&input, 2020), 27);
    }

    #[test]
    fn test_elf_game_4() {
        let input = [2, 3, 1];
        assert_eq!(run_elf_game(&input, 2020), 78);
    }

    #[test]
    fn test_elf_game_5() {
        let input = [3, 2, 1];
        assert_eq!(run_elf_game(&input, 2020), 438);
    }

    #[test]
    fn test_elf_game_6() {
        let input = [3, 1, 2];
        assert_eq!(run_elf_game(&input, 2020), 1836);
    }
}
