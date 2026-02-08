pub mod sf {
    pub mod firehose {
        pub mod v2 {
            tonic::include_proto!("sf.firehose.v2");
        }
    }
    pub mod ethereum {
        pub mod r#type {
            pub mod v2 {
                tonic::include_proto!("sf.ethereum.r#type.v2");
            }
        }
    }
}
