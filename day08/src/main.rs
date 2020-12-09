use day08::*;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    part1();
}

fn part1() {
    println!("*** PART 1 ***");

    let instructions = parse_instructions(INPUT);
    let mut vm = Vm::new(instructions);
    vm.step_until_loop();

    println!("{}", vm.acc());
}
