pub mod proto {
    mod pcl {
        include!(concat!(env!("OUT_DIR"), "/pulumipcl.rs"));
    }
    mod package {
        include!(concat!(env!("OUT_DIR"), "/pulumipackage.rs"));
    }
}