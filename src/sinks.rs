use substreams_antelope::Block;
use substreams_sink_prometheus::{PrometheusOperations, Counter};
use std::collections::HashMap;
use substreams::errors::Error;


#[substreams::handlers::map]
pub fn prom_out(block: Block) -> Result<PrometheusOperations, Error> {
    
    let mut prom_ops = PrometheusOperations::default();

    let active_producer = block.clone().header.unwrap().producer;
    let labels = HashMap::from([("active producer".to_string(), active_producer.clone())]);
    prom_ops.push(Counter::from("blocks_produced").with(labels.clone()).inc());

    for producer in block.clone().active_schedule_v2.unwrap().producers {
        let label = HashMap::from([("producer".to_string() , producer.account_name.clone())]);
        prom_ops.push(Counter::from("active_schedule").with(label.clone()).inc());
    }

    prom_ops.push(Counter::from("all_blocks_produced").inc());
    Ok(prom_ops)
}
