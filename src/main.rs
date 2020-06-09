mod cpu;
mod display;

use ggez::*;

fn main() {
    let mut cpu = cpu::Cpu::new();
    cpu.load_rom(vec![0x13, 0xC5]);

    let c = conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("Hello ggez", "caklimas@gmail.com")
        .conf(c)
        .build()
        .expect("Error building context");

    event::run(ctx, event_loop, &mut cpu).expect("Error running loop");
}