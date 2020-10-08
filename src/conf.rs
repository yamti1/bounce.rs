/// Configuration consts for controlling the game.

pub const BALL_COUNT: usize = 10;
pub const WINDOW_WIDTH: f32 = 640.0;
pub const WINDOW_HEIGHT: f32 = 480.0;
pub const LOG_FPS: bool = true;

pub mod ball {
    pub const RADIUS_RANGE: (f32, f32) = (5.0, 200.0);
    pub const FATNESS_RANGE: (f32, f32) = (0.1, 1.0);
    pub const MAX_SPEED: f32 = 10.0;
}

