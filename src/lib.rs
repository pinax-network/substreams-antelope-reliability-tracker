
mod pb;

use substreams_antelope::Block;
use substreams_sink_prometheus::{PrometheusOperations, Counter};
use std::collections::HashMap;
use substreams::errors::Error;
use substreams_sink_kv::pb::sf::substreams::sink::kv::v1::KvOperations;
use pb::antelope::antelope_block_meta::v1::AntelopeBlockMeta;
use substreams::log;
use substreams::proto;

#[substreams::handlers::map]
pub fn prom_out(block: Block) -> Result<PrometheusOperations, Error> {
    
    let mut prom_ops = PrometheusOperations::default();

    let active_producer = block.clone().header.unwrap().producer;
    let labels = HashMap::from([("producer".to_string(), active_producer.clone())]);
    prom_ops.push(Counter::from("blocks_produced").with(labels.clone()).inc());

    for producer in block.clone().active_schedule_v2.unwrap().producers {
        let label = HashMap::from([("producer".to_string() , producer.account_name.clone())]);
        prom_ops.push(Counter::from("active_schedule").with(label.clone()).inc());
    }

    prom_ops.push(Counter::from("all_blocks_produced").with(labels.clone()).inc());
    Ok(prom_ops)
}


#[substreams::handlers::map]
pub fn map_block(block: Block) -> Result<AntelopeBlockMeta, Error> {
    let timestamp = block.clone().header.unwrap().timestamp.unwrap();
    let producer = block.clone().header.unwrap().producer;
    let hash = block.clone().id;
    let current_schedule = block.clone().active_schedule_v2.unwrap();

    let mut producers_list: Vec<String> = Vec::new();
    for producer in current_schedule.producers {
        log::info!("Producer: {}", producer.account_name);
        producers_list.push(producer.account_name);
    }

    Ok(AntelopeBlockMeta {
        producer: producer,
        hash: hash,
        current_schedule: producers_list,
        timestamp: Some(timestamp),
    })

}

#[substreams::handlers::map]
pub fn kv_out(block: Block) -> Result<KvOperations, Error> {
    let mut kv_ops: KvOperations = Default::default();
 
    let timestamp = block.clone().header.unwrap().timestamp.unwrap();
    let producer = block.clone().header.unwrap().producer;
    let hash = block.clone().id;
    let current_schedule = block.clone().active_schedule_v2.unwrap();

    let mut producers_list: Vec<String> = Vec::new();
    for producer in current_schedule.producers {
        log::info!("Producer: {}", producer.account_name);
        producers_list.push(producer.account_name);
    }

    let key = format!("seconds: {}, nanos: {}", timestamp.seconds, timestamp.nanos);
    let value = proto::encode(&AntelopeBlockMeta {
        producer: producer,
        hash: hash,
        current_schedule: producers_list,
        timestamp: Some(timestamp),
    }).unwrap();

    kv_ops.push_new(key, value, 1);
    Ok(kv_ops)
}