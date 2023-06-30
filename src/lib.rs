/*#[path = "pb/sf.antelope.missing_block_count.v1.rs"]
#[allow(dead_code)]
pub mod substreams_antelope_reliability_tracker;
pub use self::substreams_antelope_reliability_tracker::*;*/

mod sinks;


use substreams_antelope::Block;
use substreams_sink_prometheus::{PrometheusOperations, Counter};
use std::{collections::HashMap};
use substreams::errors::Error;
use substreams_sink_kv::pb::sf::substreams::sink::kv::v1::KvOperations;
use substreams::log;
use chrono::{DateTime, NaiveDateTime, Utc};

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

    let all_blocks_label = HashMap::from([("producer".to_string(), "all".to_string())]);
    prom_ops.push(Counter::from("all_blocks_produced").with(all_blocks_label.clone()).inc());
    Ok(prom_ops)
}
 
#[substreams::handlers::map]
pub fn kv_out(block: Block) -> Result<KvOperations, Error> {
    let mut kv_ops: KvOperations = Default::default();

    let timestamp = block.clone().header.unwrap().timestamp.unwrap();
    let producer = block.clone().header.unwrap().producer;
    let hash = block.clone().id;
    
    if let Some(naive_dt) = NaiveDateTime::from_timestamp_opt(timestamp.seconds, timestamp.nanos as u32) {
        let datetime_utc: DateTime<Utc> = DateTime::from_utc(naive_dt, Utc);
        let datetime_str = datetime_utc.format("%Y-%m-%d %H:%M:%S%.3f").to_string();

        let key = format!("date: {}, producer: {}", datetime_str, producer);
    
        kv_ops.push_new(key, hash, 1);
    } else {
        log::info!("Timestamp is invalid");
    }

    Ok(kv_ops)
}