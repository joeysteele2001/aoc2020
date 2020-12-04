const INPUT: &str = include_str!("../input.txt");

fn main() {
    part1()
}

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => panic!("bad character"),
                })
                .collect()
        })
        .collect()
}

fn part1() {
    println!("*** PART 1 ***");

    let map = parse_input(INPUT);

    let cols = map[0].len();

    let mut count = 0;
    let mut j = 0;
    let dj = 3;

    for row in map.iter().skip(1) {
        j += dj;

        if row[j % cols] {
            // We've hit a tree
            count += 1;
        }
    }

    println!("{}", count);
}
