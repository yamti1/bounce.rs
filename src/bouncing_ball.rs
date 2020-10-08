use rand::prelude::*;
use ggez::{Context, GameResult, graphics};
use ggez::nalgebra as na;

use crate::conf::{WINDOW_WIDTH, WINDOW_HEIGHT};
use crate::ball::*;

enum BounceDirection {
    LeftRight,
    UpDown,
}

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

        let mut ball = Self {
            radius: fatness * RADIUS_RANGE.1 + RADIUS_RANGE.0,
            speed_factor: 1.0 - fatness,
            color: graphics::Color::new(
                rng.gen::<f32>(), 
                rng.gen::<f32>(), 
                rng.gen::<f32>(), 
                1.0 - fatness
            ),
            x: rng.gen_range(0.0, WINDOW_WIDTH),
            y: rng.gen_range(0.0, WINDOW_HEIGHT),
            dx: [-1, 1].choose(&mut rng).unwrap().clone(),
            dy: [-1, 1].choose(&mut rng).unwrap().clone(),
        };
        ball.direct_away_from_edge(WINDOW_WIDTH, WINDOW_HEIGHT);

        ball
    }

    pub fn new_at(x: f32, y: f32, screen_width: f32, screen_height: f32) -> Self {
        let mut ball = Self::new();
        ball.x = x;
        ball.y = y;
        ball.direct_away_from_edge(screen_width, screen_height);
        
        ball
    }

    fn direct_away_from_edge(&mut self, screen_width: f32, screen_height: f32) {
        if self.x + self.radius > screen_width { 
            self.dx = -1; 
        } else if self.x - self.radius < 0.0 {
             self.dx = 1;
        }

        if self.y + self.radius > screen_height { 
            self.dy = -1; 
        } else if self.y - self.radius < 0.0 {
             self.dy = 1;
        }
    }

    pub fn update(&mut self, ctx: &mut Context) {
        if self.is_bouncing(ctx, BounceDirection::LeftRight) { self.dx *= -1; }
        if self.is_bouncing(ctx, BounceDirection::UpDown)    { self.dy *= -1; }
        
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

    fn is_bouncing(&self, ctx: &Context, direction: BounceDirection) -> bool {
        let screen = graphics::screen_coordinates(ctx);

        // The tolerance is the step taken every frame divided by two because `equal_with_tolerance`
        // checks if the ball is at the edge +/- the tolerance.
        let tolerance = MAX_SPEED * self.speed_factor / 2.0;

        match direction {
            BounceDirection::LeftRight => {
                self.dx == -1 && equal_with_tolerance(self.x - self.radius, 0.0, tolerance) ||
                self.dx ==  1 && equal_with_tolerance(self.x + self.radius, screen.w, tolerance)
            },
            BounceDirection::UpDown => {
                self.dy == -1 && equal_with_tolerance(self.y - self.radius, 0.0, tolerance) ||
                self.dy ==  1 && equal_with_tolerance(self.y + self.radius, screen.h, tolerance)
            }
        }
    }
}

fn equal_with_tolerance(a: f32, b: f32, tolerance: f32) -> bool {
    a <= b + tolerance &&
    a >= b - tolerance
}