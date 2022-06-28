use crate::instruction::mode::Mode;

#[derive(Clone, Debug, PartialEq)]
pub enum LeftDouble {
    State(u16, Mode),
    Input(u16, Mode),
    Constant(f64),
}
