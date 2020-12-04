use day03::Map;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    part1();
    part2();
}

fn part1() {
    println!("*** PART 1 ***");

    let map = INPUT.parse::<Map>().unwrap();
    let count = map.count_trees(3, 1);

    println!("{}", count);
}

fn part2() {
    println!("*** PART 2 ***");

    let map = INPUT.parse::<Map>().unwrap();

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let product: usize = slopes
        .iter()
        .map(|&(dx, dy)| map.count_trees(dx, dy))
        .product();

    println!("{}", product)
}
