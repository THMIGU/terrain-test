pub mod texture;

use rust_embed::Embed;

#[derive(Embed)]
#[folder = "assets/textures"]
pub struct TextureFiles;
