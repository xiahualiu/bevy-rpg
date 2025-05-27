use bevy::asset::{io::Reader, AssetLoader, LoadContext};
use bevy::prelude::*;

use serde::Deserialize;
use serde_big_array::BigArray;
use thiserror::Error;

use crate::loading::{RpgAtlasLayout, RpgTextureAtlas};
use crate::GameState;

#[derive(Component)]
struct TileType(pub u32); // Index of the tile type in the tile properties array

// Tile bundle for quickly creating tiles in the game world
#[derive(Bundle)]
pub struct Tile {
    tile_type: TileType, // Index of the tile type in the tile properties array
    sprite: Sprite,      // Sprite component for rendering the tile
    position: Transform, // Position of the tile in the world
}

// Tile properties for each tile type
struct TileProperty {
    pub tile_name: &'static str,
    pub sprite: Sprite,
    // Additional properties can be added here, such as collision, walkability, etc.
}

// Resource to hold a vector of tile properties
#[derive(Resource, Default)]
pub struct TilePropertyVector {
    pub properties: Vec<TileProperty>,
}

// Plugin to manage tile maps in the game
pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Preparing), setup_tile_properties);
    }
}

// System to set up tile properties
fn setup_tile_properties(
    mut commands: Commands,
    texture_res: Res<RpgTextureAtlas>,
    layout_res: Res<RpgAtlasLayout>,
) {
    let mut tile_properties = Vec::new();
    // Define tile properties for each tile type
    tile_properties.push(TileProperty {
        tile_name: "Gravel",
        sprite: Sprite::from_atlas_image(
            texture_res.0.clone(),
            TextureAtlas {
                layout: layout_res.0.clone(),
                index: 0,
            },
        ),
    });
    tile_properties.push(TileProperty {
        tile_name: "Stone",
        sprite: Sprite::from_atlas_image(
            texture_res.0.clone(),
            TextureAtlas {
                layout: layout_res.0.clone(),
                index: 1,
            },
        ),
    });
    tile_properties.push(TileProperty {
        tile_name: "Dirt",
        sprite: Sprite::from_atlas_image(
            texture_res.0.clone(),
            TextureAtlas {
                layout: layout_res.0.clone(),
                index: 2,
            },
        ),
    });
    tile_properties.push(TileProperty {
        tile_name: "GrassBlock",
        sprite: Sprite::from_atlas_image(
            texture_res.0.clone(),
            TextureAtlas {
                layout: layout_res.0.clone(),
                index: 3,
            },
        ),
    });
    tile_properties.push(TileProperty {
        tile_name: "Plank",
        sprite: Sprite::from_atlas_image(
            texture_res.0.clone(),
            TextureAtlas {
                layout: layout_res.0.clone(),
                index: 4,
            },
        ),
    });

    // Insert the vector of tile properties into the resource
    commands.insert_resource(TilePropertyVector {
        properties: tile_properties,
    });
}

#[derive(Asset, TypePath, Debug, Deserialize)]
struct TrunkAsset{
    #[serde(with = "BigArray")]
    pub grid: [u32; 64 * 64],
}

#[derive(Default)]
struct TrunkAssetLoader;

/// Possible errors that can be produced by [`TrunkAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
enum TrunkAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [RON](ron) Error
    #[error("Could not parse RON: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),
}

impl AssetLoader for TrunkAssetLoader {
    type Asset = TrunkAsset;
    type Settings = ();
    type Error = TrunkAssetLoaderError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let trunk_asset = ron::de::from_bytes::<TrunkAsset>(&bytes)?;
        Ok(trunk_asset)
    }

    fn extensions(&self) -> &[&str] {
        &["tk"]
    }
}

#[derive(Default)]
enum TrunkLoadStatus {
    /// The trunk has been unloaded
    #[default]
    Unloaded,
    /// The trunk is loaded and ready to use
    Loaded,
}

#[derive(Default)]
struct TrunkStatus {
    pub load_status: TrunkLoadStatus,
    pub pos: [f32; 2],
    pub trunk: Handle<TrunkAsset>,
}

#[derive(Resource, Default)]
struct TrunkMap {
    pub trunks: Vec<TrunkStatus>,
    pub world_id: u32,
}
