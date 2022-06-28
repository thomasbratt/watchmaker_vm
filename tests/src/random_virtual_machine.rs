use clockwork::*;
use rand::RngCore;

pub fn create_random_virtual_machine(instructions: usize, memory: u16) -> VirtualMachine {
    let architecture = ArchitectureBuilder::default()
        .iinput(memory)
        .istate(memory)
        .ioutput(memory)
        .dinput(memory)
        .dstate(memory)
        .doutput(memory)
        .build()
        .unwrap();
    let instructions = (0..instructions)
        .into_iter()
        .map(|_| {
            let raw = rand::thread_rng().next_u64();
            deserialize(raw)
        })
        .collect();
    VirtualMachine::new(&architecture, instructions)
}
