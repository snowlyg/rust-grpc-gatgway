fn main() -> Result<(), Box<dyn std::error::Error>> {
    napi_build_ohos::setup();
    tonic_build::compile_protos("protobuf/gateway/gateway.proto")?;
    Ok(())
}
