mod cpu;
mod display;

use ggez::*;

fn main() {
    let mut c = cpu::Cpu::new();
    c.load_rom(vec![0x13, 0xC5]);

    let mut conf = conf::Conf::new();
    conf.window_setup = conf::WindowSetup::default().title("Chip8");
    conf.window_mode = conf::WindowMode::default().dimensions(cpu::VIDEO_HEIGHT as f32, cpu::VIDEO_WIDTH as f32);

    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("Chip8", "caklimas@gmail.com")
        .conf(conf)
        .build()
        .expect("Error building context");

    event::run(ctx, event_loop, &mut c).expect("Error running loop");
}