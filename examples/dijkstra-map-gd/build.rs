use gdnative_doc::{backend::BuiltinBackend, init_logger, Builder, LevelFilter};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger(LevelFilter::Info)?;
    Builder::new()
        .user_config(PathBuf::from("config.toml"))
        .add_backend(BuiltinBackend::Markdown, PathBuf::from("doc/markdown"))
        .add_backend(BuiltinBackend::Html, PathBuf::from("doc/html"))
        .add_backend(BuiltinBackend::Gut, PathBuf::from("gut"))
        .build()?;
    Ok(())
}
