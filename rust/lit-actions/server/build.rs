fn main() {
    let out_dir = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());

    // Create V8 snapshot from Deno runtime + Lit Actions extension
    lit_actions_snapshot::create_snapshot(
        out_dir.join("RUNTIME_SNAPSHOT.bin"),
        vec![lit_actions_ext::lit_actions::init_ops_and_esm()],
    );
}
