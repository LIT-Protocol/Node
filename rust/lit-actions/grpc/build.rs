use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=./schema");

    let descriptor_path =
        PathBuf::from(env::var("OUT_DIR").unwrap()).join("lit_actions_descriptor.bin");

    tonic_build::configure()
        .file_descriptor_set_path(descriptor_path)
        .protoc_arg("--experimental_allow_proto3_optional")
        .btree_map(["ExecutionRequest.http_headers"])
        .compile(&["schema/lit_actions.proto"], &["schema/"])
        .unwrap();
}
