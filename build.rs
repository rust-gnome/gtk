#[cfg(target_os = "macos")]
extern crate cc;

#[cfg(all(target_os = "windows", target_env = "msvc"))]
extern crate vcpkg;

use std::{
    env,
    fs::{copy, metadata},
};

fn main() {
    manage_docs();
    #[cfg(target_os = "macos")]
    build_foreground();
    #[cfg(all(target_os = "windows", target_env = "msvc"))]
    build_with_vcpkg();
}

#[cfg(any(feature = "embed-lgpl-docs", feature = "purge-lgpl-docs"))]
fn manage_docs () {
    extern crate lgpl_docs;
    const PATH: &'static str = "src";
    const IGNORES: &'static [&'static str] = &[
        "lib.rs",
        "prelude.rs",
        "rt.rs",
        "signal.rs",
    ];
    lgpl_docs::purge(PATH, IGNORES);
    if cfg!(feature = "embed-lgpl-docs") {
        lgpl_docs::embed(lgpl_docs::Library::Gtk, PATH, IGNORES);
    }
}

#[cfg(not(any(feature = "embed-lgpl-docs", feature = "purge-lgpl-docs")))]
fn manage_docs() { }

#[cfg(target_os = "macos")]
fn build_foreground() {
    cc::Build::new().file("src/foreground.m").compile("foreground");
    println!("cargo:rustc-link-lib=framework=AppKit");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
}

#[cfg(all(target_os = "windows", target_env = "msvc"))]
fn build_with_vcpkg() {
    fn ensure_lib_file_name(library: &vcpkg::Library, src_name: &str, dst_name: &str) {
        let src_name = format!("{}.lib", &src_name);
        let dst_name = format!("{}.lib", &dst_name);

        for src_path in &library.found_libs {
            // see if we can find the library we are looking for
            if src_path
                .file_name()
                .expect("vcpkg found a lib that is not a file")
                == src_name.as_str()
            {
                // put together path for needed lib name
                let dst_path = src_path
                    .parent()
                    .expect("vcpkg found a lib with an invalid path")
                    .join(&dst_name);

                // check if we really need to copy, maybe it's already there
                let mut to_copy = true;

                // check if we may not need to replace the file
                // only if the other file exists
                if dst_path.exists() {
                    let src_metadata = metadata(&src_path).expect("file doesn't have metadata");
                    let dst_metadata = metadata(&dst_path).expect("file doesn't have metadata");

                    // if the OS gives us the last modified time
                    if let Ok(src_modified) = src_metadata.modified() {
                        if let Ok(dst_modified) = dst_metadata.modified() {
                            // if the size and modified time is equal
                            if src_metadata.len() == dst_metadata.len()
                                && src_modified == dst_modified
                            {
                                // then we dont need to replace
                                to_copy = false;
                            }
                        }
                    }
                }

                if to_copy {
                    // copy file
                    copy(&src_path, &dst_path).unwrap_or_else(|error| {
                        panic!(
                            "copying `{}` to `{}` failed: {}",
                            src_path.to_string_lossy(),
                            dst_path.to_string_lossy(),
                            error
                        )
                    });
                }
            }
        }
    }

    // having `VCPKG_ROOT` is required
    if let Ok(_) = env::var("VCPKG_ROOT") {
        // the alternative would be `RUSTFLAGS = "-C target-feature=+crt-static"`
        // which doesn't work right now
        env::set_var("VCPKGRS_DYNAMIC", "1");

        match vcpkg::find_package("gtk") {
            Ok(library) => {
                // if we found the library, we have to fix some file names for the linker to find
                ensure_lib_file_name(&library, "gtk-3.0", "gtk-3");
                ensure_lib_file_name(&library, "gdk-3.0", "gdk-3");
            }
            Err(error) => {
                // tell the user what to do if lib isn't found
                if let vcpkg::Error::LibNotFound(error) = error {
                    // first find out the right arch to install
                    let target_triple = std::env::var("TARGET").expect(
                    "environment variable `TARGET` is not defined, it should always be passed by cargo",
                );
                    let arch;

                    if target_triple == "x86_64-pc-windows-msvc" {
                        arch = "x64-windows";
                    } else if target_triple == "i686-pc-windows-msvc" {
                        arch = "x86-windows";
                    } else {
                        panic!("unsupported target triple: {}", target_triple);
                    }

                    panic!("vcpkg couldn't find a valid installation of the `gtk` package, use `vcpkg install gtk:{}`: {}", arch, error);
                }
                // otherwise tell user what the error is
                else {
                    panic!("{}", error);
                }
            }
        }
    } else {
        panic!(
            "vcpkg needs to be installed and pointed at by the environment variable `VCPKG_ROOT`"
        );
    }
}
