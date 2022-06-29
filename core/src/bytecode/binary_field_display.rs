// Indented formatting for bit positions within a u64.
#[derive(Clone, Debug, PartialEq)]
pub struct BinaryFieldDisplay {
    raw: u64,
}

impl BinaryFieldDisplay {
    pub fn from(raw: u64) -> Self {
        Self { raw }
    }

    pub fn all(&self) -> String {
        format!("{:064b}", self.raw)
    }

    pub fn at(&self, offset: usize, length: usize) -> String {
        let value = (self.raw >> offset) & make_mask(length);
        let pad = 64 - (offset + length);

        if pad == 0 {
            format!("{0:>01$b}", value, length)
        } else {
            format!("{0:>1$}{2:>03$b}", " ", pad, value, length)
        }
    }

    pub fn raw(&self) -> u64 {
        self.raw
    }
}

fn make_mask(length: usize) -> u64 {
    let mut result = 0;
    for _ in 0..length {
        result = (result << 1) | 1;
    }
    result
}

// TODO: write up in google docs
// fn dump_original(raw: u64) {
//     let fields = watchmaker_vm::extract_fields(raw);
//     println!();
//     println!("deserialized    : {:?}", watchmaker_vm::deserialize(raw));
//     println!("raw             : {:064b}", fields.raw);
//     println!("op_code         : {:>07b}", fields.op_code);
//     println!("operand1    : {:>7}{:>019b}", " ", fields.operand1.raw);
//     println!("operand1    : {:>26}{:>019b}", " ", fields.operand2.raw);
//     println!("operand1    : {:>45}{:>019b}", " ", fields.operand3.raw);
// }
