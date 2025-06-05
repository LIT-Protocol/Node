fn main() {
    let out_dir = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());

    // Create V8 snapshot from vanilla Deno runtime without custom extensions.
    // This is required because startup times are otherwise too slow (1s+) for benchmarks.
    lit_actions_snapshot::create_snapshot(out_dir.join("BASE_SNAPSHOT.bin"), vec![]);
}
