use std::path::Path;

use anyhow::Context;
use tokio::{
    fs::{DirBuilder, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
};

pub async fn save_bytes<P: AsRef<Path>>(path: P, bytes: &[u8]) -> anyhow::Result<()> {
    let path = path.as_ref();
    let parent = path.parent().context("Failed to parent")?;
    let mut dir_builder = DirBuilder::new();
    dir_builder
        .recursive(true)
        .mode(0o700)
        .create(parent)
        .await
        .with_context(|| format!("Failed to create dir {:?}", parent))?;

    let file = OpenOptions::new()
        .write(true)
        .append(false)
        .truncate(true)
        .create(true)
        .mode(0o600)
        .open(path)
        .await
        .with_context(|| format!("Failed to open file {:?}", path))?;
    let mut writer = BufWriter::new(file);
    writer
        .write_all(bytes)
        .await
        .with_context(|| format!("Failed to write_all bytes len: {}", bytes.len()))?;
    writer.flush().await.context("Failed to flush")?;

    Ok(())
}

pub async fn load_bytes<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<u8>> {
    let path = path.as_ref();
    let file = OpenOptions::new()
        .read(true)
        .write(false)
        .open(path)
        .await
        .with_context(|| format!("Failed to open file {:?}", path))?;

    let mut bytes = Vec::new();
    let mut reader = BufReader::new(file);

    reader
        .read_to_end(&mut bytes)
        .await
        .context("Failed to read_to_end")?;
    reader.flush().await.context("Failed to flush")?;

    Ok(bytes)
}
