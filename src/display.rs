use ggez::*;

const PIXEL_SIZE: i32 = 20;

use crate::cpu;

impl ggez::event::EventHandler for cpu::Cpu {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.cycle();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        for row in 0..cpu::VIDEO_HEIGHT {
            for column in 0..cpu::VIDEO_WIDTH {
                if self.graphics[row as usize][column as usize] == 0 {
                    continue;
                }

                let rect = ggez::graphics::Rect::new_i32(
                    row as i32 * PIXEL_SIZE,
                    column as i32 * PIXEL_SIZE,
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

        Ok(())
    }
}