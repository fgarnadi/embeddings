use std::{fs, path::PathBuf};

fn main() {
    let proto_dir = PathBuf::from("proto");

    let protos: Vec<PathBuf> = fs::read_dir(&proto_dir)
        .unwrap()
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.extension()? == "proto" {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    let out_dir = PathBuf::from("src/embeddings");
    if !out_dir.exists() {
        fs::create_dir_all(&out_dir).unwrap();
    }

    tonic_build::configure()
        .build_server(false)
        .out_dir(out_dir)
        .compile_protos(&protos, &[proto_dir])
        .unwrap();
}
