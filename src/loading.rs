use bevy::prelude::*;

use crate::GameState;

// This plugin loads all assets using vanilla bevy
pub struct LoadingPlugin;

// Texture atlas layout handle
#[derive(Resource, Default)]
pub struct RpgAtlasLayout(pub Handle<TextureAtlasLayout>);

// Texture atlas handle
#[derive(Resource, Default)]
pub struct RpgTextureAtlas(pub Handle<Image>);

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RpgAtlasLayout>()
            .init_resource::<RpgTextureAtlas>()
            .add_systems(Startup, load_texture_atlas)
            .add_systems(
                Update,
                check_loading_status.run_if(in_state(GameState::Preparing)),
            );
            // Debug
            // .add_systems(OnEnter(GameState::Running), text_texture_atlas);
    }
}

fn load_texture_atlas(
    asset_server: Res<AssetServer>,
    mut asset_layout: ResMut<Assets<TextureAtlasLayout>>,
    mut layout_res: ResMut<RpgAtlasLayout>,
    mut texture_res: ResMut<RpgTextureAtlas>,
) {
    // Load the texture atlas
    texture_res.0 = asset_server.load("textures/terrain.png");
    layout_res.0 = asset_layout.add(TextureAtlasLayout::from_grid(
        UVec2::splat(16),
        16,
        16,
        None,
        None,
    ));
    info!("Loading texture atlas.");
}

fn check_loading_status(
    texture_res: Res<RpgTextureAtlas>,
    mut next_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
) {
    // Check if the texture atlas is loaded
    if asset_server.load_state(texture_res.0.id()).is_loaded() {
        info!("Texture atlas loaded successfully.");
    } else {
        // If not loaded, log the loading status
        info!("Texture atlas is still loading...");
        return;
    }
    info!("All assets loaded successfully, transitioning to Running state.");
    next_state.set(GameState::Running);
}

fn text_texture_atlas(
    mut commands: Commands,
    texture_res: Res<RpgTextureAtlas>,
    layout_res: Res<RpgAtlasLayout>,
) {
    // Create a sprite bundle using the loaded texture atlas
    let test_sprite = Sprite::from_atlas_image(
        texture_res.0.clone(),
        TextureAtlas {
            layout: layout_res.0.clone(),
            index: 0,
        },
    );

    let test_sprite_transform = Transform {
        translation: Vec3::new(0.0, 0.0, 0.0), // Position in the world
        rotation: Quat::IDENTITY,              // No rotation
        scale: Vec3::splat(10.0),              // Uniform scale
    };

    // Spawn the sprite in the world
    commands.spawn((test_sprite, test_sprite_transform));
}
