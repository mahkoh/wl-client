use {
    crate::{
        formatter::{format_interface_file, format_mod_file, format_protocol_file},
        parser::{ParserError, parse},
    },
    std::{
        env::VarError,
        fs::File,
        io::{self, BufWriter, Write},
        path::{Path, PathBuf},
    },
    thiserror::Error,
};

#[derive(Debug, Error)]
enum BuilderError {
    #[error("Could not read {}", .0.display())]
    ReadFile(PathBuf, #[source] io::Error),
    #[error("Could not open {} for reading", .0.display())]
    OpenDir(PathBuf, #[source] io::Error),
    #[error("Could not read from {}", .0.display())]
    ReadDir(PathBuf, #[source] io::Error),
    #[error("Could not parse {}", .0.display())]
    ParseFile(PathBuf, #[source] ParserError),
    #[error("Could not format {}", .0.display())]
    FormatFile(PathBuf, #[source] io::Error),
    #[error("Could not determine OUT_DIR")]
    OutDir(#[source] VarError),
    #[error("Could not create {}", .0.display())]
    CreateDir(PathBuf, #[source] io::Error),
    #[error("Could not open {} for writing", .0.display())]
    OpenFile(PathBuf, #[source] io::Error),
}

/// A builder for `wl-client` wrappers.
pub struct Builder {
    build_script: bool,
    add_default_dir: bool,
    target_dir: Option<PathBuf>,
    files: Vec<PathBuf>,
    dirs: Vec<PathBuf>,
    wl_client_path: Option<String>,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            build_script: true,
            add_default_dir: true,
            target_dir: Default::default(),
            files: Default::default(),
            dirs: Default::default(),
            wl_client_path: None,
        }
    }
}

impl Builder {
    /// Sets the rust module path to the `wl-client` crate.
    ///
    /// By default, the generated code assumes that the `wl-client` crate is accessible
    /// via `::wl_client`.
    pub fn wl_client_path(mut self, path: &str) -> Self {
        self.wl_client_path = Some(path.into());
        self
    }

    /// Adds a protocol XML file.
    pub fn xml_file(mut self, path: impl AsRef<Path>) -> Self {
        self.files.push(path.as_ref().to_path_buf());
        self
    }

    /// Adds a protocol XML dir.
    ///
    /// This behaves as if all XML files in this directory (but not in any
    /// sub-directories) had been added with [`Builder::xml_file`].
    pub fn xml_dir(mut self, path: impl AsRef<Path>) -> Self {
        self.dirs.push(path.as_ref().to_path_buf());
        self
    }

    /// Enables or disables the default `wayland-protocols` dir.
    ///
    /// By default, the builder will try to load XML files from the `wayland-protocols`
    /// directory relative to the current working directory.
    pub fn with_default_dir(mut self, default_dir: bool) -> Self {
        self.add_default_dir = default_dir;
        self
    }

    /// Enables or disables `build.rs` logic.
    ///
    /// By default, the builder assumes that it is being used from `build.rs`. It will
    /// emit `cargo::` messages and treats a relative [`Builder::target_dir`] as relative
    /// to `$OUT_DIR`.
    pub fn for_build_rs(mut self, build_rs: bool) -> Self {
        self.build_script = build_rs;
        self
    }

    /// The target directory into which to generate the `mod.rs`.
    ///
    /// By default, the target directory is `wayland-protocols`.
    ///
    /// If [`Builder::for_build_rs`] is enabled, then a relative target directory will be
    /// interpreted relative to `$OUT_DIR`.
    pub fn target_dir(mut self, target_dir: impl AsRef<Path>) -> Self {
        self.target_dir = Some(target_dir.as_ref().to_path_buf());
        self
    }

    /// Generates the code.
    pub fn build(self) -> Result<(), crate::Error> {
        self.build_().map_err(|e| crate::Error(Box::new(e)))
    }

    fn build_(mut self) -> Result<(), BuilderError> {
        let mut target_dir = PathBuf::new();
        if self.build_script {
            let out_dir = std::env::var("OUT_DIR").map_err(BuilderError::OutDir)?;
            target_dir.push(out_dir);
        }
        if let Some(d) = &self.target_dir {
            target_dir.push(d);
        } else {
            target_dir.push("wayland-protocols");
        }
        create_dir(&target_dir)?;

        let mut protocol_objects = vec![];

        if self.add_default_dir {
            self.dirs.push(PathBuf::from("wayland-protocols"));
        }
        for dir in self.dirs {
            if self.build_script {
                println!("cargo::rerun-if-changed={}", dir.display());
            }
            let iter = match std::fs::read_dir(&dir) {
                Ok(c) => c,
                Err(e) => return Err(BuilderError::OpenDir(dir, e)),
            };
            for file in iter {
                let file = match file {
                    Ok(f) => f,
                    Err(e) => return Err(BuilderError::ReadDir(dir, e)),
                };
                if !file.file_name().as_encoded_bytes().ends_with(b".xml") {
                    continue;
                }
                self.files.push(file.path());
            }
        }
        for file in self.files {
            if self.build_script {
                println!("cargo::rerun-if-changed={}", file.display());
            }
            let contents = match std::fs::read(&file) {
                Ok(c) => c,
                Err(e) => return Err(BuilderError::ReadFile(file, e)),
            };
            let protocols = match parse(&contents) {
                Ok(c) => c,
                Err(e) => return Err(BuilderError::ParseFile(file, e)),
            };
            for protocol in protocols {
                let protocol_file = format!("{}.rs", protocol.name);
                format_file(&target_dir.join(&protocol_file), |f| {
                    format_protocol_file(f, &protocol)
                })?;
                let dir = target_dir.join(&protocol.name);
                create_dir(&dir)?;
                let mut interfaces = vec![];
                for interface in protocol.interfaces {
                    let file_name = format!("{}.rs", interface.name);
                    format_file(&dir.join(&file_name), |f| {
                        format_interface_file(
                            f,
                            self.wl_client_path.as_deref().unwrap_or("::wl_client"),
                            &interface,
                        )
                    })?;
                    let mut enums = vec![];
                    for enum_ in interface.enums {
                        enums.push(enum_.name);
                    }
                    interfaces.push((interface.name, enums));
                }
                protocol_objects.push((protocol.name, interfaces));
            }
        }

        format_file(&target_dir.join("mod.rs"), |f| {
            format_mod_file(f, &protocol_objects)
        })?;
        Ok(())
    }
}

fn create_dir(path: &Path) -> Result<(), BuilderError> {
    if let Err(e) = std::fs::create_dir_all(path) {
        return Err(BuilderError::CreateDir(path.to_owned(), e));
    }
    Ok(())
}

fn open_file(path: &Path) -> Result<BufWriter<File>, BuilderError> {
    let file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path);
    match file {
        Ok(f) => Ok(BufWriter::new(f)),
        Err(e) => Err(BuilderError::OpenFile(path.to_owned(), e)),
    }
}

fn format_file(
    path: &Path,
    f: impl FnOnce(&mut BufWriter<File>) -> io::Result<()>,
) -> Result<(), BuilderError> {
    let mut file = open_file(path)?;
    let mut res = f(&mut file);
    if res.is_ok() {
        res = file.flush();
    }
    if let Err(e) = res {
        return Err(BuilderError::FormatFile(path.to_owned(), e));
    }
    Ok(())
}
