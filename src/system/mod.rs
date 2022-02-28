pub mod ai;
pub mod startup;
pub mod input;
pub mod collision;
pub mod score;
pub mod movement;

pub use ai::ai;
pub use collision::ball_bounds_check;
pub use collision::keep_paddle_in_screen;
pub use collision::paddle_collision;
pub use startup::setup;
pub use startup::setup_won;
pub use startup::setup_lost;
pub use input::user_input;
pub use score::score;
pub use movement::movement;