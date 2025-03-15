use cfg_if::cfg_if;

#[cfg(test)]
#[macro_use]
mod tests;

cfg_if! {
    if #[cfg(target_os = "linux")] {
        pub use linux::Eventfd;

        mod linux;
    } else {
        pub use fallback::Eventfd;
    }
}

#[cfg_attr(all(target_os = "linux", not(test)), expect(dead_code))]
mod fallback;
