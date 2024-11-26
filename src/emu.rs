const DISPLAY_INDEX: u16 = 0xF00;
const STACK_INDEX: u16 = 0xEA0;
struct CPUState {
    ram: [u8; 4096],
    vregisters: [u8; 16],
    iregister: u16,
    stack_pointer: u16,
    program_counter: u16,
    delay_timer: u8,
    sound_timer: u8
}

fn init_cpu(cpu: &mut CPUState) {
    cpu.stack_pointer = 0xFA0;
    cpu.program_counter = 0x200;
}