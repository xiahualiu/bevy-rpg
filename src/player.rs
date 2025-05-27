use bevy::prelude::*;

use crate::GameState;
use crate::loading::{RpgAtlasLayout, RpgTextureAtlas};

#[derive(Component)]
pub struct Player {
    pub name: String, // Player's name
    pub health: u32,  // Player's health
    pub mana: u32,    // Player's mana
    pub level: u32,   // Player's level
    pub speed: f32,   // Player's movement speed
}

impl Default for Player {
    fn default() -> Self {
        Player {
            name: "Hero".to_string(),
            health: 100,
            mana: 50,
            level: 1,
            speed: 8.0,
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Running), spawn_player);
    }
}

fn spawn_player(mut commands: Commands,
    texture_res: Res<RpgTextureAtlas>,
    layout_res: Res<RpgAtlasLayout>
) {
    commands.spawn((
        Player::default(),
        Transform::from_xyz(32.0, 32.0, 1.0), // Initial position
        Sprite::from_atlas_image(
            texture_res.0.clone(),
            TextureAtlas {
                layout: layout_res.0.clone(),
                index: 2, // Test with dirt block sprite
            }
        ),
    ));
}