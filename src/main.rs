mod cpu;

pub use crate::cpu::Cpu;

fn main() {
    let cpu = Cpu::new();
    println!("{}", cpu.program_counter);
}