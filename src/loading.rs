use bevy::prelude::*;

use crate::GameState;

// This plugin loads all assets using vanilla bevy
pub struct LoadingPlugin;

// Texture atlas layout handle
#[derive(Resource, Default)]
struct RpgAtlasLayout(Handle<TextureAtlasLayout>);

// Texture atlas handle
#[derive(Resource, Default)]
struct RpgTextureAtlas(Handle<Image>);

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RpgAtlasLayout>()
            .init_resource::<RpgTextureAtlas>()
            .add_systems(OnEnter(GameState::Loading), load_texture_atlas);
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
    info!("Texture atlas loaded.");
}
