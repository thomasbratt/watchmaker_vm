use crate::instruction::left_double::LeftDouble;
use crate::instruction::left_integer::LeftInteger;
use crate::instruction::right_double::RightDouble;
use crate::instruction::right_integer::RightInteger;
use crate::CodeOffset;

#[derive(Clone, Debug, PartialEq)]
pub enum Operand {
    LINT(LeftInteger),
    RINT(RightInteger),
    LDOUBLE(LeftDouble),
    RDOUBLE(RightDouble),
    CADDR(CodeOffset),
}
