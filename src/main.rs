use ggez::*;
use std::fs;

mod cpu;
mod display;

fn main() {
    let bytes = fs::read(r".\src\test_roms\test_opcode.ch8").expect("Cannot find file");
    let mut c = cpu::Cpu::new();
    c.load_rom(bytes);

    let mut conf = conf::Conf::new();
    conf.window_setup = conf::WindowSetup::default().title("Chip8");
    conf.window_mode = conf::WindowMode::default().dimensions(display::WINDOW_WIDTH as f32, display::WINDOW_HEIGHT as f32);

    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("Chip8", "caklimas@gmail.com")
        .conf(conf)
        .build()
        .expect("Error building context");

    event::run(ctx, event_loop, &mut c).expect("Error running loop");
}