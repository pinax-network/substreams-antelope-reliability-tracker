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

    let producer = map_missing_block_count.clone().current_producer;

    let labels = HashMap::from([("current_producer".to_string(), producer.clone()),
     ("timestamp".to_string(), map_missing_block_count.clone().timestamp.clone()),
     ("hash".to_string(), map_missing_block_count.clone().hash.clone()),
     ("producers".to_string(), map_missing_block_count.clone().producers.join(", "))]);

    if producer == "eos42freedom".to_string() {
        prom_ops.push(Counter::from("block_produced").with(labels.clone()).inc());
    }
    Ok(prom_ops)
}
