mod cpu;

fn main() {
    let mut cpu = cpu::Cpu::new();
    cpu.load_rom(vec![0x13, 0xC5]);
    println!("{}", (0x0260 & 0x00F0) >> 4);
}