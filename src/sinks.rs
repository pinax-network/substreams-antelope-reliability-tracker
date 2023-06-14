use substreams_antelope::Block;
use substreams_sink_prometheus::{PrometheusOperations, Counter};
use std::collections::HashMap;
use substreams::errors::Error;


#[substreams::handlers::map]
pub fn prom_out(block: Block) -> Result<PrometheusOperations, Error> {
    
    let mut prom_ops = PrometheusOperations::default();

    let timestamp = block.clone().header.unwrap().timestamp.unwrap().to_string();
    let hash = block.clone().id;

    let mut producers_list: Vec<String> = Vec::new();
    for producer in block.clone().active_schedule_v2.unwrap().producers {
        producers_list.push(producer.account_name.clone());
    }

    let curr_producer = block.clone().header.unwrap().producer;

    let labels = HashMap::from([("producer".to_string(), curr_producer.clone()), 
                                ("timestamp".to_string(), timestamp.clone()), 
                                ("hash".to_string(), hash.clone()),
                                ("active_schedule_producers".to_string(), producers_list.join(", "))]);
    prom_ops.push(Counter::from("blocks_produced").with(labels.clone()).inc());

    Ok(prom_ops)
}
