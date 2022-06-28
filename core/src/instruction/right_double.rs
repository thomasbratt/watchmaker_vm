use crate::instruction::mode::Mode;

#[derive(Clone, Debug, PartialEq)]
pub enum RightDouble {
    State(u16, Mode),
    Output(u16, Mode),
}
