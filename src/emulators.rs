use std::fs;

pub mod chip_8 {
    pub struct Chip8 {
        pub current_opcode: u16,
        pub memory: [u8; 4096],
        pub cpu_registers: [u8; 16],
        pub index_register: u16,
        pub program_counter: u16, // Holds the address of the next instruction to execute
        pub execution_stack: [u16; 16],
        pub stack_pointer: u8,
        pub delay_timer: u8, // If it's zero it stays zero, otherwise it counts down to zero at 60Hz
        pub sound_timer: u8, // If it's zero it stays zero, otherwise it decrements and makes a sound every time it does
        pub keypad: [u8; 16],
        pub graphics: [u32; 64 * 32]
    }

    impl Chip8 {
        pub fn load_ROM(&mut self, file_name: &str) {   
            const START_ADDRESS: usize = 0x200;

            let bytes = std::fs::read(file_name).expect("File not found");
            for (index, &byte) in bytes.iter().enumerate() {
                self.memory[START_ADDRESS + index] = byte;
            }
        }
    }

    pub fn initialize_chip_8() -> Chip8 {
        return Chip8 {
            cpu_registers: [0; 16],
            memory: [0; 4096],
            index_register: 0,
            program_counter: 0,
            execution_stack: [0; 16],
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [0; 16],
            graphics: [0; 64 * 32],
            current_opcode: 0
        };
    }
}

pub mod nes {
    pub struct Nes {

    }
}