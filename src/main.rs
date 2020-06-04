mod emulators;

pub use crate::emulators::chip_8;

fn main() {
    let mut chip8 = chip_8::new();
    chip8.load_ROM(r"C:\Users\Christopher\Desktop\Test.txt");

    for (index, &b) in chip8.memory.iter().enumerate() {
        if b != 0 {
            println!("memory[{}] = {}", index, b);
        }
    }
}