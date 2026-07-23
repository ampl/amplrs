use std::env;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};

const LIBAMPL_URL: &str = "https://portal.ampl.com/~jurgen/jl/libampl.zip";

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-env-changed=AMPL_INCLUDE");
    println!("cargo:rerun-if-env-changed=AMPL_LIB");

    let (include_path, lib_path) = match (env::var("AMPL_INCLUDE"), env::var("AMPL_LIB")) {
        (Ok(include), Ok(lib)) => (PathBuf::from(include), PathBuf::from(lib)),
        _ => fetch_libampl(),
    };

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", include_path.display()))
        .allowlist_function("AMPL_.*")
        .allowlist_type("AMPL_.*")
        .allowlist_var("AMPL_.*")
        .generate()
        .expect("bindgen failed");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");

    println!("cargo:rustc-link-search=native={}", lib_path.display());
    if env::var("CARGO_CFG_TARGET_FAMILY").as_deref() != Ok("windows") {
        // MSVC's linker doesn't understand -Wl,-rpath; on Windows the DLL
        // instead needs to be next to the executable or on PATH.
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_path.display());
    } else {
        copy_windows_dlls(&lib_path, &out_path);
    }
    println!("cargo:rustc-link-lib=ampl");
}

/// Copies the AMPL DLL(s) next to the built executables so the loader can
/// find them at runtime (MSVC has no rpath equivalent).
fn copy_windows_dlls(lib_path: &Path, out_dir: &Path) {
    let profile_dir = out_dir
        .parent()
        .and_then(Path::parent)
        .and_then(Path::parent)
        .expect("OUT_DIR has unexpected layout");

    let dlls: Vec<_> = fs::read_dir(lib_path)
        .expect("failed to read lib dir")
        .map(|e| e.expect("failed to read directory entry").path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("dll"))
        .collect();

    for dest_dir in [
        profile_dir.to_path_buf(),
        profile_dir.join("deps"),
        profile_dir.join("examples"),
    ] {
        fs::create_dir_all(&dest_dir)
            .unwrap_or_else(|e| panic!("failed to create {}: {e}", dest_dir.display()));
        for dll in &dlls {
            let dest = dest_dir.join(dll.file_name().unwrap());
            fs::copy(dll, &dest).unwrap_or_else(|e| panic!("failed to copy {}: {e}", dll.display()));
        }
    }
}

/// Downloads and caches the AMPL C API (headers + shared library) into
/// `tmp/libampl` under the crate root, mirroring amplpy's `updatelib.py`.
/// Skipped if a previous run already populated the cache.
fn fetch_libampl() -> (PathBuf, PathBuf) {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let libampl_dir = manifest_dir.join("tmp").join("libampl");
    let include_dir = libampl_dir.join("include");
    let arch_dir = libampl_dir.join(target_arch_dir());

    if !include_dir.join("ampl").join("ampl_c.h").is_file() || !arch_dir.is_dir() {
        download_and_extract(&manifest_dir, &libampl_dir);
    }

    (include_dir, arch_dir)
}

fn target_arch_dir() -> &'static str {
    "amd64"
}

fn download_and_extract(manifest_dir: &Path, libampl_dir: &Path) {
    println!("cargo:warning=Downloading AMPL C API from {LIBAMPL_URL}");

    let body = ureq::get(LIBAMPL_URL)
        .call()
        .expect("failed to download libampl.zip")
        .into_reader();
    let mut bytes = Vec::new();
    std::io::Read::read_to_end(&mut { body }, &mut bytes)
        .expect("failed to read libampl.zip response body");

    let extract_dir = manifest_dir.join("tmp").join(".extract");
    let _ = fs::remove_dir_all(&extract_dir);
    fs::create_dir_all(&extract_dir).expect("failed to create extraction dir");

    let mut archive =
        zip::ZipArchive::new(Cursor::new(bytes)).expect("libampl.zip is not a valid zip archive");
    archive
        .extract(&extract_dir)
        .expect("failed to extract libampl.zip");

    let extracted_root = extract_dir.join("libampl");
    let _ = fs::remove_dir_all(libampl_dir);
    fs::create_dir_all(libampl_dir).expect("failed to create tmp/libampl");
    copy_dir_all(&extracted_root.join("include"), &libampl_dir.join("include"));
    for arch in ["amd64", "aarch64"] {
        let src = extracted_root.join(arch);
        if src.is_dir() {
            copy_dir_all(&src, &libampl_dir.join(arch));
        }
    }

    let _ = fs::remove_dir_all(&extract_dir);
}

fn copy_dir_all(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).unwrap_or_else(|e| panic!("failed to create {}: {e}", dst.display()));
    for entry in fs::read_dir(src).unwrap_or_else(|e| panic!("failed to read {}: {e}", src.display())) {
        let entry = entry.expect("failed to read directory entry");
        let dest_path = dst.join(entry.file_name());
        if entry.file_type().expect("failed to get file type").is_dir() {
            copy_dir_all(&entry.path(), &dest_path);
        } else {
            fs::copy(entry.path(), &dest_path).expect("failed to copy file");
        }
    }
}
