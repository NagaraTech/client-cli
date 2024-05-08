use std::env;
use std::path::PathBuf;
use tonic_build;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chat_message = "./proto/ChatMessage.proto";
    let z_message = "./proto/zmessage.proto";
    let ZMsg = "./proto/ZMsg.proto";

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional") // for older systems
        .build_client(true)
        .build_server(true)
        .file_descriptor_set_path(out_dir.join("z_message_descriptor.bin"))
        .out_dir("./src/messager/")
        .compile(&[chat_message,z_message,ZMsg], &["proto"])?;

    // tonic_build::configure()
    //     .protoc_arg("--experimental_allow_proto3_optional") // for older systems
    //     .build_client(true)
    //     .build_server(true)
    //     .file_descriptor_set_path(out_dir.join("z_message_descriptor.bin"))
    //     .out_dir("./src/grpc/")
    //     .compile(&[z_message], &["proto"])?;

    Ok(())
}
