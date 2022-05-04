use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};

use crate::util::latency::shard_latency;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    if let Some(l) = shard_latency(ctx).await {
        msg.reply(ctx, &format!("The shard latency is {:2}ms", l))
            .await?;
    } else {
        msg.reply(ctx, "Unknown latency D:").await?;
    }

    Ok(())
}

#[command]
async fn echo(ctx: &Context, msg: &Message) -> CommandResult {
    if let Some(s) = msg.content_safe(ctx.cache.as_ref()).strip_prefix("~echo") {
        msg.reply(ctx, s.trim()).await?;
    }

    Ok(())
}
