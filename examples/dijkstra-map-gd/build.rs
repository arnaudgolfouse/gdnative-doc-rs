use gdnative_doc::{init_logger, Backend, Builder, LevelFilter};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger(LevelFilter::Info)?;
    Builder::new()
        .user_config(PathBuf::from("config.toml"))
        .add_backend(Backend::Markdown {
            output_dir: PathBuf::from("doc/markdown"),
        })
        .add_backend(Backend::Html {
            output_dir: PathBuf::from("doc/html"),
        })
        .add_backend(Backend::Gut {
            output_dir: PathBuf::from("gut"),
        })
        .build()?;
    Ok(())
}
