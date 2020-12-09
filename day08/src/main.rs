use day08::*;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    part1();
    part2();
}

fn part1() {
    println!("*** PART 1 ***");

    let instructions = parse_instructions(INPUT);
    let mut vm = Vm::new(instructions);
    vm.step_until_loop();

    println!("{}", vm.acc());
}

fn part2() {
    println!("*** PART 2 ***");

    let original_instructions = parse_instructions(INPUT);

    for (i, &ins) in original_instructions.iter().enumerate() {
        let mut modified_instructions = original_instructions.clone();

        match ins {
            Instruction::Acc(_) => {
                continue;
            }

            Instruction::Nop(x) => {
                modified_instructions[i] = Instruction::Jmp(x);
            }

            Instruction::Jmp(x) => {
                modified_instructions[i] = Instruction::Nop(x);
            }
        }

        let mut vm = Vm::new(modified_instructions);

        if vm.terminates() {
            println!("{}", vm.acc());
        }
    }
}
