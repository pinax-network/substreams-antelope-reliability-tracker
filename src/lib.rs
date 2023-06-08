mod pb;

use substreams::prelude::*;
use substreams::errors::Error;
use substreams_antelope::Block;

use pb::sf::antelope::missing_block_count::v1::MissingBlockCount;

static mut MISSING_BLOCK_COUNT: u64 = None;

#[substreams::handlers::map]
fn map_missing_block_count(block: Block) -> Result<MissingBlockCount, Error> {

    let mut count;
    
    unsafe {
        MISSING_BLOCK_COUNT += 1;
        count = MISSING_BLOCK_COUNT;
    }
    

    Ok(MissingBlockCount {
        timestamp: block.clone().header.unwrap().timestamp.unwrap().to_string(),
        num_missing_blocks: count,
    })
}
