use crate::ShardManagerContainer;
use serenity::{client::bridge::gateway::ShardId, prelude::Context};

type LatencyList = Vec<(ShardId, Option<u128>)>;

pub async fn latency_list(ctx: &Context) -> LatencyList {
    let data_read = ctx.data.read().await;
    let shard_manager = data_read.get::<ShardManagerContainer>().unwrap();

    let manager = shard_manager.lock().await;
    let runners = manager.runners.lock().await;

    let mut latencies: LatencyList = vec![];

    for (ids, runner) in runners.iter() {
        if let Some(l) = runner.latency {
            latencies.push((ids.clone(), Some(l.as_millis())));
        } else {
            latencies.push((ids.clone(), None));
        }
    }

    latencies
}

pub async fn average_latency(ctx: &Context) -> Option<u128> {
    let mut total: u128 = 0;

    let latencies = latency_list(ctx).await;

    let len = latencies.len() as u128;

    if len == 0 {
        return None;
    }

    for (_, latency) in &latencies {
        if let Some(l) = latency {
            total += l;
        }
    }

    Some(total / len)
}

pub async fn shard_latency(ctx: &Context) -> Option<u128> {
    let data_read = ctx.data.read().await;
    let shard_manager = data_read.get::<ShardManagerContainer>().unwrap();

    let manager = shard_manager.lock().await;
    let runners = manager.runners.lock().await;

    let runner = runners.get(&ShardId(ctx.shard_id)).unwrap();

    if let Some(duration) = runner.latency {
        Some(duration.as_millis())
    } else {
        None
    }
}
