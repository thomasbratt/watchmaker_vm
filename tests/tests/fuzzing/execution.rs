// Test execution with randomly generated instructions.

#[cfg(feature = "fuzzing")]
#[test]
fn fuzz_execute_noise() {
    for _ in 0..(1 << 10) {
        let mut vm = tests::create_random_virtual_machine(32, 4);
        vm.run(256);
    }
}
