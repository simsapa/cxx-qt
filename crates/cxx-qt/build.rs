// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_build::CxxQtBuilder;
use cxx_qt_build::{is_ios_target, thin_generated_fat_library_with_lipo};

fn main() {
    let mut builder = CxxQtBuilder::new().crate_include_root(Some("include".to_owned()));

    let cpp_files = ["src/connection.cpp"];
    let rust_bridges = ["src/connection.rs", "src/qobject.rs"];

    for bridge in &rust_bridges {
        builder = builder.file(bridge);
    }

    builder = builder.cc_builder(move |cc| {
        for cpp_file in &cpp_files {
            cc.file(cpp_file);
            println!("cargo::rerun-if-changed={cpp_file}");
        }
    });
    builder = builder.initializer(qt_build_utils::Initializer {
        file: Some("src/init.cpp".into()),
        ..qt_build_utils::Initializer::default_signature("init_cxx_qt_core")
    });

    builder.build().export();

    if is_ios_target() {
        thin_generated_fat_library_with_lipo("libcxx-qt-cxxqt-generated.a", "arm64");
    }
}
