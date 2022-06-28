use crate::instruction::*;
use crate::{
    MASK_VALUE, OFFSET_IS_ADDRESS, OFFSET_IS_INDIRECT, OFFSET_IS_STATE, OFFSET_OPERAND_1,
    OFFSET_OPERAND_2, OFFSET_OPERAND_3, OFFSET_OP_CODE,
};
use half::f16;

// Value when a 16-bit word is not used by an instruction.
const UNUSED_WORD: u64 = 0;

/// Convert from a type-safe and programmer friendly type to a raw u64 bytecode instruction.
pub fn serialize(instruction: &Instruction) -> u64 {
    match instruction {
        NOP() => 0,
        HLT() => 1 << OFFSET_OP_CODE,
        SYN() => 2 << OFFSET_OP_CODE,
        IIMOV(lint, rint) => make_instruction(3, make_lint(lint), make_rint(rint), UNUSED_WORD),
        IDMOV(lint, rdouble) => {
            make_instruction(4, make_lint(lint), make_rdouble(rdouble), UNUSED_WORD)
        }
        DIMOV(ldouble, rint) => {
            make_instruction(5, make_ldouble(ldouble), make_rint(rint), UNUSED_WORD)
        }
        DDMOV(ldouble, rdouble) => {
            make_instruction(6, make_ldouble(ldouble), make_rdouble(rdouble), UNUSED_WORD)
        }
        IADD(lint1, lint2, rint) => {
            make_instruction(7, make_lint(lint1), make_lint(lint2), make_rint(rint))
        }
        IDIV(lint1, lint2, rint) => {
            make_instruction(8, make_lint(lint1), make_lint(lint2), make_rint(rint))
        }
        IMOD(lint1, lint2, rint) => {
            make_instruction(9, make_lint(lint1), make_lint(lint2), make_rint(rint))
        }
        IMUL(lint1, lint2, rint) => {
            make_instruction(10, make_lint(lint1), make_lint(lint2), make_rint(rint))
        }
        ISUB(lint1, lint2, rint) => {
            make_instruction(11, make_lint(lint1), make_lint(lint2), make_rint(rint))
        }
        DADD(ldouble1, ldouble2, rdouble) => make_instruction(
            12,
            make_ldouble(ldouble1),
            make_ldouble(ldouble2),
            make_rdouble(rdouble),
        ),
        DDIV(ldouble1, ldouble2, rdouble) => make_instruction(
            13,
            make_ldouble(ldouble1),
            make_ldouble(ldouble2),
            make_rdouble(rdouble),
        ),
        DMOD(ldouble1, ldouble2, rdouble) => make_instruction(
            14,
            make_ldouble(ldouble1),
            make_ldouble(ldouble2),
            make_rdouble(rdouble),
        ),
        DMUL(ldouble1, ldouble2, rdouble) => make_instruction(
            15,
            make_ldouble(ldouble1),
            make_ldouble(ldouble2),
            make_rdouble(rdouble),
        ),
        DSUB(ldouble1, ldouble2, rdouble) => make_instruction(
            16,
            make_ldouble(ldouble1),
            make_ldouble(ldouble2),
            make_rdouble(rdouble),
        ),
        IJEQ(lint1, lint2, code_offset) => make_instruction(
            17,
            make_lint(lint1),
            make_lint(lint2),
            make_code_offset(code_offset.offset),
        ),
        IJNE(lint1, lint2, code_offset) => make_instruction(
            18,
            make_lint(lint1),
            make_lint(lint2),
            make_code_offset(code_offset.offset),
        ),
        IJGT(lint1, lint2, code_offset) => make_instruction(
            19,
            make_lint(lint1),
            make_lint(lint2),
            make_code_offset(code_offset.offset),
        ),
        IJLT(lint1, lint2, code_offset) => make_instruction(
            20,
            make_lint(lint1),
            make_lint(lint2),
            make_code_offset(code_offset.offset),
        ),
        DJEQ(ldouble1, ldouble2, code_offset) => make_instruction(
            21,
            make_ldouble(ldouble1),
            make_ldouble(ldouble2),
            make_code_offset(code_offset.offset),
        ),
        DJNE(ldouble1, ldouble2, code_offset) => make_instruction(
            22,
            make_ldouble(ldouble1),
            make_ldouble(ldouble2),
            make_code_offset(code_offset.offset),
        ),
        DJGT(ldouble1, ldouble2, code_offset) => make_instruction(
            23,
            make_ldouble(ldouble1),
            make_ldouble(ldouble2),
            make_code_offset(code_offset.offset),
        ),
        DJLT(ldouble1, ldouble2, code_offset) => make_instruction(
            24,
            make_ldouble(ldouble1),
            make_ldouble(ldouble2),
            make_code_offset(code_offset.offset),
        ),
    }
}

pub fn make_instruction(op_code: u64, operand1: u64, operand2: u64, operand3: u64) -> u64 {
    (op_code << OFFSET_OP_CODE)
        | (operand1 << OFFSET_OPERAND_1)
        | (operand2 << OFFSET_OPERAND_2)
        | (operand3 << OFFSET_OPERAND_3)
}

fn make_operand(is_address: bool, is_indirect: bool, is_state: bool, value: u16) -> u64 {
    (if is_address {
        1 << OFFSET_IS_ADDRESS
    } else {
        0
    }) | (if is_indirect {
        1 << OFFSET_IS_INDIRECT
    } else {
        0
    }) | (if is_state { 1 << OFFSET_IS_STATE } else { 0 })
        | value as u64
}

fn make_ldouble(ldouble: &LeftDouble) -> u64 {
    match ldouble {
        LeftDouble::State(index, mode) => make_operand(true, *mode == Mode::Indirect, true, *index),
        LeftDouble::Input(index, mode) => {
            make_operand(true, *mode == Mode::Indirect, false, *index)
        }
        LeftDouble::Constant(value) => {
            make_operand(false, false, false, f16::from_f64(*value).to_bits())
        }
    }
}

fn make_lint(lint: &LeftInteger) -> u64 {
    match lint {
        LeftInteger::State(index, mode) => {
            make_operand(true, *mode == Mode::Indirect, true, *index)
        }
        LeftInteger::Input(index, mode) => {
            make_operand(true, *mode == Mode::Indirect, false, *index)
        }
        LeftInteger::Constant(value) => make_operand(false, false, false, *value as u16),
    }
}

fn make_rdouble(rdouble: &RightDouble) -> u64 {
    match rdouble {
        RightDouble::State(index, mode) => {
            make_operand(true, *mode == Mode::Indirect, true, *index)
        }
        RightDouble::Output(index, mode) => {
            make_operand(true, *mode == Mode::Indirect, false, *index)
        }
    }
}

fn make_rint(rint: &RightInteger) -> u64 {
    match rint {
        RightInteger::State(index, mode) => {
            make_operand(true, *mode == Mode::Indirect, true, *index)
        }
        RightInteger::Output(index, mode) => {
            make_operand(true, *mode == Mode::Indirect, false, *index)
        }
    }
}

fn make_code_offset(code_offset: i16) -> u64 {
    (code_offset as u64) & MASK_VALUE
}
