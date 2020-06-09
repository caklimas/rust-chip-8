use rand::Rng;
const START_ADDRESS: u16 = 0x200;
const FONT_START_ADDRESS: usize = 0x50;

pub const VIDEO_WIDTH: u8 = 64;
pub const VIDEO_HEIGHT: u8 = 32;

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
    pub fn new() -> Self {
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

    pub fn cycle(&mut self) {
        self.fetch_opcode();
        self.execute_opcode();
        self.update_timers();
    }

    fn fetch_opcode(&mut self) {
        let first = self.memory[self.program_counter as usize] as u16;
        let second = self.memory[(self.program_counter + 1) as usize] as u16;
        let opcode = (first << 8) | (second & 0xFF);

        self.current_opcode = opcode;
    }

    fn execute_opcode(&mut self) {
        let opcode_parts = (
            (self.current_opcode & 0xF000) >> 12,
            (self.current_opcode & 0x0F00) >> 8,
            (self.current_opcode & 0x00F0) >> 4,
            (self.current_opcode & 0x000F) 
        );

        match opcode_parts {
            (0x0, 0x0, 0xE, 0x0) => self.op_00E0(),
            (0x0, 0x0, 0xE, 0xE) => self.op_00EE(),
            (0x1, _, _, _) => self.op_1nnn(),
            (0x2, _, _, _) => self.op_2nnn(),
            (0x3, _, _, _) => self.op_3xkk(),
            (0x4, _, _, _) => self.op_4xkk(),
            (0x5, _, _, _) => self.op_5xy0(),
            (0x6, _, _, _) => self.op_6xkk(),
            (0x7, _, _, _) => self.op_7xkk(),
            (0x8, _, _, 0x0) => self.op_8xy0(),
            (0x8, _, _, 0x1) => self.op_8xy1(),
            (0x8, _, _, 0x2) => self.op_8xy2(),
            (0x8, _, _, 0x3) => self.op_8xy3(),
            (0x8, _, _, 0x4) => self.op_8xy4(),
            (0x8, _, _, 0x5) => self.op_8xy5(),
            (0x8, _, _, 0x6) => self.op_8xy6(),
            (0x8, _, _, 0x7) => self.op_8xy7(),
            (0x8, _, _, 0xE) => self.op_8xyE(),
            (0x9, _, _, _) => self.op_9xy0(),
            (0xA, _, _, _) => self.op_Annn(),
            (0xB, _, _, _) => self.op_Bnnn(),
            (0xC, _, _, _) => self.op_Cxkk(),
            (0xD, _, _, _) => self.op_Dxyn(),
            (0xE, _, 0x9, 0xE) => self.op_Ex9E(),
            (0xE, _, 0xA, 0x1) => self.op_ExA1(),
            (0xF, _, 0x0, 0x7) => self.op_Fx07(),
            (0xF, _, 0x0, 0xA) => self.op_Fx0A(),
            (0xF, _, 0x1, 0x5) => self.op_Fx15(),
            (0xF, _, 0x1, 0x8) => self.op_Fx18(),
            (0xF, _, 0x1, 0xE) => self.op_Fx1E(),
            (0xF, _, 0x2, 0x9) => self.op_Fx29(),
            (0xF, _, 0x3, 0x3) => self.op_Fx33(),
            (0xF, _, 0x5, 0x5) => self.op_Fx55(),
            (0xF, _, 0x6, 0x5) => self.op_Fx65(),
            _ => panic!("Unrecognized opcode")
        };
    }

    fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    /// CLS - Clears the display
    fn op_00E0(&mut self) {
        for i in 0..self.graphics.len() {
            for j in 0..self.graphics[i].len() {
                self.graphics[i][j] = 0;
            }
        }
    }

    /// RET - Sets program counter to top of stack and then decrements pointer 
    fn op_00EE(&mut self) {
        self.program_counter = self.execution_stack[self.stack_pointer];
        self.stack_pointer = self.stack_pointer - 1;
    }

    /// JP addr - Sets program counter to nnn
    fn op_1nnn(&mut self) {
        let address = self.current_opcode & 0x0FFF;
        self.program_counter = address;
    }

    /// CALL addr - Increments the pointer and sets execution stack to program counter.
    /// It then sets the program counter to nnn
    fn op_2nnn(&mut self) {
        let address = self.current_opcode & 0x0FFF;
        self.stack_pointer = self.stack_pointer + 1;
        self.execution_stack[self.stack_pointer] = self.program_counter;
        self.program_counter = address;
    }

    /// SE Vx, byte - if Vx equals kk then increment program counter by 2
    fn op_3xkk(&mut self) {
        let x = self.get_x();
        let kk = self.get_kk();

        if self.cpu_registers[x as usize] != kk {
            return;
        }

        self.program_counter += 2;
    }

    /// SNE Vx, byte - if Vx does not equal kk then increment program counter by 2
    fn op_4xkk(&mut self) {
        let x = self.get_x();
        let kk = self.get_kk();

        if self.cpu_registers[x as usize] == kk {
            return;
        }

        self.program_counter += 2;
    }

    /// SE Vx, Vy - Compare Vx to Vy. If they are equal, then increment counter by 2
    fn op_5xy0(&mut self) {
        let x = self.get_x();
        let y = self.get_y();

        if self.cpu_registers[x as usize] != self.cpu_registers[y as usize] {
            return;
        }

        self.program_counter += 2;
    }

    /// LD Vx, byte - Sets Vx to kk 
    fn op_6xkk(&mut self) {
        let x = self.get_x();
        let kk = self.get_kk();

        self.cpu_registers[x as usize] = kk;
    }

    /// ADD Vx, byte - Adds kk to Vx
    fn op_7xkk(&mut self) {
        let x = self.get_x();
        let kk = self.get_kk();

        self.cpu_registers[x] = self.cpu_registers[x] + kk;
    }

    /// LD Vx, Vy - Sets Vx to Vy
    fn op_8xy0(&mut self) {
        let x = self.get_x();
        let y = self.get_y();

        self.cpu_registers[x] = self.cpu_registers[y];
    }

    /// OR Vx, Vy - Does a bitwise OR on Vx and Vy and stores it in Vx
    fn op_8xy1(&mut self) {
        let x = self.get_x();
        let y = self.get_y();

        self.cpu_registers[x] = self.cpu_registers[x] | self.cpu_registers[y]; 
    }

    /// AND Vx, Vy - Does a bitwise AND on Vx and Vy and stores it in Vx
    fn op_8xy2(&mut self) {
        let x = self.get_x();
        let y = self.get_y();

        self.cpu_registers[x] = self.cpu_registers[x] & self.cpu_registers[y]; 
    }

    /// XOR Vx, Vy - Does a bitwise XOR on Vx and Vy and stores it in Vx
    fn op_8xy3(&mut self) {
        let x = self.get_x();
        let y = self.get_y();

        self.cpu_registers[x] = self.cpu_registers[x] ^ self.cpu_registers[y];
    }

    /// ADD Vx, Vy
    fn op_8xy4(&mut self) {
        let x = self.get_x();
        let y = self.get_y();

        let sum = self.cpu_registers[x] + self.cpu_registers[y];
        self.cpu_registers[0xF as usize] = if sum > (0xFF as u8) { 1 } else { 0 };
        self.cpu_registers[x] = sum & 0xFF;
    }

    /// SUB Vx, Vy
    fn op_8xy5(&mut self) {
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
    fn op_8xy6(&mut self) {
        let x = self.get_x();

        self.cpu_registers[0xF as usize] = self.cpu_registers[x] & 0x1;
        self.cpu_registers[x] = self.cpu_registers[x] >> 1;
    }

    /// SUBN Vx, Vy
    fn op_8xy7(&mut self) {
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
    fn op_8xyE(&mut self) {
        let x = self.get_x();

        self.cpu_registers[0xF as usize] = (self.cpu_registers[x] & 0x80) >> 7;
        self.cpu_registers[x] = self.cpu_registers[x] << 1;
    }

    /// SNE Vx, Vy
    fn op_9xy0(&mut self) {
        let x = self.get_x();
        let y = self.get_y();

        if (self.cpu_registers[x] == self.cpu_registers[y]) {
            return;
        }

        self.program_counter += 2;
    }

    /// LD I, addr
    fn op_Annn(&mut self) {
        let address = self.current_opcode & 0x0FFF;
        self.index_register = address;
    }

    /// JP V0, addr
    fn op_Bnnn(&mut self) {
        let address = self.current_opcode & 0x0FFF;
        self.program_counter = (self.cpu_registers[0] as u16) + address;
    }

    /// RND Vx, byte
    fn op_Cxkk(&mut self) {
        let x = self.get_x();
        let kk = self.get_kk();
        let mut rand = rand::thread_rng();
        let random = rand.gen_range(0, 255);

        self.cpu_registers[x] = kk & random;
    }

    /// DRW Vx, Vy, nibble
    fn op_Dxyn(&mut self) {
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

    /// SKP Vx
    fn op_Ex9E(&mut self) {
        let x = self.get_x();

        if self.keypad[x] == 0 {
            return;
        }

        self.program_counter += 2;
    }

    /// SKNP Vx
    fn op_ExA1(&mut self) {
        let x = self.get_x();

        if self.keypad[x] != 0 {
            return;
        }

        self.program_counter += 2;
    }

    /// LD Vx, DT
    fn op_Fx07(&mut self) {
        let x = self.get_x();

        self.cpu_registers[x] = self.delay_timer;
    }

    /// LD Vx, K
    fn op_Fx0A(&mut self) {
        let x = self.get_x();

        let mut key_pressed = false;
        for i in 0..self.keypad.len() {
            let key = self.keypad[i];
            if key != 0 {
                key_pressed = true;
                self.cpu_registers[x] = key;
                break;
            }
        }

        if !key_pressed {
            self.program_counter -= 2;
        }
    }

    /// LD DT, Vx
    fn op_Fx15(&mut self) {
        let x = self.get_x();

        self.delay_timer = self.cpu_registers[x];
    }

    /// LD ST, Vx
    fn op_Fx18(&mut self) {
        let x = self.get_x();

        self.sound_timer = self.cpu_registers[x];
    }

    /// ADD I, Vx
    fn op_Fx1E(&mut self) {
        let x = self.get_x();

        self.index_register += self.cpu_registers[x] as u16;
    }

    /// LD F, Vx
    fn op_Fx29(&mut self) {
        let x = self.get_x();
        let digit = self.cpu_registers[x] as u16;

        self.index_register = FONT_START_ADDRESS as u16 + (5 * digit);
    }

    /// LD B, Vx
    fn op_Fx33(&mut self) {
        let x = self.get_x();
        let mut vx = self.cpu_registers[x];
        
        self.memory[(self.index_register + 2) as usize] = vx % 10;
        vx /= 10;

        self.memory[(self.index_register + 1) as usize] = vx % 10;
        vx /= 10;

        self.memory[(self.index_register) as usize] = vx % 10;
    }

    /// LD [I], Vx
    fn op_Fx55(&mut self) {
        for i in 0..self.cpu_registers.len() - 1 {
            self.memory[self.index_register as usize + i] = self.cpu_registers[i];
        }
    }

    /// LD Vx, [I]
    fn op_Fx65(&mut self) {
        for i in 0..self.cpu_registers.len() - 1 {
            self.cpu_registers[i] = self.memory[self.index_register as usize + i];
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