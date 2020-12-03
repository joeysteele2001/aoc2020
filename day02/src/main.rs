use day02::PasswordDBEntry;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    part2()
}

fn part2() {
    let entries = INPUT.lines().map(|l| l.parse::<PasswordDBEntry>().unwrap());

    let num_valid = entries.filter(|e| e.is_valid()).count();

    println!("*** PART 2 ***");
    println!("{}", num_valid);
}
