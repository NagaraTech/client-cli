
fn main() -> Result<(), Box<dyn std::error::Error>> {


    println!("xxxxxxxxxxx");

    let msg = "src/proto/msg.proto";

    prost_build::compile_protos(&[msg], &["src/"])?;

    Ok(())
}
