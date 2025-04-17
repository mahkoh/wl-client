mod protocols {
    include!(concat!(env!("OUT_DIR"), "/wayland-protocols/mod.rs"));
}

mod protocls_with_data {
    include!(concat!(env!("OUT_DIR"), "/wayland-protocols-data/mod.rs"));
}

fn main() {}
