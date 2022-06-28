use crate::bytecode::bits::extract_fields;
use crate::*;

/// Convert from a raw u64 bytecode instruction to a type-safe and programmer friendly type.
///
/// Each instruction consists of 64 bits, divided as follows:
///
///   7 bit opcode    - the instruction field
///  19 bit operand 1 - as tuple (is_address:1-bit, is_indirect:1-bit, is_state:1-bit, value:16-bit)
///  19 bit operand 2 - as tuple (is_address:1-bit, is_indirect:1-bit, is_state:1-bit, value:16-bit)
///  19 bit operand 3 - as tuple (is_address:1-bit, is_indirect:1-bit, is_state:1-bit, value:16-bit)
pub fn deserialize(raw: u64) -> Instruction {
    let fields = extract_fields(raw);
    match fields.op_code {
        1 => HLT(),
        2 => SYN(),

        3 => IIMOV(lint(fields.operand1), rint(fields.operand2)),
        4 => IDMOV(lint(fields.operand1), rdouble(fields.operand2)),
        5 => DIMOV(ldouble(fields.operand1), rint(fields.operand2)),
        6 => DDMOV(ldouble(fields.operand1), rdouble(fields.operand2)),

        7 => IADD(
            lint(fields.operand1),
            lint(fields.operand2),
            rint(fields.operand3),
        ),
        8 => IDIV(
            lint(fields.operand1),
            lint(fields.operand2),
            rint(fields.operand3),
        ),
        9 => IMOD(
            lint(fields.operand1),
            lint(fields.operand2),
            rint(fields.operand3),
        ),
        10 => IMUL(
            lint(fields.operand1),
            lint(fields.operand2),
            rint(fields.operand3),
        ),
        11 => ISUB(
            lint(fields.operand1),
            lint(fields.operand2),
            rint(fields.operand3),
        ),

        12 => DADD(
            ldouble(fields.operand1),
            ldouble(fields.operand2),
            rdouble(fields.operand3),
        ),
        13 => DDIV(
            ldouble(fields.operand1),
            ldouble(fields.operand2),
            rdouble(fields.operand3),
        ),
        14 => DMOD(
            ldouble(fields.operand1),
            ldouble(fields.operand2),
            rdouble(fields.operand3),
        ),
        15 => DMUL(
            ldouble(fields.operand1),
            ldouble(fields.operand2),
            rdouble(fields.operand3),
        ),
        16 => DSUB(
            ldouble(fields.operand1),
            ldouble(fields.operand2),
            rdouble(fields.operand3),
        ),

        17 => IJEQ(
            lint(fields.operand1),
            lint(fields.operand2),
            code_offset(fields.operand3),
        ),
        18 => IJNE(
            lint(fields.operand1),
            lint(fields.operand2),
            code_offset(fields.operand3),
        ),
        19 => IJGT(
            lint(fields.operand1),
            lint(fields.operand2),
            code_offset(fields.operand3),
        ),
        20 => IJLT(
            lint(fields.operand1),
            lint(fields.operand2),
            code_offset(fields.operand3),
        ),

        21 => DJEQ(
            ldouble(fields.operand1),
            ldouble(fields.operand2),
            code_offset(fields.operand3),
        ),
        22 => DJNE(
            ldouble(fields.operand1),
            ldouble(fields.operand2),
            code_offset(fields.operand3),
        ),
        23 => DJGT(
            ldouble(fields.operand1),
            ldouble(fields.operand2),
            code_offset(fields.operand3),
        ),
        24 => DJLT(
            ldouble(fields.operand1),
            ldouble(fields.operand2),
            code_offset(fields.operand3),
        ),
        _ => NOP(),
    }
}

fn lint(operand: OperandFields) -> LeftInteger {
    if operand.is_address {
        let mode = if operand.is_indirect {
            Mode::Indirect
        } else {
            Mode::Direct
        };
        if operand.is_state {
            LeftInteger::State(operand.value, mode)
        } else {
            LeftInteger::Input(operand.value, mode)
        }
    } else {
        LeftInteger::Constant(operand.value as i16)
    }
}

fn rint(operand: OperandFields) -> RightInteger {
    let mode = if operand.is_indirect {
        Mode::Indirect
    } else {
        Mode::Direct
    };
    if operand.is_state {
        RightInteger::State(operand.value, mode)
    } else {
        RightInteger::Output(operand.value, mode)
    }
}

fn ldouble(operand: OperandFields) -> LeftDouble {
    if operand.is_address {
        let mode = if operand.is_indirect {
            Mode::Indirect
        } else {
            Mode::Direct
        };
        if operand.is_state {
            LeftDouble::State(operand.value, mode)
        } else {
            LeftDouble::Input(operand.value, mode)
        }
    } else {
        LeftDouble::Constant(half::f16::from_bits(operand.value).to_f64())
    }
}

fn rdouble(operand: OperandFields) -> RightDouble {
    let mode = if operand.is_indirect {
        Mode::Indirect
    } else {
        Mode::Direct
    };
    if operand.is_state {
        RightDouble::State(operand.value, mode)
    } else {
        RightDouble::Output(operand.value, mode)
    }
}

fn code_offset(operand: OperandFields) -> CodeOffset {
    CodeOffset {
        offset: operand.value as i16,
    }
}
