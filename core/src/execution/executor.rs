use crate::execution::memory::Memory;
use crate::Instruction::*;
use crate::{Instruction, LeftDouble, LeftInteger, Mode, RightDouble, RightInteger};
use std::num::Wrapping;

// Execute a single instruction on the provided memory.
// Returns the next relative code instruction to execute.
pub(crate) fn execute(instruction: &Instruction, memory: &mut Memory) -> i16 {
    match instruction {
        NOP() => 1,
        HLT() => 1,
        SYN() => 1,
        IIMOV(lint, rint) => {
            *i_write(rint, memory) = i_read(lint, memory);
            1
        }
        IDMOV(lint, rdouble) => {
            *d_write(rdouble, memory) = i_read(lint, memory) as f64;
            1
        }
        DIMOV(ldouble, rint) => {
            *i_write(rint, memory) = d_read(ldouble, memory) as i64;
            1
        }
        DDMOV(ldouble, rdouble) => {
            *d_write(rdouble, memory) = d_read(ldouble, memory);
            1
        }
        IADD(lint1, lint2, rint) => {
            *i_write(rint, memory) =
                (Wrapping(i_read(lint1, memory)) + Wrapping(i_read(lint2, memory))).0;
            1
        }
        IDIV(lint1, lint2, rint) => {
            let divisor = i_read(lint2, memory);
            *i_write(rint, memory) = if divisor == 0 {
                0
            } else {
                i_read(lint1, memory) / divisor
            };
            1
        }
        IMOD(lint1, lint2, rint) => {
            let divisor = i_read(lint2, memory);
            *i_write(rint, memory) = if divisor == 0 {
                0
            } else {
                i_read(lint1, memory) % divisor
            };
            1
        }
        IMUL(lint1, lint2, rint) => {
            *i_write(rint, memory) =
                (Wrapping(i_read(lint1, memory)) * Wrapping(i_read(lint2, memory))).0;
            1
        }
        ISUB(lint1, lint2, rint) => {
            *i_write(rint, memory) =
                (Wrapping(i_read(lint1, memory)) - Wrapping(i_read(lint2, memory))).0;
            1
        }
        DADD(ldouble1, ldouble2, rldouble) => {
            *d_write(rldouble, memory) = d_read(ldouble1, memory) + d_read(ldouble2, memory);
            1
        }
        DDIV(ldouble1, ldouble2, rldouble) => {
            let divisor = d_read(ldouble2, memory);
            *d_write(rldouble, memory) = if divisor == 0.0 {
                0.0
            } else {
                d_read(ldouble1, memory) / divisor
            };
            1
        }
        DMOD(ldouble1, ldouble2, rldouble) => {
            let divisor = d_read(ldouble2, memory);
            *d_write(rldouble, memory) = if divisor == 0.0 {
                0.0
            } else {
                d_read(ldouble1, memory) % divisor
            };
            1
        }
        DMUL(ldouble1, ldouble2, rldouble) => {
            *d_write(rldouble, memory) = d_read(ldouble1, memory) * d_read(ldouble2, memory);
            1
        }
        DSUB(ldouble1, ldouble2, rldouble) => {
            *d_write(rldouble, memory) = d_read(ldouble1, memory) - d_read(ldouble2, memory);
            1
        }

        IJEQ(lint1, lint2, code_offset) => {
            if code_offset.offset != 0 && (i_read(lint1, memory) == i_read(lint2, memory)) {
                code_offset.offset
            } else {
                1
            }
        }
        IJNE(lint1, lint2, code_offset) => {
            if code_offset.offset != 0 && i_read(lint1, memory) != i_read(lint2, memory) {
                code_offset.offset
            } else {
                1
            }
        }
        IJGT(lint1, lint2, code_offset) => {
            if code_offset.offset != 0 && i_read(lint1, memory) > i_read(lint2, memory) {
                code_offset.offset
            } else {
                1
            }
        }
        IJLT(lint1, lint2, code_offset) => {
            if code_offset.offset != 0 && i_read(lint1, memory) < i_read(lint2, memory) {
                code_offset.offset
            } else {
                1
            }
        }

        DJEQ(ldouble1, ldouble2, code_offset) => {
            if code_offset.offset != 0 && d_read(ldouble1, memory) == d_read(ldouble2, memory) {
                code_offset.offset
            } else {
                1
            }
        }
        DJNE(ldouble1, ldouble2, code_offset) => {
            if code_offset.offset != 0 && d_read(ldouble1, memory) != d_read(ldouble2, memory) {
                code_offset.offset
            } else {
                1
            }
        }
        DJGT(ldouble1, ldouble2, code_offset) => {
            if code_offset.offset != 0 && d_read(ldouble1, memory) > d_read(ldouble2, memory) {
                code_offset.offset
            } else {
                1
            }
        }
        DJLT(ldouble1, ldouble2, code_offset) => {
            if code_offset.offset != 0 && d_read(ldouble1, memory) < d_read(ldouble2, memory) {
                code_offset.offset
            } else {
                1
            }
        }
    }
}

