use crate::execution::architecture::*;
use crate::execution::executor::execute;
use crate::execution::memory::Memory;
use crate::Instruction;
use std::num::Wrapping;

/// A running instance of a virtual machine.
///
/// All internal registers are initialized to zero at start up.
/// The type exposes methods to either submit new data (`iinput`, `dinput`) or to extract results
/// (`ioutput`, `doutput`).
///
/// Operand addresses are evaluated modulo the size of the register array they refer to, to keep them
/// within the allocated memory for that array.
///
/// This type is not thread safe.
/// However, multiple instances do not share state and so can be used concurrently (for example, one
/// virtual machine instance per thread) without interfering with each other.
#[derive(Debug)]
pub struct VirtualMachine {
    memory: Memory,
    instructions: Vec<Instruction>,
    // Next instruction to execute
    next: u16,
    // Count of instructions executed.
    count: u64,
}

impl VirtualMachine {
    /// Create a new instance of a virtual machine.
    /// This is the main method exposed by the crate.
    ///
    /// # Arguments
    ///
    /// * `architecture` - Configuration of memory sizes and other parameters.
    /// * `instructions` - Instructions to execute.
    ///
    pub fn new(architecture: &Architecture, instructions: Vec<Instruction>) -> Self {
        let memory = Memory::new(architecture);
        Self {
            memory,
            instructions,
            next: 0,
            count: 0,
        }
    }

    /// Call to run the virtual machine until one of the following criteria are met.
    ///
    /// # Stopping Criteria
    ///
    /// * `SYN` instruction executed. Caller should use results in output memory, update input
    /// memory if required and call `run` again.
    /// * `HLT` instruction executed. Execution has finished. Caller should use results in output
    /// memory and not call `run` again.
    /// * The maximum number of instructions has been executed. Execution has finished.
    /// Caller should use results in output memory and not call `run` again.
    ///
    /// # End of code behavior
    ///
    /// Code execution will jump back to the first instruction on reaching the end of the provided
    /// instructions (ie wraps around). To prevent this behaviour, add a HLT instruction as the last
    /// instruction.
    ///
    /// # Arguments
    ///
    /// * `budget` - The maximum number of instructions to implement.
    ///
    pub fn run(&mut self, budget: usize) {
        for _ in 0..budget {
            let instruction = &self.instructions[self.next as usize];
            match instruction {
                Instruction::HLT() => break,
                Instruction::SYN() => {
                    self.update_code_offset(1);
                    break;
                }
                _ => {
                    let code_offset = execute(instruction, &mut self.memory);
                    self.update_code_offset(code_offset);
                }
            }
        }
    }

    fn update_code_offset(&mut self, code_offset: i16) {
        self.next = (Wrapping(self.next as i16) + Wrapping(code_offset)).0 as u16
            % self.instructions.len() as u16;
        self.count += 1;
    }

    /// A modifiable bank of integers that act as input to the virtual machine.
    pub fn iinput(&mut self) -> &mut Vec<i64> {
        &mut self.memory.iinput
    }

    /// A bank of floating point values that act as input to the virtual machine.
    pub fn dinput(&mut self) -> &mut Vec<f64> {
        &mut self.memory.dinput
    }

    /// A snapshot of the bank of integers that act as output from the virtual machine.
    pub fn ioutput(&self) -> &Vec<i64> {
        &self.memory.ioutput
    }

    /// A snapshot of the bank of floating point values that act as output from the virtual machine.
    pub fn doutput(&self) -> &Vec<f64> {
        &self.memory.doutput
    }

    /// A snapshot copy of the memory of the virtual machine.
    pub fn snapshot(&self) -> &Memory {
        &self.memory
    }

    /// The next instruction to execute.
    pub fn next_instruction(&self) -> &Instruction {
        &self.instructions[self.next as usize]
    }

    /// The next instruction to execute.
    pub fn next_instruction_position(&self) -> usize {
        self.next as usize
    }

    /// The number of instructions executed so far.
    pub fn count(&self) -> u64 {
        self.count
    }
}
