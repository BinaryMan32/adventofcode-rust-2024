use advent_of_code::{create_runner, named, Named, Runner};
use itertools::Itertools;
use num::pow;
use std::str::Lines;

type Integer = u64;
type Registers = [Integer; 3];
struct Instruction {
    opcode: u8,
    operand: u8,
}

struct Computer {
    registers: Registers,
    instructions: Vec<u8>,
    instruction_pointer: usize,
    output: Vec<u8>,
}

impl Computer {
    fn parse(mut input: Lines) -> Self {
        let registers = input
            .by_ref()
            .take(3)
            .map(|line| {
                line.split_once(": ")
                    .expect("register colon")
                    .1
                    .parse::<Integer>()
                    .expect("register value numeric")
            })
            .collect_vec()
            .try_into()
            .expect("3 registers");
        let instructions = input.nth(1)
            .expect("program")
            .split_once(": ")
            .expect("Program: prefix")
            .1
            .split(',').map(|n| {
                n.parse().expect("numeric")
            })
            .collect_vec();
        let instruction_pointer = 0;
        let output = Vec::new();
        Self{registers, instructions, instruction_pointer, output}
    }

    fn run(&mut self) {
        while let Some(instruction) = self.fetch_instruction() {
            self.execute_instruction(instruction);
        }
    }

    fn fetch_instruction(&self) -> Option<Instruction> {
        self.instructions.get(self.instruction_pointer)
            .zip(self.instructions.get(self.instruction_pointer + 1))
            .map(|(&opcode, &operand)| Instruction{opcode, operand})
    }

    /**
     * The value of a combo operand can be found as follows:

     */
    fn combo_operand(&self, operand: u8) -> Integer {
        match operand {
            // Combo operands 0 through 3 represent literal values 0 through 3.
            0..=3 => operand as Integer,
            // Combo operand 4 represents the value of register A.
            4 => self.registers[0],
            // Combo operand 5 represents the value of register B.
            5 => self.registers[1],
            // Combo operand 6 represents the value of register C.
            6 => self.registers[2],
            // Combo operand 7 is reserved and will not appear in valid programs.
            7 => panic!("operand 7 is reserved"),
            _ => panic!("operand {operand} is not 3 bit")
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        self.instruction_pointer = match instruction.opcode {
            /* The adv instruction (opcode 0) performs division. The numerator is the value in the A
             * register. The denominator is found by raising 2 to the power of the instruction's
             * combo operand. (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would
             * divide A by 2^B.) The result of the division operation is truncated to an integer and
             * then written to the A register.
             */
            0 => {
                self.registers[0] /= pow(2, self.combo_operand(instruction.operand) as usize);
                self.instruction_pointer + 2
            }
            /* The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the
             * instruction's literal operand, then stores the result in register B.
             */
            1 => {
                self.registers[1] ^= instruction.operand as Integer;
                self.instruction_pointer + 2
            }
            /* The bst instruction (opcode 2) calculates the value of its combo operand modulo 8
             * (thereby keeping only its lowest 3 bits), then writes that value to the B register.
             */
            2 => {
                self.registers[1] = self.combo_operand(instruction.operand) % 8;
                self.instruction_pointer + 2
            }
            /* The jnz instruction (opcode 3) does nothing if the A register is 0. However, if the
             * A register is not zero, it jumps by setting the instruction pointer to the value of
             * its literal operand; if this instruction jumps, the instruction pointer is not
             * increased by 2 after this instruction.
             */
            3 => {
                if self.registers[0] == 0 {
                    self.instruction_pointer + 2
                } else {
                    instruction.operand as usize
                }
            }
            /* The bxc instruction (opcode 4) calculates the bitwise XOR of register B and
             * register C, then stores the result in register B. (For legacy reasons, this
             * instruction reads an operand but ignores it.)
             */
            4 => {
                self.registers[1] ^= self.registers[2];
                self.instruction_pointer + 2
            }
            /* The out instruction (opcode 5) calculates the value of its combo operand modulo 8,
             * then outputs that value. (If a program outputs multiple values, they are separated
             * by commas.)
             */
            5 => {
                self.output.push((self.combo_operand(instruction.operand) % 8) as u8);
                self.instruction_pointer + 2
            }
            /* The bdv instruction (opcode 6) works exactly like the adv instruction except that
             * the result is stored in the B register. (The numerator is still read from the A
             * register.)
             */
            6 => {
                self.registers[1] = self.registers[0] / pow(2, self.combo_operand(instruction.operand) as usize);
                self.instruction_pointer + 2
            }
            /*
             * The cdv instruction (opcode 7) works exactly like the adv instruction except that
             * the result is stored in the C register. (The numerator is still read from the A
             * register.)
             */
            7 => {
                self.registers[2] = self.registers[0] / pow(2, self.combo_operand(instruction.operand) as usize);
                self.instruction_pointer + 2
            }
            _ => {
                panic!("invalid opcode {opcode}", opcode=instruction.opcode);
            }
        }

    }

    fn get_output(&self) -> String {
        self.output.iter().join(",")
    }
}

fn part1(input: Lines) -> String {
    let mut computer = Computer::parse(input);
    computer.run();
    computer.get_output()
}

/**
 * Single step of the program loop.
 * This can be used to test candidate values to see if they output the expected value.
 * Uses only the bottom `10` bits of `a`.
 */
fn simulate_step(a: Integer) -> u8 {
    let b = a & 7; 
    let b = b ^ 7;
    let c = a >> b;
    let b = b ^ c;
    let b = b ^ 4;
    (b & 7) as u8
}

/**
 * Reference implementation created by directly translating input instructions.
 */
#[cfg(test)]
fn simulate(mut a: Integer) -> Vec<u8> {
    let mut output = Vec::new();
    while a != 0 {
        output.push(simulate_step(a));
        a >>= 3;
    }
    output
}

fn reverse_simulate(output: &[u8], a: Integer) -> Option<Integer> {
    if output.is_empty() {
        return Some(a);
    }
    // guess at values for the bottom 3 bits
    let upper = a << 3;
    (0..(1<<3))
        .into_iter()
        .map(|lower| upper | lower)
        .filter(|&candidate| simulate_step(candidate) == output[0])
        .find_map(|candidate| reverse_simulate(&output[1..], candidate))
}

fn find_a_register(instructions: &[u8]) -> Option<Integer> {
    let reversed_instructions = instructions.into_iter().rev().cloned().collect_vec();
    // 7 bits of context could be anything, try all of the possible values
    (0..(1<<7))
        .into_iter()
        .find_map(|a| reverse_simulate(&reversed_instructions, a))
}

fn part2(input: Lines) -> String {
    let computer = Computer::parse(input);
    find_a_register(&computer.instructions).unwrap().to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let runner: &Runner = create_runner!();
    runner.run(named!(part1), input);
    runner.run(named!(part2), input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::verify;

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_simulate() {
        let input = include_str!("input.txt").lines();
        let mut computer = Computer::parse(input);
        let a = computer.registers[0];
        computer.run();
        assert_eq!(simulate(a), computer.output);
    }
}
