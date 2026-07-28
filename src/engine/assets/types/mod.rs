use std::path::{Path, PathBuf};

// Sync: load to gpu sync
pub mod shader;
pub mod texture;

// Async: all files here load async
// - Reads file off disk, join all to finish loading
pub struct TextSource {
    pub path: PathBuf,
    pub source: String,
}

impl TextSource {
    pub async fn new(file_path: &Path) -> anyhow::Result<Self> {
        let source = tokio::fs::read_to_string(file_path).await?;
        return Ok(Self {
            path: file_path.to_path_buf(),
            source,
        });
    }
}

pub struct RawSource {
    pub pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl RawSource {
    pub async fn new(file_path: &Path) -> anyhow::Result<Self> {
        let source = tokio::fs::read(file_path).await?;
        let img = image::load_from_memory(&source)?;
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();

        return Ok(Self {
            pixels: rgba.into_raw(),
            width,
            height,
        });
    }
}




