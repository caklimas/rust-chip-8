const START_ADDRESS: u16 = 0x200;

pub struct Cpu {
    pub current_opcode: u16,
    pub memory: [u8; 4096],
    pub cpu_registers: [u8; 16],
    pub index_register: u16,
    pub program_counter: u16, // Holds the address of the next instruction to execute
    pub execution_stack: [u16; 16],
    pub stack_pointer: usize,
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

    pub fn load_rom(&mut self, bytes: Vec<u8>) {
        for (index, &byte) in bytes.iter().enumerate() {
            self.memory[START_ADDRESS as usize + index] = byte;
        }
    }

    /// CLS - Clears the display
    pub fn op_00E0(&mut self) {
        for i in 0..self.graphics.len() {
            self.graphics[i] = 0;
        }
    }

    /// RET - Sets program counter to top of stack and then decrements pointer 
    pub fn op_00EE(&mut self) {
        self.program_counter = self.execution_stack[self.stack_pointer];
        self.stack_pointer = self.stack_pointer - 1;
    }

    /// JP addr - Sets program counter to nnn
    pub fn op_1nnn(&mut self) {
        let address = self.current_opcode & 0x0FFF;
        self.program_counter = address;
    }

    /// CALL addr - Increments the pointer and sets execution stack to program counter.
    /// It then sets the program counter to nnn
    pub fn op_2nnn(&mut self) {
        let address = self.current_opcode & 0x0FFF;
        self.stack_pointer = self.stack_pointer + 1;
        self.execution_stack[self.stack_pointer] = self.program_counter;
        self.program_counter = address;
    }

    pub fn op_3xkk(&mut self) {
        let x = (self.current_opcode & 0x0F00) >> 8;
        let kk = (self.current_opcode & 0x00FF) as u8;

        if self.cpu_registers[x as usize] != kk {
            return;
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