use rand::Rng;
const START_ADDRESS: u16 = 0x200;
const VIDEO_WIDTH: u8 = 64;
const VIDEO_HEIGHT: u8 = 32;

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
    pub graphics: [[u8; VIDEO_WIDTH as usize]; VIDEO_HEIGHT as usize]
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
            graphics: [[0; VIDEO_WIDTH as usize]; VIDEO_HEIGHT as usize],
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
            for j in 0..self.graphics[i].len() {
                self.graphics[i][j] = 0;
            }
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

    /// SE Vx, byte - if Vx equals kk then increment program counter by 2
    pub fn op_3xkk(&mut self) {
        let x = self.get_x();
        let kk = self.get_kk();

        if self.cpu_registers[x as usize] != kk {
            return;
        }

        self.program_counter = self.program_counter + 2;
    }

    /// SNE Vx, byte - if Vx does not equal kk then increment program counter by 2
    pub fn op_4xkk(&mut self) {
        let x = self.get_x();
        let kk = self.get_kk();

        if self.cpu_registers[x as usize] == kk {
            return;
        }

        self.program_counter = self.program_counter + 2;
    }

    /// SE Vx, Vy - Compare Vx to Vy. If they are equal, then increment counter by 2
    pub fn op_5xy0(&mut self) {
        let x = self.get_x();
        let y = self.get_y();

        if self.cpu_registers[x as usize] != self.cpu_registers[y as usize] {
            return;
        }

        self.program_counter = self.program_counter + 2;
    }

    /// LD Vx, byte - Sets Vx to kk 
    pub fn op_6xkk(&mut self) {
        let x = self.get_x();
        let kk = self.get_kk();

        self.cpu_registers[x as usize] = kk;
    }

    /// ADD Vx, byte - Adds kk to Vx
    pub fn op_7xkk(&mut self) {
        let x = self.get_x();
        let kk = self.get_kk();

        self.cpu_registers[x] = self.cpu_registers[x] + kk;
    }

    /// LD Vx, Vy - Sets Vx to Vy
    pub fn op_8xy09(&mut self) {
        let x = self.get_x();
        let y = self.get_y();

        self.cpu_registers[x] = self.cpu_registers[y];
    }

    /// OR Vx, Vy - Does a bitwise OR on Vx and Vy and stores it in Vx
    pub fn op_8xy1(&mut self) {
        let x = self.get_x();
        let y = self.get_y();

        self.cpu_registers[x] = self.cpu_registers[x] | self.cpu_registers[y]; 
    }

    /// AND Vx, Vy - Does a bitwise AND on Vx and Vy and stores it in Vx
    pub fn op_8xy2(&mut self) {
        let x = self.get_x();
        let y = self.get_y();

        self.cpu_registers[x] = self.cpu_registers[x] & self.cpu_registers[y]; 
    }

    /// XOR Vx, Vy - Does a bitwise XOR on Vx and Vy and stores it in Vx
    pub fn op_8xy3(&mut self) {
        let x = self.get_x();
        let y = self.get_y();

        self.cpu_registers[x] = self.cpu_registers[x] ^ self.cpu_registers[y];
    }

    /// ADD Vx, Vy
    pub fn op_8xy4(&mut self) {
        let x = self.get_x();
        let y = self.get_y();

        let sum = self.cpu_registers[x] + self.cpu_registers[y];
        self.cpu_registers[0xF as usize] = if sum > (0xFF as u8) { 1 } else { 0 };
        self.cpu_registers[x] = sum & 0xFF;
    }

    /// SUB Vx, Vy
    pub fn op_8xy5(&mut self) {
        let x = self.get_x();
        let y = self.get_y();

        if self.cpu_registers[x] > self.cpu_registers[y] {
            self.cpu_registers[0xF as usize] = 1;
        } else {
            self.cpu_registers[0xF as usize] = 0;
        }

        self.cpu_registers[x] = self.cpu_registers[x] - self.cpu_registers[y];
    }

    /// SHR Vx {, Vy}
    pub fn op_8xy6(&mut self) {
        let x = self.get_x();

        self.cpu_registers[0xF as usize] = self.cpu_registers[x] & 0x1;
        self.cpu_registers[x] = self.cpu_registers[x] >> 1;
    }

    /// SUBN Vx, Vy
    pub fn op_8xy7(&mut self) {
        let x = self.get_x();
        let y = self.get_y();

        if self.cpu_registers[y] > self.cpu_registers[x] {
            self.cpu_registers[0xF as usize] = 1;
        } else {
            self.cpu_registers[0xF as usize] = 0;
        }

        self.cpu_registers[x] = self.cpu_registers[y] - self.cpu_registers[x];
    }

    /// SHL Vx {, Vy}
    pub fn op_8xyE(&mut self) {
        let x = self.get_x();

        self.cpu_registers[0xF as usize] = (self.cpu_registers[x] & 0x80) >> 7;
        self.cpu_registers[x] = self.cpu_registers[x] << 1;
    }

    /// SNE Vx, Vy
    pub fn op_9xy0(&mut self) {
        let x = self.get_x();
        let y = self.get_y();

        if (self.cpu_registers[x] == self.cpu_registers[y]) {
            return;
        }

        self.program_counter = self.program_counter + 2;
    }

    /// LD I, addr
    pub fn op_Annn(&mut self) {
        let address = self.current_opcode & 0x0FFF;
        self.index_register = address;
    }

    /// JP V0, addr
    pub fn op_Bnnn(&mut self) {
        let address = self.current_opcode & 0x0FFF;
        self.program_counter = (self.cpu_registers[0] as u16) + address;
    }

    /// RND Vx, byte
    pub fn op_Cxkk(&mut self) {
        let x = self.get_x();
        let kk = self.get_kk();
        let mut rand = rand::thread_rng();
        let random = rand.gen_range(0, 255);

        self.cpu_registers[x] = kk & random;
    }

    /// DRW Vx, Vy, nibble
    pub fn op_Dxyn(&mut self) {
        let x = self.get_x();
        let y = self.get_y();
        let n = self.current_opcode & 0xF;

        self.cpu_registers[0xF] = 0;
        
        for byte in 0..n {
            let y_pos = (self.cpu_registers[y + byte as usize] % VIDEO_HEIGHT) as usize;
            let sprite_byte = self.memory[(self.index_register + byte) as usize];
            for bit in 0..8 {
                let x_pos = (self.cpu_registers[x + bit] % VIDEO_WIDTH) as usize;
                let color = (sprite_byte >> (7 - bit)) & 1;
                self.cpu_registers[0xF] |= color & self.graphics[y_pos as usize][x_pos as usize];
                self.graphics[y_pos as usize][x_pos as usize];
            }
        }
    }

    fn get_x(&mut self) -> usize {
        ((self.current_opcode & 0x0F00) >> 8) as usize
    }

    fn get_y(&mut self) -> usize {
        ((self.current_opcode & 0x00F0) >> 4) as usize
    }

    fn get_kk(&mut self) -> u8 {
        (self.current_opcode & 0x00FF) as u8
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