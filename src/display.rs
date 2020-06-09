use ggez::*;

const PIXEL_SIZE: i32 = 20;

pub struct Display {
    pos_x: f32
}

impl Display {
    pub fn new() -> Self {
        return Display {
            pos_x: 0.0
        };
    }
}

impl ggez::event::EventHandler for Display {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let rect = ggez::graphics::Rect::new(50.0, 50.0, 100.0, 100.0);
        let rectangle = ggez::graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            rect,
            graphics::WHITE
        )?;

        graphics::draw(ctx, &rectangle, (nalgebra::Point2::new(0.0, 0.0),))?;
        graphics::present(ctx)?;

        Ok(())
    }
}