use day02::PasswordDBEntry;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    part1()
}

fn part1() {
    let entries = INPUT.lines()
        .map(|l| l.parse::<PasswordDBEntry>().unwrap());

    let num_valid = entries
        .filter(|e| e.is_valid())
        .count();

    println!("*** PART 1 ***");
    println!("{}", num_valid);
}
