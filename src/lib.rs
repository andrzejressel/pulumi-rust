use std::fs::create_dir_all;
use std::path::Path;
use crate::golang::{G2RCall, G2RCallImpl, GeneratePackageRequest, GeneratePackageResult};
use prost::Message;

mod golang;
mod package_model;
mod proto;

impl G2RCall for G2RCallImpl {
    fn generate_package(req: GeneratePackageRequest) -> GeneratePackageResult {
        let package =
            proto::proto::package::Package::decode(&*req.protobuf).expect("invalid package bytes");
        let _model_package = package_model::map_package(package);

        let dir = req.directory.clone();
        let dir = Path::new(&dir);
        if (!dir.exists()) {
            create_dir_all(dir).expect("failed to create output directory");
        }
        
        pulumi_gestalt_generator::generate_rust(
            &_model_package, 
            dir
        ).expect("failed to generate package");
        
        GeneratePackageResult {}
    }
}
