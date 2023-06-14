use substreams::errors::Error;
use substreams_antelope::Block;

use crate::substreams_antelope_reliability_tracker::MissingBlockCount;

#[substreams::handlers::map]
fn map_block_time(block: Block) -> Result<MissingBlockCount, Error> {

    let mut producers_list: Vec<String> = Vec::new();

    let active_schedule = block.clone().active_schedule_v2;
    for producer in active_schedule.unwrap().producers {
        producers_list.push(producer.account_name.clone());
    }

    let curr_producer = block.clone().header.unwrap().producer;

    Ok(MissingBlockCount {
        timestamp: block.clone().header.unwrap().timestamp.unwrap().to_string(),
        hash: block.clone().id,
        producers: producers_list,
        current_producer: curr_producer,
    })
}
