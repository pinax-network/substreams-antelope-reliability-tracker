use prost_types::Timestamp;
use substreams::log;
use substreams_sink_prometheus::{PrometheusOperations, Gauge, Counter};
use crate::substreams_antelope_reliability_tracker::MissingBlockCount;
use std::collections::HashMap;
use substreams::errors::Error;


#[substreams::handlers::map]
pub fn prom_out(
    map_missing_block_count: MissingBlockCount,
) -> Result<PrometheusOperations, Error> {
    
    let mut prom_ops = PrometheusOperations::default();

    let timestamp = map_missing_block_count.clone().timestamp;
    let hash = map_missing_block_count.clone().hash;
    let current_producer = map_missing_block_count.clone().current_producer;
    let active_schedule = map_missing_block_count.clone().producers;

    
    let labels = HashMap::from([("producer".to_string(), current_producer.clone()),
        ("timestamp".to_string(), timestamp.clone()),
        ("hash".to_string(), hash.clone()),
        ("active_shedule".to_string(), active_schedule.join(", "))]);
    prom_ops.push(Counter::from("blocks_produced").with(labels.clone()).inc());

    Ok(prom_ops)
}
