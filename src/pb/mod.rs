// @generated
pub mod google {
    // @@protoc_insertion_point(attribute:google.protobuf)
    pub mod protobuf {
        include!("google.protobuf.rs");
        // @@protoc_insertion_point(google.protobuf)
    }
}
pub mod pinax {
    pub mod substreams {
        pub mod sink {
            pub mod prometheus {
                // @@protoc_insertion_point(attribute:pinax.substreams.sink.prometheus.v1)
                pub mod v1 {
                    include!("pinax.substreams.sink.prometheus.v1.rs");
                    // @@protoc_insertion_point(pinax.substreams.sink.prometheus.v1)
                }
            }
        }
    }
}
pub mod sf {
    pub mod antelope {
        pub mod missing_block_count {
            // @@protoc_insertion_point(attribute:sf.antelope.missing_block_count.v1)
            pub mod v1 {
                include!("sf.antelope.missing_block_count.v1.rs");
                // @@protoc_insertion_point(sf.antelope.missing_block_count.v1)
            }
        }
    }
    pub mod substreams {
        pub mod rpc {
            // @@protoc_insertion_point(attribute:sf.substreams.rpc.v2)
            pub mod v2 {
                include!("sf.substreams.rpc.v2.rs");
                // @@protoc_insertion_point(sf.substreams.rpc.v2)
            }
        }
        // @@protoc_insertion_point(attribute:sf.substreams.v1)
        pub mod v1 {
            include!("sf.substreams.v1.rs");
            // @@protoc_insertion_point(sf.substreams.v1)
        }
    }
}
