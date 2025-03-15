use {std::os::unix::ffi::OsStrExt, walkdir::WalkDir, wl_client_builder::Builder};

fn main() {
    let mut builder = Builder::default().with_default_dir(false);
    builder = builder.xml_file("../wayland/protocol/wayland.xml");
    for dir in ["stable", "staging", "unstable"] {
        let path = format!("../wayland-protocols/{dir}");
        for file in WalkDir::new(&path) {
            let file = file.unwrap();
            if dir == "unstable" {
                if file.file_name() == "tablet-unstable-v2.xml" {
                    continue;
                }
                if file.file_name() == "xdg-shell-unstable-v5.xml" {
                    continue;
                }
                if file.file_name() == "linux-dmabuf-unstable-v1.xml" {
                    continue;
                }
            }
            if file.file_name().as_bytes().ends_with(b".xml") {
                builder = builder.xml_file(file.path());
            }
        }
    }
    builder.build().unwrap();
}
