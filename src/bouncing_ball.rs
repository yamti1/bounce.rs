use rand::prelude::*;
use ggez::{Context, GameResult, graphics};
use ggez::nalgebra as na;

use super::{WINDOW_WIDTH, WINDOW_HEIGHT};

type Range = (f32, f32);

const RADIUS_RANGE: Range = (5.0, 200.0);
const FATNESS_RANGE: Range = (0.1, 1.0);
const MAX_SPEED: f32 = 10.0;
const DIRECTIONS: [i8; 2] = [-1, 1];

pub struct BouncingBall {
    radius: f32,
    speed_factor: f32,
    color: graphics::Color,
    x: f32,
    y: f32,
    dx: i8,
    dy: i8,
}

impl BouncingBall {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let fatness_distribution = rand::distributions::Uniform::new_inclusive(FATNESS_RANGE.0, FATNESS_RANGE.1);
        let fatness = rng.sample(fatness_distribution);
        Self {
            radius: fatness * RADIUS_RANGE.1 + RADIUS_RANGE.0,
            speed_factor: 1.0 - fatness,
            color: graphics::Color::new(
                rng.gen::<f32>(), 
                rng.gen::<f32>(), 
                rng.gen::<f32>(), 
                1.0 - fatness
            ),
            x: WINDOW_WIDTH / 2.0,
            y: WINDOW_HEIGHT / 2.0,
            dx: DIRECTIONS.choose(&mut rng).unwrap().clone(),
            dy: DIRECTIONS.choose(&mut rng).unwrap().clone(),
        }
    }

    pub fn new_at(x: f32, y: f32) -> Self {
        let mut ball = Self::new();
        ball.x = x;
        ball.y = y;
        ball
    }

    pub fn update(&mut self, ctx: &mut Context) {
        let screen = graphics::screen_coordinates(ctx);

        if self.x + self.radius >= screen.w || self.x - self.radius <= 0.0 {
            self.dx *= -1;
        }
        if self.y + self.radius >= screen.h || self.y - self.radius <= 0.0 {
            self.dy *= -1;
        }
        
        self.x += self.speed_factor * MAX_SPEED * self.dx as f32;
        self.y += self.speed_factor * MAX_SPEED * self.dy as f32;
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let ball = graphics::Mesh::new_circle(
            ctx, 
            graphics::DrawMode::fill(), 
            na::Point2::new(0.0, 0.0), 
            self.radius,
            0.25, 
            self.color,
        )?;
        graphics::draw(ctx, &ball, (na::Point2::new(self.x, self.y),))?;

        Ok(())
    }
}
