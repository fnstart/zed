#![allow(clippy::disallowed_methods, reason = "build scripts are exempt")]

fn main() {
    println!("cargo::rustc-check-cfg=cfg(gles)");

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    if target_os == "windows" {
        #[cfg(feature = "windows-manifest")]
        embed_resource();
    }
}

#[cfg(feature = "windows-manifest")]
fn embed_resource() {
    let pkg_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let manifest = pkg_dir.join("resources/windows/gpui.manifest.xml");
    let rc_src = pkg_dir.join("resources/windows/gpui.rc");
    // llvm-rc (cargo-xwin) resolves the .rc's manifest path against CWD, not the
    // .rc's dir (unlike native rc.exe). Rewrite the .rc into OUT_DIR with an
    // absolute manifest path (forward slashes: llvm-rc accepts them and they avoid
    // .rc backslash-escaping and \\?\ verbatim-prefix issues) so it resolves
    // regardless of the resource compiler's CWD.
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let rc_file = out_dir.join("gpui.rc");
    let manifest_str = manifest.display().to_string().replace('\\', "/");
    let rc_src_text = std::fs::read_to_string(&rc_src).unwrap();
    let rc_text = rc_src_text.replace("resources/windows/gpui.manifest.xml", &manifest_str);
    assert_ne!(
        rc_text, rc_src_text,
        "gpui.rc manifest path substitution matched nothing — .rc format changed"
    );
    std::fs::write(&rc_file, rc_text).unwrap();
    println!("cargo:rerun-if-changed={}", manifest.display());
    println!("cargo:rerun-if-changed={}", rc_src.display());
    println!("cargo:rerun-if-changed=build.rs");
    embed_resource::compile(rc_file, embed_resource::NONE)
        .manifest_required()
        .unwrap();
}
