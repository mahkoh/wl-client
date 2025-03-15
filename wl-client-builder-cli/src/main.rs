use {
    clap::{Parser, ValueHint},
    error_reporter::Report,
    std::path::PathBuf,
    wl_client_builder::Builder,
};

/// Generate safe protocol wrappers for the `wl-client` crate.
#[derive(Parser, Debug)]
struct Cli {
    /// The directory to generate code into.
    ///
    /// The generated code will be available via the `mod.rs` file in this directory.
    ///
    /// The directory will be created if it does not already exist. Files that already
    /// exist in the directory will not be removed but might be overwritten. You should
    /// remove the directory before generating code if you want to ensure that it only
    /// contains required files.
    #[clap(value_hint = ValueHint::DirPath)]
    out_dir: PathBuf,
    /// The path to an XML protocol file.
    #[clap(long, value_hint = ValueHint::FilePath)]
    xml_file: Vec<PathBuf>,
    /// The path to a directory containing XML protocol files.
    ///
    /// This behaves as if all XML files in this directory (but not any sub-directories)
    /// had been specified explicitly via `--xml-file` arguments.
    #[clap(long, value_hint = ValueHint::DirPath)]
    xml_dir: Vec<PathBuf>,
    /// The rust module path to the `wl-client` crate.
    ///
    /// By default, the generated code assumes that the crate can be accessed via
    /// `::wl_client`. If your crate imports the crate under a different name, the path
    /// can be modified with this parameter.
    #[clap(long)]
    wl_client_path: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let mut builder = Builder::default()
        .with_default_dir(false)
        .for_build_rs(false)
        .target_dir(&cli.out_dir);
    for dir in &cli.xml_dir {
        builder = builder.xml_dir(dir);
    }
    for file in &cli.xml_file {
        builder = builder.xml_file(file);
    }
    if let Some(pf) = &cli.wl_client_path {
        builder = builder.wl_client_path(pf);
    }
    if let Err(e) = builder.build() {
        eprintln!("Error: {}", Report::new(e).pretty(true));
        std::process::exit(1);
    }
}
