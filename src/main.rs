mod cpu;

fn main() {
    let mut cpu = cpu::Cpu::new();
    cpu.load_rom(vec![0x13, 0xC5]);
    println!("{}", cpu.cpu_registers.len() - 1);
}