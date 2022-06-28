use crate::execution::architecture::Architecture;

// Defines the memory architecture specification of a virtual machine.
//
#[derive(Clone, Default, Debug, PartialEq)]
pub struct ArchitectureBuilder {
    // II: linear array of integer input registers
    pub iinput: u16,
    // IS: linear array of integer state registers
    pub istate: u16,
    // IO: linear array of integer output registers
    pub ioutput: u16,
    // DI: linear array of double precision floating point input registers
    pub dinput: u16,
    // DS: linear array of double precision floating point state registers
    pub dstate: u16,
    // DO: linear array of double precision floating point output registers
    pub doutput: u16,
}

impl ArchitectureBuilder {
    // II: linear array of integer input registers
    pub fn iinput(mut self, iinput: u16) -> Self {
        self.iinput = iinput;
        self
    }

    // IS: linear array of integer state registers
    pub fn istate(mut self, istate: u16) -> Self {
        self.istate = istate;
        self
    }

    // IO: linear array of integer output registers
    pub fn ioutput(mut self, ioutput: u16) -> Self {
        self.ioutput = ioutput;
        self
    }

    // DI: linear array of double precision floating point input registers
    pub fn dinput(mut self, dinput: u16) -> Self {
        self.dinput = dinput;
        self
    }

    // DS: linear array of double precision floating point state registers
    pub fn dstate(mut self, dstate: u16) -> Self {
        self.dstate = dstate;
        self
    }

    // DO: linear array of double precision floating point output registers
    pub fn doutput(mut self, doutput: u16) -> Self {
        self.doutput = doutput;
        self
    }

    pub fn build(self) -> Result<Architecture, String> {
        if self.iinput == 0 {
            return Result::Err(format!("iinput size invalid: {}", self.iinput));
        }
        if self.istate == 0 {
            return Result::Err(format!("istate size invalid: {}", self.istate));
        }
        if self.ioutput == 0 {
            return Result::Err(format!("ioutput size invalid: {}", self.ioutput));
        }
        if self.dinput == 0 {
            return Result::Err(format!("dinput size invalid: {}", self.dinput));
        }
        if self.dstate == 0 {
            return Result::Err(format!("dstate size invalid: {}", self.dstate));
        }
        if self.doutput == 0 {
            return Result::Err(format!("doutput size invalid: {}", self.doutput));
        }
        Result::Ok(Architecture {
            iinput: self.iinput,
            istate: self.istate,
            ioutput: self.ioutput,
            dinput: self.dinput,
            dstate: self.dstate,
            doutput: self.doutput,
        })
    }
}
