#[derive(rust2go::R2G, Clone)]
pub struct GeneratePackageRequest {
    pub protobuf: Vec<u8>,
    pub directory: String,
}

#[derive(rust2go::R2G, Clone)]
pub struct GeneratePackageResult {}

#[rust2go::g2r]
pub trait G2RCall {
    fn generate_package(req: GeneratePackageRequest) -> GeneratePackageResult;
}
