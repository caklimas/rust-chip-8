mod cpu;

fn main() {
    let mut cpu = cpu::Cpu::new();
    cpu.load_rom(vec![0x13, 0xC5]);
    let b = 0xFF;
    for i in 0..8 {
        println!("{}", format!("{:b}", (b >> (7 - i)) & 1))
    }

    cpu.op_00E0();
}