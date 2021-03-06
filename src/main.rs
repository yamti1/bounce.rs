mod bouncing_ball;
mod conf;

use ggez::*;
use conf::*;
use bouncing_ball::BouncingBall;

struct State {
    balls: Vec<BouncingBall>,
}

impl State {
    pub fn new() -> Self {
        let mut balls = Vec::with_capacity(BALL_COUNT);
        for _ in 0..BALL_COUNT {
            balls.push(BouncingBall::new());
        }

        Self {
            balls
        }
    }
}

impl event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        for ball in self.balls.iter_mut() {
            ball.update(ctx);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        for ball in self.balls.iter() {
            ball.draw(ctx)?;
        }
        graphics::present(ctx)?;

        if LOG_FPS && timer::ticks(ctx) % 50 == 0 { println!("{0:.0} fps", timer::fps(ctx)); }
        Ok(())
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: event::MouseButton, x: f32, y: f32) {
        match button {
            event::MouseButton::Left => {
                let screen = graphics::screen_coordinates(ctx);
                let ball = BouncingBall::new_at(x, y, screen.w, screen.h);
                self.balls.push(ball);
            },
            _ => {}
        }
    }
}


fn main() -> GameResult {
    let state = &mut State::new();

    let mut c = ggez::conf::Conf::new();
    c.window_mode.width = WINDOW_WIDTH;
    c.window_mode.height = WINDOW_HEIGHT;
    c.window_setup.title = String::from("Bouncing Balls");

    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("Bouncing Balls", "yamti")
        .conf(c).build()?;
    
    event::run(ctx, event_loop, state)
}
