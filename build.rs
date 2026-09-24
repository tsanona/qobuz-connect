use std::{error::Error, path::PathBuf};
use protox::prost::Message;

fn main() -> Result<(), Box<dyn Error>> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let proto = root.join("proto");
    let files = [
        proto.join("qcloud.proto"),
        proto.join("qconnect.proto"),
    ];

    for file in &files {
        println!("cargo:rerun-if-changed={}", file.display());
    }

    let out_dir = root.join("src/proto");
    let descriptors = protox::compile(files, [proto])?;
    
    let encoded_descriptors = descriptors.encode_to_vec();
    pbjson_build::Builder::new()
        .out_dir(&out_dir)
        .register_descriptors(&encoded_descriptors)?
        .build(&["."])?;

    Ok(prost_build::Config::new()
        .out_dir(out_dir)
        // .compile_well_known_types()
        // .extern_path(".google.protobuf", "::pbjson_types")
        .compile_fds(descriptors)?)
}
