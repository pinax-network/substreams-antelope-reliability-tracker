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
    
    let current_producer = map_missing_block_count.clone().current_producer;

    let labels = HashMap::from([("producer".to_string(), current_producer.clone())]);
    prom_ops.push(Counter::from("blocks_produced").with(labels.clone()).inc());

    Ok(prom_ops)
}
