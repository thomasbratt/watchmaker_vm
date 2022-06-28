use clockwork::*;

// Test extraction and display of bits from raw u64 instruction values.
// TODO: actually assert that output strings match expected values.

#[test]
fn extract_nop() {
    let instruction = Instruction::NOP();
    println!();
    println!("instruction    : {:?}", instruction);
    println!(
        "{}",
        BytecodeDisplay::from(clockwork::serialize(&instruction))
    );
}

#[test]
fn extract_idmov() {
    let instruction = Instruction::IDMOV(
        LeftInteger::Constant(42),
        RightDouble::Output(1, Mode::Direct),
    );
    println!();
    println!("instruction    : {:?}", instruction);
    println!(
        "{}",
        BytecodeDisplay::from(clockwork::serialize(&instruction))
    );
}

#[test]
fn extract_dsub() {
    let instruction = Instruction::DSUB(
        LeftDouble::Constant(100.456),
        LeftDouble::Constant(42.123),
        RightDouble::State(1234, Mode::Indirect),
    );
    println!();
    println!("instruction    : {:?}", instruction);
    println!(
        "{}",
        BytecodeDisplay::from(clockwork::serialize(&instruction))
    );
}

#[test]
fn dump_idmov() {
    let instruction = Instruction::IDMOV(
        LeftInteger::Constant(42),
        RightDouble::Output(1, Mode::Direct),
    );
    println!();
    println!("instruction    : {:?}", instruction);
    println!(
        "{}",
        BytecodeDisplay::from(clockwork::serialize(&instruction))
    );
}
