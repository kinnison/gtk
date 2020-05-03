#[cfg(target_os = "macos")]
extern crate cc;

fn main() {
    manage_docs();
    #[cfg(target_os = "macos")]
    build_foreground();
    check_features();
}

#[cfg(any(feature = "embed-lgpl-docs", feature = "purge-lgpl-docs"))]
fn manage_docs() {
    extern crate lgpl_docs;
    const PATH: &str = "src";
    const IGNORES: &[&str] = &["lib.rs", "prelude.rs", "rt.rs", "signal.rs"];
    lgpl_docs::purge(PATH, IGNORES);
    if cfg!(feature = "embed-lgpl-docs") {
        lgpl_docs::embed(lgpl_docs::Library::Gtk, PATH, IGNORES);
    }
}

#[cfg(not(any(feature = "embed-lgpl-docs", feature = "purge-lgpl-docs")))]
fn manage_docs() {}

#[cfg(target_os = "macos")]
fn build_foreground() {
    cc::Build::new()
        .file("src/foreground.m")
        .compile("foreground");
    println!("cargo:rustc-link-lib=framework=AppKit");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
}

fn check_features() {
    // gdk-sys provides a backends variable to tell us what backends are
    // available for us to use in GTK.
    // We extract that and create gdk_backend="x11" and the like
    // as configuration variables.
    // For reference, the backend set at time of writing consists of:
    // x11 win32 quartz broadway wayland
    if let Ok(targets) = std::env::var("DEP_GDK_BACKENDS") {
        for target in targets.split_whitespace() {
            println!("cargo:rustc-cfg=gdk_backend=\"{}\"", target);
        }
    } else {
        panic!("No GDK targets found, is gdk-sys working?");
    }
}
