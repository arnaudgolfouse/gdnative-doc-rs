use clap::{App, Arg};
use gdnative_doc::{
    backend::BuiltinBackend, init_logger, Builder, ConfigFile, LevelFilter, Package,
};
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let matches = make_app().get_matches();
    init_logger(match matches.occurrences_of("verbosity") {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    })?;

    let mut builder = Builder::new();

    if let Some(config_path) = matches.value_of("config") {
        builder = builder.user_config(ConfigFile::load_from_path(PathBuf::from(config_path))?);
    }
    if let Some(output_dir) = matches.value_of("markdown") {
        builder = builder.add_backend(BuiltinBackend::Markdown, PathBuf::from(output_dir));
    }
    if let Some(output_dir) = matches.value_of("html") {
        builder = builder.add_backend(BuiltinBackend::Html, PathBuf::from(output_dir));
    }
    if let Some(output_dir) = matches.value_of("gut") {
        builder = builder.add_backend(BuiltinBackend::Gut, PathBuf::from(output_dir));
    }

    if let Some(package_name) = matches.value_of("package") {
        builder = builder.package(Package::Name(package_name.to_string()))
    }
    if let Some(root_file) = matches.value_of("root_file") {
        builder = builder.package(Package::Root(PathBuf::from(root_file)))
    }

    Ok(builder.build()?)
}

fn make_app() -> App<'static, 'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .version_short("V")
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            Arg::with_name("config")
                .long("config")
                .short("c")
                .value_name("PATH")
                .help("Configuration file for gdnative-doc"),
        )
        .arg(
            Arg::with_name("markdown")
                .long("markdown")
                .visible_alias("md")
                .value_name("PATH")
                .help("Directory in which to put the markdown output"),
        )
        .arg(
            Arg::with_name("html")
                .long("html")
                .value_name("PATH")
                .help("Directory in which to put the html output"),
        )
        .arg(
            Arg::with_name("gut")
                .long("gut")
                .value_name("PATH")
                .help("Directory in which to put the gut output"),
        )
        .arg(
            Arg::with_name("package")
                .long("package")
                .short("p")
                .value_name("NAME")
                .help(
                    r"Name of the package for which to build the documentation.
This is useful if you are working within a workspace.",
                ),
        )
        .arg(
            Arg::with_name("root_file")
                .long("root_file")
                .value_name("PATH")
                .help(
                    r"Path to the root file of the package for which to build the documentation.",
                ),
        )
        .arg(
            Arg::with_name("verbosity")
                .long("verbose")
                .short("v")
                .multiple(true)
                .help("Use verbose output (-vv very verbose)"),
        )
}
