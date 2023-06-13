use substreams::errors::Error;
use substreams_antelope::Block;

use crate::substreams_antelope_reliability_tracker::MissingBlockCount;

#[substreams::handlers::map]
fn map_block_time(block: Block) -> Result<MissingBlockCount, Error> {

    let mut producers_list: Vec<String> = Vec::new();

    let active_schedule = block.clone().active_schedule_v2;
    for (counter, producer) in active_schedule.unwrap().producers.iter().enumerate() {
        producers_list.push(format!("{}: {}", counter + 1, producer.account_name));
    }

    let curr_producer = block.clone().header.unwrap().producer;

    Ok(MissingBlockCount {
        timestamp: block.clone().header.unwrap().timestamp.unwrap().to_string(),
        hash: block.clone().id,
        producers: producers_list,
        current_producer: curr_producer,
    })
}
