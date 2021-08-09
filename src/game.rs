use bevy::prelude::*;

#[derive(Default)]
pub struct Game {
    pub won: bool,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_game);
    }
}

fn setup_game(mut game: ResMut<Game>) {
    game.won = false;
}

