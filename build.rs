// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=frontend/src");
    println!("cargo:rerun-if-changed=frontend/static");
    println!("cargo:rerun-if-changed=frontend/package.json");
    println!("cargo:rerun-if-changed=frontend/vite.config.ts");

    if std::env::var("PROFILE").unwrap() == "release" {
        let status = Command::new("bun")
            .args(["run", "build"])
            .current_dir("frontend")
            .status()
            .expect("Failed to execute bun build command");

        if !status.success() {
            panic!("Frontend build process failed");
        }
    }
}
