use std::{fs, path::PathBuf};

use godot_doc_rs::{backend, config, documentation, files};

fn main() {
    simplelog::TermLogger::init(
        simplelog::LevelFilter::Info,
        simplelog::Config::default(),
        simplelog::TerminalMode::Stderr,
    )
    .unwrap();

    let path = match std::env::args_os().nth(1) {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from("./config.toml"),
    };
    let config = config::UserConfig::read_from(&path).unwrap();
    std::env::set_current_dir(path.parent().unwrap()).unwrap();
    let output_dir = config.output.clone();
    let root_file = match &config.root_file {
        Some(path) => path.clone(),
        None => {
            PathBuf::from("./src/lib.rs") // TODO: determine the root file via cargo
        }
    };
    let package = files::Package::from_root_file(root_file).unwrap();

    let mut documentation = documentation::Documentation::new();
    for (module_id, module) in package.modules {
        let root_module = if module_id == package.root_module {
            match module.attributes.as_ref() {
                Some(attrs) => Some(attrs.as_slice()),
                None => None,
            }
        } else {
            None
        };
        documentation
            .parse_from_module(&module.items, root_module)
            .unwrap();
    }

    let backend_config = backend::Config::with_user_config(config);
    let backend = backend_config.backend;
    let extension = backend.extension();
    let generator = backend::Generator::new(
        backend_config,
        documentation,
        // Maybe move this to backend::Config ?
        match backend {
            backend::Backend::Markdown => Box::new(backend::encode_markdown),
            backend::Backend::Html => Box::new(backend::encode_html),
        },
    );
    let files = generator.generate_files();
    let root_file = generator.generate_root_file();
    fs::create_dir_all(&output_dir).unwrap();

    fs::write(
        output_dir.join("index").with_extension(extension),
        root_file,
    )
    .unwrap();
    for (name, content) in files {
        fs::write(output_dir.join(name).with_extension(extension), content).unwrap();
    }
}
