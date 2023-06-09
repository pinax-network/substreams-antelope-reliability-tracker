use substreams::errors::Error;
use substreams_antelope::Block;

use crate::substreams_antelope_reliability_tracker::MissingBlockCount;

#[substreams::handlers::map]
fn map_block_time(block: Block) -> Result<MissingBlockCount, Error> {

    Ok(MissingBlockCount {
        timestamp: (block.clone().header.unwrap().timestamp.unwrap().seconds) as f64,
        hash: block.clone().id,
    })
}
