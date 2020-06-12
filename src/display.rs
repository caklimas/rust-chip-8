use ggez::*;
use ggez::event::KeyCode;
use ggez::event::KeyMods;

const PIXEL_SIZE: i32 = 20;
pub const WINDOW_WIDTH: f32 = cpu::SCREEN_WIDTH as f32 * PIXEL_SIZE as f32;
pub const WINDOW_HEIGHT: f32 = cpu::SCREEN_HEIGHT as f32 * PIXEL_SIZE as f32;

use crate::cpu;

impl ggez::event::EventHandler for cpu::Cpu {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.cycle();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        if !self.can_draw {
            return Ok(());
        }

        graphics::clear(ctx, graphics::BLACK);

        for row in 0..cpu::SCREEN_HEIGHT {
            for column in 0..cpu::SCREEN_WIDTH {
                if self.graphics[row as usize][column as usize] == false {
                    continue;
                }

                let rect = ggez::graphics::Rect::new_i32(
                    column as i32 * PIXEL_SIZE,
                    row as i32 * PIXEL_SIZE,
                    PIXEL_SIZE,
                    PIXEL_SIZE
                );

                let rectangle = ggez::graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    rect,
                    graphics::WHITE
                )?;

                graphics::draw(ctx, &rectangle, (nalgebra::Point2::new(0.0, 0.0),))?;
            }
        }
        
        graphics::present(ctx)?;
        self.can_draw = false;

        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Key1 => {
                self.keypad[0x1] = true;
            },
            KeyCode::Key2 => {
                self.keypad[0x2] = true;
            },
            KeyCode::Key3 => {
                self.keypad[0x3] = true;
            },
            KeyCode::Key4 => {
                self.keypad[0xC] = true;
            },
            KeyCode::Q => {
                self.keypad[0x4] = true;
            },
            KeyCode::W => {
                self.keypad[0x5] = true;
            },
            KeyCode::E => {
                self.keypad[0x6] = true;
            },
            KeyCode::R => {
                self.keypad[0xD] = true;
            },
            KeyCode::A => {
                self.keypad[0x7] = true;
            },
            KeyCode::S => {
                self.keypad[0x8] = true;
            },
            KeyCode::D => {
                self.keypad[0x9] = true;
            },
            KeyCode::F => {
                self.keypad[0xE] = true;
            },
            KeyCode::Z => {
                self.keypad[0xA] = true;
            },
            KeyCode::X => {
                self.keypad[0x0] = true;
            },
            KeyCode::C => {
                self.keypad[0xB] = true;
            },
            KeyCode::V => {
                self.keypad[0xF] = true;
            },
            _ => ()
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        match keycode {
            KeyCode::Key1 => {
                self.keypad[0x1] = false;
            },
            KeyCode::Key2 => {
                self.keypad[0x2] = false;
            },
            KeyCode::Key3 => {
                self.keypad[0x3] = false;
            },
            KeyCode::Key4 => {
                self.keypad[0xC] = false;
            },
            KeyCode::Q => {
                self.keypad[0x4] = false;
            },
            KeyCode::W => {
                self.keypad[0x5] = false;
            },
            KeyCode::E => {
                self.keypad[0x6] = false;
            },
            KeyCode::R => {
                self.keypad[0xD] = false;
            },
            KeyCode::A => {
                self.keypad[0x7] = false;
            },
            KeyCode::S => {
                self.keypad[0x8] = false;
            },
            KeyCode::D => {
                self.keypad[0x9] = false;
            },
            KeyCode::F => {
                self.keypad[0xE] = false;
            },
            KeyCode::Z => {
                self.keypad[0xA] = false;
            },
            KeyCode::X => {
                self.keypad[0x0] = false;
            },
            KeyCode::C => {
                self.keypad[0xB] = false;
            },
            KeyCode::V => {
                self.keypad[0xF] = false;
            },
            _ => ()
        }
    }
}