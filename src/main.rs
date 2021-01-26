use clap::{App, Arg};
use gdnative_doc::{backend::Backend, init_logger, Builder};
use std::path::PathBuf;

fn real_main() -> gdnative_doc::Result<()> {
    init_logger();
    let matches = make_app().get_matches();

    let config_path = PathBuf::from(matches.value_of("config").unwrap());
    let mut builder = Builder::from_user_config(config_path)?;
    if let Some(output_dir) = matches.value_of("markdown") {
        builder = builder.add_backend(Backend::Markdown {
            output_dir: PathBuf::from(output_dir),
        })
    }
    if let Some(output_dir) = matches.value_of("html") {
        builder = builder.add_backend(Backend::Html {
            output_dir: PathBuf::from(output_dir),
        })
    }
    if let Some(output_dir) = matches.value_of("gut") {
        builder = builder.add_backend(Backend::Gut {
            output_dir: PathBuf::from(output_dir),
        })
    }
    builder.build()
}

fn main() -> Result<(), String> {
    real_main().map_err(|err| format!("{}", err))
}

fn make_app() -> App<'static, 'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            Arg::with_name("config")
                .long("config")
                .value_name("FILE")
                .required(true)
                .help("Configuration file for gdnative-doc"),
        )
        .arg(
            Arg::with_name("markdown")
                .long("md")
                .value_name("DIRECTORY")
                .help("Directory in which to put the markdown output"),
        )
        .arg(
            Arg::with_name("html")
                .long("html")
                .value_name("DIRECTORY")
                .help("Directory in which to put the html output"),
        )
        .arg(
            Arg::with_name("gut")
                .long("gut")
                .value_name("DIRECTORY")
                .help("Directory in which to put the gut output"),
        )
}
