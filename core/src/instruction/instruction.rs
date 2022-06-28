use crate::instruction::code_offset::CodeOffset;
use crate::instruction::left_double::LeftDouble;
use crate::instruction::left_integer::LeftInteger;
use crate::instruction::right_double::RightDouble;
use crate::instruction::right_integer::RightInteger;

/// A single virtual machine instruction.
///
#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
    NOP(), // 0
    HLT(), // 1
    SYN(), // 2

    IIMOV(LeftInteger, RightInteger), // 3
    IDMOV(LeftInteger, RightDouble),  // 4
    DIMOV(LeftDouble, RightInteger),  // 5
    DDMOV(LeftDouble, RightDouble),   // 6

    IADD(LeftInteger, LeftInteger, RightInteger), // 7
    IDIV(LeftInteger, LeftInteger, RightInteger), // 8
    IMOD(LeftInteger, LeftInteger, RightInteger), // 9
    IMUL(LeftInteger, LeftInteger, RightInteger), // 10
    ISUB(LeftInteger, LeftInteger, RightInteger), // 11

    DADD(LeftDouble, LeftDouble, RightDouble), // 12
    DDIV(LeftDouble, LeftDouble, RightDouble), // 13
    DMOD(LeftDouble, LeftDouble, RightDouble), // 14
    DMUL(LeftDouble, LeftDouble, RightDouble), // 15
    DSUB(LeftDouble, LeftDouble, RightDouble), // 16

    IJEQ(LeftInteger, LeftInteger, CodeOffset), // 17
    IJNE(LeftInteger, LeftInteger, CodeOffset), // 18
    IJGT(LeftInteger, LeftInteger, CodeOffset), // 19
    IJLT(LeftInteger, LeftInteger, CodeOffset), // 20

    DJEQ(LeftDouble, LeftDouble, CodeOffset), // 21
    DJNE(LeftDouble, LeftDouble, CodeOffset), // 22
    DJGT(LeftDouble, LeftDouble, CodeOffset), // 23
    DJLT(LeftDouble, LeftDouble, CodeOffset), // 24
}
