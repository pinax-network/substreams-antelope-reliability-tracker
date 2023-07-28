
mod pb;

use substreams_antelope::Block;
use substreams_sink_prometheus::{PrometheusOperations, Counter};
use std::collections::HashMap;
use substreams::errors::Error;
use substreams_sink_kv::pb::sf::substreams::sink::kv::v1::KvOperations;
use pb::antelope::antelope_block_meta::v1::AntelopeBlockMeta;
use substreams::proto;
use substreams::log;

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
    let timestamp = block.header.as_ref().and_then(|header| header.timestamp.as_ref()).unwrap().to_string();
    let producer = block.header.as_ref().map(|header| header.producer.clone()).unwrap();
    let hash = block.id.clone();
    let current_schedule = &block.active_schedule_v2.unwrap().producers;

    let producers_list: Vec<String> = current_schedule.iter().map(|producer| producer.account_name.clone()).collect();

    Ok(AntelopeBlockMeta {
        producer,
        hash,
        current_schedule: producers_list,
        timestamp,
    })
}

#[substreams::handlers::map]
pub fn kv_out(block: Block) -> Result<KvOperations, Error> {
    let timestamp = block.header.as_ref().unwrap().timestamp.clone().as_ref().unwrap().to_string();
    let producer = block.header.as_ref().unwrap().producer.to_string();
    let hash = block.id.to_string();
    let current_schedule = &block.active_schedule_v2.unwrap().producers;

    let producers_list: Vec<String> = current_schedule.iter().map(|producer| producer.account_name.clone()).collect();

    let value = proto::encode(&AntelopeBlockMeta {
        producer,
        hash,
        current_schedule: producers_list,
        timestamp: timestamp.clone(),
    }).unwrap();

    let mut kv_ops = KvOperations::default();
    kv_ops.push_new(format!("date: {}", date_to_sortable_string(&timestamp)), value, 1);
    Ok(kv_ops)
}

fn date_to_sortable_string(date_str: &str) -> String {
    // Split the date string into components
    let (date_part, time_part) = date_str.split_at(10);
    let time_part = &time_part[1..time_part.len() - 1];

    // Split the time part into seconds and milliseconds
    let mut parts = time_part.splitn(2, '.');

    // Pad seconds with zeros to fixed length
    let padded_seconds_part = format!("{:<02}", parts.next().unwrap());

    // Pad milliseconds with zeros to fixed length
    let padded_milliseconds_part = parts.next().unwrap_or("000");
    let padded_milliseconds_part = format!("{:<03}", padded_milliseconds_part);

    // Combine the padded components and return the sortable string
    format!("{}T{}.{}Z", date_part, padded_seconds_part, padded_milliseconds_part)
}