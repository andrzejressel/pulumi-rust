pub mod proto {
    pub mod pcl {
        include!(concat!(env!("OUT_DIR"), "/pulumipcl.rs"));
    }
    pub mod package {
        include!(concat!(env!("OUT_DIR"), "/pulumipackage.rs"));
    }
}
