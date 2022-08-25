use tonic_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("./proto/veronymous_user_token_service.proto")?;

    Ok(())
}
