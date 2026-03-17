use crate::golang::{
    FileWithContent, G2RCall, G2RCallImpl, GeneratePackageRequest, GenerateProgramRequest,
    GenerateProgramResult, GenerateProjectRequest,
};
use generator::generate_main;
use prost::Message;
use std::fs;
use std::fs::create_dir_all;
use std::path::Path;

mod generator;
mod golang;
mod package_model;
mod pcl_model;
mod proto;

impl G2RCall for G2RCallImpl {
    fn generate_package(req: GeneratePackageRequest) {
        let package =
            proto::proto::package::Package::decode(&*req.protobuf).expect("invalid package bytes");
        let _model_package = package_model::map_package(package);

        let dir = req.directory.clone();
        let dir = Path::new(&dir);
        if !dir.exists() {
            create_dir_all(dir).expect("failed to create output directory");
        }

        pulumi_gestalt_generator::generate_rust(&_model_package, dir)
            .expect("failed to generate package");

        fs::write(
            dir.join("Cargo.toml"),
            include_str!("./Cargo.toml.template"),
        )
        .expect("failed to write mod.rs");
    }

    fn generate_program(req: GenerateProgramRequest) -> GenerateProgramResult {
        let program = proto::proto::pcl::PclProtobufProgram::decode(&*req.protobuf)
            .expect("invalid program bytes");
        let model_program = pcl_model::map_program(program);

        let main_rs = generate_main(&model_program).expect("failed to generate main.rs");
        let file = vec![FileWithContent {
            path: "main.rs".to_string(),
            content: main_rs.into_bytes(),
        }];
        GenerateProgramResult {
            files_content: file,
        }
    }

    fn generate_project(req: GenerateProjectRequest) {
        let program = proto::proto::pcl::PclProtobufProgram::decode(&*req.protobuf)
            .expect("invalid program bytes");
        let model_program = pcl_model::map_program(program);
        let main_rs = generate_main(&model_program).expect("failed to generate main.rs");
        let cargo_rs = include_str!("./Cargo.toml.template");
        let files = vec![
            FileWithContent {
                path: "src/main.rs".to_string(),
                content: main_rs.into_bytes(),
            },
            FileWithContent {
                path: "Cargo.toml".to_string(),
                content: cargo_rs.as_bytes().to_vec(),
            },
        ];

        let dir = Path::new(&req.directory);
        for file in &files {
            let path = dir.join(file.path.clone());
            // let path = Path::new(&file.path);
            if let Some(parent) = path.parent() {
                create_dir_all(parent).expect("failed to create output directory");
            }
            fs::write(path, &file.content).expect("failed to write file");
        }
    }
}

pub fn generate_project_from_protobuf(protobuf: Vec<u8>, directory: String) {
    G2RCallImpl::generate_project(GenerateProjectRequest {
        protobuf,
        directory,
    });
}
