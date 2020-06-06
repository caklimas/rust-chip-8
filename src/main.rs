mod cpu;

pub use crate::cpu::Cpu;

fn main() {
    let mut cpu = Cpu::new();
    cpu.load_ROM(vec![0x13, 0xC5]);
    println!("{}", cpu.program_counter);
}