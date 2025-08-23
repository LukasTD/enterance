use std::io::Result;
fn main() -> Result<()> {
    // Use vendored protoc so users don't need a system install
    let protoc_path = protoc_bin_vendored::protoc_bin_path().expect("failed to fetch vendored protoc");
    // Set PROTOC for prost-build; wrap in unsafe to satisfy current compiler requirement
    unsafe { std::env::set_var("PROTOC", protoc_path); }

    prost_build::compile_protos(&["src/serverlist.proto"], &["src/"])?;
    Ok(())
}
