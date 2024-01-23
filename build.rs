use std::{env, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .build_server(true)
        .file_descriptor_set_path(out_dir.join("game_server_descriptor.bin"))
        .compile(
            &["proto/game_data.proto", "proto/game_update.proto"],
            &["proto"],
        )
        .unwrap();
}
