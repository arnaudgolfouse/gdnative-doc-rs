use godot_doc_rs::{
    backend::{self, Backend},
    config, documentation, files,
};
use std::{fs, path::PathBuf};

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
    let config = fs::read_to_string(&path).unwrap();
    let config = config::UserConfig::read_from(&config).unwrap();
    std::env::set_current_dir(path.parent().unwrap()).unwrap();
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

    let backend_config = backend::Config::from_user_config(config);
    backend_config.rename_classes(&mut documentation);
    let backends = backend_config.backends.clone();
    for (backend, output_dir) in backends {
        let extension = backend.extension();
        let mut generator = backend::Generator::new(
            &backend_config,
            &documentation,
            match backend {
                Backend::Markdown => Box::new(backend::MarkdownCallbacks::default()),
                Backend::Html => Box::new(backend::HtmlCallbacks::default()),
                Backend::Gut => Box::new(backend::GutCallbacks::default()),
            },
        );
        let files = generator.generate_files();
        let root_file = generator.generate_root_file(backend);
        fs::create_dir_all(&output_dir).unwrap();

        if backend != Backend::Gut {
            fs::write(
                output_dir.join("index").with_extension(extension),
                root_file,
            )
            .unwrap();
        }

        for (name, content) in files {
            fs::write(output_dir.join(name).with_extension(extension), content).unwrap();
        }
    }
}
