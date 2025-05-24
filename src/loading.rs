use bevy::prelude::*;

// This plugin loads all assets using vanilla bevy
pub struct LoadingPlugin;

// Separate the loading state from the game state
#[derive(States, Debug, Clone, Default, Eq, PartialEq, Hash)]
enum LoadingState {
    #[default]
    Loading,
    Ready,
}

// Texture atlas layout handle
#[derive(Resource)]
struct RpgAtlasLayout(Handle<TextureAtlasLayout>);

// Texture atlas handle
#[derive(Resource)]
struct RpgTextureAtlas(Handle<Image>);

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<LoadingState>()
        .add_systems(OnEnter(LoadingState::Loading), load_texture_atlas);
    }
}

fn load_texture_atlas(
    asset_server: Res<AssetServer>,
    mut asset_layout: ResMut<Assets<TextureAtlasLayout>>,
    mut layout_res: ResMut<RpgAtlasLayout>,
    mut texture_res: ResMut<RpgTextureAtlas>,
) {
    // Load the texture atlas
    texture_res.0 = asset_server.load("assets/textures/terrain.png");
    layout_res.0 = asset_layout.add(TextureAtlasLayout::from_grid(
        UVec2::splat(16),
        16,
        16,
        None,
        None,
    ));
    info!("Texture atlas loaded.");
}
