use pulumi_rust::generate_project_from_protobuf;
use std::fs;
use std::path::{Path, PathBuf};

fn projects_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("pkg")
        .join("cmd")
        .join("pulumi-language-rust")
        .join("testdata")
        .join("projects")
}

fn regenerate_project(test_name: &str) {
    let project_dir = projects_root().join(test_name);
    let protobuf_path = project_dir.join("protobuf.bin");
    let protobuf = fs::read(&protobuf_path).unwrap_or_else(|error| {
        panic!(
            "failed reading protobuf for test '{test_name}' at {}: {error}",
            protobuf_path.display()
        )
    });

    generate_project_from_protobuf(protobuf, project_dir.to_string_lossy().into_owned());

    assert!(
        project_dir.join("Cargo.toml").exists(),
        "missing generated Cargo.toml for test '{test_name}'"
    );
    assert!(
        project_dir.join("src").join("main.rs").exists(),
        "missing generated src/main.rs for test '{test_name}'"
    );
}

// Keep in sync with testNames in pkg/cmd/pulumi-language-rust/language_test.go.
#[test]
// #[cfg(feature = "fast-feedback-tests")]
fn regenerate_l1_empty() {
    regenerate_project("l1-empty");
}

#[test]
// #[cfg(feature = "fast-feedback-tests")]
fn regenerate_l1_main() {
    regenerate_project("l1-main");
}
