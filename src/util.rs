use connectionpool::ConnectionPool;
use serenity::client::Context;
use serenity::model::id::ChannelId;

pub fn get_pool(ctx: &Context) -> ConnectionPool {
    let mut data = ctx.data.lock();
    data.get_mut::<ConnectionPool>().unwrap().clone()
}

pub fn digits(mut number: i64) -> usize {
    let mut digits = 0;
    while number != 0 {
        number /= 10;
        digits += 1;
    }
    return digits;
}

pub fn print_or_log_error(message: &str, channel_id: &ChannelId) {
    if let Err(e) = channel_id.say(message) {
        error!("Failed to send message: {}", e);
    }
}