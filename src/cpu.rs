use std::fs;
pub const START_ADDRESS: u16 = 0x200;

pub struct Cpu {
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

impl Cpu {
    pub fn new() -> Cpu {
        let mut chip8 = Cpu {
            cpu_registers: [0; 16],
            memory: [0; 4096],
            index_register: 0,
            program_counter: START_ADDRESS,
            execution_stack: [0; 16],
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [0; 16],
            graphics: [0; 64 * 32],
            current_opcode: 0
        };
    
        chip8.initialize_fontset();
    
        return chip8;
    }

    pub fn load_ROM(&mut self, file_name: &str) {
        let bytes = std::fs::read(file_name).expect("File not found");
        for (index, &byte) in bytes.iter().enumerate() {
            self.memory[START_ADDRESS as usize + index] = byte;
        }
    }

    fn initialize_fontset(&mut self) {
        const FONT_START_ADDRESS: usize = 0x50;
        let font_set: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];

        for (index, &b) in font_set.iter().enumerate() {
            self.memory[FONT_START_ADDRESS + index] = b;
        }
    }
}