fn d_read(ldouble: &LeftDouble, memory: &Memory) -> f64 {
    match ldouble {
        LeftDouble::State(index, Mode::Direct) => read(&memory.dstate, *index),
        LeftDouble::Input(index, Mode::Direct) => read(&memory.dinput, *index),
        LeftDouble::Constant(value) => *value,
        LeftDouble::State(index, Mode::Indirect) => {
            let is = indirect(&memory.istate, *index);
            read(&memory.dstate, is)
        }
        LeftDouble::Input(index, Mode::Indirect) => {
            let is = indirect(&memory.istate, *index);
            read(&memory.dinput, is)
        }
    }
}

fn i_read(lint: &LeftInteger, memory: &Memory) -> i64 {
    match lint {
        LeftInteger::State(index, Mode::Direct) => read(&memory.istate, *index),
        LeftInteger::Input(index, Mode::Direct) => read(&memory.iinput, *index),
        LeftInteger::Constant(value) => *value as i64,
        LeftInteger::State(index, Mode::Indirect) => {
            let is = indirect(&memory.istate, *index);
            read(&memory.istate, is)
        }
        LeftInteger::Input(index, Mode::Indirect) => {
            let is = indirect(&memory.istate, *index);
            read(&memory.iinput, is)
        }
    }
}

fn d_write<'a>(rdouble: &RightDouble, memory: &'a mut Memory) -> &'a mut f64 {
    match rdouble {
        RightDouble::State(index, Mode::Direct) => write(&mut memory.dstate, *index),
        RightDouble::Output(index, Mode::Direct) => write(&mut memory.doutput, *index),
        RightDouble::State(index, Mode::Indirect) => {
            let is = indirect(&memory.istate, *index);
            write(&mut memory.dstate, is)
        }
        RightDouble::Output(index, Mode::Indirect) => {
            let is = indirect(&memory.istate, *index);
            write(&mut memory.doutput, is)
        }
    }
}

fn i_write<'a>(rint: &RightInteger, memory: &'a mut Memory) -> &'a mut i64 {
    match rint {
        RightInteger::State(index, Mode::Direct) => write(&mut memory.istate, *index),
        RightInteger::Output(index, Mode::Direct) => write(&mut memory.ioutput, *index),
        RightInteger::State(index, Mode::Indirect) => {
            let is = indirect(&memory.istate, *index);
            write(&mut memory.istate, is)
        }
        RightInteger::Output(index, Mode::Indirect) => {
            let is = indirect(&memory.istate, *index);
            write(&mut memory.ioutput, is)
        }
    }
}

fn read<T: Copy>(bank: &[T], index: u16) -> T {
    bank[index as usize % bank.len()]
}

fn write<T: Copy>(bank: &mut [T], index: u16) -> &mut T {
    let index_bank = index as usize % bank.len();
    &mut bank[index_bank]
}

fn indirect(istate: &[i64], index: u16) -> u16 {
    let index_istate = index as usize % istate.len();
    istate[index_istate] as u16
}
