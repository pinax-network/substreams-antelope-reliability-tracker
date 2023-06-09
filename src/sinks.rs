use substreams::log;
use substreams_sink_prometheus::{PrometheusOperations, Gauge};
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

    let labels = HashMap::from([("hash".to_string(), hash)]);

    let mut gauge = Gauge::from("block_time").with(labels.clone());

    prom_ops.push(gauge.set(timestamp));
    Ok(prom_ops)
}
