const INPUT: &str = include_str!("../input.txt");

fn main() {
    part1();
    part2();
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

    let count = count_trees(&map, 3, 1);

    println!("{}", count);
}

fn count_trees(map: &Vec<Vec<bool>>, dx: usize, dy: usize) -> usize {
    let n_rows = map.len();
    let n_cols = map[0].len(); // assume all rows are the same length

    let mut count = 0;
    let mut j = 0;
    let mut i = 0;

    while i < n_rows - 1 {
        i += dy;
        j += dx;

        if map[i][j % n_cols] {
            // This is a tree
            count += 1;
        }
    }

    return count;
}

fn part2() {
    println!("*** PART 2 ***");

    let map = parse_input(INPUT);

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let product: usize = slopes
        .iter()
        .map(|&(dx, dy)| count_trees(&map, dx, dy))
        .product();

    println!("{}", product)
}
