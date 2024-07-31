use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let profile = env::var("PROFILE").unwrap();
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    println!("cargo:rustc-env=TARGET={target}");
    println!("cargo:rustc-env=PROFILE={profile}");

    // Create V8 snapshot from Deno runtime + Lit Actions extension
    lit_actions_snapshot::create_snapshot(
        out_dir.join("RUNTIME_SNAPSHOT.bin"),
        vec![lit_actions_ext::lit_actions::init_ops_and_esm()],
    );
}
