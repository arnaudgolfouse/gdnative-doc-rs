use gdnative_doc::{init_logger, Builder, BuiltinBackend, LevelFilter};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger(LevelFilter::Info)?;
    Builder::new()
        .user_config(PathBuf::from("config.toml"))
        .add_builtin_backend(BuiltinBackend::Markdown, PathBuf::from("doc/markdown"))
        .add_builtin_backend(BuiltinBackend::Html, PathBuf::from("doc/html"))
        .add_builtin_backend(BuiltinBackend::Gut, PathBuf::from("gut"))
        .build()?;
    Ok(())
}
