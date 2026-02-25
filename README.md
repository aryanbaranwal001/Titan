# Titan

Titan is a blazingly fast, high thoroughput EVM compatible blockchain indexer

## Road Map

### Phase 1

- [x] setup cargo workspaces
- [x] setup required dependencies
- [x] download required .proto files
- [x] implement build.rs script to compile .proto schemas into rust types
- [x] add settings crate for env and blockchain index configuration
- [x] export build.rs generated code from proto-build module
- [x] add bearer_token function
- [x] add build_client function
- [x] add stream_blocks function
- [x] add docs for all functions in ingestor crate
- [x] add store_blocks function to store blocks in raw binary format from ingestor
- [x] add stream_blocks_mock which streams mock blocks from data stored using store_blocks

### Phase 2

- [x] create default.toml with fields for selective data filtering
- [x] map the Block struct (generated via prost) and include all its fields in default.toml
- [x] parse default.toml into rust structs using serde
- [x] map the Block struct into ExtractedBlock struct capable of containing all the block data
- [x] add extraction logic for ExtractedBlock and nested structs
- [x] implement Display for ExtractedBlock and nested struct for easier debugging
