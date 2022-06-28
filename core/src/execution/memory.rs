use crate::execution::architecture::*;
use std::mem;

/// The memory banks of an instance of a virtual machine.
///
/// All registers are initialized to zero at start up, except any constant registers to which values
/// have been assigned.
/// The mechanism for setting and changing the input registers is application specific.
///
/// Operand addresses are evaluated modulo the size of the register array they refer to, to keep them
/// within the allocated memory for that array.
///
#[derive(Clone, Debug, PartialEq)]
pub struct Memory {
    /// II: linear array of integer input registers
    pub iinput: Vec<i64>,
    /// IS: linear array of integer state registers
    pub istate: Vec<i64>,
    /// IO: linear array of integer output registers
    pub ioutput: Vec<i64>,
    /// DI: linear array of double precision floating point input registers
    pub dinput: Vec<f64>,
    /// DS: linear array of double precision floating point state registers
    pub dstate: Vec<f64>,
    /// DO: linear array of double precision floating point output registers
    pub doutput: Vec<f64>,
}

impl Memory {
    /// Create a new memory architecture.
    pub fn new(ar: &Architecture) -> Memory {
        Memory {
            iinput: make(0, ar.iinput),
            istate: make(0, ar.istate),
            ioutput: make(0, ar.ioutput),
            dinput: make(0.0, ar.dinput),
            dstate: make(0.0, ar.dstate),
            doutput: make(0.0, ar.doutput),
        }
    }

    /// Count the total number of addressable memory elements.
    pub fn count_elements(&self) -> usize {
        self.iinput.len()
            + self.istate.len()
            + self.ioutput.len()
            + self.dinput.len()
            + self.dstate.len()
            + self.doutput.len()
    }

    /// Count the number of bytes used by all addressable memory elements.
    pub fn size_of(&self) -> usize {
        let ints = self.iinput.len() + self.istate.len() + self.ioutput.len();
        let floats = self.dinput.len() + self.dstate.len() + self.doutput.len();
        ints * mem::size_of::<u64>() + floats * mem::size_of::<f64>()
    }
}

fn make<T>(value: T, length: u16) -> Vec<T>
where
    T: Clone,
{
    let mut result = vec![value; length as usize];
    result.shrink_to_fit();
    result
}
