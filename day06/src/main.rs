const INPUT: &str = include_str!("../input.txt");

fn main() {
    part1();
    part2();
}

fn part1() {
    println!("*** PART 1 ***");

    let groups = INPUT.split("\n\n");

    let sum: usize = groups.map(parse_group).map(count_set_bits).sum();

    println!("{}", sum);
}

fn part2() {
    println!("*** PART 2 ***");

    let groups = INPUT.split("\n\n");

    let sum: usize = groups.map(parse_group_part2).map(count_set_bits).sum();

    println!("{}", sum);
}

fn parse_group(group: &str) -> u32 {
    group
        .lines()
        .fold(0, |acc, person| acc | parse_person(person))
}

fn parse_group_part2(group: &str) -> u32 {
    group
        .lines()
        .fold(u32::MAX, |acc, person| acc & parse_person(person))
}

fn parse_person(person: &str) -> u32 {
    person.chars().fold(0, |acc, c| {
        assert!(c.is_ascii());

        let c = c as u8;
        let question_num = c - b'a';
        acc | (1 << question_num)
    })
}

fn count_set_bits(mut x: u32) -> usize {
    // Use Brian Kernighan's algorithm
    let mut count = 0;

    while x > 0 {
        x &= x - 1;
        count += 1;
    }

    count
}
