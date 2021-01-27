use gdnative_doc::{Backend, Builder};
use std::path::PathBuf;

fn main() {
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
        .build()
        .unwrap();
}
