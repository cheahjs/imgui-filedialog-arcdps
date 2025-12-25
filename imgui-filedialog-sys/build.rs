//! Build script for imgui-filedialog-sys
//!
//! Compiles ImGuiFileDialog C++ code and links with arcdps-imgui-sys

use std::{env, io, path::Path};

fn assert_file_exists(path: &str) -> io::Result<()> {
    match std::fs::metadata(path) {
        Ok(_) => Ok(()),
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
            panic!(
                "Can't access {}. Did you forget to fetch git submodules? Run: git submodule update --init --recursive",
                path
            );
        }
        Err(e) => Err(e),
    }
}

fn main() -> io::Result<()> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    // The library is in a nested submodule: ImGuiFileDialog/ImGuiFileDialog/
    let igfd_path = Path::new(&manifest_dir).join("../third-party/ImGuiFileDialog/ImGuiFileDialog");
    let cpp_file = igfd_path.join("ImGuiFileDialog.cpp");

    assert_file_exists(cpp_file.to_str().unwrap())?;

    let mut build = cc::Build::new();

    // Take over imgui preprocessor defines from arcdps-imgui-sys
    // This ensures ABI compatibility
    for (key, val) in env::vars() {
        if let Some(suffix) = key.strip_prefix("DEP_IMGUI_DEFINE_") {
            build.define(suffix, val.as_str());
        }
    }

    // Configure ImGuiFileDialog features
    // v0.5.4 uses USE_BOOKMARK instead of USE_PLACES_FEATURE
    #[cfg(feature = "bookmark")]
    build.define("USE_BOOKMARK", None);

    #[cfg(feature = "exploration_by_keys")]
    {
        build.define("USE_EXPLORATION_BY_KEYS", None);
        // Define key codes for ImGui 1.80 (uses ImGuiKey_ enum values)
        // These map to the appropriate ImGuiKey values in imgui.h
        build.define("IGFD_KEY_UP", "ImGuiKey_UpArrow");
        build.define("IGFD_KEY_DOWN", "ImGuiKey_DownArrow");
        build.define("IGFD_KEY_ENTER", "ImGuiKey_Enter");
        build.define("IGFD_KEY_BACKSPACE", "ImGuiKey_Backspace");
    }

    // Get imgui include paths from arcdps-imgui-sys
    let imgui_include = env::var_os("DEP_IMGUI_THIRD_PARTY")
        .expect("DEP_IMGUI_THIRD_PARTY not defined - is arcdps-imgui-sys a dependency?");
    let imgui_include_path = Path::new(&imgui_include);
    let imgui_path = imgui_include_path.join("imgui");

    build
        .file(&cpp_file)
        .include(&igfd_path)
        .include(imgui_include_path)
        .include(&imgui_path)
        .warnings(false)
        .cpp(true);

    // Platform-specific flags
    let target = env::var("TARGET").unwrap();
    let compiler = build.get_compiler();

    if compiler.is_like_gnu() || compiler.is_like_clang() {
        build.flag("-std=c++11");
        build.flag("-fno-exceptions");
        build.flag("-fno-rtti");
    } else if compiler.is_like_msvc() {
        build.flag("/std:c++14");
        build.flag("/EHsc");
    }

    // Windows-specific
    if target.contains("windows") {
        build.define("_WINDOWS", None);
    }

    build.compile("imgui_file_dialog");

    // Link with system libraries on Windows
    if target.contains("windows") {
        println!("cargo:rustc-link-lib=ole32");
        println!("cargo:rustc-link-lib=shell32");
    }

    Ok(())
}
