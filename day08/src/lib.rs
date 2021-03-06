use std::collections::HashSet;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Instruction {
    /// Increment or decrement global `accumulator` by an amount
    Acc(i32),

    /// Jump relative to current instruction
    Jmp(isize),

    /// No-op
    Nop(isize),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum StepError {
    InstructionAlreadyExecuted(usize),
    LastInstructionExecuted,
}

#[derive(Clone, Debug)]
pub struct Vm {
    instructions: Vec<Instruction>,
    current_instruction: usize,
    accumulator: i32,
    instructions_executed: HashSet<usize>,
}

impl Vm {
    pub fn new<I: Into<Vec<Instruction>>>(instructions: I) -> Self {
        let instructions = instructions.into();

        Self {
            instructions,
            current_instruction: 0,
            accumulator: 0,
            instructions_executed: HashSet::new(),
        }
    }

    fn fetch(&mut self) -> Result<Instruction, StepError> {
        let ins_num = self.current_instruction;

        // The number immediately following the last instruction is `self.instructions.len()`
        if ins_num == self.instructions.len() {
            return Err(StepError::LastInstructionExecuted);
        }

        // Try inserting instruction number
        if !self.instructions_executed.insert(ins_num) {
            // Instruction number was already present
            return Err(StepError::InstructionAlreadyExecuted(ins_num));
        }

        Ok(self.instructions[self.current_instruction])
    }

    fn execute(&mut self, instruction: Instruction) {
        self.instructions_executed.insert(self.current_instruction);

        match instruction {
            Instruction::Acc(x) => {
                self.accumulator += x;
                self.current_instruction += 1;
            }

            Instruction::Jmp(x) => {
                let new_instruction = (self.current_instruction as isize) + x;

                if new_instruction.is_negative() {
                    panic!("attempt to jump to negative instruction");
                }

                self.current_instruction = new_instruction as usize;
            }

            Instruction::Nop(_) => {
                /* do nothing */
                self.current_instruction += 1;
            }
        }
    }

    pub fn step(&mut self) -> Result<(), StepError> {
        let next_instruction = self.fetch()?;
        self.execute(next_instruction);
        Ok(())
    }

    pub fn terminates(&mut self) -> bool {
        use StepError::*;

        loop {
            if let Err(e) = self.step() {
                match e {
                    InstructionAlreadyExecuted(_) => {
                        return false;
                    }

                    LastInstructionExecuted => {
                        return true;
                    }
                }
            }
        }
    }

    pub fn step_until_loop(&mut self) {
        use StepError::InstructionAlreadyExecuted as AlreadyExec;

        loop {
            if let Err(AlreadyExec(_)) = self.step() {
                return;
            }
        }
    }

    pub fn acc(&self) -> i32 {
        self.accumulator
    }
}

pub fn parse_instructions(instructions: &str) -> Vec<Instruction> {
    instructions
        .lines()
        .map(|inst| {
            let mut parts = inst.split_whitespace();
            let op = parts.next().unwrap();
            let arg = parts.next().unwrap();

            match op {
                "acc" => {
                    let inc = arg.parse().unwrap();
                    Instruction::Acc(inc)
                }

                "jmp" => {
                    let amt = arg.parse().unwrap();
                    Instruction::Jmp(amt)
                }

                "nop" => {
                    let arg = arg.parse().unwrap();
                    Instruction::Nop(arg)
                }

                x => panic!("invalid opcode {}", x),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        use Instruction::*;

        let input = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6";

        let parsed = parse_instructions(input);

        assert_eq!(
            &parsed,
            &[
                Nop(0),
                Acc(1),
                Jmp(4),
                Acc(3),
                Jmp(-3),
                Acc(-99),
                Acc(1),
                Jmp(-4),
                Acc(6)
            ]
        );
    }

    #[test]
    fn test_run_acc() {
        use Instruction::Acc;

        let mut vm = Vm::new([Acc(1)]);
        assert!(vm.step().is_ok());
        assert_eq!(vm.acc(), 1);
    }

    #[test]
    fn test_run_jmp() {
        use Instruction::{Jmp, Nop};

        let mut vm = Vm::new([Jmp(2), Nop(0), Nop(0)]);
        assert!(vm.step().is_ok());
        assert_eq!(vm.current_instruction, 2);
    }

    #[test]
    fn test_run_nop() {
        use Instruction::Nop;

        let mut vm = Vm::new([Nop(0), Nop(0)]);
        assert!(vm.step().is_ok());
        assert_eq!(vm.acc(), 0);
        assert_eq!(vm.current_instruction, 1);
    }

    #[test]
    fn test_detect_loop() {
        use Instruction::*;

        let instructions = vec![
            Nop(0),
            Acc(1),
            Jmp(4),
            Acc(3),
            Jmp(-3),
            Acc(-99),
            Acc(1),
            Jmp(-4),
            Acc(6),
        ];

        let mut vm = Vm::new(instructions);
        vm.step_until_loop();
        assert_eq!(vm.acc(), 5);
    }

    #[test]
    fn test_vm_terminates() {
        use Instruction::*;

        let instructions = vec![
            Nop(0),
            Acc(1),
            Jmp(4),
            Acc(3),
            Jmp(-3),
            Acc(-99),
            Acc(1),
            Nop(-4),
            Acc(6),
        ];

        let mut vm = Vm::new(instructions);
        assert!(vm.terminates())
    }
}
