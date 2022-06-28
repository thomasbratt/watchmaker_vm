/// Specifies the memory architecture of a virtual machine.
///
#[derive(Clone, Debug, PartialEq)]
pub struct Architecture {
    /// II: linear array of integer input registers
    pub iinput: u16,
    /// IS: linear array of integer state registers
    pub istate: u16,
    /// IO: linear array of integer output registers
    pub ioutput: u16,
    /// DI: linear array of double precision floating point input registers
    pub dinput: u16,
    /// DS: linear array of double precision floating point state registers
    pub dstate: u16,
    /// DO: linear array of double precision floating point output registers
    pub doutput: u16,
}
