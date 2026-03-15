use crate::golang::{G2RCall, G2RCallImpl, GeneratePackageRequest, GeneratePackageResult};

mod golang;
mod proto;

impl G2RCall for G2RCallImpl {
    fn generate_package(req: GeneratePackageRequest) -> GeneratePackageResult {
        
        GeneratePackageResult {}
    }
}