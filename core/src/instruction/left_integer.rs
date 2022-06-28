use crate::instruction::mode::Mode;

#[derive(Clone, Debug, PartialEq)]
pub enum LeftInteger {
    State(u16, Mode),
    Input(u16, Mode),
    Constant(i16),
}
