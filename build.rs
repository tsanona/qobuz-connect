use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let root = env!("CARGO_MANIFEST_DIR");
    let proto = format!("{root}/proto");
    let files = [
        format!("{proto}/qcloud.proto"),
        format!("{proto}/qconnect.proto"),
    ];
    for file in &files {
        println!("cargo:rerun-if-changed={file}");
    }
    let descriptors = protox::compile(files, [proto])?;
    Ok(prost_build::Config::new()
        .out_dir(format!("{root}/src/proto"))
        .compile_fds(descriptors)?)
}
