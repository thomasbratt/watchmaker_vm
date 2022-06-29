use watchmaker_vm::*;

// Provide common test functionality.

const IS_LOGGING_ENABLED: bool = false;

pub fn make_test_architecture() -> Architecture {
    ArchitectureBuilder::default()
        .iinput(3)
        .istate(3)
        .ioutput(3)
        .dinput(3)
        .dstate(3)
        .doutput(3)
        .build()
        .unwrap()
}

pub fn check_equal_i64(actual: i64, expected: i64) {
    if IS_LOGGING_ENABLED {
        println!("actual   : 0x{:X} {:?}", actual as u64, actual);
        println!("expected : 0x{:X} {:?}", expected as u64, expected);
    }
    assert_eq!(actual, expected);
}

pub fn check_equal_u64(actual: u64, expected: u64) {
    if IS_LOGGING_ENABLED {
        println!("actual   : 0x{:X}", actual);
        println!("expected : 0x{:X}", expected);
    }
    assert_eq!(actual, expected);
}

pub fn check_equal_f64(actual: f64, expected: f64, delta: f64) {
    assert!(delta >= 0.0);
    let difference = (actual - expected).abs();
    if IS_LOGGING_ENABLED {
        println!("actual     : {:?}", actual);
        println!("expected   : {:?}", expected);
        println!("difference : {:?}", difference);
        println!("delta      : {:?}", delta);
    }
    assert!(difference <= delta);
}

pub fn check_equal_instruction(actual: Instruction, expected: Instruction) {
    if IS_LOGGING_ENABLED {
        println!("actual   : {:?}", actual);
        println!("expected : {:?}", expected);
    }
    assert_eq!(actual, expected);
}

pub fn count_zeroes(memory: &Memory) -> usize {
    if IS_LOGGING_ENABLED {
        println!("memory: {:?}", memory);
    }
    let mut count = 0;

    for bank in [&memory.iinput, &memory.istate, &memory.ioutput] {
        count += bank.iter().filter(|&x| *x == 0).count();
    }
    for bank in [&memory.dinput, &memory.dstate, &memory.doutput] {
        count += bank.iter().filter(|&x| *x == 0.0).count();
    }

    count
}

pub fn all_lints() -> Vec<LeftInteger> {
    let mut result = Vec::new();
    for index in [u16::MIN, u16::MAX] {
        for mode in [Mode::Indirect, Mode::Direct] {
            result.push(LeftInteger::State(index, mode.clone()));
            result.push(LeftInteger::Input(index, mode.clone()));
        }
    }
    result.push(LeftInteger::Constant(i16::MIN));
    result
}

pub fn all_code_offsets() -> Vec<CodeOffset> {
    let mut result = Vec::new();
    for x in [i16::MIN, i16::MAX, 0, -1, 1] {
        result.push(CodeOffset { offset: x });
    }
    result
}

pub fn all_rints() -> Vec<RightInteger> {
    let mut result = Vec::new();
    for index in [u16::MIN, u16::MAX] {
        for mode in [Mode::Indirect, Mode::Direct] {
            result.push(RightInteger::State(index, mode.clone()));
            result.push(RightInteger::Output(index, mode.clone()));
        }
    }
    result
}

pub fn all_ldoubles() -> Vec<LeftDouble> {
    let mut result = Vec::new();
    for index in [u16::MIN, u16::MAX] {
        for mode in [Mode::Indirect, Mode::Direct] {
            result.push(LeftDouble::State(index, mode.clone()));
            result.push(LeftDouble::Input(index, mode.clone()));
        }
    }
    result.push(LeftDouble::Constant(half::f16::MIN.to_f64()));
    result.push(LeftDouble::Constant(half::f16::MAX.to_f64()));
    result
}

pub fn all_rdoubles() -> Vec<RightDouble> {
    let mut result = Vec::new();
    for index in [u16::MIN, u16::MAX] {
        for mode in [Mode::Indirect, Mode::Direct] {
            result.push(RightDouble::State(index, mode.clone()));
            result.push(RightDouble::Output(index, mode.clone()));
        }
    }
    result
}
